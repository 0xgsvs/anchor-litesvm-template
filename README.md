# Anchor LiteSVM Template

A minimal Solana program template using Anchor framework with LiteSVM for testing.

## Overview

This template provides a streamlined setup for developing Solana programs with:
- **Anchor Framework** - Solana's most popular development framework
- **LiteSVM** - Fast, in-memory Solana VM for testing (no validator needed)
- **Bun** - Fast JavaScript/TypeScript runtime for tooling
- **Mise** - Task runner and tool version manager
- **Codama** - Client code generation (Rust)

## Prerequisites

| Tool | Purpose | Installation |
|------|---------|--------------|
| **Mise** | Task runner & version manager | `curl https://mise.run \| sh` |
| **Solana CLI** | Solana toolchain | `sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"` |
| **Anchor CLI** | Solana framework | `cargo install --git https://github.com/solana-foundation/anchor --locked anchor-cli` |
| **Rust** | Build programs | `rustup target add bpfel-unknown-unknown` |

Verify installations:

```bash
mise --version
solana --version
anchor --version
```

## Quick Start

```bash
# Clone and setup
git clone <repo-url>
cd anchor-litesvm-template
mise trust    # Trust the mise configuration
mise install  # Install tools

# Install dependencies
mise setup

# Build and test
mise test
```

## Tech Stack

| Component | This Template | Notes |
|-----------|---------------|-------|
| **Package Manager** | Bun | For JS tooling |
| **Test Framework** | LiteSVM (Rust) | In-memory VM testing |
| **Client Generation** | Codama (Rust client) | TypeScript client not configured |
| **Automation** | Mise tasks | Defined in `mise.toml` |
| **Formatter** | Oxfmt (TS) + cargo fmt (Rust) | Code formatting |
| **Linter** | Oxlint | TypeScript linting |

## Project Structure

```
├── programs/litesvm-template/     # Solana program (Rust)
│   ├── src/
│   │   ├── lib.rs                # Program entry point
│   │   ├── instructions/         # Instruction handlers
│   │   │   └── initialize.rs     # Initialize instruction
│   │   ├── instructions.rs       # Instructions module
│   │   ├── state.rs              # Account state definitions
│   │   ├── constants.rs          # Program constants
│   │   └── error.rs              # Custom errors
│   ├── tests/
│   │   └── test_initialize.rs    # LiteSVM tests
│   └── client/                   # Generated Rust client
│       └── src/generated/        # Codama-generated code
├── migrations/
│   └── deploy.ts                 # Anchor deployment script
├── target/                       # Build artifacts
│   └── deploy/
│       └── litesvm_template.so   # Compiled program
├── Anchor.toml                   # Anchor configuration
├── Cargo.toml                    # Rust workspace config
├── codama.json                   # Codama client generation config
├── mise.toml                     # Mise task runner config
├── package.json                  # Bun dependencies
└── tsconfig.json                 # TypeScript configuration
```

## NPM Dependencies

| Package | Purpose | Version |
|---------|---------|---------|
| **@anchor-lang/core** | Anchor client library | ^1.0.0 |
| **@codama/nodes-from-anchor** | Codama Anchor IDL parser | ^1.4.0 |
| **@codama/renderers-rust** | Rust client renderer | ^3.0.0 |
| **oxlint** | TypeScript linter | ^1.59.0 |
| **oxfmt** | TypeScript formatter | ^0.44.0 |

## Rust Dependencies

| Package | Purpose | Version |
|---------|---------|---------|
| **anchor-lang** | Anchor framework | 1.0.0 |
| **litesvm** | In-memory Solana VM | 0.11.0 |
| **solana-keypair** | Keypair generation | 3.1.2 |
| **solana-signer** | Transaction signing | 3.0.0 |
| **solana-transaction** | Transaction handling | 3.0.0 |
| **solana-message** | Message creation | 3.0.1 |
| **solana-instruction** | Instruction building | 3.3.0 |
| **solana-address** | Address types | 2.5.0 |

## Development Workflow

```bash
# ====================================
# SETUP
# ====================================
mise setup      # Install Bun dependencies
mise sync       # Sync Anchor program keys

# ====================================
# DEVELOPMENT
# ====================================
mise fmt        # Format code (TypeScript + Rust)
mise lint       # Lint TypeScript
mise build      # Build program and generate Codama clients

# ====================================
# TESTING
# ====================================
mise test       # Run LiteSVM tests (Rust)

# ====================================
# DEPLOYMENT
# ====================================
mise deploy     # Deploy to localnet
```

## Available Mise Tasks

| Task | Description | Dependencies |
|------|-------------|--------------|
| **setup** | Install Bun dependencies | - |
| **sync** | Sync Anchor program keys | - |
| **build** | Build program + generate clients | sync |
| **test** | Run LiteSVM test suite | build, lint, fmt |
| **lint** | Lint TypeScript with auto-fix | - |
| **fmt** | Format TypeScript and Rust code | - |
| **deploy** | Deploy program to localnet | build |
| **update** | Update Bun and Cargo dependencies | - |
| **clean** | Clean all build artifacts and caches | - |

## Configuration Files

- **Anchor.toml** - Program configuration and provider settings
- **mise.toml** - Task runner and tool versions
- **codama.json** - Codama Rust client generation configuration
- **Cargo.toml** - Rust workspace configuration
- **tsconfig.json** - TypeScript compiler configuration

## Testing with LiteSVM

Tests use LiteSVM for fast, in-memory testing without a validator:

```rust
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn test_initialize() {
    let program_id = litesvm_template::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    
    // Load program
    let bytes = include_bytes!("../../../target/deploy/litesvm_template.so");
    svm.add_program(program_id, bytes).unwrap();
    
    // Fund payer
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    
    // Build and send transaction...
}
```

Run tests:

```bash
# Via mise
mise test

# Or directly with cargo
cargo nextest run
```

## Client Generation

The template uses **Codama** to generate Rust clients from the Anchor IDL:

1. Build the program: `anchor build` generates the IDL at `target/idl/litesvm_template.json`
2. Run Codama: `codama run rust` generates Rust client in `programs/litesvm-template/client/`

## Program Info

- **Program Name**: litesvm_template
- **Program ID**: GUj2aE6R63T2QpusqZfL4o2d5Fg4V9PP9mzScpDyfqWV
- **Instructions**:
  - `initialize` - Simple greeting instruction
- **State**: None (template)

## Network Configuration

Default network: **localnet**

- Update `provider.cluster` in `Anchor.toml` for different networks
- Program address: `GUj2aE6R63T2QpusqZfL4o2d5Fg4V9PP9mzScpDyfqWV`

## Maintenance

```bash
mise update    # Update project dependencies
mise clean     # Clean build artifacts and caches
```

## Code Quality

- **Formatter**: Oxfmt (TypeScript) + cargo fmt (Rust)
- **Linter**: Oxlint with auto-fix and type-aware checking
- **Client Generation**: Codama generates Rust clients from Anchor IDL
- **Type Safety**: Strict TypeScript with `noUncheckedIndexedAccess`

## Differences from Standard Anchor Template

| Feature | This Template | Standard Anchor |
|---------|---------------|-----------------|
| **Tests** | LiteSVM (Rust) | Mocha (TypeScript) |
| **Test Location** | `programs/*/tests/` | `tests/` |
| **Client** | Rust (Codama) | JavaScript |
| **Validator** | None needed for tests | `solana-test-validator` |
| **Runtime** | Bun | Node.js/Yarn |
