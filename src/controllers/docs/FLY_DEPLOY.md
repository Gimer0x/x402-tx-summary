# Deploy to Fly.io (from scratch)

## Prerequisites

- [Fly CLI](https://fly.io/docs/hands-on/install-flyctl/) installed and logged in (`fly auth login`).
- This repo builds with the root `Dockerfile` (multi-stage Rust + `ubuntu:24.04` runtime).

## 1. Create the app (first time only)

From the repo root:

```bash
cd /path/to/x402-server-rust
fly launch --no-deploy
```

- Choose a region (e.g. `dfw` to match `fly.toml`, or another).
- When asked for Postgres/Redis, say **no** unless you add them later.
- If `app` name in `fly.toml` is taken, either pick a new name in the wizard or edit `app = "..."` in `fly.toml` after launch.

Alternatively create the app first:

```bash
fly apps create YOUR_APP_NAME
```

Then set `app = "YOUR_APP_NAME"` in `fly.toml` and run `fly deploy`.

> **Important:** Set secrets **before** you expect the app to pass health checks. If `FACILITATOR_URL`, `RECEIVER_ADDRESS`, or `REQUEST_PRICE` are missing, the binary **exits on startup** (`config.rs`), so `/health` never listens and Fly will show **stopped** / **max restart count** with logs like `FACILITATOR_URL missing`.

## 2. Set secrets (required)

The server reads these at runtime (`src/config.rs` and RPC helpers):

```bash
fly secrets set -a YOUR_APP_NAME \
  FACILITATOR_URL="https://..." \
  RECEIVER_ADDRESS="0x..." \
  REQUEST_PRICE="0.01"
```

RPC URLs (set the ones you use):

```bash
fly secrets set -a YOUR_APP_NAME \
  ETHEREUM_RCP_URL="https://..." \
  BASE_RPC_URL="https://..." \
  BASE_SEPOLIA_RPC_URL="https://..."
```

Optional (CORS for browser clients):

```bash
fly secrets set -a YOUR_APP_NAME CORS_ORIGIN="https://your-frontend.example.com"
```

`SERVER_ADDR` is already set in `fly.toml` `[env]` as `0.0.0.0:8080`. Override with a secret only if you need a different port.

## 3. Deploy

Run **after** section 2 (secrets). Setting secrets triggers a new release; you can also run:

```bash
fly deploy -a YOUR_APP_NAME
```

First deploy can take several minutes (Rust compile in Docker).

## 4. Verify

```bash
curl -i https://YOUR_APP_NAME.fly.dev/health
```

Expect `200` and body `ok`.

## 5. Custom domain (optional)

1. `fly certs add api.example.com -a YOUR_APP_NAME`
2. Add the DNS records Fly shows (usually CNAME to `YOUR_APP_NAME.fly.dev`).
3. Set `CORS_ORIGIN` to the **browser Origin** that will call the API (often your main site, not necessarily the API hostname).

## Troubleshooting

| Symptom | Likely cause |
|--------|----------------|
| `GLIBC_2.38 not found` | Old image; rebuild with current `Dockerfile` (`ubuntu:24.04` runtime). |
| `502` / empty body | No healthy machine; check `fly logs -a YOUR_APP_NAME` and machine status. |
| Deploy stuck on old machine ID | Destroy broken machines: `fly machines list -a YOUR_APP_NAME` then `fly machine destroy ID -a YOUR_APP_NAME --force`, or `fly scale count 0` then `fly deploy`. |
