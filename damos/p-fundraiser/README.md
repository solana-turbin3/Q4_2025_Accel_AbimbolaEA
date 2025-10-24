# p-fundraiser

A Solana program built with Pinocchio.

## Features

- Token support (pinocchio-token)
- Associated Token Account support (pinocchio-associated-token-account)
- Logging support (pinocchio-log)
- System program support (pinocchio-system)
- Pubkey utilities (pinocchio-pubkey)

## Prerequisites

- Rust (latest stable)
- Solana CLI tools
- Anchor CLI (optional, for additional tooling)

## Building

```bash
make build
```

## Testing

```bash
make test
```

## Deployment

```bash
# Deploy to devnet
make deploy-devnet

# Deploy to mainnet
make deploy-mainnet
```

## Development

```bash
# Clean build artifacts
make clean

# Run clippy
make clippy

# Format code
make fmt
```

## Project Structure

```
.
├── src/
│   └── lib.rs          # Program entrypoint
├── tests/              # Integration tests
├── target/
│   └── deploy/         # Compiled program and keypair
├── Cargo.toml
├── Makefile
└── README.md
```

## Resources

- [Pinocchio Documentation](https://github.com/febo/pinocchio)
- [Solana Documentation](https://docs.solana.com/)

---

Generated with [pino-cli](https://github.com/yourusername/pino-cli)
