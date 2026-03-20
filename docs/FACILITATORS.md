# Facilitator-specific notes

## OpenX402 (`https://facilitator.openx402.ai`)

Per [facilitator discovery](https://facilitator.openx402.ai), this service supports **Base mainnet** (`eip155:8453`), **Base Sepolia**, v2 headers, and v1 compatibility.

If verify returns **`address_not_registered`**, the **`RECEIVER_ADDRESS`** you configured (the `payTo` field in the 402 body) is **not registered** with OpenX402. Complete their onboarding (e.g. **https://openx402.ai/register** as linked from the facilitator) so that payout address is allowed.

Also set **`X402_PUBLIC_BASE_URL=https://…`** (your real public HTTPS origin) so `resource` in payment requirements is not `http://…` behind ngrok.

## Public x402.rs facilitator (`https://facilitator.x402.rs`)

See `/supported`: Base **mainnet** is not listed for v1 `exact`; use **`X402_USDC_NETWORK=base_sepolia`** there, or use a facilitator that lists Base (e.g. OpenX402 above) after registering your receiver.
