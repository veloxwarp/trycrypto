# TryCrypto implementation plan

## Product boundary

TryCrypto teaches cryptographic primitives. It is not a general-purpose crypto
toolbox and it is not ClaimCrypt.

The recurring educational question is:

> What did this cryptographic operation actually prove?

That gives TryCrypto a natural bridge to ClaimCrypt without coupling the two
projects.

## Initial lessons

### 1. Hashes
- SHA-256 via WebCrypto.
- Live mutation of input.
- Explain determinism, fixed-size output, avalanche behavior.
- Explicitly state that hashes do not establish authorship or truth.

### 2. Shared-secret encryption
- AES-GCM.
- Generate key and nonce safely in-browser.
- Encrypt/decrypt UTF-8 text.
- Allow copy/paste between two simulated parties.
- Explain confidentiality vs authenticity.

### 3. Public/private key pairs
- Generate an asymmetric keypair.
- Visually separate public material from private material.
- Export public key; make private export deliberately explicit.

### 4. Public-key encryption
- RSA-OAEP unless a cleaner broadly-supported WebCrypto primitive is preferable
  when implemented.
- Show "encrypt for Alice" without Alice's private key.

### 5. Digital signatures
- Prefer Ed25519 if target-browser WebCrypto support is satisfactory at
  implementation time; otherwise ECDSA P-256.
- Sign, verify, mutate the message, verify again.

### 6. Identity is not a key
- Start with a valid signature.
- Let the user label the key "Alice".
- Show that the cryptographic result has not changed.
- Distinguish:
  - "Key K signed message M."
  - "Alice signed message M."
- Link conceptually to ClaimCrypt's entity/trust layer.

## Non-goals

- Production key custody.
- Password manager / wallet functionality.
- Implementing cryptographic primitives ourselves.
- Comprehensive algorithm coverage.
- Backend persistence.
- Accounts.
- Analytics that capture lesson input.

## Architecture

- Leptos CSR.
- Rust compiled to WebAssembly.
- `web-sys` bindings to browser WebCrypto.
- Thin `crypto` module hides JS/WebCrypto interop from components.
- Trunk build.
- Static `dist/`.
- Cloudflare Pages.
