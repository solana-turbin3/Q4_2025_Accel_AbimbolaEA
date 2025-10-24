.PHONY: build test clean deploy-devnet deploy-mainnet clippy fmt

# Build the program
build:
	cargo build-sbf

# Run tests
test: build
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -rf test-ledger

# Deploy to devnet
deploy-devnet: build
	solana program deploy \
		--program-id target/deploy/p_fundraiser-keypair.json \
		target/deploy/p_fundraiser.so \
		--url devnet

# Deploy to mainnet (use with caution!)
deploy-mainnet: build
	@echo "WARNING: Deploying to mainnet!"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		solana program deploy \
			--program-id target/deploy/p_fundraiser-keypair.json \
			target/deploy/p_fundraiser.so \
			--url mainnet-beta; \
	fi

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Show program ID
program-id:
	@solana-keygen pubkey target/deploy/p_fundraiser-keypair.json

# Create a new program keypair
new-keypair:
	solana-keygen new --no-bip39-passphrase -o target/deploy/p_fundraiser-keypair.json --force
