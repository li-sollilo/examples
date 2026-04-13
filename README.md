# Arcium Examples

Example programs showing how to build encrypted applications on Solana with [Arcium](https://arcium.com) MPC.

## First time? Run this.

```bash
git clone https://github.com/arcium-hq/examples
cd examples/coinflip
yarn install
arcium build
arcium test
```

The first `arcium test` is slow (pulls the trusted dealer Docker image, 3-5 min); subsequent runs are seconds.

> Every `arcium test` spins up actual MPC nodes in Docker containers on your machine — no mocked computations.

## Examples

| Example | Pattern | Difficulty | Stateful | Main concept | Best for |
|---|---|:-:|:-:|---|---|
| [Coinflip](./coinflip/) | Stateless random | Beginner | No | `ArcisRNG`, encrypted input comparison | Your first Arcium example |
| [RPS vs House](./rock_paper_scissors/against-house/) | Stateless random | Beginner | No | MPC-generated random opponent | Comparing with coinflip |
| [RPS vs Player](./rock_paper_scissors/against-player/) | Multi-party hidden input | Intermediate | Yes | Two encrypted submissions, async reveal | Hidden information games |
| [Voting](./voting/) | Encrypted state accumulation | Intermediate | Yes | `Enc<Mxe, T>` state, callbacks | Encrypted on-chain state |
| [Medical Records](./share_medical_records/) | Re-encryption | Intermediate | No | Patient-controlled data sharing | Access-controlled data |
| [Sealed-Bid Auction](./sealed_bid_auction/) | Encrypted comparison | Advanced | Yes | First-price + Vickrey mechanisms, multi-instruction | Encrypted bidding / matching |
| [Blackjack](./blackjack/) | Encrypted game state | Advanced | Yes | `Pack<T>` for efficient storage, 6 instructions | Complex multi-step protocols |
| [Ed25519](./ed25519/) | Distributed signing | Advanced | No | `MXESigningKey`, threshold signatures | Distributed key management |

**Recommended path**: Coinflip -> Voting -> Sealed-Bid Auction -> Blackjack -> Ed25519

See also [Rock Paper Scissors](./rock_paper_scissors/) for the family overview comparing the two RPS variants.

## Environment

| Requirement | Minimum version | Install |
|---|---|---|
| Rust | 1.89.0 | [rustup.rs](https://rustup.rs) (pinned via `rust-toolchain.toml`) |
| Anchor | 0.32.1 | [anchor-lang.com](https://www.anchor-lang.com/docs/installation) |
| Arcium CLI | 0.9.6 | [docs.arcium.com/developers/installation](https://docs.arcium.com/developers/installation) |
| Solana CLI / Agave | 2.x | [docs.anza.xyz/cli/install](https://docs.anza.xyz/cli/install) |
| Docker | Any recent | [docker.com](https://docs.docker.com/get-docker/) (MPC nodes run in containers) |
| Node.js | LTS recommended | [nodejs.org](https://nodejs.org) (no engine pin in repo) |
| Yarn | 1.x | `npm install -g yarn` (no version pin in repo) |

- **Wallet**: Solana keypair at `~/.config/solana/id.json`
- **Package manager**: Examples use Yarn. Anchor supports alternatives via `toolchain.package_manager` -- if you switch, update `Anchor.toml` and `[scripts]`.

## Each example contains

```
example/
  encrypted-ixs/src/lib.rs   # MPC circuit (Arcis)
  programs/*/src/lib.rs       # Solana program (Anchor)
  tests/*.ts                  # End-to-end test
  Arcium.toml                 # Localnet MPC config
  Anchor.toml                 # Solana program config
```

## Run

Every example follows the same three commands:

```bash
yarn install
arcium build
arcium test
```

`arcium build` compiles circuits and syncs program IDs. `arcium test` spins up a local MPC cluster in Docker and runs the end-to-end test.

## Troubleshooting

These apply to all examples:

**`arcium test` first run is slow** -- The first run pulls the trusted dealer Docker image and warms up the localnet cluster (3-5 min). Subsequent runs are seconds. `arcium build` itself is fast -- it only compiles + syncs keys.

**Program ID drift** -- `arcium build` automatically syncs `declare_id!()` and `Anchor.toml` to the keypair at `target/deploy/`. If you see `DeclaredProgramIdMismatch`, run `arcium keys sync`.

**Test hangs at "Waiting for MPC computation"** -- Localnet ARX containers may have crashed. Inspect: `docker ps | grep arx`. If unhealthy: `docker stop $(docker ps -q --filter "name=arx-")`, then re-run `arcium test`. Note: `arcium clean` does NOT stop Docker containers.

**Need a bigger mempool?** -- Default is `medium`. Pass `--mempool-size large` (or `tiny|small|medium|large`) to `arcium test`. Localnet only.

## Documentation

- [Mental Model](https://docs.arcium.com/developers/arcis/mental-model) -- how Arcium MPC works
- [Computation Lifecycle](https://docs.arcium.com/developers/computation-lifecycle) -- queue -> MPC -> callback flow
- [Arcis Framework](https://docs.arcium.com/developers/arcis) -- encrypted instruction reference
- [Best Practices](https://docs.arcium.com/developers/arcis/best-practices) -- performance and design guidance

For questions and support, join the [Discord community](https://discord.com/invite/arcium).

---

*Maintainers: see [`scripts/sync-version.sh`](./scripts/sync-version.sh) for bumping the Arcium version across all examples.*
