
Goal: a simulation environment that couples (a) network/RF simulation with realistic jamming and propagation to (b) a minimal vehicle-dynamics sim, so we can benchmark FHSS hop-rate, Zenoh/CRDT/MLS, routing against EW under realistic mobility, other methods of EW warfare and switching to Satelite communication.

Secure, decentralized radio communication and state-sharing architectures for a small swarm of ground vehicles (under 5 nodes). Tested in simulation/emulation.

Our development is operating under a military/government authorization, so not limited from the legal standpoint.

Environment: Open fields and forests (requiring solid penetration characteristics).
Role: This system will serve as a resilient, peer-to-peer backup communication network in the event that the primary satellite uplink (Starlink) to the central command node fails.
Threat Model: The network must be highly resilient to Electronic Warfare (EW), specifically active RF jamming, and must prevent eavesdropping via robust encryption. Decentralization is mandatory; there can be no single point of failure or centralized key manager.

Technical & Physical Constraints:
- Payload Priority: 1. Low-bandwidth critical state sharing (telemetry, coordinates, basic commands). 2. High-bandwidth data (if link quality permits).
- SWaP Limitations (Size, Weight, and Power): The vehicles have very limited payload space for antennas and a restricted power budget. Hardware must be ultra-compact, and antennas must be small or low-profile. The solution cannot rely on high-wattage transmission to overcome jamming.
- Hardware Baseline: Intel N100 (or equivalent low-power x86 compute).
- Software/Middleware: Agnostic. Performance, security, and low latency over lossy links are prioritized over framework compliance, though compatibility with ROS 2 (e.g., using Zenoh or lightweight DDS) is a minor plus.

Package manegement: pixi

Documentation: see [docs/](docs/README.md) — architecture, RF/EW models, benchmarking methodology, roadmap, and decision records (ADRs).