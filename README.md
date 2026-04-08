# StellarScope

**The interpretation layer for Soroban — a block explorer that translates Stellar transactions into human-readable activity.**

---

## Overview

StellarScope is a Soroban-native block explorer built for the next phase of the Stellar ecosystem. It provides a unified, human-readable interface for both classic Stellar operations and smart contract interactions.

As Soroban introduces programmable finance to Stellar, existing tools expose low-level data but lack meaningful interpretation. StellarScope bridges this gap by decoding raw blockchain activity into clear, understandable financial actions.

---

## The Problem

Existing explorers such as StellarExpert provide strong support for classic Stellar operations (payments, trustlines, offers). However, Soroban smart contract activity remains difficult to interpret.

While Soroban data is accessible via RPC and explorer interfaces:

* Contract invocations are displayed as low-level XDR or minimally decoded structures
* Event logs lack semantic meaning
* There is no intent-based interpretation (e.g. swaps, lending, liquidations)
* No unified view combines classic operations and Soroban activity

Protocols like Soroswap, Blend, and Aquarius generate meaningful on-chain activity, but this activity is not surfaced in a human-readable or aggregated way.

This creates a major usability gap for:

* Developers debugging contracts
* Users tracking transactions
* Analysts understanding ecosystem activity

---

## The Solution

StellarScope introduces an **intent decoding layer** that transforms raw blockchain data into human-readable activity.

Instead of:

```
invoke_contract → fn: swap → args: [...]
```

Users see:

> “Swapped 500 USDC for 3,200 XLM on Soroswap”

This enables:

* Clear transaction understanding
* Faster debugging and development
* Visibility into DeFi activity

---

## Features

### 1. Unified Transaction View

* Combines classic Stellar operations and Soroban contract calls
* Displays a single chronological timeline
* Human-readable transaction summaries

---

### 2. Contract Inspector

* Function signatures (ABI-like decoding)
* Event logs (decoded and labeled)
* Storage entries
* Invocation history
* Verified source links (where available)

---

### 3. Token Tracker

* Supports both classic assets and Soroban tokens
* Holder distribution
* Transfer history
* Liquidity insights
* Price data via DEX integrations

---

### 4. DeFi Activity Feed

* Real-time ecosystem activity
* Soroswap trades
* Blend lending events
* Aquarius liquidity updates

---

## Core Innovation

StellarScope introduces a **protocol-aware decoding engine** that:

* Interprets contract calls using ABI-like structures
* Maps low-level events to high-level financial actions
* Produces intent-based summaries of transactions

This transforms Stellar data from:

> Raw XDR → Human-readable financial activity

---

## Architecture

```
                ┌────────────────────┐
                │  Stellar RPC /     │
                │  Horizon API       │
                └─────────┬──────────┘
                          │
                          ▼
                ┌────────────────────┐
                │  Ingestion Layer   │  (Rust)
                │  - Ledger parsing  │
                │  - XDR decoding    │
                └─────────┬──────────┘
                          │
                          ▼
                ┌────────────────────┐
                │  Data Layer        │  (PostgreSQL)
                │  - Indexed txns    │
                │  - Events          │
                │  - Contracts       │
                └─────────┬──────────┘
                          │
                          ▼
                ┌────────────────────┐
                │  API Layer         │
                │  - Query engine    │
                │  - Aggregations    │
                └─────────┬──────────┘
                          │
                          ▼
                ┌────────────────────┐
                │  Frontend          │  (Next.js)
                │  - Explorer UI     │
                │  - Dashboards      │
                └────────────────────┘
```

---

## Tech Stack

* **Rust** — ingestion and indexing
* **PostgreSQL** — structured blockchain storage
* **Stellar RPC + Horizon API** — data sources
* **Next.js** — frontend
* **Custom XDR Decoder** — transaction parsing
* **Protocol Adapters** — Soroswap, Blend, Aquarius

---

## Use Cases

### Developers

* Debug smart contract interactions
* Inspect contract behavior and logs
* Verify integrations

### Users

* Understand wallet activity
* Track DeFi transactions
* View clear transaction summaries

### Analysts

* Monitor ecosystem activity
* Analyze protocol usage
* Track liquidity and trading patterns

---

## Roadmap

### Phase 1 — MVP

* Basic Soroban transaction decoding
* Unified address view
* Explorer UI

### Phase 2 — Protocol Awareness

* Protocol adapters (Soroswap, Blend, Aquarius)
* Intent-based decoding
* DeFi activity feed

### Phase 3 — Advanced Features

* Contract verification system
* Token analytics dashboards
* Public API

---

## Grant Alignment

StellarScope aligns with the Stellar Community Fund (SCF) RFP under:

* Infrastructure
* Developer Tooling
* Block Explorer

It directly addresses the lack of a Soroban-native explorer and unlocks visibility into Stellar’s emerging DeFi ecosystem.

---

## Future Vision

StellarScope evolves into:

* The default explorer for Soroban
* A data layer for wallets and dashboards
* A foundation for analytics and developer tools

---

## License

MIT License

---

## Contact

* Email: [your-email@example.com](mailto:your-email@example.com)
* Twitter: @yourhandle
* Website: coming soon

---
