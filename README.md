# TryCrypto

TryCrypto is an interactive introduction to practical cryptography.

The project began as a small Haskell/Yesod experiment in 2016 and is being
revived as a client-side Rust/Leptos application. Cryptographic operations are
performed in the browser with the Web Crypto API; plaintext, keys, and
signatures are not sent to a backend.

## Goals

TryCrypto is deliberately narrower than a general-purpose toolbox such as
CyberChef. It teaches a short sequence of concepts by letting you manipulate
them directly:

1. Hashes and content fingerprints
2. Shared-secret (symmetric) encryption
3. Public/private key pairs
4. Public-key encryption
5. Digital signatures and verification
6. What cryptographic evidence does—and does not—prove about identity

The last point is intentional: a valid signature establishes that a particular
key signed a particular message. Connecting that key to a human, organization,
or other identity requires additional trust assumptions and evidence.

## Development

Requirements:

- Rust stable
- `wasm32-unknown-unknown` target
- Trunk

```console
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve --open
```

Production build:

```console
trunk build --release
```

The static site is written to `dist/`.

## Hosting

The intended host is Cloudflare Pages. The repository includes a GitHub Actions
workflow that builds the Rust/WASM application and uses Wrangler to upload the
prebuilt `dist/` directory.

Required GitHub Actions secrets:

- `CLOUDFLARE_ACCOUNT_ID`
- `CLOUDFLARE_API_TOKEN` with Cloudflare Pages edit permission

## History

An earlier project, `snoyberg/crypto-demo-site`, explored similar ideas in 2013,
including an interactive AES demo. This repository was started in 2016 as a
second attempt. The modern version keeps that history while replacing the old
server-side application with a browser-only implementation.
