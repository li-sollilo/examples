# Rock Paper Scissors vs House

Play against a house move generated inside the MXE. The house cannot see your move before responding, and no single Arx node can predict or bias the random outcome.

## How it works

**Use this pattern when**: you need randomness bounded to a small set (rejection sampling inside the MXE).

The player encrypts their move (rock/paper/scissors) and submits it. A single encrypted computation decrypts the move, generates a random house move via `ArcisRNG` using rejection sampling (16 iterations to get a uniform value in 0-2), compares the two, and reveals only the outcome (tie/player wins/house wins). Neither the player's move nor the house's random move is ever exposed.

```rust
#[instruction]
pub fn play_rps(player_move_ctxt: Enc<Shared, PlayerMove>) -> u8 {
    let player_move = player_move_ctxt.to_arcis();
    // ... rejection sampling for uniform random in [0, 2] ...
    result.reveal()
}
```

Like coinflip, this is stateless — the callback (`play_rps_callback`) emits the result as an event, no on-chain state.

## Concepts demonstrated

- **Rejection sampling**: `ArcisRNG` produces uniform bytes; iteration rejects values outside `0..=2` to avoid modulo bias
- **Stateless, fire-and-forget**: result delivered as an event; no on-chain state mutates
- **Bounded-set randomness**: same primitive as coinflip, adapted from a binary to a ternary outcome

## Run

```bash
yarn install
arcium build
arcium test
```

## Key files

- `encrypted-ixs/src/lib.rs` — the circuit: `play_rps`
- `programs/rock_paper_scissors_against_rng/src/lib.rs` — the on-chain program
- `tests/rock_paper_scissors_against_rng.ts` — end-to-end test

## Pitfalls

**`AbortedComputation` on callback** — the Arx cluster is unhealthy or a node went down mid-computation. See the [top-level troubleshooting](../../README.md#troubleshooting) for Docker recovery steps.

## Limitations

- No score tracking across games — each call is independent
- House move is not seedable; randomness is freshly generated every call (by design, but means this is not a deterministic test harness)

See also: [Arcis Primitives](https://docs.arcium.com/developers/arcis/primitives) for `ArcisRNG` reference.

**Next**: [Against Player](../against-player/) — same game, but two humans instead of RNG | **Back to core path**: [Voting](../../voting/)
