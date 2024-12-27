# ArenatonEngine

ArenatonEngine is a decentralized betting engine designed for the Arenaton platform. Built using the Arbitrum Stylus SDK, it leverages the performance, scalability, and cost-effectiveness of Rust-based smart contracts on the Arbitrum blockchain. The engine facilitates the creation, management, and settlement of sports betting events with enhanced transparency and user control.

## Features

### Core Functionalities

- **Event Management**: Create, open, close, and settle sports betting events.
- **Flexible Staking**: Support for staking in both Ether (ETH) and the platform's native ATON token.
- **Decentralized Governance**: Controlled through `AccessControl` and `Ownable` modules for role-based permissions.
- **Efficient Gas Usage**: Optimized for reduced transaction costs using the Arbitrum Stylus SDK.

### Event Details

- Players can participate in events by staking tokens.
- Events include features such as:
  - Multiple sports categories.
  - Start and end times.
  - Player-specific stakes and team selections.
  - Automatic payouts to winners.

### Secure and Transparent

- Role-based access control ensures only authorized entities can create or settle events.
- Immutable event logs for transparency.

## Contracts and Components

### 1. **ArenatonEngine**

- Main entrypoint for the Arenaton platform.
- Manages events, stakes, and player participation.
- Implements `Ownable` and `AccessControl` for secure role and ownership management.

### 2. **Player and Event Structures**

- **Player**: Tracks participation, commissions, and levels.
- **Event**: Stores event-specific details, including players, stakes, teams, and results.

### 3. **Interfaces**

- Implements `IATON` for interacting with the ATON token.

## Getting Started

### Prerequisites

- Install Rust and the Stylus SDK (`cargo stylus`).
- Setup a local Arbitrum Stylus development node.

### Compilation

```bash
cargo build --release
