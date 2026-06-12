//! net_com_zen node agent (M2 stage 1: zenoh + automerge telemetry sharing).
//!
//! Each agent owns one entry in a shared automerge document. Every tick it
//! broadcasts the incremental changes since the last save (`ncz/state/<id>/inc`,
//! small and gossip-redundant: merged peer changes ride along); every
//! FULL_EVERY-th tick it broadcasts a full snapshot (`.../full`) so late
//! joiners and nodes that missed increments resynchronize. Received payloads
//! are merged (CRDT). Every publish/receive is logged as a JSONL metrics event
//! for the harness (age-of-information is computed offline from timestamps).

use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use automerge::transaction::Transactable;
use automerge::{AutoCommit, ObjType, ReadDoc};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Node id (e.g. v1)
    #[arg(long)]
    id: String,
    /// Zenoh listen endpoint, e.g. tcp/10.99.0.1:7447
    #[arg(long)]
    listen: String,
    /// Zenoh connect endpoints of the other peers (repeatable)
    #[arg(long)]
    connect: Vec<String>,
    /// Path for JSONL metrics output
    #[arg(long)]
    metrics: PathBuf,
    /// Telemetry publish period in ms
    #[arg(long, default_value_t = 500)]
    period_ms: u64,
    /// Run duration in seconds
    #[arg(long, default_value_t = 10.0)]
    duration_s: f64,
}

const FULL_EVERY: u64 = 20;

fn now_us() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64
}

struct Metrics {
    file: std::fs::File,
}

impl Metrics {
    fn open(path: &PathBuf) -> Result<Self> {
        Ok(Self { file: std::fs::File::create(path)? })
    }
    fn log(&mut self, value: serde_json::Value) {
        let _ = writeln!(self.file, "{value}");
    }
}

/// Update this node's entry in the shared doc: nodes/<id> = {seq, ts_us}.
fn update_own_entry(doc: &mut AutoCommit, id: &str, seq: u64) -> Result<()> {
    let nodes = match doc.get(automerge::ROOT, "nodes")? {
        Some((_, obj)) => obj,
        None => doc.put_object(automerge::ROOT, "nodes", ObjType::Map)?,
    };
    let me = match doc.get(&nodes, id)? {
        Some((_, obj)) => obj,
        None => doc.put_object(&nodes, id, ObjType::Map)?,
    };
    doc.put(&me, "seq", seq as i64)?;
    doc.put(&me, "ts_us", now_us() as i64)?;
    Ok(())
}

/// Read peer entries: (peer_id, seq, ts_us) for AoI bookkeeping.
fn peer_entries(doc: &AutoCommit) -> Vec<(String, i64, i64)> {
    let mut out = Vec::new();
    let Ok(Some((_, nodes))) = doc.get(automerge::ROOT, "nodes") else {
        return out;
    };
    for key in doc.keys(&nodes) {
        if let Ok(Some((_, entry))) = doc.get(&nodes, key.as_str()) {
            let seq = match doc.get(&entry, "seq") {
                Ok(Some((automerge::Value::Scalar(s), _))) => s.to_i64().unwrap_or(-1),
                _ => -1,
            };
            let ts = match doc.get(&entry, "ts_us") {
                Ok(Some((automerge::Value::Scalar(s), _))) => s.to_i64().unwrap_or(-1),
                _ => -1,
            };
            out.push((key, seq, ts));
        }
    }
    out
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut metrics = Metrics::open(&args.metrics)?;

    let mut zconf = zenoh::Config::default();
    zconf.insert_json5("mode", "\"peer\"").unwrap();
    zconf
        .insert_json5("listen/endpoints", &serde_json::json!([args.listen]).to_string())
        .unwrap();
    zconf
        .insert_json5("connect/endpoints", &serde_json::json!(args.connect).to_string())
        .unwrap();
    zconf.insert_json5("scouting/multicast/enabled", "false").unwrap();

    let session = zenoh::open(zconf).await.map_err(|e| anyhow::anyhow!("{e}"))?;
    let subscriber = session
        .declare_subscriber("ncz/state/*/*")
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut doc = AutoCommit::new();
    update_own_entry(&mut doc, &args.id, 0)?;

    metrics.log(serde_json::json!({
        "type": "start", "id": args.id, "ts_us": now_us()
    }));

    let deadline = tokio::time::Instant::now() + Duration::from_secs_f64(args.duration_s);
    let mut ticker = tokio::time::interval(Duration::from_millis(args.period_ms));
    let mut peer_probe = tokio::time::interval(Duration::from_secs(1));
    let mut seq: u64 = 0;

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                seq += 1;
                update_own_entry(&mut doc, &args.id, seq)?;
                let (kind, bytes) = if seq % FULL_EVERY == 0 {
                    ("full", doc.save())
                } else {
                    ("inc", doc.save_incremental())
                };
                if bytes.is_empty() {
                    continue;
                }
                session.put(format!("ncz/state/{}/{}", args.id, kind), bytes.clone())
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"))?;
                metrics.log(serde_json::json!({
                    "type": "pub", "id": args.id, "seq": seq, "kind": kind,
                    "ts_us": now_us(), "bytes": bytes.len()
                }));
            }
            sample = subscriber.recv_async() => {
                let sample = sample.map_err(|e| anyhow::anyhow!("subscriber closed: {e}"))?;
                let mut segs = sample.key_expr().as_str().rsplit('/');
                let kind = segs.next().unwrap_or("?").to_string();
                let from = segs.next().unwrap_or("?").to_string();
                if from == args.id {
                    continue;
                }
                let payload = sample.payload().to_bytes().to_vec();
                let recv_us = now_us();
                let merged = if kind == "full" {
                    AutoCommit::load(&payload)
                        .and_then(|mut other| doc.merge(&mut other).map(|_| ()))
                        .map_err(|e| e.to_string())
                } else {
                    doc.load_incremental(&payload).map(|_| ()).map_err(|e| e.to_string())
                };
                match merged {
                    Ok(()) => {
                        // mark merged changes as saved so our next inc carries
                        // only our own change (no gossip echo; mesh is full)
                        let _ = doc.save_incremental();
                        for (peer, pseq, pts) in peer_entries(&doc) {
                            if peer == from {
                                metrics.log(serde_json::json!({
                                    "type": "recv", "id": args.id, "from": from,
                                    "kind": kind, "peer_seq": pseq, "peer_ts_us": pts,
                                    "ts_us": recv_us, "bytes": payload.len()
                                }));
                            }
                        }
                    }
                    Err(e) => {
                        // missing deps after loss/late join: next full snapshot heals
                        metrics.log(serde_json::json!({
                            "type": "decode_error", "id": args.id, "from": from,
                            "kind": kind, "ts_us": recv_us, "error": e
                        }));
                    }
                }
            }
            _ = peer_probe.tick() => {
                let peers: Vec<String> = session.info().peers_zid().await
                    .map(|z| z.to_string()).collect();
                metrics.log(serde_json::json!({
                    "type": "zenoh_peers", "id": args.id, "ts_us": now_us(),
                    "n": peers.len(), "zids": peers
                }));
            }
            _ = tokio::time::sleep_until(deadline) => {
                break;
            }
        }
    }

    // final state summary: what this node knows about everyone
    let known: Vec<_> = peer_entries(&doc)
        .into_iter()
        .map(|(p, s, t)| serde_json::json!({"peer": p, "seq": s, "ts_us": t}))
        .collect();
    metrics.log(serde_json::json!({
        "type": "final_state", "id": args.id, "ts_us": now_us(), "known": known
    }));
    session.close().await.map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(())
}
