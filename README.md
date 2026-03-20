# Agent-Ready Semantic Transaction Decoder for Web3

This project is an open infrastructure layer in `Rust` that converts raw blockchain transactions into a **structured semantic representation** (intents and normalized value fields) that is easy for AI agents to consume.

It also integrates with the **x402 pay-per-call protocol** via `X402Middleware`, so the endpoint is ready for agent/tool monetization. This is an MVP version, only a few networks and tokens are supported yet.

## What you get

Given a transaction hash and a network, the server:

1. Fetches the transaction from the configured RPC.
2. Detects the high-level action (native ETH transfer vs ERC-20 `transfer`).
3. Produces a JSON payload with normalized fields like `intent`, `summary`, `amount`, `recipient`, and token metadata.

### Why this helps

- **Agent-friendly schema**: downstream LLMs/tools get deterministic fields instead of parsing calldata manually.
- **Semantic intent**: outputs represent “what happened” (transfer, recipient, amount) rather than “how it happened” (RLP, calldata words).
- **Pay-per-call readiness (x402)**: the server is wrapped with x402 middleware so calls can be billed and controlled at the protocol layer.

## API

### Endpoint

`POST /summary/{network}/{tx_hash}`

The server route is defined in `src/main.rs`.

#### Path parameters

- `network`: numeric chain id as a string
  - `1` (Ethereum mainnet)
  - `8453` (Base mainnet)
  - `84532` (Base Sepolia)
- `tx_hash`: transaction hash (0x-prefixed 32-byte hex string)

#### Example
To run the client define the .env variables `EVM_PRIVATE_KEY` and `SERVER_HOST`. The account must have enough `USDC` funds on `Base Sepolia` to pay for the service (e.g., 0.001 USDC). The `SERVER_HOST` is the host name of the API server, if you are running the project locally, you have to expose it via a tunneling setup (e.g., ngrok).

 and send a request, example:
```bash
cargo run -q 8453 0x84fdb2d79c552eea98f01d905ac775d78f7c135ddd1c9b8a6bb76b701797cd8f
```

This is an example of the API call:
```bash
https://valery-neon-lyon.ngrok-free.dev/summary/8453/0x84fdb2d79c552eea98f01d905ac775d78f7c135ddd1c9b8a6bb76b701797cd8f
```

### Response shape

On success (`200 OK`), the JSON includes:

- `schema_version`: current semantic schema version
- `signer`: sender address
- `block_number`, `block_hash`, `transaction_index`, `effective_gas_price`
- `data`: decoded semantic object
  - `chain`: `{ chain_id, name, native_asset }`
  - `tx_type`: `"legacy"` or `"eip1559"` (based on tx envelope)
  - `nonce`, `gas_limit`, `gas_price`, `to`
  - `input_data`: the semantic action
    - `type` / `subtype` / `intent`
    - `summary`: human-readable sentence
    - `recipient` (ERC-20 transfer) or `to` (native transfer)
    - `asset_in`: list of `TokenInfo { name, symbol, decimals }`
    - `asset_out`: currently empty string
    - `amount`: raw amount string in token smallest units

### Errors

- `404 Not Found`: transaction not found on the provided RPC
- `400 Bad Request`: semantic decoding failed due to Etherscan/ABI issues (when applicable)
- `500 Internal Server Error`: unexpected RPC/decoding failures

## Configuration (environment variables)

The server expects these variables:

### x402 middleware

- `FACILITATOR_URL` (e.g., https://facilitator.x402.rs)
- `RECEIVER_ADDRESS` (Ethereum address)
- `REQUEST_PRICE` (numeric; price used for the x402 price tag)

### RPC endpoints (per chain)

- `ETHEREUM_RCP_URL` — Ethereum mainnet (`chain id 1`)
- `BASE_RPC_URL` — Base mainnet (`8453`)
- `BASE_SEPOLIA_RPC_URL` — Base Sepolia (`84532`)

### Optional

- `SERVER_ADDR` — bind address (default `0.0.0.0:8080`; Fly sets this in `fly.toml` `[env]`)
- `CORS_ORIGIN` — exact browser `Origin` allowed for CORS (default in code may be your site)

### Etherscan

- `ETHERSCAN_API_KEY` — required when ABI lookup / Etherscan paths are used

### Deploy on Fly.io

See **[docs/FLY_DEPLOY.md](docs/FLY_DEPLOY.md)** for `fly launch`, secrets, and verification.

## How the semantic layer works

At a high level:

1. **Fetch** the transaction via RPC.
2. **Classify** the transaction:
   - empty calldata + non-zero value => native transfer
   - calldata selector `0xa9059cbb` => ERC-20 `transfer(address,uint256)`
3. **Decode semantics**:
   - native transfer => builds `InputTxData { type: "transfer", subtype: "native", ... }`
   - ERC-20 transfer => extracts recipient and amount from ABI encoding and attaches token metadata

### Token metadata

Token name/symbol/decimals are currently **hardcoded for a small set of known token addresses** (plus the native asset case). This keeps the implementation simple while we iterate on the semantic capabilities.

## Interacting with x402

This server is wrapped with `X402Middleware`:

- The middleware is configured in `src/main.rs`.
- When the x402 client integrates with the protocol, calls can be billed/authorized at the x402 layer.

For details on how to construct x402 tool/payment requests, refer to the x402 client/protocol documentation and your client SDK usage.

## Local development

1. Set the required environment variables (RPC URLs + x402 vars).
2. Run the server (example):
   - `cargo run`
3. Expose it via your tunneling setup (ngrok, etc.).
4. Call:
   - `POST /summary/{network}/{tx_hash}`

