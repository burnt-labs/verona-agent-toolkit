# Verona Agent Toolkit

**Gasless Verona development toolkit for humans and AI agents.**

A command-line tool for managing Verona MetaAccounts, Treasury contracts, CosmWasm contracts, and CW721 assets — without handling private keys or paying gas fees.

---

## Core Features

- **Auth** — OAuth2 / MetaAccount (Google, Email, Passkey); PKCE; credentials encrypted on disk
- **Account** — MetaAccount address, authenticators, balances
- **Treasury** — Create, fund, withdraw; gasless grants & fee allowances; admin; backup via export/import
- **OAuth2 clients** — App registration and lifecycle (`oauth2 client`, needs `auth login --dev-mode`)
- **Contract** — Instantiate, instantiate2, execute, query (CosmWasm)
- **Asset** — CW721 collections: mint, batch mint, query, predictable deploy address
- **Batch** — Run or validate multi-message JSON batches (optional dry-run)
- **Transactions** — Check status or wait for confirmation
- **Faucet** — Testnet XION (1 per claim, 24h cooldown)
- **Config** — Default network and config keys (`testnet` / `mainnet`)
- **Automation-friendly** — JSON / human / GitHub Actions output, `status`, shell completions, `--no-interactive`

---

## Installation

### Install CLI

**macOS / Linux:**

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/verona-agent-toolkit/releases/latest/download/verona-agent-toolkit-installer.sh | sh
```

**Windows (PowerShell):**

```powershell
powershell -c "irm https://github.com/burnt-labs/verona-agent-toolkit/releases/latest/download/verona-agent-toolkit-installer.ps1 | iex"
```

**From Source:**

```bash
git clone https://github.com/burnt-labs/verona-agent-toolkit
cd verona-agent-toolkit
cp .env.example .env
cargo install --path .
```

### Install Skills (for AI Agents)

```bash
# 1. Install CLI first (see above)

# 2. Install skills
npx skills add burnt-labs/verona-agent-toolkit -g

# 3. Authenticate
verona-toolkit auth login
```

Bundled skills: `verona-dev`, `verona-toolkit-init`, `verona-oauth2`, `verona-oauth2-client`, `verona-treasury`, `verona-faucet`, `verona-asset`. See [INSTALL-FOR-AGENTS.md](./INSTALL-FOR-AGENTS.md) for full details.

### Authenticate

```bash
verona-toolkit auth login
```

Opens your browser for OAuth2 authentication. Tokens are stored encrypted locally. If you already have credentials, prefer `verona-toolkit auth refresh` before starting a new `login`.

---

## Quick Start

### 1. Login

```bash
verona-toolkit auth login
```

Authenticate with Google, Email, or Passkey — no seed phrases required.

### 2. Check environment

```bash
verona-toolkit status
verona-toolkit account info
```

### 3. Claim Testnet Tokens

```bash
verona-toolkit faucet claim
```

Receive 1 XION (1,000,000 uxion) for testing. Use `faucet status` and `faucet info` for cooldown and contract config.

### 4. Create a Treasury

```bash
verona-toolkit treasury create --name "My Treasury" --redirect-url "https://your-app.com/callback"
```

Creates a gasless transaction contract. Fund it with claimed tokens:

```bash
verona-toolkit treasury fund xion1... --amount 1000000uxion
```

Configure **grant-config** and **fee-config** when you need delegated messages and sponsored fees — see [CLI Reference](./docs/cli-reference.md).

### 5. (Optional) Create NFT Collection

```bash
verona-toolkit asset types
verona-toolkit asset create --type cw721-base --name "My Collection" --symbol "NFT"
verona-toolkit asset mint --contract xion1... --token-id "1" --owner xion1...
```

Use `asset predict` and **instantiate2**-style flows for predictable contract addresses (documented in the CLI reference).

### 6. (Optional) Follow a transaction

```bash
verona-toolkit tx wait <TX_HASH>
```

---

## Global CLI Options

```text
verona-toolkit [OPTIONS] <COMMAND>

  -n, --network <NETWORK>     testnet | mainnet (default: testnet)
  -o, --output <FORMAT>       json | json-compact | github-actions | human (default: json)
  -c, --config <PATH>         Config file path
      --no-interactive        Fail if required args are missing (no prompts)
```

Run `verona-toolkit --help` and `verona-toolkit <command> --help` for full flags.

---

## Shell completions

```bash
verona-toolkit completions --install   # install for your shell
verona-toolkit completions bash        # print script to stdout
```

---

## For AI Agents

If you want your AI agent to install and use this toolkit, give it this instruction:

```
Follow this guide https://raw.githubusercontent.com/burnt-labs/verona-agent-toolkit/main/INSTALL-FOR-AGENTS.md to install and configure the Verona Agent Toolkit skills for AI agents.
```

---

## Documentation

| Document | Description |
|----------|-------------|
| [CLI Reference](./docs/cli-reference.md) | Complete command documentation |
| [Quick Reference (AI)](./docs/QUICK-REFERENCE.md) | Condensed reference for AI agents |
| [Skills Guide](./docs/skills-guide.md) | AI agent skills usage |
| [Error Codes](./docs/ERROR-CODES.md) | Error code reference |
| [Exit Codes](./docs/EXIT-CODES.md) | CI/CD exit codes |
| [Configuration](./docs/configuration.md) | Network and config settings |
| [Install for AI Agents](./INSTALL-FOR-AGENTS.md) | Agent integration guide |
| [Contributing](./CONTRIBUTING.md) | Contribution guidelines |

---

## Security

- **No Private Keys** — OAuth2 and MetaAccount authentication only
- **PKCE (RFC 7636)** — Prevents authorization code interception
- **AES-256-GCM** — Encrypted credential storage
- **Localhost Only** — Callback server accepts localhost connections only
- **HTTPS Only** — All API communications encrypted

---

## Troubleshooting

**CLI not found after install:**

```bash
export PATH="$HOME/.cargo/bin:$PATH"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
```

**Token expired:**

```bash
verona-toolkit auth refresh
```

**Port in use during login:**

```bash
verona-toolkit auth login --port 54322
```

---

## License

Apache License 2.0 — see [LICENSE](LICENSE) for details.

---

## Resources

- [Verona Documentation](https://docs.verona.dev)
- [Developer Portal](https://dev.testnet2.burnt.com)
- [Contributing Guide](./CONTRIBUTING.md)
- [xion-skills](https://github.com/burnt-labs/xion-skills) — Advanced chain operations
