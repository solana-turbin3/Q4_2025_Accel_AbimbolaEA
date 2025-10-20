Silicon + Vault (Transfer-fee hook) — README

Overview

- This repo contains two Anchor programs: `vault` and `silicon`.
- `vault` implements a token vault that uses the SPL Token `transfer_hook` extension to run a hook program during token transfers (for example to collect fees or enforce rules).
- `silicon` is the hook program and an oracle-driven "agent" used to whitelist users. It integrates with `solana_gpt_oracle` to let a GPT-based oracle decide whether to whitelist a user. The hook enforces that transfers only succeed for whitelisted addresses.

Key components

- programs/vault: Anchor program that exposes vault operations (initialize, deposit, withdraw) and expects a transfer hook-compatible program for transfer validation.
- programs/silicon: Anchor program that implements:
  - an Agent (LLM context) using `solana_gpt_oracle` to interact with a GPT tool;
  - functions to initialize the agent, interact with the GPT tool, and receive callbacks from the oracle;
  - a simple whitelist account and helper instructions to add/remove users manually or via the GPT callback;
  - a `hook` instruction (marked with the `ExecuteInstruction::SPL_DISCRIMINATOR_SLICE` discriminator) which the vault's transfer-hook CPI will call to validate transfers.

How it works (high level)

1. Deploy both programs (`vault` and `silicon`) to your cluster. The `vault` program is configured to call `silicon` as its transfer-hook (via CPI) during transfers that have the transfer-hook extension set.
2. Use `silicon::initialize_agent` to create an LLM context for the agent (creates a context account through the `solana_gpt_oracle` program). This links the agent to an on-chain GPT context described by `AGENT_DESC` in `programs/silicon/src/constants.rs`.
3. Call `silicon::interact_agent` with user-provided text. This forwards the text to `solana_gpt_oracle::interact_with_llm`. The oracle will later call back into `silicon::__client_accounts_callback_from_agent` (the callback) with the GPT response.
4. The callback should set the user's whitelist flag (the code stores a boolean in the `Whitelist` account). The hook checks that the token transfer's source owner matches the `Whitelist.address` and that `is_whitelisted` is true. If not, it returns an error and prevents the transfer.

Versions (from repository)

- Anchor: `@coral-xyz/anchor` ^0.32.1 (JS/TS)
- Rust/Anchor runtime: `anchor-lang = 0.32.1`, `anchor-spl = 0.32.1`
- solana-gpt-oracle: `0.1.1`
- spl-transfer-hook-interface: `2.0.0`
- Node tooling: pnpm (toolchain configured in `Anchor.toml`)

Quick setup (brief)

1. Install dependencies

   - Install pnpm (if not installed): https://pnpm.io/installation
   - Install Rust + Solana toolchain (as required by Anchor)

2. From the repo root, install JS dev deps

```bash
pnpm install
```

3. Build and test programs locally

- Build Rust programs (Anchor workspace):

```bash
# build the programs
cargo build-bpf --manifest-path programs/silicon/Cargo.toml --release
cargo build-bpf --manifest-path programs/vault/Cargo.toml --release
```

- Alternatively, use Anchor CLI to build and deploy to localnet (if you use Anchor CLI):

```bash
anchor build
anchor localnet
anchor deploy --provider.cluster localnet
```

4. Deploy and configure

- Deploy both programs and note their program IDs (or use the IDs in `Anchor.toml` for localnet).
- When creating mints/accounts in `vault`, set the transfer-hook extension (transfer_hook) and point the hook's program id to `silicon`.
- Initialize the agent (call `silicon::initialize_agent`) to create the on-chain LLM context and counter accounts used by `solana_gpt_oracle`.
- Users call `silicon::interact_agent` to send text to the LLM. The oracle will call back into `silicon` to update the whitelist account.

Caveats & notes

- I have not implemented automated tests for this project yet.
- The `silicon` program uses `solana_gpt_oracle` which expects the oracle program to perform callbacks — be sure the oracle program is deployed and the `oracle_program` account check uses the correct program ID.
- There are a few small inconsistencies/typos in the code (for example `constant` vs `constants` module naming and a few misspelled identifiers like `soutce_token` and `OK` vs `Ok`) that should be fixed before production use. Review the code and run `cargo check` / `anchor build` to surface any compiler errors.

Security

- The README and code assume the GPT oracle's output is trusted for whitelisting decisions. Treat any automated whitelisting with caution and consider adding manual review or rate limits.

Contact

- For questions about this repo, open an issue or contact the maintainer.
