# Voting — Encrypted Tallying

Tally votes without decrypting individual ballots. Votes stay encrypted throughout — only aggregate results are revealed by the poll authority.

## How it works

**Use this pattern when**: running totals must accumulate without revealing individual contributions.

A poll stores two encrypted `u64` counters (yes/no) as ciphertexts in an Anchor account, initialized via `init_vote_stats`. Each vote triggers an encrypted computation that decrypts the ballot and the current tallies *inside the MXE*, increments the matching counter, re-encrypts, and writes the result back via a callback. A `VoterRecord` PDA prevents double-voting — the account's existence proves a voter already voted.

```rust
#[instruction]
pub fn vote(
    vote_ctxt: Enc<Shared, UserVote>,
    vote_stats_ctxt: Enc<Mxe, VoteStats>,
) -> Enc<Mxe, VoteStats>
```

The voter's ballot is encrypted to them (`Shared`), the running tallies are encrypted to the MXE — neither is ever exposed in plaintext. The authority can call `reveal_result` at any time, which compares the encrypted totals and publishes only the boolean outcome (yes > no), never the raw counts.

## Concepts demonstrated

- **Encrypted state accumulation**: running counters stored as `Enc<Mxe, T>` ciphertexts, updated across multiple encrypted computations via callbacks
- **Authority-gated reveal**: only the poll creator can trigger result revelation
- **Double-vote prevention**: `VoterRecord` PDA initialized on first vote; a second attempt fails because the account already exists

## Run

```bash
yarn install
arcium build
arcium test
```

## Key files

- `encrypted-ixs/src/lib.rs` — the circuits: `init_vote_stats`, `vote`, `reveal_result`
- `programs/voting/src/lib.rs` — the on-chain program
- `tests/voting.ts` — end-to-end test

## Pitfalls

**`InvalidAuthority` on reveal** — only the wallet that created the poll can call `reveal_result`. If testing with a different keypair, recreate the poll or switch wallets.

## Limitations

- Reveals only a boolean (yes > no), not the raw vote counts — by design, to minimize information leakage
- No vote weighting or ranked-choice — the circuit processes a single `bool` per vote
- No poll close or deadline — the authority can reveal at any time, and voters can continue voting after reveal

See also: [Callback Type Generation](https://docs.arcium.com/developers/program/callback-type-generation) for how computation results land back on-chain.

**Next**: [Sealed-Bid Auction](../sealed_bid_auction/) — encrypted comparison with multiple mechanisms | **Optional branch**: [Medical Records](../share_medical_records/) — re-encryption pattern
