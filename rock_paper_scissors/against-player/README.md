# Rock Paper Scissors — Player vs Player

Two players submit encrypted moves asynchronously. Neither can see the other's choice until both are submitted and the MPC cluster compares them.

## How it works

**Use this pattern when**: two parties submit hidden inputs asynchronously and only the comparison is revealed.

A game is initialized with both move slots set to `3` (empty sentinel — valid moves are 0-2). Player A encrypts their move and submits it — the MPC circuit writes it into the encrypted game state (`Enc<Mxe, GameMoves>`). Player B does the same later. Player B cannot see Player A's move because it's encrypted to the MXE, not to any individual.

```rust
#[instruction]
pub fn player_move(
    players_move_ctxt: Enc<Shared, PlayersMove>,
    game_ctxt: Enc<Mxe, GameMoves>,
) -> Enc<Mxe, GameMoves>
```

The circuit validates that the player hasn't already moved and that the move is valid (0-2) before updating the encrypted state. Once both slots are filled, `compare_moves` decrypts both inside MPC, determines the winner via standard RPS rules, and reveals only the outcome (tie / A wins / B wins). Individual moves are never exposed.

## Concepts demonstrated

- **Encrypted state with async updates**: game state updated by two different players in separate transactions
- **Sentinel values**: `3u8` represents an empty move slot, checked inside the MPC circuit as a guard
- **Guard logic inside MPC**: the circuit validates move legality before accepting a submission

## Run

```bash
yarn install
arcium build
arcium test
```

## Key files

- `encrypted-ixs/src/lib.rs` — the circuits: `init_game`, `player_move`, `compare_moves`
- `programs/rock_paper_scissors/src/lib.rs` — the on-chain program
- `tests/rock_paper_scissors.ts` — end-to-end test

## Pitfalls

**`NotAuthorized` on move submission** — only the registered player for that slot can submit. Check that the signer matches the player from game creation.

**`compare_moves` returns `3`** — both players must have submitted before comparison. Return value `3` means the game is incomplete.

## Limitations

- Only 2 players per game — no multiplayer tables
- No timeout mechanism — if Player B never submits, the game stays open indefinitely

See also: [Computation Lifecycle](https://docs.arcium.com/developers/computation-lifecycle) for the async queue → MPC → callback pattern.
