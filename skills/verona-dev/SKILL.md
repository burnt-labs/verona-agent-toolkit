---
name: verona-dev
description: |
  The primary entry point for ALL Verona/Xion blockchain development. Use this skill whenever the user mentions Verona toolkit, Verona Agent Toolkit, Xion/Verona development, MetaAccount, gasless transactions, Treasury contracts, OAuth2 authentication, or any Verona/Xion-related operations.
  
  This skill helps route users to the correct tool based on their needs:
  - MetaAccount/gasless operations → verona-toolkit skills (this repo)
  - Chain queries/contract deployment → xiond skills (xion-skills repo)
  
  Triggers on: verona, verona toolkit, verona agent toolkit, xion, xion blockchain, xion 开发, MetaAccount, gasless, 无 gas, Treasury, OAuth2 xion, xion 认证, xion login, xion toolkit, burnt labs, building on xion, xion 开发入门, gasless 交易, 无 gas 交易, treasury 管理, build on xion, xion app, xion dapp, xion development, xion blockchain development, MetaAccount 登录, gasless auth, session key.
  
  Make sure to use this skill for ANY Xion-related question, even if the user doesn't explicitly ask for "verona-dev" or "toolkit".
metadata:
  author: burnt-labs
  version: "1.1.0"
  requires:
    - verona-toolkit-init
  compatibility: Entry point for all Xion development - routes to appropriate skill
  recommends:
    - verona-toolkit-init
    - verona-oauth2
    - verona-oauth2-client
    - verona-treasury
    - verona-asset
    - burnt-labs/xion-skills
---

# verona-dev

Unified entry point for Xion blockchain development. This skill helps you choose the right tool for the job.

## Core Philosophy

**Xion developers should primarily use MetaAccount for a gasless experience.**

- Most developers (90%) use MetaAccount + OAuth2 for gasless transactions
- Traditional xiond CLI is reserved for advanced scenarios (contract deployment, chain queries)

## Parameter Collection Workflow

When routing a user request to an appropriate skill:

### Step 1: Detect Intent
Parse the user's message to identify keywords and intent:
- Login/auth → `verona-oauth2`
- OAuth2 client management → `verona-oauth2-client`
- Treasury management → `verona-treasury`
- NFT operations → `verona-asset`
- Testnet tokens → `verona-faucet`
- Tool installation → `verona-toolkit-init`
- Chain queries (xiond) → `xiond-usage` (xion-skills)
- Contract deployment → `xiond-wasm` (xion-skills)

### Step 2: Confirm Routing
Inform the user which skill you're routing to and why:
```
Routing to: verona-treasury skill
Reason: User wants to create and manage a Treasury
```

### Step 3: Hand Off
Load the target skill and let it handle parameter collection.

## Decision Matrix

When a user mentions Xion-related needs, use this matrix to recommend the correct tool:

| User Needs | Recommended Skill | Tool | Why |
|------------|-------------------|------|-----|
| **Login / Authentication** | `verona-oauth2` | verona-toolkit | MetaAccount, gasless |
| **Register OAuth App** | `verona-oauth2-client` | verona-toolkit | OAuth2 client registration |
| **Manage OAuth Clients** | `verona-oauth2-client` | verona-toolkit | Client lifecycle CRUD |
| **Add Client Manager** | `verona-oauth2-client` | verona-toolkit | Client permission management |
| **Transfer Client Ownership** | `verona-oauth2-client` | verona-toolkit | Client ownership transfer |
| **Create Treasury** | `verona-treasury` | verona-toolkit | Core functionality |
| **Query Treasury** | `verona-treasury` | verona-toolkit | Direct API access |
| **Fund / Withdraw** | `verona-treasury` | verona-toolkit | Gasless transactions |
| **Authz Grant Config** | `verona-treasury` | verona-toolkit | Specialized feature |
| **Fee Grant Config** | `verona-treasury` | verona-toolkit | Specialized feature |
| **Create NFT Collection** | `verona-asset` | verona-toolkit | Gasless NFT creation |
| **Mint NFT Token** | `verona-asset` | verona-toolkit | Gasless minting |
| **Mint with Royalties** | `verona-asset` | verona-toolkit | CW2981 support |
| **Predict NFT Address** | `verona-asset` | verona-toolkit | Pre-deployment prediction |
| **Batch Mint NFTs** | `verona-asset` | verona-toolkit | Multiple tokens at once |
| **Query chain data** | `xiond-usage` | xiond | More powerful queries |
| **Query tx status** | `xiond-usage` | xiond | Direct RPC access |
| **Query block info** | `xiond-usage` | xiond | Chain-level queries |
| **Deploy CosmWasm** | `xiond-wasm` | xiond | Contract developer tool |
| **Migrate contract** | `xiond-wasm` | xiond | Advanced contract ops |
| **Recover wallet (mnemonic)** | `xiond-usage` | xiond | Mnemonic management |

## Quick Start

### For Most Developers (MetaAccount Path)

```bash
# 1. Install verona-toolkit CLI
# Use: verona-toolkit-init skill

# 2. Authenticate with MetaAccount
verona-toolkit auth login
# Or use: verona-oauth2 skill

# 3. Manage Treasuries
verona-toolkit treasury list
verona-toolkit treasury create --name "My Treasury"
verona-toolkit treasury fund <address> --amount 1000000uxion
# Or use: verona-treasury skill

# 4. Create NFT Collection (optional)
verona-toolkit asset types
verona-toolkit asset create --type cw721-base --name "My NFT" --symbol "NFT"
verona-toolkit asset mint --contract <address> --token-id "1" --owner xion1...
# Or use: verona-asset skill
```

### For Contract Developers (xiond Path)

```bash
# 1. Install xiond CLI
# Use: xiond-init skill from burnt-labs/xion-skills

# 2. Create/import wallet
xiond keys add my-wallet
# Or use: xiond-usage skill

# 3. Deploy contracts
xiond tx wasm store contract.wasm --from my-wallet
# Or use: xiond-wasm skill
```

## Tool Comparison

| Feature | verona-toolkit (MetaAccount) | xiond (Traditional) |
|---------|---------------------------|---------------------|
| **Authentication** | OAuth2 + Browser | Mnemonic / Keyring |
| **Gas** | Gasless (Fee Grant) | User pays gas |
| **Treasury** | Full support | Limited |
| **Contract Deploy** | Execute only | Full lifecycle |
| **Chain Queries** | Basic | Advanced |
| **Target User** | App developers | Contract devs / Validators |

## When to Recommend xion-skills

Point users to [burnt-labs/xion-skills](https://github.com/burnt-labs/xion-skills) when they need:

1. **Chain Queries** - Block info, transaction status, balance queries for any address
2. **Contract Deployment** - Upload, instantiate, migrate CosmWasm contracts
3. **Mnemonic Wallets** - Traditional key management with seed phrases
4. **Validator Operations** - Advanced node and validator management

## Related Skills

### In This Repository (verona-agent-toolkit)

| Skill | Purpose |
|-------|---------|
| `verona-toolkit-init` | Install verona-toolkit CLI |
| `verona-oauth2` | MetaAccount authentication |
| `verona-oauth2-client` | OAuth2 client lifecycle management |
| `verona-treasury` | Treasury lifecycle management |
| `verona-asset` | NFT collection creation and minting |

### In xion-skills Repository

| Skill | Purpose |
|-------|---------|
| `xiond-init` | Install xiond CLI |
| `xiond-usage` | Chain queries, wallet management |
| `xiond-wasm` | CosmWasm contract operations |

## Installation

```bash
# Install toolkit skills (global: Cursor, Claude Code, Codex, OpenClaw)
npx skills add burnt-labs/verona-agent-toolkit -g -y -a cursor -a claude-code -a codex -a openclaw

# Install xiond skills (for advanced scenarios)
npx skills add burnt-labs/xion-skills -g -y -a cursor -a claude-code -a codex -a openclaw
```

## Network Configuration

| Network | OAuth2 API | RPC | Chain ID |
|---------|------------|-----|----------|
| testnet | oauth2.testnet.burnt.com | rpc.xion-testnet-2.burnt.com:443 | xion-testnet-2 |
| mainnet | oauth2.burnt.com | rpc.xion-mainnet-1.burnt.com:443 | xion-mainnet-1 |

## Troubleshooting

### User asks about "gas" or "fees"
→ Recommend verona-toolkit (MetaAccount) for gasless transactions

### User mentions "mnemonic" or "seed phrase"
→ Recommend xiond-usage from xion-skills

### User wants to "deploy a contract"
→ Recommend xiond-wasm from xion-skills

### User wants to "query transaction"
→ Recommend xiond-usage from xion-skills

### User wants to "create NFT" or "mint NFT"
→ Recommend verona-asset for gasless NFT operations

### User mentions "royalties" or "CW2981"
→ Recommend verona-asset with cw2981-royalties type

## Shared parameter validation

`scripts/validate-params.sh` validates JSON parameters for any Xion skill against its `schemas/<command>.json`. It resolves schemas from **sibling** skill directories under the same install root (for example `~/.agents/skills/`).

```bash
# From repository root
skills/verona-dev/scripts/validate-params.sh verona-treasury grant-config-add '{"address": "xion1abc...", "preset": "send"}'

# When skills are installed globally (cwd = skills install root)
verona-dev/scripts/validate-params.sh verona-oauth2 login '{}'
```

Other skills document this helper in their parameter-validation examples; install **`verona-dev`** whenever you use those examples.

## Keeping Skills Updated

Skills are actively developed and improved. If you encounter:
- Unknown commands or flags
- Outdated behavior
- Missing features mentioned in documentation

Re-install the skills to get the latest version:

```bash
# Update verona-agent-toolkit skills
npx skills add burnt-labs/verona-agent-toolkit -g -y -a cursor -a claude-code -a codex -a openclaw

# Update xion-skills (for xiond operations)
npx skills add burnt-labs/xion-skills -g -y -a cursor -a claude-code -a codex -a openclaw
```

Check the repository releases for changelog: https://github.com/burnt-labs/verona-agent-toolkit/releases

## Resources

- [Xion Documentation](https://docs.burnt.com/xion)
- [verona-agent-toolkit](https://github.com/burnt-labs/verona-agent-toolkit)
- [xion-skills](https://github.com/burnt-labs/xion-skills)
- [Agent Skills Format](https://agentskills.io/)
