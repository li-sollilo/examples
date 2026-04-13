# Rock Paper Scissors vs House

Play against an MPC-generated random opponent. The house cannot see your move before generating its response, and no single node can predict or bias the random outcome.

## How it works

**Use this pattern when**: you need randomness bounded to a small set (rejection sampling inside MPC).

The player encrypts their move (rock/paper/scissors) and submits it. A single MPC computation decrypts the move, generates a random house move via `ArcisRNG` using rejection sampling (16 iterations to get a uniform value in 0-2), compares the two, and reveals only the outcome (tie/player wins/house wins). Neither the player's move nor the house's random move is ever exposed.

```rust
#[instruction]
pub fn play_rps(player_move_ctxt: Enc<Shared, PlayerMove>) -> u8 {
    let player_move = player_move_ctxt.to_arcis();
    // ... rejection sampling for uniform random in [0, 2] ...
    result.reveal()
}
```

Like coinflip, this is stateless — the callback (`play_rps_callback`) emits the result as an event, no on-chain state.

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

See also: [Arcis Primitives](https://docs.arcium.com/developers/arcis/primitives) for `ArcisRNG` reference.

**Next**: [Against Player](../against-player/) — same game, but two humans instead of RNG
