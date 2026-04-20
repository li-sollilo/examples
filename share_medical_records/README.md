# Medical Records — Re-Encryption

Transfer encrypted patient data from one party's key to another's without exposing it in transit. Data is decrypted inside the MXE and re-encrypted for the recipient.

## How it works

**Use this pattern when**: encrypted data must change encryption keys between parties without exposing the plaintext in transit.

The patient first stores their encrypted medical record on-chain (`store_patient_data` — a pure Anchor write, no encrypted computation). When sharing, `share_patient_data` sends the encrypted record to the MXE, which decrypts it inside the secure environment, re-encrypts it under the recipient doctor's public key, and emits the re-encrypted data as an event. Only the doctor can decrypt the emitted ciphertext. Plaintext is never written on-chain and does not leave the MXE during re-encryption.

```rust
#[instruction]
pub fn share_patient_data(
    receiver: Shared,
    input_ctxt: Enc<Shared, PatientData>,
) -> Enc<Shared, PatientData>
```

Input is encrypted to the patient, output is encrypted to the receiver. The callback emits the re-encrypted fields as an event.

## Concepts demonstrated

- **MXE-based re-encryption**: data is decrypted and re-encrypted to a new recipient's key inside the MXE — only ciphertext is emitted
- **`Enc<Shared, T>` key rotation**: input encrypted to sender, output encrypted to receiver — same ciphertext type, different key holders
- **Event-based delivery**: the callback emits re-encrypted data as an event rather than persisting it on-chain

## Run

```bash
yarn install
arcium build
arcium test
```

## Key files

- `encrypted-ixs/src/lib.rs` — the circuit: `share_patient_data`
- `programs/share_medical_records/src/lib.rs` — the on-chain program
- `tests/share_medical_records.ts` — end-to-end test

## Limitations

- Shares the entire `PatientData` struct — no per-field selective disclosure yet
- Single recipient per share operation

See also: [Input/Output Patterns](https://docs.arcium.com/developers/arcis/input-output) for re-encryption reference.

**Back to [Examples](../README.md)** | **Back to core path**: [Sealed-Bid Auction](../sealed_bid_auction/)
