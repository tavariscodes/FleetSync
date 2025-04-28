# 🚛 FleetSync

**FleetSync** is a lightweight Rust prototype designed to simulate **real-time fleet operations management** for trucks and cargo logistics.  
This project explores **stateful command-driven devices**, **multi-role coordination**, and **event logging**, serving as a practical playground for **Rust systems programming**, **embedded systems concepts**, and **real-world logistics modeling**.

---

## 🛠 Project Intent

FleetSync was created to:
- **Rebuild my fluency in Rust** through hands-on systems modeling.
- **Simulate embedded device concepts** (state machines, command processing) in a simple environment.
- **Practice collaborative, multi-user system flows** relevant to real-time operational platforms (like mission planning, fleet ops, logistics).
- **Balance structured prototype development** with room for playful tinkering and iteration.

Rather than just a CRUD app, FleetSync models **autonomous truck units** that behave like embedded smart devices — reacting to commands, reporting status, and progressing through delivery missions.

---

## ⚙️ Core Concepts

- **Truck Unit State Machines**: Each truck operates as a simulated "smart device" with internal state transitions (Idle → Assigned → En Route → Delivered).
- **Command-Driven Interactions**: Dispatchers and drivers send structured commands to control truck units (assign cargo, start route, update status).
- **Event Logging**: Every system interaction (assignment, acceptance, status update) is timestamped and recorded for auditing.
- **Multi-Role CLI Interface**: Two main roles — **Dispatcher** and **Driver** — with distinct capabilities and views.
- **Embedded Thinking, No Hardware**: All embedded concepts (device control, message parsing, state machines) are simulated in-memory.

---

## 📋 Project Roadmap

### Phase 1: Core Prototype (MVP)

- [x] CLI menu system for Dispatcher and Driver roles.
- [ ] Define TruckUnit, Cargo, Route, and Event models.
- [ ] Implement state machine for truck units.
- [ ] Implement dispatcher commands (assign cargo, view trucks, view cargo).
- [ ] Implement driver commands (accept/reject cargo, update route status).
- [ ] Record events with timestamps.
- [ ] Enforce cargo and truck assignment rules (no double-assignments).

### Phase 2: Embedded Systems Extensions (Optional)

- [ ] Simulate periodic "heartbeat" telemetry updates from trucks.
- [ ] Implement basic event loop to tick truck units and process status reports.
- [ ] Expand command types (e.g., refuel truck, reroute delivery).
- [ ] Model fuel consumption and truck maintenance events.

### Phase 3: Networking & Real-Time Collaboration (Stretch Goals)

- [ ] Add WebSocket server to allow real multi-client connections (dispatcher ↔ drivers).
- [ ] Synchronize truck unit state across networked clients.
- [ ] Persist event logs and system state to disk.
- [ ] Implement simple mission report exports (JSON/CSV).

---

## ✨ Why FleetSync?

FleetSync exists to bridge three worlds:
- **Rust mastery** (ownership, enums, traits, concurrency patterns)
- **Embedded device thinking** (state machines, low-level command processing)
- **Operational systems modeling** (fleet coordination, logistics planning)

---

## 🚀 Quickstart (Coming Soon)

```bash
# Clone the project
git clone https://github.com/tavariscodes/fleetsync.git
cd fleetsync

# Build and run
cargo run
