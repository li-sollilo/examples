# Blackjack — Hidden Game State

Encrypted blackjack where the deck, dealer's hole card, and undealt cards remain hidden throughout gameplay. Information is revealed only when game rules require it.

## How it works

**Use this pattern when**: game state must persist across turns with selective reveals (player sees hand, dealer hole card stays hidden).

At game start, a 52-card deck is shuffled using `ArcisRNG` inside MPC. The deck is packed into 2 field elements via `Pack<T>`, compressing 52 bytes of card data from 1,664 bytes of ciphertext to 64 bytes:

```rust
type Deck = Pack<[u8; 52]>;
type Hand = Pack<[u8; 11]>;
```

The packed deck and dealer hand are stored encrypted to the MXE (hidden from everyone); the player hand and dealer's face-up card are encrypted to the player (`Shared`). Each game action (hit, stand, double down) triggers an MPC computation that reads the encrypted deck and hands, processes the action, and writes updated state back via callback. The dealer's automated play and final resolution are separate MPC instructions — 6 instructions total. Cards use `53u8` as a sentinel for empty hand slots.

## Concepts demonstrated

- **`Pack<T>`**: compresses large structs into minimal field elements, reducing on-chain ciphertext size
- **Multi-instruction protocol**: `shuffle_and_deal_cards`, `player_hit`, `player_stand`, `player_double_down`, `dealer_play`, `resolve_game`
- **Selective disclosure**: player sees own cards (`Enc<Shared, Hand>`), dealer's hole card stays encrypted to the MXE until resolution
- **Account byte offsets**: reading deck/hand ciphertexts from specific positions in the game account via `ArgBuilder`

## Run

```bash
yarn install
arcium build
arcium test
```

## Key files

- `encrypted-ixs/src/lib.rs` — all 6 circuits plus `calculate_hand_value` helper
- `programs/blackjack/src/lib.rs` — the on-chain program
- `tests/blackjack.ts` — end-to-end test
- `multi_tx/` — variant using multi-transaction patterns
- `serialized/` — variant using serialized computation

## Pitfalls

**`InvalidGameState` on player action** — game actions are only valid during `PlayerTurn`. Check the game state before calling hit/stand/double down.

**`InvalidMove` on double down** — only allowed as the player's first action (hand size = 2).

## Limitations

- Single-player only (one player vs dealer) — no multiplayer table
- No split or insurance actions — only hit, stand, and double down
- `53u8` sentinel in hand arrays means card index 53 is reserved

See also: [Best Practices](https://docs.arcium.com/developers/arcis/best-practices) for `Pack<T>` and performance guidance.

**Next**: [Ed25519](../ed25519/) — MPC for cryptographic operations, not just data
