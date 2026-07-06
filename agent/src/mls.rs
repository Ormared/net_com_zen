//! MLS group layer (ADR in docs/architecture.md: deterministic committer =
//! lowest member id creates the group and serves the Welcome; key packages are
//! served via zenoh queryables so the handshake is robust to startup timing).
//!
//! M2 scope: group established once at startup, all state payloads encrypted
//! as MLS application messages. Rekeys/committer failover come with M3/M4.
//!
//! Loss tolerance (EMANE spike follow-up, emane_spike/FINDINGS.md): every
//! query carries a short explicit timeout — zenoh's session default is 10 s,
//! so a single lost query or reply used to stall the handshake for that long
//! and push weak-link nodes past the deadline. A failed attempt now costs at
//! most QUERY_TIMEOUT plus a capped backoff gap, and the committer fetches all
//! key packages concurrently so per-member losses overlap instead of adding up.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use openmls::prelude::*;
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;
use tls_codec::{Deserialize as TlsDeserialize, Serialize as TlsSerialize};
use zenoh::Session;

const CIPHERSUITE: Ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;
/// Per-query deadline: long enough for a TCP retransmit or two over a lossy
/// emulated link, short enough that a dead attempt is cheap to abandon.
const QUERY_TIMEOUT: Duration = Duration::from_secs(2);
const RETRY_GAP_MIN: Duration = Duration::from_millis(250);
const RETRY_GAP_MAX: Duration = Duration::from_secs(1);

pub struct MlsLayer {
    provider: OpenMlsRustCrypto,
    signer: SignatureKeyPair,
    group: MlsGroup,
}

impl MlsLayer {
    pub fn epoch(&self) -> u64 {
        self.group.epoch().as_u64()
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let msg = self
            .group
            .create_message(&self.provider, &self.signer, plaintext)
            .map_err(|e| anyhow!("create_message: {e}"))?;
        Ok(msg.tls_serialize_detached()?)
    }

    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let msg = MlsMessageIn::tls_deserialize_exact(ciphertext)?;
        let protocol = msg
            .try_into_protocol_message()
            .map_err(|e| anyhow!("not a protocol message: {e}"))?;
        let processed = self
            .group
            .process_message(&self.provider, protocol)
            .map_err(|e| anyhow!("process_message: {e}"))?;
        match processed.into_content() {
            ProcessedMessageContent::ApplicationMessage(am) => Ok(am.into_bytes()),
            other => bail!("unexpected MLS content: {other:?}"),
        }
    }
}

fn new_identity(id: &str) -> Result<(OpenMlsRustCrypto, SignatureKeyPair, CredentialWithKey)> {
    let provider = OpenMlsRustCrypto::default();
    let credential = BasicCredential::new(id.as_bytes().to_vec());
    let signer = SignatureKeyPair::new(CIPHERSUITE.signature_algorithm())
        .map_err(|e| anyhow!("keygen: {e}"))?;
    signer
        .store(provider.storage())
        .map_err(|e| anyhow!("key store: {e}"))?;
    let cwk = CredentialWithKey {
        credential: credential.into(),
        signature_key: signer.public().into(),
    };
    Ok((provider, signer, cwk))
}

/// Query `key` with short per-attempt timeouts until a reply payload arrives
/// or the deadline passes. `queries` counts attempts (handshake diagnostics).
async fn get_until(session: &Session, key: &str, deadline: tokio::time::Instant,
                   queries: &AtomicU64) -> Result<Vec<u8>> {
    let mut gap = RETRY_GAP_MIN;
    loop {
        let now = tokio::time::Instant::now();
        if now >= deadline {
            bail!("timeout querying {key}");
        }
        queries.fetch_add(1, Ordering::Relaxed);
        if let Ok(replies) = session.get(key).timeout(QUERY_TIMEOUT.min(deadline - now)).await {
            while let Ok(reply) = replies.recv_async().await {
                if let Ok(sample) = reply.result() {
                    return Ok(sample.payload().to_bytes().to_vec());
                }
            }
        }
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        tokio::time::sleep(gap.min(remaining)).await;
        gap = (gap * 2).min(RETRY_GAP_MAX);
    }
}

/// Serve `bytes` forever on a zenoh queryable (key packages / welcome).
async fn serve(session: &Session, key: String, bytes: Vec<u8>) -> Result<()> {
    let queryable = session
        .declare_queryable(key.clone())
        .await
        .map_err(|e| anyhow!("queryable {key}: {e}"))?;
    tokio::spawn(async move {
        while let Ok(query) = queryable.recv_async().await {
            let _ = query.reply(query.key_expr().clone(), bytes.clone()).await;
        }
    });
    Ok(())
}

/// Establish the MLS group. Committer = lexicographically lowest member id.
/// Returns the layer plus the number of zenoh queries issued (retries show up
/// as counts > members-1, a direct read on how lossy the handshake was).
pub async fn setup(session: &Session, id: &str, members: &[String],
                   timeout: Duration) -> Result<(MlsLayer, u64)> {
    let deadline = tokio::time::Instant::now() + timeout;
    let queries = AtomicU64::new(0);
    let (provider, signer, cwk) = new_identity(id)?;
    let committer = members.iter().min().cloned().unwrap_or_default();

    if committer == id {
        // collect everyone's key package (concurrently: weak-link members'
        // retries overlap instead of serializing), create group, serve welcome
        let fetched = futures::future::try_join_all(
            members.iter().filter(|m| *m != id).map(|m| {
                let key = format!("ncz/mls/kp/{m}");
                let queries = &queries;
                async move {
                    get_until(session, &key, deadline, queries)
                        .await
                        .map(|bytes| (m, bytes))
                }
            }),
        )
        .await?;
        let mut key_packages = Vec::new();
        for (m, bytes) in fetched {
            let kp_in = KeyPackageIn::tls_deserialize_exact(&bytes)?;
            let kp = kp_in
                .validate(provider.crypto(), ProtocolVersion::Mls10)
                .map_err(|e| anyhow!("invalid key package from {m}: {e}"))?;
            key_packages.push(kp);
        }
        let config = MlsGroupCreateConfig::builder()
            .ciphersuite(CIPHERSUITE)
            .use_ratchet_tree_extension(true)
            .build();
        let mut group = MlsGroup::new(&provider, &signer, &config, cwk)
            .map_err(|e| anyhow!("group create: {e}"))?;
        let (_commit, welcome, _info) = group
            .add_members(&provider, &signer, &key_packages)
            .map_err(|e| anyhow!("add_members: {e}"))?;
        group
            .merge_pending_commit(&provider)
            .map_err(|e| anyhow!("merge commit: {e}"))?;
        let welcome_bytes = welcome.tls_serialize_detached()?;
        serve(session, "ncz/mls/welcome".into(), welcome_bytes).await?;
        Ok((MlsLayer { provider, signer, group }, queries.into_inner()))
    } else {
        // serve our key package, wait for the welcome, join
        let kp = KeyPackage::builder()
            .build(CIPHERSUITE, &provider, &signer, cwk)
            .map_err(|e| anyhow!("key package: {e}"))?;
        let kp_bytes = kp.key_package().tls_serialize_detached()?;
        serve(session, format!("ncz/mls/kp/{id}"), kp_bytes).await?;
        let welcome_bytes = get_until(session, "ncz/mls/welcome", deadline, &queries).await?;
        let msg = MlsMessageIn::tls_deserialize_exact(&welcome_bytes)?;
        let welcome = match msg.extract() {
            MlsMessageBodyIn::Welcome(w) => w,
            other => bail!("expected welcome, got {other:?}"),
        };
        let join_config = MlsGroupJoinConfig::builder()
            .use_ratchet_tree_extension(true)
            .build();
        let staged = StagedWelcome::new_from_welcome(&provider, &join_config, welcome, None)
            .map_err(|e| anyhow!("staged welcome: {e}"))?;
        let group = staged
            .into_group(&provider)
            .map_err(|e| anyhow!("join group: {e}"))?;
        Ok((MlsLayer { provider, signer, group }, queries.into_inner()))
    }
}
