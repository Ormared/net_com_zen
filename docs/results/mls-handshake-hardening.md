# MLS handshake hardening: loss-tolerant group join

The [EMANE spike](../../emane_spike/FINDINGS.md) found the OpenMLS group handshake
was **not loss-tolerant**: realistic over-the-air latency inflated it ~10× and a
weak foliage-edge node failed to join at all. The cause was in the queryable
exchange, not the crypto: the committer collects each member's key package and
serves a Welcome over zenoh queryables, and every `session.get()` used zenoh's
**default 10 s query timeout**. A single lost query or reply therefore stalled the
whole handshake for 10 s before the retry loop tried again — and within a fixed
30 s deadline that bought only ~3 attempts, so lossy links ran out of budget and
failed.

## The change (`agent/src/mls.rs`)

- **Bounded per-query cost.** Each `get()` now carries an explicit **2 s** timeout
  plus exponential backoff (250 ms → 1 s). A lost query costs ~2 s, not 10 s, so a
  given deadline buys ~5× more attempts.
- **Concurrent key-package fetch.** The committer fetches all members' key
  packages concurrently (`try_join_all`) instead of serially, so slow/lossy
  members' retries overlap rather than summing.
- **Configurable deadline.** `--mls-timeout-s` (default 30 s) — cheap retries mean
  raising it actually converts time into completions instead of longer stalls.
- The `mls_ready` metric now records the `queries` count (retries = a direct read
  on link loss during the handshake).

## A/B under loss (2 netns, veth + netem 10 % loss / 30 ms delay, udp)

Old vs hardened binary, **interleaved per rep** (alternated under one netem
session so both see the same time-varying conditions — netem loss is unseeded, so
separate phases bias the medians), 30 reps each, 30 s deadline:

| binary | reps fully joined | node-handshakes OK | median | p90 | 8–12 s "cliff" incidence |
|---|---|---|---|---|---|
| old | 12 / 30 (40 %) | 29 / 60 (48 %) | 431 ms | **21.0 s** | 10.3 % |
| hardened | **25 / 30 (83 %)** | **53 / 60 (88 %)** | 813 ms | **6.8 s** | **0 %** |

- **Completion roughly doubles**: 12 → 25 reps fully joined; node-level 48 % → 88 %.
- **The 10 s cliff is gone**: the old binary's p90 is 21 s (a query loss eating the
  10 s timeout, sometimes twice); the hardened p90 is 6.8 s, and *zero* handshakes
  land in the 8–12 s dead-timeout band (vs 10.3 % for old).
- **The median rises slightly** (431 → 813 ms) — and that is the mechanism working,
  not a regression: the old binary's low median is survivorship bias over the easy
  links that round-trip on the first query, while it abandons the hard ones. The
  hardened binary spends an extra second or two *completing* those hard links
  instead of failing them. Completing a link slowly beats dropping it — exactly the
  loss tolerance the spike asked for.

## Caveat / honest limits

Handshake time is **highly variable** run-to-run: netem loss is unseeded and zenoh
transport/queryable establishment over lossy UDP is itself stochastic, so single
small batches disagree (a separate 24-rep phase-split run showed the two binaries
near-parity — the interleaving above is what exposes the real difference). The
robust, repeatable signals are the **completion rate** and the **elimination of the
10 s cliff**; treat the median/p90 as indicative, not precise. The residual ~12 %
node failures at 10 % loss are dominated by zenoh session establishment, which this
MLS-layer change does not address — a gossiped/re-announced Welcome (spike
follow-up menu) would be the next lever if higher-loss links must join.
