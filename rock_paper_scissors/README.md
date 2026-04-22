# Rock Paper Scissors — Encrypted Moves

Fair asynchronous gameplay where moves stay hidden until resolution, even though submissions land on a public blockchain.

## How it works

**Use this family when**: two parties submit small hidden inputs asynchronously and only the comparison result should be revealed.

Both variants use encrypted move submission so the first player cannot be front-run by the second. The player-vs-player variant stores encrypted game state between transactions; the player-vs-house variant resolves in one computation using MXE-generated randomness.

## Variants

- [**Player vs Player**](./against-player/) -- two encrypted submissions, async reveal. Stateful (tracks game state between moves).
- [**Player vs House**](./against-house/) -- player vs MXE-generated house move. Stateless (single computation).

## Further reading

- [Arcis Primitives](https://docs.arcium.com/developers/arcis/primitives) -- `ArcisRNG` used in the against-house variant
- [Input/Output Patterns](https://docs.arcium.com/developers/arcis/input-output) -- encrypted move submission and result revelation

**Back to [Examples](../README.md)**
