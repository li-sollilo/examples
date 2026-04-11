# Ed25519 Signatures — Distributed Key Management

Sign and verify Ed25519 signatures where the private key is split across MPC nodes and never exists in a single location.

## How it works

**Signing**: A plaintext message is sent to the MPC cluster. Each node uses its key share in a distributed signing protocol to produce a standard Ed25519 signature. The signature is revealed — it's publicly verifiable by anyone with the corresponding public key.

```rust
#[instruction]
pub fn sign_message(message: [u8; 5]) -> ArcisEd25519Signature {
    let signature = MXESigningKey::sign(&message);
    signature.reveal()
}
```

**Verification**: An encrypted public key (`Enc<Shared, Pack<VerifyingKey>>`) and a plaintext message + signature are sent to MPC. The cluster unpacks the key, verifies the signature, and returns the boolean result encrypted to a designated observer. This hides *which* public key was checked.

```rust
#[instruction]
pub fn verify_signature(
    verifying_key_enc: Enc<Shared, Pack<VerifyingKey>>,
    message: [u8; 5],
    signature: [u8; 64],
    observer: Shared,
) -> Enc<Shared, bool>
```

## Concepts demonstrated

- **`MXESigningKey`**: Ed25519 private key shares split across MPC nodes — the full key is never reconstructed
- **Publicly verifiable output**: the signature is revealed and verifiable by anyone with the public key — unlike other examples where only a boolean or encrypted state is returned
- **Blind verification**: `verify_signature` hides *which* public key was checked, returning only an encrypted boolean to a designated observer

## Run

```bash
yarn install
arcium build
arcium test
```

## Key files

- `encrypted-ixs/src/lib.rs` — the circuits: `sign_message`, `verify_signature`
- `programs/ed_25519/src/lib.rs` — the on-chain program
- `tests/ed_25519.ts` — end-to-end test

## Pitfalls

**Signatures are public on reveal** — unlike other examples where the output stays encrypted or is a boolean, here the full 64-byte signature is exposed. This is intentional (Ed25519 signatures must be publicly verifiable), but developers expecting MPC to keep the output private should understand the distinction.

**Message size is fixed at 5 bytes** — the circuit signature is `message: [u8; 5]`. Larger messages require a different circuit definition.

## Limitations

- Only standard Ed25519 — no custom signing algorithms
- No key rotation — the MXE signing key is fixed per deployment
- 5-byte message cap in the example circuit

See also: [Arcis Primitives](https://docs.arcium.com/developers/arcis/primitives) for `MXESigningKey` reference.

---

*End of the recommended path. See [Mental Model](https://docs.arcium.com/developers/arcis/mental-model) to revisit the theory behind what you just built.*
