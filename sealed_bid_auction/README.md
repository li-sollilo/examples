# Sealed-Bid Auction

Run first-price and Vickrey (second-price) auctions where bid amounts stay encrypted throughout. Individual bid amounts are never exposed on-chain — only the winner and payment amount are revealed after the auction closes.

## How it works

**Use this pattern when**: you need to determine a winner (or second-price clearing) without revealing losing bids.

The authority creates an auction with a type (first-price or Vickrey), min bid, and end time. Each bid triggers an encrypted computation that compares the encrypted bid against the current encrypted state — tracking both highest and second-highest bids — and writes the updated state back via callback.

```rust
#[instruction]
pub fn place_bid(
    bid_ctxt: Enc<Shared, Bid>,
    state_ctxt: Enc<Mxe, AuctionState>,
) -> Enc<Mxe, AuctionState>
```

The bid is encrypted to the bidder (`Shared`), the auction state is encrypted to the MXE. The comparison `bid.amount > state.highest_bid` happens entirely inside the MXE. After the auction closes, a separate instruction (`determine_winner_first_price` or `determine_winner_vickrey`) reveals only the winner's pubkey and payment amount. In Vickrey mode the winner pays the second-highest bid, so there's no incentive to bid below your true valuation.

## Concepts demonstrated

- **Encrypted state comparison**: comparing encrypted values against encrypted running state inside the MXE
- **`SerializedSolanaPublicKey`**: handling 32-byte Solana pubkeys via lo/hi `u128` splitting inside Arcis
- **Two winner-determination mechanisms**: first-price (pay own bid) vs Vickrey (pay second-highest) sharing the same encrypted state

## Run

```bash
yarn install
arcium build
arcium test
```

## Key files

- `encrypted-ixs/src/lib.rs` — the circuits: `init_auction_state`, `place_bid`, `determine_winner_first_price`, `determine_winner_vickrey`
- `programs/sealed_bid_auction/src/lib.rs` — the on-chain program
- `tests/sealed_bid_auction.ts` — end-to-end test

## Pitfalls

**`AuctionNotOpen` or `AuctionEnded` when placing a bid** — the auction has a time-based lifecycle. Check that the status is still `Open` and `end_time` hasn't passed.

**`NoBids` when determining winner** — at least one bid must be placed first.

## Limitations

- `min_bid` is stored on-chain but not enforced against encrypted bids — on-chain validation is impossible (the bid is encrypted), and circuit-side validation would need `min_bid` passed as plaintext
- No per-bidder deduplication — a bidder can submit multiple bids (non-exploitable in both pricing modes)

See also: [Arcis Types](https://docs.arcium.com/developers/arcis/types) for `SerializedSolanaPublicKey` reference.

**Next**: [Blackjack](../blackjack/) — complex multi-step protocol with Pack\<T\>
