# Coinflip — Distributed Randomness

Generate a random coin flip using MPC where no single node can predict or bias the outcome. The simplest Arcium example — one encrypted instruction, one fire-and-forget callback.

## How it works

The player encrypts their choice (heads/tails) and submits it. The MPC cluster generates a random boolean via `ArcisRNG::bool()`, compares it against the encrypted choice, and reveals only the win/loss outcome. Neither the player's choice nor the random value is ever exposed — only the comparison result.

```rust
#[instruction]
pub fn flip(input_ctxt: Enc<Shared, UserChoice>) -> bool {
    let input = input_ctxt.to_arcis();
    let toss = ArcisRNG::bool();
    (input.choice == toss).reveal()
}
```

The callback (`flip_callback`) emits the result as an event — no on-chain state is updated.

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

See also: [Arcis Primitives](https://docs.arcium.com/developers/arcis/primitives) for `ArcisRNG` reference.

**Next**: [Voting](../voting/) — encrypted state that persists across transactions
