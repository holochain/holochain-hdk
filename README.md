# holochain-hdk

The Holochain Development Kit (`hdk`) for coordinator zomes, and the crates
it is built from.

| Crate                  | Licence    | Purpose                                        |
| ---------------------- | ---------- | ---------------------------------------------- |
| `hdk`                  | CAL-1.0    | Coordinator zome SDK, compiled to WASM         |
| `holochain_zome_types` | Apache-2.0 | Types shared by coordinator zomes and the host |
| `holochain_nonce`      | Apache-2.0 | Nonce generation                               |

All crates share one version, set in the root `Cargo.toml`.

## Development

Install [cargo-make](https://github.com/sagiegurari/cargo-make) and
[taplo](https://taplo.tamasfe.dev/), then:

```bash
cargo make static   # formatting, clippy, docs, WASM build
cargo make test     # tests with all features
cargo make verify   # both
```

## Local development against holochain-hdi

Clone [holochain-hdi](https://github.com/holochain/holochain-hdi) next to
this repository and uncomment the `[patch.crates-io]` entries in the root
`Cargo.toml`. Re-comment them before committing.

## Releasing

Releases are prepared and published by the
[holochain release integration](https://github.com/holochain/release-integration).
Run the "Prepare a release" workflow, review the PR it opens, and merge it.
Publishing to crates.io happens on merge.

## History

Commits before the split were extracted from
[holochain/holochain](https://github.com/holochain/holochain) with
`git filter-repo`; blame and log work across the split.
