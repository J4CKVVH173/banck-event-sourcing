# Event Sourcing Bank Account Implementation

A Rust implementation of event sourcing pattern for a bank account system, following clean architecture principles.

## Features

- **Event Sourcing Core**: All changes to account state are stored as a sequence of events
- **Clean Architecture**:
  - Domain layer with aggregates and events
  - Application layer for use cases
  - Infrastructure layer for I/O implementations
- **Modular Design**:
  - CLI interface
  - File-based event storage
  - HTTP API endpoints (WIP)
- **CQRS Support**: Separate paths for commands and queries

## Getting Started

### Prerequisites

- Rust 1.65+
- Cargo

### Installation

```bash
git clone https://github.com/yourusername/event-sourcing.git
cd event-sourcing
cargo build
```

## Project Structure

```
src/
├── application/       # Use cases and application logic
│   ├── storage/
│   └── use_cases/
├── domain/            # Core business logic
│   ├── event_aggregate.rs
│   ├── event.rs
│   └── tests/         # Domain model tests
├── infrastructure/    # I/O implementations
│   ├── input/
│   │   ├── cli/       # Command-line interface
│   │   └── http/      # HTTP API
│   └── storages/      # Storage implementations
└── main.rs            # Application entry point
```

## Domain Model

### BankAccount Aggregate

```rust
// Example from src/domain/event_aggregate.rs
pub struct BankAccount {
    pub id: AccountId,
    pub balance: Money,
    pub version: Version,
}

impl BankAccount {
    pub fn apply_event(&mut self, event: AccountEvent) {
        match event {
            AccountEvent::Deposited(amount) => self.balance += amount,
            AccountEvent::Withdrawn(amount) => self.balance -= amount,
        }
        self.version += 1;
    }
}
```

### Events

```rust
// Example from src/domain/event.rs
pub enum Event {
    Add(usize),
    Remove(usize),
}
```

## Running the Application

```bash
# Start CLI interface
cargo run -- cli

# Run tests
cargo test --all

# Generate documentation
cargo doc --open
```

## License

MIT
