# Firewall

## Roadmap

## Epic 0 – Project Setup & Design

**Goal:** Prepare like a real security tool project.

### Tasks

- Define scope (host firewall, not router)

- Decide supported protocols (start: IPv4, TCP, UDP, ICMP)

- Create repo structure

- Create GitHub Project board

- Write initial README (vision + phases)


### Deliverables

- Clean repo structure

- Design document (`docs/architecture.md`)

- Roadmap in README


---

## Epic 1 – Packet Capture (Passive Mode)

**Goal:** See traffic before touching it.

### Tasks

- Capture packets from an interface

- Print packet metadata:

    - Source IP

    - Destination IP

    - Protocol

    - Ports

- Handle graceful shutdown


### Concepts you’ll learn

- Promiscuous mode

- Packet buffers

- Endianness

- Permissions (`CAP_NET_RAW`)


### Deliverables

- CLI tool: `firewall sniff`

- Logs packets without modifying traffic


**Red team lens:**
“How would malware hide from packet capture?”

---

## Epic 2 – Packet Parsing (L2 → L4)

**Goal:** Understand packet internals deeply.

### Tasks

- Parse:

    - Ethernet

    - IPv4

    - TCP / UDP / ICMP

- Validate checksums

- Handle malformed packets safely


### Concepts

- Header offsets

- Fragmentation

- MTU issues

- Defensive parsing (avoid crashes)


### Deliverables

- Structured packet object

- Debug output (`--verbose`)


**Red team lens:**
“What malformed packets might crash naive parsers?”

---

## Epic 3 – Rule Engine (Stateless Firewall)

**Goal:** Decide what traffic _should_ pass.

### Tasks

- Design rule syntax (YAML or TOML)

- Rule fields:

    - src/dst IP

    - src/dst port

    - protocol

    - action (ALLOW/DROP/LOG)

- Rule evaluation engine

- Default deny vs allow modes


### Example rule

`[[rule]] protocol = "TCP" dst_port = 22 action = "DROP"`

### Deliverables

- Configurable rules

- Rule matching engine

- Unit tests for rule logic


**Red team lens:**
“How can attackers bypass naive rule matching?”

---

## Epic 4 – Packet Blocking (Active Mode)

**Goal:** Actually block traffic.

### Options

- Userspace blocking via:

    - NFQUEUE (recommended)

    - Raw socket manipulation


### Tasks

- Integrate with Netfilter NFQUEUE

- Decide verdicts (ACCEPT / DROP)

- Measure latency impact


### Deliverables

- Firewall enforces rules

- Safe fail-open behavior


**Red team lens:**
“What happens if the firewall crashes?”

---

## Epic 5 – Stateful Inspection

**Goal:** Track connections like real firewalls.

### Tasks

- Track TCP handshake state

- Maintain connection table

- Timeout handling

- Allow related traffic only


### Concepts

- SYN / ACK logic

- Half-open connections

- Resource exhaustion


### Deliverables

- Stateful filtering

- Connection tracking module


**Red team lens:**
“SYN floods, connection exhaustion attacks”

---

## Epic 6 – Logging, Metrics & Alerts

**Goal:** Visibility.

### Tasks

- Structured logging (JSON)

- Log dropped packets

- Rate-limit logs

- Export metrics (Prometheus-style)


### Deliverables

- Log files

- Stats dashboard (optional)

