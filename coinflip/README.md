# Coinflip — Distributed Randomness

Generate a random coin flip inside the MXE so no single Arx node can predict or bias the outcome. The simplest Arcium example — one encrypted instruction, one fire-and-forget callback.

## How it works

**Use this pattern when**: you need randomness where no single party can bias the outcome.

The player encrypts their choice (heads/tails) and submits it. The MXE generates a random boolean via `ArcisRNG::bool()`, compares it against the encrypted choice, and reveals only the win/loss outcome. Neither the player's choice nor the random value is ever exposed — only the comparison result.

```rust
#[instruction]
pub fn flip(input_ctxt: Enc<Shared, UserChoice>) -> bool {
    let input = input_ctxt.to_arcis();
    let toss = ArcisRNG::bool();
    (input.choice == toss).reveal()
}
```

The callback (`flip_callback`) emits the result as an event — no on-chain state is updated.

## Concepts demonstrated

- **`ArcisRNG::bool()`**: network-generated randomness — no single Arx node can predict or bias the outcome
- **Fire-and-forget callback**: result is emitted as an event; nothing persists on-chain
- **`Enc<Shared, T>` input**: player's choice is encrypted client-side, decryptable only inside the MXE and by the player

## Run

```bash
yarn install
arcium build
arcium test
```

## Key files

- `encrypted-ixs/src/lib.rs` — the circuit: `flip`
- `programs/coinflip/src/lib.rs` — the on-chain program
- `tests/coinflip.ts` — end-to-end test

## Pitfalls

**`AbortedComputation` on callback** — the Arx cluster is unhealthy or a node went down mid-computation. See the [top-level troubleshooting](../README.md#troubleshooting) for Docker recovery steps.

## Limitations

- One flip per call — no bet amount, no payout, no streak tracking
- Result is delivered as an event only; downstream programs cannot read it directly on-chain

See also: [Arcis Primitives](https://docs.arcium.com/developers/arcis/primitives) for `ArcisRNG` reference.

**Next**: [Voting](../voting/) — encrypted state that persists across transactions | **Optional branch**: [RPS vs House](../rock_paper_scissors/against-house/) — randomness bounded to a small set
