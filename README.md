# DroneForce Protocol

A Solana smart contract for managing drone mission tasks on-chain. This contract handles the creation, acceptance, and completion of drone missions with associated metadata.

## Overview

The DroneForce Protocol enables:
- Creation of drone mission tasks with detailed metadata
- Acceptance of tasks by drone operators
- Completion of tasks with verifiable mission logs and signatures

## Contract Structure

The smart contract provides three main instructions:

1. **create_task** - Creates a new drone mission task with the following metadata:
   - Location coordinates
   - Task type
   - Altitude
   - Area size
   - Geofencing status
   - Description

2. **accept_task** - Allows an operator to accept a pending task

3. **complete_task** - Marks a task as completed and stores:
   - Arweave transaction ID (for off-chain log storage)
   - Mission log hash
   - Operator signature

## Task States

Tasks can be in one of three states:
- CREATED (0) - Initial state when created
- ACCEPTED (1) - After an operator accepts the task
- COMPLETED (2) - After an operator completes the task

## Setup & Development

### Prerequisites
- Rust and Cargo
- Solana CLI
- Anchor Framework
- Node.js and Yarn (for tests)

### Build Instructions

```bash
# Build the program
anchor build

# Deploy to Solana devnet
anchor deploy --provider.cluster devnet
```

### Account Structure

Each task is stored in a PDA derived from the task ID with the seed "task".

## Testing

Tests will be implemented in the future. The test structure is set up in the `tests/` directory.

## Target Network

This contract is configured for deployment on Solana devnet.

## License

[MIT License](LICENSE)
