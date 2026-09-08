# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[[0.9.0-dev.0](https://github.com/holochain/holochain-hdk/commits/v0.9.0-dev.0)\] - 2026-09-08

### Features

- *(hc)* Export the TypeScript bindings with an hc subcommand (#5959) by @veeso
  - Feat(hc): export the TypeScript bindings with an hc subcommand
  - Replace the ts_rs-gated export-ts-bindings binary with a built-in hc export-ts-bindings subcommand, so every hc build (including Holonix's stock packages.hc) can write the conductor API TypeScript bindings for holochain-client-js without enabling a feature or naming a second binary. The output directory is a flag (--out-dir, default ./bindings); the tree is staged and only replaces the directory once the export succeeds, and the command refuses to replace the working directory or an ancestor.
  - Hc now depends on holochain_conductor_api with ts_rs always on, which through feature unification compiles the type crates with ts_rs in every workspace build. To keep that side-effect free, drop the export flag from every #[ts(export, export_to = ...)] (export_to alone still places the declaration; the tree is driven by the export_ts_bindings chain), remove the two manual export tests and the .cargo/config.toml env block that only served them.
  - Unstable-countersigning stays an hc feature: a workspace build would unify it into holochain's holochain_conductor_api while holochain's own feature is off, and its exhaustive match on AppRequest would not compile. make ts-bindings enables it; the ts-bindings-test target now runs the hc tests that check the written tree.
- Export the conductor API types to TypeScript with ts-rs (#5934) by @veeso
  - Feat: export the conductor API types to TypeScript with ts-rs
  - Add an opt-in ts_rs cargo feature across the API type crates that generates TypeScript declarations for the conductor admin and app interfaces, with hash types exported as byte arrays, timestamps as plain numbers, and kitsune2 network types replaced by holochain-owned mirrors. The export directory is chosen via TS_RS_EXPORT_DIR (scripts/export-ts-bindings.sh / make ts-bindings).
- Implement the new DNA migration design with init properties (#5831) by @cdunster
  - Feat: add init properties install parameter for chain migration
  - # Conflicts: #	crates/holochain/CHANGELOG.md
  - Feat: add init properties to the conductor database in a new table
  - Feat: add get_init_properties host function and HDK wrapper
  - Feat: clear init properties after use in init
  - Test: use init properties in migration test WASMs and update tests
  - Docs: combine all changelog entries about DNA migration into one entry
- Add ChainStatus::Closed for closed source chains (#5832) by @ThetaSinner
  - Feat: add ChainStatus::Closed variant and combine_chain_status helper
  - Feat: detect closed chain head and report ChainStatus::Closed
  - Feat: carry ChainStatus::Closed through agent-activity merge
  - Fix: status-only agent activity requests now return the real status
  - Docs: changelog for ChainStatus::Closed and status-only fix
  - Style: cargo fmt for ChainStatus::Closed changes
  - Style: move activity.rs test module to end of file
  - Clippy's items_after_test_module (a -D clippy::style deny) rejects items declared after a #[cfg(test)] mod. The status_only test module was inserted before the ChainItemsSource trait; relocate it to the end of the file.
  - Fixup! feat: add ChainStatus::Closed variant and combine_chain_status helper
  - Fixup! docs: changelog for ChainStatus::Closed and status-only fix
  - Fixup! feat: add ChainStatus::Closed variant and combine_chain_status helper
  - Fixup! feat: detect closed chain head and report ChainStatus::Closed
  - Fixup! docs: changelog for ChainStatus::Closed and status-only fix
- Migrate to v2 action/record data model (#5730) (#5822) by @ThetaSinner
  - Docs: design for v2 data-model migration (phase 0)
  - Refactor(integrity): scaffold dht_v2 record/op submodules
  - Feat(integrity): add v2 Record type
  - Feat(integrity): add ActionData-based v2 Op with validating constructors
  - Feat(integrity): add v2 Op header/data accessors
  - Test(integrity): serde + hashing tests for v2 Record and Op
  - Refactor(hdi): scaffold flat_op_v2 staging module
  - Feat(hdi): add v2 OpRecord over dht_v2::Action
  - Feat(hdi): add v2 OpEntry/OpUpdate/OpDelete over dht_v2::Action
  - Feat(hdi): add v2 OpActivity over dht_v2::Action
  - Feat(hdi): add v2 FlatOp enum over dht_v2 op types
  - Style(hdi): cargo fmt flat_op_v2 submodules
  - Refactor(hdi): expose flatten helpers and scaffold op_v2 trait
  - Feat(hdi): flatten v2 Op into v2 FlatOp
  - Style(hdi): cargo fmt op_v2 and widened helper signature
  - Test(hdi): cover v2 flattened conversion
  - Style(hdi): enforce missing_docs on v2 staging modules
  - Docs: design for v2 migration phase 1 (cascade reads + wire cutover)
  - Feat(state): DhtStore::retrieve_action
  - Feat(state): DhtStore::retrieve_entry
  - Feat(state): DhtStore::retrieve_record
  - Feat(state): DhtStore::get_live_record
  - Feat(holochain_data): add get_live_entry_creates query
  - Feat(state): DhtStore::get_live_entry
  - Feat(holochain_data): add delete/update-actions-for-record queries
  - Feat(state): DhtStore::get_record_details
  - Feat(holochain_data): add entry-creates and by-entry delete/update queries
  - Feat(state): DhtStore::get_entry_details
  - Feat(holochain_data): add get_live_link_actions query
  - Feat(state): DhtStore::get_links
  - Feat(holochain_data): add link-create and delete-link-actions queries
  - Feat(state): DhtStore::get_link_details
  - Docs(v2-migration): design agent-activity 1a reads
  - Pin the design for the highest-risk 1a reads: split agent activity into 1a-viii (get_agent_activity, executed first) and 1a-ix (must_get_agent_activity dht-only core), both store-only. Fix the SQL-vs-Rust boundary (scans in holochain_data; pure assembly moved from holochain_cascade into holochain_state), the rich LEFT JOIN Entry scan for Full mode, single Full/Hashes DhtStore method, separate warrant fetch, and legacy return boundary.
  - Docs(v2-migration): agent-activity reads scan ChainOp only
  - Settle the pending-ops question: v2 get_agent_activity scans the integrated ChainOp table only (validated activity), dropping the legacy behavior where pending LimboChainOp ops raised highest_observed. Pin the holochain_state-local options struct (GetActivityOptions lives in holochain_p2p, off the holochain_state dep graph) and reuse the existing get_warrants_by_warrantee primitive.
  - Feat(holochain_data): add get_agent_activity scan
  - Feat(state): DhtStore::get_agent_activity
  - Test(state): get_agent_activity rejected, fork, full, warrants
  - Feat(holochain_data): add must_get_agent_activity scans
  - Feat(state): DhtStore::must_get_agent_activity
  - Test(state): must_get_agent_activity response variants
  - Docs(v2-migration): reshape Phase 1 tail around data-serving model
  - Data serving is about records, not ops: serve actions/entries with a record-level validation status + warrants, with the invariant that a rejected record always carries a proving warrant (checked up front by the receiver). The receiver expands valid records to ops (produce_ops_from_record) filtered by request type + storage arc and re-validates locally; rejected+warranted records go to limbo.
  - Because the serving wire type is shared by authority, holochain_p2p, and requester, reshaping it is atomic across the three crates, so the old authority (1b) and requester (1c) slices merge into one phased-commit slice 1b. The old gossip wire-break becomes the new final phase 1c.
  - Fix(state): integrate_ready_ops populates action indexes
  - Test(state): integration indexes delete-link
  - Feat(holochain_data): authority link reads (locally_validated)
  - Feat(state): DhtStore authority link reads
  - Fix(holochain_data): upgrade cached ChainOp to locally_validated on integration
  - Feat(holochain_data): authority record reads (locally_validated)
  - Feat(state): DhtStore authority record reads
  - Feat(holochain_data): authority entry reads (locally_validated)
  - Feat(state): DhtStore authority entry reads
  - Docs(v2-migration): design the 1b-vi data-serving wire reshape
  - The authority-serving DhtStore reads (locally_validated-guarded) are done; what remains is the wire cutover. Record the finding that the wire reshape and the requester's expand-to-ops are the same change (WireOps::render becomes produce_ops_from_record), so they are one slice; the reshaped wire shape (Judged<SignedActionHashed> + warrants, dropping the op-shaped forms); and the decision to land it as one monolithic, tightly-coupled commit across holochain_types + holochain_cascade + holochain_p2p, executed inline.
  - Feat(cascade): serve records (not ops) on the data-serving wire
  - Reshape the get authorities' data-serving wire from op-shaped forms to records. WireRecordOps/WireEntryOps/WireLinkOps now carry Judged<SignedAction> (with their record-level validation status) plus a warrants field, instead of the op-shaped WireDelete/WireCreateLink/etc. A Rejected record is always paired with a warrant proving it; the requester checks that invariant up front so a malicious peer cannot force pointless validation work, before caching Valid ops + warrants.
  - The authority handlers are rewritten onto the locally-validated DhtStore reads (get_authority_*) + retrieve_entry, with the production Cell caller passing space.dht_store.as_read(); a public get_warrants_by_warrantee is added to DhtStoreRead for warrant pairing. WireOps::render now rebuilds the request-relevant op per served action rather than reassembling op-shaped forms.
  - Removes the legacy authority Query structs and the legacy-DbKindDht cascade integration-test layer (PassThroughNetwork + op-shaped test-data builders), which could not be bridged to a holochain_data DhtStore; focused new-path unit tests cover the rejected-requires-warrant invariant. The requester's local read path keeps CascadeTxnWrapper for now (a separate cutover). See docs/design/v2_model_migration.md for the deviations and follow-ups.
  - Docs(v2-migration): design the 1c op + action hash cutover
  - Expand the Phase 1 1c section into the full op + action hash cutover: action and op hashes become content-derived v2 (no weight), op construction moves to the v2 dht_v2 op types, the gossip wire carries v2 ops, and the network-path legacy<->v2 conversions + hash-preservation hack are deleted.
  - Records it as an identity flip (coordinated, green only at the end) rather than additive reads, with two additive foundation slices (1c-i v2 op-hash, 1c-ii v2 produce-ops) preceding the coordinated cutover (1c-iii). Notes the source-chain action-hash seam as the corner to trace when planning 1c-iii.
  - Feat(types): content-derived v2 op hash (ChainOp::to_hash)
  - Feat(types): v2 op-hash/basis/op-type primitives + AnyLinkableHash basis
  - Feat(types): v2 produce_ops_from_record -> Vec<HashedChainOp>
  - Docs(v2-migration): resolve the 1c-iii boundary
  - Record the 1c-iii scope decision: "data-layer v2, authoring keeps legacy types". Action and op hashes become content-derived v2 by flipping the canonical hash (HashableContent for Action + the op ChainOpUniqueForm) to the weightless v2 projection, so everything on disk and wire is v2 and prev_action chains in v2, while the ActionBuilder/scratch/ribosome keep legacy Action types (full v2-native authoring stays Phase 3).
  - Notes the shape: a single coordinated identity flip (green only at the end, executed inline like 1b-vi), with test fallout as the bulk of the work; dropping weight from the hash is collision-safe and the v2 projection is total via from_legacy_signed_action.
  - Feat: content-derived v2 action/op hashes + v2 gossip wire
  - Flip the canonical ActionHash and DhtOpHash to the v2 projection of the action (ActionHeader + ActionData, no weight), so data on disk and on the gossip wire carries v2 hashes and prev_action chains in v2. Authoring still builds legacy Action types; only the hashed identity is flipped (full v2-native authoring stays Phase 3).
  - Add holochain_integrity_types::dht_v2::from_legacy_action, the total   legacy->v2 action projection; from_legacy_signed_action reuses it. - HashableContent for the legacy Action, the per-ref action impls, and   EntryCreationAction now hash the v2 projection (drop the dead ActionRef). - The legacy op ChainOpUniqueForm hashes the v2 projection (via the v2   ChainOpUniqueForm::op_hash); route ChainOp::hashable_content through the   unique form so both hash paths agree byte-for-byte with ChainOp::to_hash. - op_store: encode/decode the v2 DhtOp on the gossip wire natively, building   it straight from the stored rows (delete the legacy reconstruction); op ids   use the native v2 rehash. Add dht_v2::DhtOp::{to_hash,dht_basis} and   dht_v2::to_legacy_dht_op. - Ripple handle_publish / the p2p event signatures to Vec<dht_v2::DhtOp>;   Space::handle_publish reconstructs the legacy op for the still-legacy DHT   table + record_incoming_ops during the migration.
  - Docs(v2-migration): mark 1c-iii / Phase 1 complete
  - 1c-iii (the content-derived v2 action+op hash flip + v2 gossip wire) has landed, completing the 1c sub-slice and Phase 1. Records the one remaining transitional bridge (receive-side legacy reconstruction for the still-legacy ingest), which Phase 2 removes.
  - Feat(state): retrieve_* scratch overlay reads
  - Feat(state): get_live_* scratch overlay reads
  - Feat(state): get_*_details scratch overlay reads
  - Feat(state): get_links* scratch overlay reads
  - Refactor(state): address CR-1 scratch-overlay review
  - Collapse `scratch_deletes_targeting` into `scratch_delete_targets().contains()`   (one scratch-scan helper, not two). - Use the module-level `RecordValidity` / `ValidationStatus` imports in the   details overlays, matching their store-only siblings. - Note that `scratch_creates_for_entry` includes tombstoned creates (vs   `scratch_live_entry_creates`), since `EntryDetails.actions` lists all creates.
  - Feat(state): get_agent_activity scratch overlay
  - Feat(state): must_get_agent_activity scratch overlay
  - Refactor(state): address CR-1-aa agent-activity review
  - Convert the action once per item in `get_agent_activity_with_scratch` (match   on validity first, dropping the redundant `to_legacy_signed_action` calls). - Document that valid records carry no entry (cascade `cached_entry: None`   convention) while rejected records keep theirs. - `agent_activity_from_scratch` takes `Option<u32>` for the chain-top bound   instead of a `u32::MAX` sentinel.
  - Feat(cascade): wire live record/entry reads to DhtStore
  - Feat(cascade): wire details reads to DhtStore
  - `get_entry_details` and `get_record_details` now take the `dht_store`-present path: optional network fetch (honouring `GetStrategy`/`am_i_authoring`/`am_i_an_authority` gating) followed by a local read via `DhtStoreRead::get_entry_details_with_scratch` / `get_record_details_with_scratch`.  The legacy `get_latest_with_query` path is kept as the `dht_store`-absent fallback until CR-3.
  - Two new tests in `dht_store_scratch_overlay_tests` cover: - `get_record_details_reflects_scratch_delete` — scratch Delete on an   integrated record appears in `RecordDetails.deletes`. - `get_entry_details_reflects_scratch_delete` — scratch Delete flips   `entry_dht_status` to Dead in `EntryDetails`.
  - Feat(cascade): wire link reads to DhtStore
  - Feat(cascade): wire agent-activity reads to DhtStore
  - Fix(state): hide private entries from non-authors in record reads
  - The requester record reads attached an entry resolved purely by hash, so a private entry could leak to a non-author whenever that caller happened to hold a same-hash private entry of their own (the entry hash is shared by content, but the privacy is per author). The by-hash `get_entry` lookup alone does not provide the legacy read's entry-visibility hiding.
  - `retrieve_record` / `retrieve_record_with_scratch` now attach a private entry only when the caller is the action's author; otherwise the entry is `Hidden` (the action itself stays visible). Regression test `retrieve_record_hides_private_entry_from_non_author`.
  - Fix(cascade-read): resolve full-suite regressions from the DhtStore cutover
  - The CR-2 cascade read cutover (local reads now via DhtStore *_with_scratch) surfaced 16 failures across workflows, host fns and sweettests. Two were real read-path bugs in holochain_state; the rest were tests seeding the legacy DBs the cascade no longer reads.
  - Production read fixes (dht_store/reads.rs): - get_record_details_with_scratch gated on an integrated StoreRecord op before   consulting the scratch, so a scratch-only record (an author's just-created   entry mid-call) returned None. This broke in-call update->create resolution   and, via stalled validation dependency resolution, the consistency   sweettests and multi_create_link_validation. Resolve the record first;   scratch-only records are Valid from the author's view. - get_links / get_links_with_scratch sort by timestamp, matching the legacy   GetLinksQuery creation-order contract.
  - Test seeding migrated to the DhtStore (the cascade's new local source): - get_zomes_to_invoke, main_workflow, run_validation_callback,   must_get_valid_record_short_circuit, adds_init_marker now seed via   record_incoming_ops / cache_chain_ops (+reject_chain_ops) / genesis-into-store   instead of insert_op_dht / insert_op_cache. reject_chain_ops only transitions   network-cached ops, so rejected-record cases use cache_chain_ops (integrated),   not record_incoming_ops (limbo). - must_get_entry rewritten to reject a cached (network-fetched) record rather   than an authored op (reject_chain_ops is a no-op on locally-validated ops by   design) -- the realistic rejected-record scenario.
  - Add a regression test that the op hash is weight-independent: the v2 action projection drops weight, so the op hash must survive the v2 round-trip.
  - Refactor(cascade): remove the legacy dht_store-None read path (CR-3)
  - Now that CR-1/CR-2 made the DhtStore the live local-read source, delete the legacy fallback that is no longer reachable in production:
  - Make `dht_store` mandatory on `CascadeImpl` (drop the `Option`); `empty` now   takes a `DhtStore`. Remove the read-only `authored`/`dht` `DbRead` fields and   the `with_authored`/`with_dht` builders. - Delete the `dht_store`-`None` fallback arms in every read method, plus the   legacy read helpers they used (`cascading`, `find_map`,   `get_local_first_with_query`, `get_latest_with_query`) and the now-dead   cascade duration metric. - Delete the unused `authority::get_agent_activity_query` module. - Update cascade constructors at their call sites (sys_validation   `local_cascade`/`network_and_cache_cascade`, app_validation `full_cascade`,   countersigning) to the `empty(dht_store)` shape. `local_cascade` previously   omitted the DhtStore and silently used the legacy path.
  - The `cache` field and the network-fetch cache write (`merge_ops_into_cache`) are kept; `CascadeTxnWrapper` / `DbScratch` / `Query` in holochain_state remain (still used by source_chain authoring) -- their removal is a later phase.
  - Fix two sys-validation unit tests to seed a held dependency as a cached op (`save_chain_op_as_cached`) rather than via `record_incoming_ops`: the latter queues the dependency itself for sys validation, so the workflow would try to validate it and fetch its prev_action from the network.
  - Refactor(publish): read the publish queue from the DhtStore (Phase 2a)
  - Move the publish workflow off the legacy authored database onto the v2 DhtStore. `get_ops_to_publish` / `num_still_needing_publish` are now SQL queries over ChainOp/Action/ChainOpPublish (and WarrantOp/Warrant/ WarrantPublish) in holochain_data, surfaced as DhtStoreRead methods.
  - The private-entry leak guard is preserved verbatim: StoreEntry ops whose action carries a private entry are never returned for publishing.
  - Warrants live in a separate publish table in the v2 schema, so record_published_op_hashes now records warrant publish times in WarrantPublish (warrants still publish once); the legacy single DhtOp.last_publish_time covered both.
  - The publish workflow and its consumer no longer take a DbKindAuthored handle. The legacy publish_query module is removed; its coverage moves to DhtStore-backed tests beside the workflow.
  - Refactor(genesis): read genesis status from the DhtStore
- Serve K2 DHT data from new holochain_data DHT store (#5792) by @ThetaSinner
  - Feat: serve K2 DHT data from new holochain_data DHT store (#5731)
  - Switch all Kitsune2 op-store reads in holochain_p2p from the old holochain_sqlite DHT/Cache databases to the new holochain_data per-DNA DHT database via holochain_state::DhtStore.
  - Schema additions to the pre-stable DHT migration: - New SliceHash table (arc bounds + slice_index -> hash) - Warrant: + signature, when_integrated, serialized_size - LimboWarrant: + signature
  - New holochain_data::dht::inner::k2_reads module covers the six cross-table K2 reads; slice_hash covers the four SliceHash ops. DhtStore (generic over Db: AsRef<DbRead<Dht>>) exposes them so both DhtStoreRead and the write variant share the read methods.
- Include rejection reason in InvalidChainOp warrants (#5797) by @veeso
  - Add a `reason: String` field to `ChainIntegrityWarrant::InvalidChainOp` so peers receiving a warrant can see why sys/app validation rejected the op.
  - Truncate reasons to `MAX_WARRANT_REASON_BYTES` (512) at warrant creation   and reject oversized reasons during warrant validation to prevent   griefing via oversized payloads. - Exclude `reason` from `PartialEq`/`Hash` so warrant deduplication still   collapses warrants for the same op regardless of the reason text. - Plumb the rejection reason through sys and app validation workflows   into `make_invalid_chain_warrant_op`.
- Parallel writes from cache to new unified DhtStore (#5779) by @ThetaSinner
  - Feat(cascade,state): add optional DhtStore field to CascadeImpl; thread through HostFnWorkspace
  - Add an optional DhtStore<DbWrite<Dht>> field to CascadeImpl along with a with_dht_store builder. Wire dht_store as a stored field on HostFnWorkspace and expose it via HostFnStores so cascades constructed from a workspace inherit the mirror target. The Option is None for test cascades and workspaces that do not have a writable DhtStore.
  - This is the plumbing for mirroring cache writes into the new unified per-DNA DhtStore; subsequent commits use it.
  - Feat(state): add DhtStore cache mirror methods in dht_store/cache.rs
  - Adds record_cached_chain_ops, record_cached_activity_ops, and record_incoming_cached_warrants on DhtStore<DbWrite<Dht>>. Cached chain ops go directly into ChainOp with locally_validated=false; warrants are inserted into LimboWarrant for local validation.
  - Feat(cascade): mirror merge_ops_into_cache and merge_link_ops_into_cache to DhtStore
  - Pre-render WireOps / WireLinkOps responses outside the legacy cache transaction so the rendered data is available to both the legacy DbKindCache write and the new DhtStore mirror.
  - After the legacy cache write succeeds, mirror each RenderedOps to the new per-DNA DhtStore (if configured):   - record_cached_chain_ops when ops or entry are present   - record_incoming_cached_warrants for any embedded warrant
  - The legacy cache write remains authoritative. Mirror failures are logged via tracing::warn! and swallowed so a misbehaving mirror cannot break the cascade. Reads still hit the legacy cache.
  - Pre-rendering means a WireOps::render / WireLinkOps::render failure now short-circuits before the legacy transaction starts, rather than rolling it back - strictly better behaviour.
  - Feat(cascade): mirror add_activity_into_cache to DhtStore
  - Builds Vec<DhtOpHashed> for activity and warrants, then mirrors via record_cached_activity_ops and record_incoming_cached_warrants after the legacy cache write succeeds. Mirror failures are logged at warn level and do not propagate.
  - Feat(holochain): wire DhtStore into cascade construction at workflow sites
  - After this commit, every production cascade build path supplies the new DhtStore mirror target alongside the legacy cache, so cached chain ops and warrants are mirrored into the unified per-DNA DhtStore.
  - Sites updated: - AppValidationWorkspace::full_cascade - SysValidationWorkspace::network_and_cache_cascade - countersigning_workflow::incomplete cascade
  - Fixup! feat(state): add DhtStore cache mirror methods in dht_store/cache.rs
  - Fixup! feat(cascade): mirror add_activity_into_cache to DhtStore
  - Fixup! feat(state): add DhtStore cache mirror methods in dht_store/cache.rs
  - Fixup! feat(cascade,state): add optional DhtStore field to CascadeImpl; thread through HostFnWorkspace
  - Test(ci): bump nextest retries 1→2 for Windows headroom
  - Windows CI consistently surfaces transient flakes in schedule, warrants, p2p node_messaging and metrics tests that pass on a second attempt. With retries = 1 (2 total attempts) one of those can still escape as a hard fail if the runner is contended; bumping to retries = 2 (3 attempts) absorbs the long tail without changing test logic.
  - No effect on tests that pass first try.
- New DHT database schema and skeleton API (#5743) by @ThetaSinner
  - Feat(integrity-types): scaffold dht_v2 with RecordValidity and ActionType
  - Feat(integrity-types): add dht_v2 ActionHeader and per-variant data structs
  - Feat(integrity-types): add dht_v2 ActionData and Action with HashableContent impl
  - Feat(zome-types): add dht_v2 module with SignedAction/SignedActionHashed and ChainOpType i64 mapping
  - Fix(zome-types): correct ChainOpType 6/7 mapping to match state_model ordering
  - Test(zome-types): pin ChainOpType i64 mapping with forward-direction assertions
  - Feat(types): add dht_v2 OpEntry, ChainOp, WarrantOp and DhtOp
  - Feat(types): add dht_v2 HashedChainOp
  - Feat(holochain-data): wire DbKind::Dht and DHT_MIGRATOR
  - # Conflicts: #	crates/holochain_data/src/kind.rs #	crates/holochain_data/src/lib.rs
  - Feat(holochain-data): add initial DHT schema migration
  - Feat(holochain-data): add DHT row models
  - Feat(holochain-data): add Action insert/read primitives with round-trip tests
  - Feat(holochain-data): add Entry and PrivateEntry insert/read with isolation and tx tests
  - Feat(holochain-data): add CapGrant and CapClaim operations with FK test
  - Feat(holochain-data): add ChainLock acquire/read/release/prune operations
  - Feat(holochain-data): add LimboChainOp insert/read/delete with state-based queries
  - Feat(holochain-data): add LimboWarrant insert/read/delete with state-based queries
  - Feat(holochain-data): add ChainOp insert/read with basis and action queries
  - Feat(holochain-data): add Warrant insert/read with warrantee lookup
  - Feat(holochain-data): add ChainOpPublish, WarrantPublish, and ValidationReceipt operations
  - Feat(holochain-data): add Link/DeletedLink/UpdatedRecord/DeletedRecord operations with CASCADE test
  - Test(holochain-data): cover every ActionData variant through round-trip
  - Docs(holochain-data): add changelog entry for DHT schema and skeleton API
  - Style: cargo fmt and elide needless lifetimes
  - Chore: review changes
  - Chore: review changes
  - Chore: remove redundant 'TxWrite-only' module comments
  - Docs(holochain-data): replace private intra-doc links with code formatting
  - The static-doc check uses RUSTDOCFLAGS=-Dwarnings, and rustdoc was warning that public docs link to private items (`inner`, `db_operations`, `tx_operations`, and `super::super::inner::chain_lock::acquire_chain_lock`). Demote those references to plain code spans.
  - Chore(holochain-data): drop dead DEFAULT 0, document FK convention, close test gaps
  - Remove DEFAULT 0 from ChainOp.serialized_size (the insert always binds   it, so the default was unreachable). Mirror the change in   state_model.md so the design and schema agree. - Add a foreign-key delete-behaviour note to the migration header   explaining why the index tables cascade and the rest do not. - Add actions_by_author_excludes_other_authors covering the negative   case for the author filter, and cap_grants_ordered_by_action_seq   asserting the ORDER BY Action.seq tiebreaker for both grant queries.
  - Test: Improve `actions_by_author` and `actions_by_author_excludes_other_authors` so that actions are compared directly rather than just some fields
- Wasmer upgrade (#5717) by @ThetaSinner
  - Feat: wasmer upgrade
  - # Conflicts: #	Cargo.lock #	crates/client/Cargo.toml #	crates/hc_bundle/Cargo.toml #	crates/holochain/Cargo.toml #	crates/holochain_cascade/Cargo.toml #	crates/holochain_conductor_api/Cargo.toml #	crates/holochain_data/Cargo.toml #	crates/holochain_keystore/Cargo.toml #	crates/holochain_p2p/Cargo.toml #	crates/holochain_state/Cargo.toml #	crates/holochain_types/Cargo.toml #	crates/holochain_websocket/Cargo.toml #	crates/test_utils/wasm_common/Cargo.toml
  - # Conflicts: #	crates/holochain_data/Cargo.toml
  - Chore: progress with dependency upgrades
  - Chore: Integrate the wasmer upgrade
  - Chore: lint fixes
  - Chore: lint fixes
  - Chore: review changes
  - Chore: update Rust version for nix
  - Chore: address PR review comments
  - Chore: update weekly Rust version
  - Chore: fix build
  - Chore: review changes
  - Chore: improve changelog
- Remove `block_agent` and `unblock_agent` from the HDK (#5534) by @ThetaSinner
  - Feat: Remove `block_agent` and `unblock_agent` from the HDK
  - Fix: remove orphaned code and blocks module reference
  - Removed reference to blocks module in test mod.rs - Cleaned up orphaned code in capability coordinator test WASM - All formatting and clippy checks now pass
  - Chore: log if block_agent or unblock_agent called
  - Chore: address clippy issues
- Support `query` with seq limits and other filters (#5384) by @ThetaSinner
  - Feat: Support `query` with seq limits and other filters
  - Chore: next steps of query and test updates
  - Chore: fix build
  - Chore: fix all the broken-ness
  - Docs: Update changelog
  - Test: Remove debug query
  - Chore: Review improvements
- Implement kitsune2 blocks trait (#5300) by @jost-s
  - Feat: add Blocks implementation to holochain_p2p for network block management
  - Feat: add block method to HcP2p
  - Chore: remove obsolete kitsune_p2p_block types
  - Test: hdk call block_agent creates a block in database
  - Refactor: move holochain-based blocks to p2p actor
  - Docs: change Blocks::block function body comment
- Add query are_all_blocked to holochain_state (#5281) by @jost-s
  - Feat: add query are_all_blocked
  - Refactor: upgrade kitsune2 to 0.3.0
  - Docs: mark block target id as doc link in query_are_all_blocked
  - Refactor: improve vector initialization in query_are_all_blocked
  - Docs: fix changelog punctuation
  - Docs: fix comment in test query_are_all_blocked_false_when_outside_of_interval
- Add `ChainOpType` to chain integrity warrants (#5236) by @ThetaSinner
- *(admin-api)* Added a new `revoke_zome_call_capability` endpoint to the Admin API (#5136) by @veeso
  - Feat(admin-api): Added a new `revoke_zome_call_capability` endpoint to the Admin API
  - The new endpoint allows you to revoke a previously granted Zome Capability
- \[**BREAKING**\] Removed Hash host function (#5129) by @veeso
  - Feat(holochain)!: Removed `hash` from Host functions
- Integrate k2 (#4791) by @ThetaSinner
  - Feat: Add local agent (#4725)
  - Feat: Add local agent
  - Chore: Update changelog
  - Chore: Build on integration PRs
  - Chore: Rename `client` to `keystore_client` in the HolochainP2pLocalAgent
  - Remove legacy kitsune (#4728)
  - Remove legacy kitsune  * remove origin_time and toml-fix  * clippy
  - Holochain implementation of op store (#4720)
  - Feat: Implement OpStore for Holochain  * feat: Implement slice hash logic for the Holochain op store  * feat: Update host integration to "publish" ops rather than storing them directly  * chore: Clippy fix  * chore: Clippy fix  * chore: Clippy unstable fixes  * chore: Review changes  * chore: Review changes  ---------
- Don't use wasmer module cache with wasmer_wamr feature (#4441) by @mattyg
  - Feat: don't use wasmer module cache with wasmer_wamr feature  * feat: bypassing module cache in wamr mode working  * build: bump holochain_wasmer_* crates  * chore: taplo  * fix: RealRibosome construction with optional cache  * build: bump proptest & proptest-derive  * wip: attempt to fix macos-latest wasmer_wamr building  * build: install ninja for macos wasmer_wamr ci runs  * fix: disable test that wasm cache works in wasmer_wamr mode  * chore: unused import  * test: increase maximum response time to concurrent zome calls in wasmer_wamr mode, where zome calls are expected to take longer due to wasm interpreting  * chore: changelogs  * fix: test assertion message  * chore: clearer doc comment  * build: fix holochain_wasmer_host feature
- *(app-api)* Add countersigning calls for unresolved sessions (#4253) by @jost-s
  - Fix locking logic  * Check fixes  * Add test for noisy signatures regression  * Format  * Add single lock test  * Update changelog  * Split witnessing workflow out of the countersigning workflow  * Move pre-flight acceptance into the workflow to allow state to be updated  * Rewire triggers and prepare countersigning workflow  * Workflow triggering  * Simplify chain locks and remove automatic expiry  * Refactor success flow  * Debug test failures  * Format  * Fix tests  * Clear sessions that time out without making a commit  * Format  * Implement authority queries and abandon  * Format  * Signal abandoned countersigning sessions  * Resolve and retry sessions in a bad state  * Publish when the session completed without us  * Static  * Fix race conditions in signal emission  * Add failing test to debug recovery  * Update unknown state  * Make session abandon work  * Add completed session recovery test and fix the refresh logic to make it work  * Split up the workflow and fix warnings  * Rename SessionState to CountersigningSessionState  * Set up testing for the countersigning workflow  * Rough success test  * Tidy up test code into reusable fixture  * Add abandon recovery test  * Add complete recovery test  * Lint  * Test stay in unknown state  * Avoid duplicate triggers  * Less unused now there are tests  * Time out if signatures not received and add more tests  * Fix session timeout test  * Avoid missing signals in recovery test  * Stabilise sig bundle noise test  * Update crates/holochain/src/core/workflow/countersigning_workflow/complete.rs
- *(hdk)* Add call get_agent_key_lineage (#4215) by @jost-s
  - Fix app validation re-trigger delay  * add key lineage host fn  * uniformly use dpkiapi type  * add zome tests for key lineage  * add tests for calling key lineage from init  * update changelog  * indicate where tests can be found  * styel: remove redundant closure  * panic when dpki is called from wrong context  * fix  * Update crates/hdk/src/agent.rs
- *(Cargo)* Add clippy lints to workspace and enable for holochain (#3318) by @steveej
  - Feat(Cargo): add clippy lints to workspace and enable for holochain  * feat(nix/holochain/clippy): read args from workspace and remove nix config file  by reusing the workspace lints we avoid the CI requirement of checking for the inheritance of lints in each crate.  * Do legwork to add lints section to crates...  ---------
- *(hdk)* Add get options to get_links & get_link_details calls (#3363) by @jost-s
  - Feat: add GetOptions to get_links call  * feat: add GetOptions to get_link_details call  * docs(hdk): udpate changelog  * rebuild hc_demo_cli wasms
- \[**BREAKING**\] *(hdk)* Add 'base' field to Link struct (#2573) by @mattyg
  - Feat: add 'base' field to Link struct  * test: add assertions to sweettests to check all Link fields  ---------
- *(CHANGELOGs)* Facilitate bump to next minor pre-release (#1823) by @steveej
- Mark all crates for minor version bump (#1757) by @steveej
- \[**BREAKING**\] *(cap-grant)* Add wildcard for zomes and fns (#1732) by @zippy
  - Fixed signed_zome_call test  * updates to convert GrantedFunctions to enum with All and Listed  * added change log for Cap grant update  * add 'arbitrary' attribute to GrantedFunctions  * fix release workflow  * remove unused import
- \[**BREAKING**\] *(app-api)* Extend AppInfo to include all types of cells (#1719) by @jost-s
  - Compile extended app info at conductor level  * rename cell_data to cell_info  * update get_app_info in websocket test  * remove unused fn get_app_info from conductor state  * update AppInfo::from_installed_app  * fmt  * clippy fixes  * delete crypto request enum  * add constructor for CellInfo  * merge develop & rename InstalledAppInfo to AppInfo  * fix intra-doc link to CellInfo  * rename RequestAgentInfo to GetAgentInfo  * add an enabled field to cell struct to reject zome calls to disabled cells  * add comments  * test for calling enabled/disabled clone cells after conductor restarts  * update changelog  * rename GetAgentInfo to AgentInfo  * rename GetAppInfo to AppInfo  * fix intra-doc links  * check for cell missing error in cloning test  * revert cell.enabled & include clones in all app cells  * refactor is_enabled  * clippy fix in cell.is_enabled  * fix clone_management test in app
- Set all frontmatters to facilitate the 0.1.0-beta-rc.0 bump (#1717) by @steveej
  - The changes were autogenerated using the following command:  ``` $ nix-shell --pure --argstr flavor release --run 'release-automation \   --workspace-path=$PWD --log-level=debug --match-filter=".*" changelog \   set-frontmatter <(cat <<EOF default_semver_increment_mode: !pre_minor beta-rc EOF ) ' ```
- \[**BREAKING**\] *(app-api)* Require zome calls to be signed (#1510) by @thedavidmeister
  - Wip signed zome calls  * wip on signed calls  * wip on signed calls  * signed zome calls is compiling  * fix signed zome call tests  * fix tests  * wip on nonces  * wip on nonce in conductor  * nonces compiling  * compiling nonce expiries  * fix compile issues  * dont witness fresh nonce  * zome call witness nonce tests  * fix tets  * better error message in app interface  * fix tests  * lint  * fix tests  * debug  * fix test  * hash signed call data  * fix tests  * add tests for zome call auth  * changelog  * reorder checks  * remove package lock  * fix duplicate imports & version conflicts  * fix conductor tests  * fix more conductor handle methods  * fix compilation  * move unsigned zome call into zome types  * lint  * add blake2b-256 export to holo_hash  * debug ci  * add feature "encoding" to holo_hash import in holochain_zome_types  * extend key authorization to signed zome call test  * impl from, debug, eq for Nonce256Bits  * add failing test of signed zome call w/o cap secret  * update keystore dep in conductor_api  * impl visitor in secure primitives macro  * delete duplicate secure_primitive macro from zome_types  * clippy fixes: do not clone nonce  * comment out failing signed zome call test  * enable encoding feat for holo_hash in zome_types  * Revert "delete duplicate secure_primitive macro from zome_types"  This reverts commit 2fe9c8db0e6ceefce99681849f83b4ad7b6d80e7.  * fix holochain websocket integration tests  * cleanup websocket integration tests  * refactor has_initialized to check for init marker  * remove unused import  * fix error & simplify has_initialized check  * extend nonce expiry in signed zome call test  * fix clippy field reassign with default in source_chain  * replace zome init complete query by explicit field  * revert to query source for init zomes complete  * fmt  * simplify equality check against true in source_chain  * warn about multiple init zome complete actions in source_chain
- *(admin-api)* Add calls to authorize signing key & get dna def (#1641) by @jost-s
  - Adding 2 calls to the Admin API: * authorize public key to authorize zome calls * get DNA definition
- *(cloning)* Add calls to archive/restore/delete clones (#1578) by @jost-s
  - Mark a clone cell deleted on app level  * add fn to restore cell to app  * add call to delete archived clone cells  * simplify archiving/restoring clone cells  * fmt  * update changelogs  * clippy corrections  * anuddr clippy opt  * fix spelling  * remove duplicate field doc comment  * add breaking change for role ids to changelog  * catch error when parsing malformed clone ids  * add responses to admin api docs  * separate payload structs for clone archive & restore  * fmt  * delete duplicate zome call definition from clone test  * refactor(conductor): rename phenotype to modifiers  * chore(changelogs): add phenotype rename  * add modifiers to fixture app manifest  * add InstallAppBundle change to changelog  * Update crates/holochain_types/CHANGELOG.md
- Feature by @freesig

### Bug Fixes

- List unrevoked capability grants when include_revoked is false (#5958) by @ThetaSinner
  - Test: restore conductor test modules to the build
  - `#[cfg(test)] mod tests;` was dropped from `conductor.rs` while resolving merge conflicts in #5524, so the eleven test files under `src/conductor/tests/` have not been compiled or run since.
  - Restoring the declaration surfaced the drift that accumulated while they were invisible:
  - `InstallAppPayload` gained `restore_from_dht`. - `SweetConductor::startup` no longer takes an argument. - `SourceChainRead::new` is now `SourceChain::new(..).as_read()`, reached   through `SweetConductor::get_agent_source_chain`. - `Conductor::get_dna_def` returns a `DnaDefHashed`. - A private-entry create also produces a store-entry op, which is   filtered out when publishing, so the cap grant op counts are one   higher than the tests expected. - The peer meta tests wrote entries that expired immediately; reads   filter expired entries, and expiry is stored per second.
- Remove unused dependencies (#5671) by @ThetaSinner
- Remove uses of dependencies as features (#5663) by @ThetaSinner
- Stabilize chain fork warrants (#5641) by @veeso
- 0-arc `get_details` should go to the network if permitted (#5420) by @ThetaSinner
  - Fix: 0-arc `get_details` should go to the network if permitted
  - Docs: Update changelog
- Expand/correct documentation around `get_agent_activity` and `ChainQueryFilter` (#5287) by @pdaoust
  - Fix: expand/correct documentation around `get_agent_activity` and `ChainQueryFilter`
  - Fix: feedback from mattyg plus some fixes and refinements
  - Fix: feedback from @jost-s
  - Fix: suggestions from the Rabbit
  - Fix: very bad merge
  - Fix: simplify HighestObserved language
- Return signed warrants from `get_agent_activity` so that the warrants can be validated when received (#5237) by @ThetaSinner
- More descriptive error messages and remove unnecessary if/else clause that caused inconsistent behavior (#5105) by @matthme
  - Fix: remove unnecessary if/else clause that caused inconsistent behavior, improved error message if conversion to ScopeLinkType or ScopedEntryDefIndex fails
  - Fix: add required dependencies argument to InlineZomeSet constructors, fix missing zome dependency values
- \[**BREAKING**\] Make links getters more consistent (#5109)
  - Refactor(hdk)!: Made links getters more consistent
  - Get_links, get_link_details and count_links had the same scope, but totally different APIs
  - **Breaking Change**: Changed the API signatures

* refactor(hdk)!: Renamed `get_link_details` to `get_links_details` for a more correct naming
- *(holochain)* Delete link as zero arc node (#4945) by @jost-s
  - Test(holochain): add regression test where a zero arc node deletes a link
  - Refactor(holochain): allow create link action to be fetched from the network when deleting link
- Compile error when `hdk_extern` is used with wrong type (#4617) by @c12i
  - Check hdk_extern annonatated function return types  * Remove migrate_agent handling  * Fix clippy warning  * Handle post_commit  * Update CHANGELOG  * Better name util  * Remove entry_defs check  * Better context with diagnostics  * Add tests  * Fix clippy warnings  * Add validate infallible test and rename genesis self check infallible test  * Improve infallible suggestion  * Remove out of scope testcase  * Move tests to `hdk` crate, replace compiletest_rs with trybuild  * Fix hdk_extern_infallible_invalid_return test  * Fix failing tests and update hdk changelog  * Improve diagnostic messages  * Replace validate_invalid_return test_wasm test with inline zome  * Replace init_invalid_return test_wasm test with inline zome  * Remove unused TestWasm variants  * Revert unwanted diffs  * Remove un-required testwasm test
- Wasmer memory access fault & caching (#3152) by @jost-s
  - Test(ribosome): add guard for zome call response time  * test(ribosome): add cache test  * test(ribosome): add test for module and instance cache  * remove unused import  * test: add concurrent zome call test of response times  * refactor(real-ribosome): remove instance caching fn  * refactor(real-ribosome): remove instance cache altogether  * refactor(real-ribosome): move cache logic to holochain-wasmer  * refactor: move all wasmer-types items to holochain-wasmer  * refactor(real-ribosome): change identifier names according to refactors  * fix: clean up contexts from memory after calls  * docs: add changelogs  * doc: update changelog  * refactor: move ios precompiled module getter to holochain-wasmer  * refactor: rename precompiled module fn  * build: remove wasmer dependencies from holochain_types  * test: fix concurrent zome call test  * docs: update breaking change in changelog  * build: update holochain wasmer crates to v0.0.92  * delete unused import  * delete unused import  * Fix symlink path mismatch  * Use the path that sqlite gives back  * Clippy fix  * fix: check if const fn exists  * refactor: simplify error handling in call_zome_fn  * lint: clippy fixy  * test: add must_get_agent_activity bug reproduction  * test: update test name  * test: update test  * fix: must_get_agent_activity wasm  * build: update both workspaces' cargo locks  * test: fix must_get_agent_activity saturation  ---------
- Remove shadowing public glob re-exports (#3092) by @jost-s
  - Fix: remove shadowing public glob re-exports  * refactor: export error types directly instead of error mod  * docs(changelog): add refactors  * docs(changelog): mark as breaking changes  * style: fmt
- *(hdk)* Pin the HDI version by @steveej
- *(hdk)* Don't mark as default unreleasable by @steveej
  - Now that this feature is actually implemented, this would prevent the HDK from being releasable after the next release.
- Fix test errors
- *(hdk/tests)* Bless actual output by @steveej
- Fix test by @freesig
- Fix ser by @neonphog
- Fix docs by @freesig
- Fix errors by @freesig
- Fixed intra-docs again - this should succeed by @jost-s
- Fix links in hdk by @jost-s
- Fix links in holochain zome types by @jost-s
- Fix trait by @freesig
- Fix bad merge by @thedavidmeister
- Fixes by @maackle
- Fix tests by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix profile by @freesig
- Fix clippy issues by @freesig
- Fix tests by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix tests for path by @thedavidmeister
- Fix entry_def_index! by @thedavidmeister
- Fix clippy lints by @steveej
- Fix tests by @thedavidmeister
- Fixing manifeset documentation urls
- Fix tests and add docs by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix and add to CHANGELOGs by @steveej
- Fix bad merge by @freesig
- Fix merge by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix merge by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix merge by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix map_extern by @thedavidmeister
- Fix invalid deps from short circuit in must get by @thedavidmeister
- Fix clippy by @freesig
- Fix tests by @thedavidmeister
- Fix host args by @thedavidmeister
- Fix macro dependency by @thedavidmeister
- Fix mutable elements by @thedavidmeister
- Fix tests by @thedavidmeister
- Prepare READMEs and Cargo.toml for releasing by @steveej
- Fix tests by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix remote call by @thedavidmeister
- Fix zome types by @freesig
- Fix tests by @thedavidmeister
- Fix compiler for crypto by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix diagnostic compile errors by @freesig
- Fixed by @freesig
- Fix path by @freesig
- Fix call remote tests by @thedavidmeister
- Fix validate return type by @freesig
- Fix validate fns and send entry with update by @freesig
- Fixed tests by @freesig
- Fix test by @freesig
- Fix cap grant tests by @thedavidmeister
- Fix flakey tests and remove default impl by @thedavidmeister
- Fix infinite compiler loops by @thedavidmeister
- Fix tests by @thedavidmeister
- Fix deps by @thedavidmeister
- Fix tests by @thedavidmeister
- Fixes by @freesig
- Fix tests by @thedavidmeister
- Fix utf32 back to utf8 in anchors by @thedavidmeister
- Fix merge by @thedavidmeister
- Fix tests by @thedavidmeister

### Miscellaneous Tasks

- Update the AI_POLICY.md with shared content
- Update the CONTRIBUTING.md with shared content
- Switch to dev releases for 0.8 by @ThetaSinner
- Prepare the 0.7.0 release by @ThetaSinner
- Prepare 0.7 for RC releases (#5884) by @ThetaSinner
- Switch from serde_yaml to yaml_serde (#5754) by @ThetaSinner
  - Chore: Switch from `serde_yaml` to `yaml_serde`
  - # Conflicts: #	crates/holochain/CHANGELOG.md
  - Chore: review changes
  - Chore: fix build
  - Chore: fix build
- Remove unused mock HDI (#5484) by @ThetaSinner
  - Chore: Remove unused mock HDI
  - Docs: Update changelog
  - Chore: update lock
  - Chore: Review changes
- Update project to use Rust 1.91 by @ThetaSinner
- Switch back to dev releases (#5459) by @ThetaSinner
- Prepare 0.6.0 release (#5453) by @ThetaSinner
- Switch to RC releases (#5432) by @ThetaSinner
  - Chore: Switch to RC releases
  - Chore: Permit RC releases
- Upgrade Lair and Rust (#5317) by @ThetaSinner
  - Chore: Upgrade Lair and Rust
  - Docs: Update changelog
- Remove obsolete kitsune_p2p_block types (#5295) by @jost-s
- Deprecate BlockTarget::NodeDna and BlockTarget::Node (#5286) by @jost-s
- Declare schedule host fn stable in code (#5266) by @ddd-mtl
- Upgrade `strum` and `strum_macros` to `0.27.x` (#5267) by @ddd-mtl
- One-off format of imports using unstable `group_imports` (#5239) by @ThetaSinner
- Remove links to the Holochain forum (deprecated) and tidy READMEs (#5115) by @ThetaSinner
- Fix some typos (#5106) by @bytetigers
- Upgrade dependencies (#5059) by @ThetaSinner
- Update flakes, Rust version and split 0.5 version (#5039) by @ThetaSinner
  - Chore: Update flakes and split 0.5 version
  - Chore: Update workspace Rust version and fix clippy issues
  - Chore: Resolve clippy warnings about features
- Reference repository rather than homepage (#5040) by @ThetaSinner
- *(holochain)* Bump wasmer to v6 (#5001) by @jost-s
  - Chore(holochain): bump wasmer to v6
  - Ci: install llvm for llvm-objcopy for wasmer-wamr
  - Fix(hc_sandbox): await cmd of non running sandbox with output to prevent test from hanging
  - 
- Generate bundle schemas (#4935) by @ThetaSinner
  - Chore: Generate bundle schemas
  - Docs: Update changelog
  - Update crates/holochain_zome_types/Cargo.toml
- WIP simplify Mr. Bundle (#4881) by @ThetaSinner
  - Chore: WIP simplify Mr. Bundle
  - Chore: wip update bundle internals to not use paths
  - Chore: continued wip for simplifying bundling
  - Chore: Get initial tests passing
  - Chore: code complete on mr bundle refactor
  - Chore: Improve mr_bundle docs and tidy error type
  - Chore: Use new Mr. Bundle in Holochain types
  - Chore: Fixy fixy fixy
  - Chore: Fixy!
  - Chore: Cleanup
  - Chore: Self review
  - Docs: Update changelogs
  - Chore: Cleanup and test fixes
  - Chore: Fix sandbox tests
  - Update crates/mr_bundle/src/manifest.rs
- Remove all dpki related code (#4901) by @jost-s
  - Chore: remove all dpki related code
  - Chore: remove remnants of get_agent_key_lineage
  - Ci: move nextest installation to workflow to prevent complete rebuilds
  - Chore: remove all deepkey related code
  - Docs(changelog): mention dpki removal
  - Chore(holochain): remove revoke_agent_key call
  - Docs(changelog): mention removal of revoke_agent_key call
  - 
- Switch back to dev releases (#4883) by @ThetaSinner
  - Chore: Switch back to dev releases
  - Chore: Try pre minor
  - Chore: Match script
  - Chore: add debug
  - Chore: Update versions as though 0.5 had released
  - Chore: Format TOML
- Prepare for RC releases by @ThetaSinner
- Enable and move hdk extern tests (#4799) by @c12i
  - Fix failing tests
  - Move tests
  - Remove cyclic dev dependency
- Bump workspace Rust to latest stable (#4505) by @ThetaSinner
  - Chore: Bump workspace Rust to latest stable
  - Fix(kitsune_p2p): Invalid ordering strategy
  - Fixup! fix(kitsune_p2p): Invalid ordering strategy
- Remove deprecated code (#3885) by @jost-s
  - Chore: remove deprecated code  * docs: add deprecations to changelogs  * Update crates/holochain/CHANGELOG.md
- Prepare 0.2.0 release and 0.3.0-beta-dev.N series (#2290) by @steveej
  - Chore: prepare 0.2.0 release and 0.3.0-beta-dev.N series  * docs(CHANGELOG): add toplevel release notes  * fixup! chore: prepare 0.2.0 release and 0.3.0-beta-dev.N series
- Change all version suffix to beta-rc (#2076) by @steveej
- *(all)* Update changelogs
- Rename `holochain_deterministic_integrity` to `hdi` by @steveej
- *(CHANGELOGs)* Backfill for #1386 by @steveej

### Build System

- Add repository tooling and licences by @veeso
  - Cargo-make tasks, taplo config, pinned toolchain with the wasm32 target, Apache-2.0 and CAL-1.0 licence texts, README and changelog seed.
- Create the holochain-hdk workspace by @veeso
  - Crates moved from holochain/holochain (holochain/holochain#5400) now share one workspace version and depend on the published holochain-hdi crates.
- Remove `holochain_sqlite` crate (#5869) by @veeso
- Bump holochain_wasmer_* crates to 0.0.99 (#4632) by @mattyg
- \[**BREAKING**\] Update serialization to v0.0.54 (#3757) by @jost-s
  - Test: update conductor api serialization tests  * build: update holochain_serialized_bytes to v0.0.54  * build: bump serialization & wasmer  * build: bump wasmer  * build: bump holochain_wasmer_common  * build: bump rmp-serde  * test: adapt interface serialization tests  * docs: mention breaking change in changelog  * the fix the clip  * test: update serialized bytes test  * fix: network info request test  * update hc demo cli wasms

### CI

- Add test and release workflows by @veeso in [#4](https://github.com/holochain/holochain-hdk/pull/4)
  - Test workflow with a ci_pass aggregate job for the branch ruleset, and the holochain release integration prepare/publish workflows.

### Testing

- Fix hdk documentation examples by @veeso in [#3](https://github.com/holochain/holochain-hdk/pull/3)
  - Update link and validation receipt examples to compile against the standalone HDK workspace.
- Recover tests that blocked agents aren t published and gossiped to by @jost-s
  - Test: recover tests that publish and gossip doesn't contact blocked nodes
  - Chore!: remove unusable block variant CellBlockReason::App
  - Chore!: delete deprecated block targets
  - 
- Add tests for the `schedule` host fn (#5206) by @ddd-mtl
- App operations with app and cell state by @jost-s
  - Chore: remove commented code in test can_create_inline_dna
  - Chore: delete key revocation comment in InstallAppPayload
  - Chore: delete obsolete reference to agent key lineage wasm
  - Test: app operations with app and cell state
  - 
- Tests for sweet topos (#2533) by @thedavidmeister
- Test dna yaml with dependencies and test hot swapping by @freesig
- Tests for baseless links by @thedavidmeister
- Test_check_countersigning_session_data_responses_indexes by @thedavidmeister
- Tests for countersigning workflows by @thedavidmeister
- Test for must_get_x by @thedavidmeister
- Test for ephemeral sig by @thedavidmeister
- Tests working for wasm ribosome by @thedavidmeister
- Test by @thedavidmeister
- Test for entry delete in wasm by @thedavidmeister
- Test_validate_implemented_valid by @thedavidmeister
- Test_validate_link_add_implemented_invalid by @thedavidmeister
- Test stuff by @freesig
- Tests for remove link by @thedavidmeister
- Tests for hash path ls by @thedavidmeister
- Test for random bytes in ribosome by @thedavidmeister
- Test validation callback results by @thedavidmeister
- Tests for validation package callback by @thedavidmeister
- Tests for post commit by @thedavidmeister
- Tests for init callback by @thedavidmeister

### Refactor

- Update cap model to make space for other grant types than zome calls and rename types to keep the set of types coherent (#5947) by @ThetaSinner
  - Refactor: Update cap model to make space for other grant types than zome calls and rename types to keep the set of types coherent.
  - Chore: review fixes
  - Chore: more review fixes
  - Chore: yet more review fixes
  - Chore: adjust the review fixes
  - Chore: condense capability grant changelog entries
  - Chore: guard
  - Fix: clippy ignore of an enum size difference by boxing
  - Chore: review changes
  - Chore: review changes
  - Docs: Mention `GrantZomeCallCapabilityGrant`
- Dissolve dht_v2 naming and remove migration-history language (#5881) by @ThetaSinner
  - Docs: remove stale legacy-vs-current comparison from state_model.md
  - Refactor(holochain): remove stale legacy-model comments and dead CellError variants
  - Refactor: remove stale legacy-model comments in holochain_data/conductor_api/state
  - Refactor(holochain_integrity_types): dissolve dht_v2 into action/op/record
  - Refactor(holochain_zome_types): dissolve dht_v2 into action/op/record
  - Refactor(holochain_types): unify dht_v2::WarrantOp into warrant::WarrantOp
  - Refactor(holochain_types): dissolve dht_v2 into op
  - Refactor(holochain_state,holochain_cascade): repoint dht_v2 references
  - Refactor(holochain_p2p,holochain_data): repoint dht_v2 references
  - Refactor(holochain): repoint dht_v2 references
  - Refactor(hdi): repoint dht_v2 references
  - Refactor(holochain_state): resolve the dht_v2-as-v2 alias in source_chain.rs
  - Refactor: repoint remaining dht_v2 references in holochain_p2p tests and holochain_data docs
  - Refactor: rename remaining _v2-suffixed identifiers and reword v2-model prose
  - Docs: add CHANGELOG entries for the _v2 identifier renames
  - Fix: resolve static-all fallout from dht_v2 dissolution
  - Remove a now-redundant explicit rustdoc link target on wire_rows_to_ops left over from the Task 13 rename.
  - Docs: note the hc binary build-order gap for hc_client/hc_sandbox tests
  - Docs: fix two more stale DB-split doc comments found in final review
  - Fixup! refactor: rename remaining _v2-suffixed identifiers and reword v2-model prose
  - Fixup! refactor(holochain_integrity_types): dissolve dht_v2 into action/op/record
  - Fixup! docs: add CHANGELOG entries for the _v2 identifier renames
  - Fix: validate the exact CreateLink hash in RegisterDeleteLink::new
- Reduce usage of DNA file and split inline zomes into a separate implementation (#5828) by @ThetaSinner
  - Refactor: Reduce usage of DNA file and split inline zomes into a separate implementation
  - Chore: Follow up
  - Chore: Follow up
  - Chore: static
  - Docs: Document inline-zome split and DNA-file API changes
  - Add an Unreleased changelog entry covering the breaking API changes (WasmZome -> WasmZomeDef, zome_hash, InlineZome::hash, ZomeDef::Inline holding an InlineHash, the DnaHash change from dropping the untagged serialization, removed DnaWithRole::replace_dna) and the additions (InlineHash/ZomeHash, DnaDef::replace_coordinators, ribosome restructure).
  - Also refresh the now-stale ZomeDef doc comment to match the new serialization and inline-zome identification.
  - Chore: review changes
  - Chore: review changes
  - Chore: fix feature logic
- Complete removal of block_agent/unblock_agent host functions (#5536) by @ThetaSinner
  - Removed block_agent and unblock_agent host function implementations, registrations, and type definitions. These functions have been removed from the HDK layer in the previous commit, and this completes the removal by eliminating the underlying host function infrastructure.
  - System-level blocking through warrants continues to function correctly. This change is not backward compatible - WASMs that attempt to call these host functions will fail to instantiate.
- Replace `BTreeSet` with `HashSet` in `GrantedFunctions::Listed` (#5349) by @ThetaSinner
  - Refactor: Replace `BTreeSet` with `HashSet` in `GrantedFunctions::Listed`
  - Docs: Update changelog
  - Fixup! refactor: Replace `BTreeSet` with `HashSet` in `GrantedFunctions::Listed`
- Persist warrants in separate table (#4937) by @jost-s
  - Feat(holochain_state)!: add warrant table and modify mutation to insert into this table
  - Fix: sys validation warrant tests
  - Docs(changelog): mention warrant table
  - Docs(holochain_state): note that multiple warrants may have to be stored per agent
  - Update crates/holochain_sqlite/src/sql/cell/schema/6-up.sql
- \[**BREAKING**\] *(holochain)* Authenticate zome calls early in conductor (#4474) by @jost-s
  - Refactor: verify signature directly after deserializing params  * refactor: remove signature verification from zome call invocation  * tbc  * remove accidentally kept test  * refactor(holochain): verify signature in remote calls  * refactor: remove ZomeCall struct  Effectively this means that only zome calls from the network need to be authenticated. Zome calls to another cell within the conductor no longer need authentication other than the zome call that came from outside the conductor to call the function that calls the other cell.  * refactor: remove unused call_zome from CellConductorApi  * docs: add changelog entry  * style: fix clippies  * test: add unit tests for is_valid_signature  * fix: remove more ZomeCalls in countersigning tests  * fix grammar in changelog  * remove println  * fix comments  * refactor(holochain): move external zome call handling to conductor  * refactor(holochain): move is_signature_valid to conductor  * docs(holochain_conductor_api): cross-ref CallZome from AuthenticationFailure  * test: split up websocket tests  * fix: update response matching in coordinators  * style: clippy fix  * fix: zome call authentication test  ---------
- \[**BREAKING**\] *(holochain)* Zome call signature verification (#4433) by @jost-s
  - Refactor!: add opaque field zome_call_payload to ZomeCall  * refactor(holochain): change zome call to signature and bytes  * refactor: rename ZomeCall to SignedZomeCall  * refactor: rename ZomeCallUnsigned to ZomeCallParams  * refactor: rename ZomeCallDeserialized to ZomeCall  * docs(holochain): metion zome call signing change in changelog  * refactor: rename SignedZomeCall to ZomeCallParamsSigned  * fix websocket interface test  * rename some variables  * refactor(holochain-types): delete unused module  * refactor(holochain-p2p): rename zome_call_payload to zome_call_params_serialized  * refactor: move ZomeCall to conductor crate  * refactor: remove unused method resign_zome_call  * fix imports  * fix: zomecall import  * yet another feature clippy fix  * refactor(holochain)!: sign hash of serialized bytes of zome call params  * refactor(holochain)!: sign hash of serialized bytes for remote signals  * refactor(holochain): dry hashing into serialization fn  * refactor: simplify serialized bytes and hash types  * refactor(holochain-p2p)!: pass params as serialized bytes into call_remote  * docs(holochain): explain in changelog how signature is verified  * refactor(holochain-p2p)!: simplify CallRemoteSignal  * style: fmt  * feat(holo-hash): add sha2 512-bit hasher  * refactor(holochain!:hash zome params to sign with sha2 512-bit  * docs(holochain): update changelog to reflect sha2 512-bit hasher  * fix(holochain): hash with sha2 512-bit to verify signature  * test(holochain): reintegrate and fix ser_regression_test  * style: fix clippiness  * docs(holochain-conductor-api): fix hash algorithm in CallZome  ---------
- \[**BREAKING**\] *(holochain)* Put countersigning related functions behind feature unstable-countersigning (#4393) by @maackle
  - Refactor(hdk): put preflight request behind feature unstable-countersigning  * test: put dpki test behind unstable-dpki feature  * refactor(holochain)!: put everything else countersigning related behind feature unstable-countersigning  * refactor(holochain): move everything in cs workflow behind feature unstable-countersigning  * docs(holochain): mention unstable-countersigning feature in changelog  * style(holochain): clippy fix  * ci: test unstable-countersigning in workflow  * style(holochain): allow unused_variables instead of renaming vars  * style: move feature closer to mod definition  * refactor: separete unstable-functions from unstable-countersigning  * style(holochain): allow unused_variables instead of renaming vars  ---------
- \[**BREAKING**\] Rename get options and make get calls network or local only (#3378) by @jost-s
  - Refactor: rename GetOptions::Content to Local  * refactor: gets return only local data for GetOptions::Local  * refactor: get_details returns only local data for GetOptions::Local  * refactor: rename GetOptions::content to local  * test: add test for get by entry hash  * refactor: fetch locally only for GetOptions::Local in get & get_details  * refactor: rename GetOptions::Latest to Network  * docs(holochain_zome_types): update changelog  * fix: type in changelog  * fix: another changelog change  * Apply suggestions from code review
- *(cargo)* Move some common deps into workspace (#3285) by @steveej
- *(generate-readme)* Switch to cargo-rdme (#1820) by @jost-s
  - Docs(hdi): prepare readme and lib for cargo-rdme  * docs(hdk): prepare lib and readme for cargo-rdme  * docs(holochain_keystore): prepare readme for cargo-rdme  * docs(holochain_state): prepare lib and readme for cargo-rdme  * docs(hdk+hdi): delete obsolete tpl files  * refactor(generate_readmes): replace cargo-readme with cargo-rdme  * fix(hdk): intra-doc link to hdk_extern macro  * ci(readme): remove cargo-readme and install cargo-rdme  * ci(create-readme): configure git  * build(generate-readme): unset git name & email afterwards  * docs(crate-level): generate readmes from doc comments  * fix clippy  ---------
- \[**BREAKING**\] *(admin-api)* Rename authorize key call (#1647) by @jost-s
  - Rename: AuthorizeZomeCallSigningKey -> GrantZomeCallCap  * remove provenance  * delete unused conductor error variant  * more renames  * update holochain_zome_types changelog
- *(clone-id)* Convert to new type
- \[**BREAKING**\] *(uid)* Rename uid to network_seed (#1493) by @jost-s
  - Refactor(uid): rename uid to network_seed  * chore(all): update changelogs  * docs(dna): describe network seed  * docs(network_seed): add cross-links  * chore(changelogs): highlight breaking change
- Refactor batch api for remote calls by @thedavidmeister

### Documentation

- *(crate-level)* Generate readmes from doc comments by @holochain-release-automation2
- Better documentation for TypedPath::ensure (#5527) by @veeso
- *(crate-level)* Generate readmes from doc comments by @holochain-release-automation2
- Remove references to remove_agent_* from the HDK docs (#5464) by @ThetaSinner
- Remove stray double doc comment in HDK `query` docs (#5411) by @ThetaSinner
- *(crate-level)* Generate readmes from doc comments by @holochain-release-automation2
- *(hdk)* Fixed documentation for `get_links` and `get_links_details` (#5122)
- *(crate-level)* Generate readmes from doc comments by @holochain-release-automation2
- *(crate-level)* Generate readmes from doc comments by @holochain-release-automation2
- *(crate-level)* Generate readmes from doc comments by @holochain-release-automation2
- Update hdk docs (#3612) by @c12i
- Fix some rust doc links (#3552) by @jost-s
- *(crate-level)* Generate readmes from doc comments by @holochain-release-automation2
- *(crate-level)* Generate readmes from doc comments
- *(crate-level)* Generate readmes from doc comments
- *(conductor-api-hdk)* Add links (#2033) by @jost-s
  - Add links to conductor api docs  * add links to hdk docs  * easier intro into hdk  * add changes to log  * fmt  * Update crates/hdk/src/lib.rs
- *(hdk)* Add note about get_links not deduping (#1791) by @pdaoust
  - Add note about get_links not deduping  * add mention of docs change into changelog
- *(hdk)* Extend docs about get_detail variants (#1231) by @zippy
  - Added some clarity about get detail variants  * docs typo  * docs typo  * fix type in EntryDetails  * complement ElementDetails description  * improvements do docs from review  * rust fmt
- Docs for call remote (#1787) by @thedavidmeister
- *(hdk)* Clean up and add example wasm links (#1771) by @jost-s
  - Add validation example to Op  * fix validation link in hdi  * add link to capability wasm example  * add example to emit_signal  * add some commas to entry mod  * link init callback to examples  * update changelog  * cargo fmt
- Docs for scheduler in hdk by @thedavidmeister
- *(crate-level)* Generate READMEs from doc comments
- *(hdk)* Explain op type & how to get typed path (#1505) by @jost-s
  - Docs(hdk): reference hdi from integrity zome section  * docs(hdi): add a validation example to hdi docs  * docs(link): document get_links arg link_type  * docs(link): add same docs to get_link_details  * docs(link): add comments to LInkTypeFilterExt  * docs(link): add comments to LInkTypeFilterExt  * docs(hdk): intra-link to wasm error  * docs(derive): explain unit enum some more  * docs(get_link): filter only takes full range  * docs(link): clarify get links scope  * fmt  * docs(hdk): shot entry_defs cb as plain text  * docs(typed_link): add example how to get from path  * docs(validation): add op type example  * chore(changelogs): add docs  * add more explanatory text to OpType helper
- *(crate-level)* Generate READMEs from doc comments
- *(hdk/hdi)* Add more details to get_links (#1486) by @jost-s
  - Docs(hdk): reference hdi from integrity zome section  * docs(hdi): add a validation example to hdi docs  * docs(link): document get_links arg link_type  * docs(link): add same docs to get_link_details  * docs(link): add comments to LInkTypeFilterExt  * docs(link): add comments to LInkTypeFilterExt  * docs(hdk): intra-link to wasm error  * docs(derive): explain unit enum some more  * docs(get_link): filter only takes full range  * docs(link): clarify get links scope
- *(crate-level)* Generate READMEs from doc comments
- *(hdi)* Add crate level documentation (#1477) by @jost-s
  - Docs(hdk): reduce to only validate callback  * write create docs for holochain_deterministic_integrity  * flesh out doc comments  * add links to examples of integrity & ccordinator zomes  * fmt  * remove irrelevant phrase  * add dna manifest examples  * Update crates/holochain_deterministic_integrity/src/lib.rs
- *(crate-level)* Generate READMEs from doc comments
- *(hdi)* Add/edit crate level docs for hdi and hdk (#1443) by @jost-s
  - Docs(hdk): reduce to only validate callback  * write create docs for holochain_deterministic_integrity  * flesh out doc comments  * add links to examples of integrity & ccordinator zomes  * fmt  * remove irrelevant phrase  * add dna manifest examples  * Update crates/holochain_deterministic_integrity/src/lib.rs
- *(crate-level)* Generate READMEs from doc comments
- *(crate-level)* Generate READMEs from doc comments
- Docs and hash_external by @thedavidmeister
- Docs by @thedavidmeister
- Docs for hash_header by @thedavidmeister
- Docs for paths by @thedavidmeister
- Docs by @thedavidmeister
- Docs review by @thedavidmeister
- Docs by @thedavidmeister
- Docs and renaming capgrant variants by @thedavidmeister
- Docs for entry defs by @thedavidmeister
- Docs and tests by @thedavidmeister
- Docs by @thedavidmeister
- Docs tweaks by @thedavidmeister

### Automated Changes

- Update CODEOWNERS with shared content in [#1](https://github.com/holochain/holochain-hdk/pull/1)
- Update dependabot.yml with shared content in [#2](https://github.com/holochain/holochain-hdk/pull/2)
- Merge pull request #1684 from holochain/release-20221130.011217 by @github-actions[bot]
- Merge pull request #1665 from holochain/release-20221123.011302 by @github-actions[bot]
- Merge pull request #1658 from holochain/release-20221116.012050 by @github-actions[bot]
- Merge pull request #1653 from holochain/release-20221109.012313 by @github-actions[bot]
- Merge pull request #1646 from holochain/release-20221103.145333 by @github-actions[bot]
- Merge pull request #1643 from holochain/release-20221102.014648 by @github-actions[bot]
- Merge pull request #1637 from holochain/release-20221026.192152 by @github-actions[bot]
- Merge pull request #1632 from holochain/release-20221019.014538 by @github-actions[bot]
- Merge pull request #1608 from holochain/release-20221005.164304 by @github-actions[bot]
- Merge pull request #1595 from holochain/release-20220928.014801 by @github-actions[bot]
- Merge pull request #1587 from holochain/release-20220921.145054 by @github-actions[bot]
- Merge pull request #1574 from holochain/release-20220914.013149 by @github-actions[bot]
- Merge pull request #1564 from holochain/release-20220908.155008 by @github-actions[bot]
- Merge pull request #1561 from holochain/release-20220907.100911 by @github-actions[bot]
- Merge pull request #1555 from holochain/release-20220907.014838 by @github-actions[bot]
- Merge pull request #1546 from holochain/release-20220831.015922 by @github-actions[bot]
- Merge pull request #1534 from holochain/release-20220820.111904 by @github-actions[bot]
- Merge pull request #1528 from holochain/release-20220817.013233 by @github-actions[bot]
- Merge pull request #1513 from holochain/release-20220810.012252 by @github-actions[bot]
- Merge pull request #1506 from holochain/release-20220803.124141 by @github-actions[bot]
- Merge pull request #1504 from holochain/release-20220728.122329 by @github-actions[bot]
- Merge pull request #1487 from holochain/release-20220713.013021 by @github-actions[bot]
- Merge pull request #1482 from holochain/release-20220710.155915 by @github-actions[bot]
- Merge pull request #1468 from holochain/release-20220701.181019 by @github-actions[bot]
- Merge pull request #1465 from holochain/release-20220629.012044 by @github-actions[bot]
- Merge pull request #1456 from holochain/release-20220622.133046 by @github-actions[bot]
- Merge pull request #1439 from holochain/release-20220616.084359 by @github-actions[bot]
- Merge pull request #1411 from holochain/release-20220608.011447 by @github-actions[bot]
- Merge pull request #1401 from holochain/release-20220601.012853 by @github-actions[bot]
- Merge pull request #1395 from holochain/release-20220525.012131 by @github-actions[bot]
- Merge pull request #1385 from holochain/release-20220518.010753 by @github-actions[bot]
- Merge pull request #1382 from holochain/release-20220511.012519 by @github-actions[bot]
- Merge pull request #1368 from holochain/release-20220429.205522 by @github-actions[bot]
- Merge pull request #1345 from holochain/release-20220414.075333 by @github-actions[bot]
- Merge pull request #1317 from holochain/release-20220330.010719 by @github-actions[bot]
- Merge pull request #1309 from holochain/release-20220323.023956 by @github-actions[bot]

### Other Changes

- Create a release from branch release-20260907.022923
- Create a release from branch release-20260831.030531
- Create a release from branch release-20260820.161351
- Create a release from branch release-20260817.005712
- Create a release from branch release-20260803.022130
- Create a release from branch release-20260730.150702
- Create a release from branch release-20260729.163910
- Create a release from branch release-20260727.022800
- Create a release from branch release-20260721.103110
- Reduce keystore dep impact (#5896) by @ThetaSinner
- Fix type name collisions and imports (#5899) by @ThetaSinner
- Create a release from branch release-20260716.145843
- Create a release from branch release-20260715.141229
- Unused deps and feature independence (#5868) by @ThetaSinner
- Complete switch to v2 model types (#5854) (#5860) by @ThetaSinner
- Store the source chain in the unified per-DNA database (#5844) by @ThetaSinner
- Create a release from branch release-20260701.171007
- Create a release from branch release-20260629.004342
- Create a release from branch release-20260601.004429
- Create a release from branch release-20260525.004052
- Create a release from branch release-20260518.003858
- Create a release from branch release-20260511.004219
- Create a release from branch release-20260504.004108
- Create a release from branch release-20260427.003100
- Integrate new conductor db (#5501) by @ThetaSinner
- Create a release from branch release-20260420.002833
- Create a release from branch release-20260330.002514
- Create a release from branch release-20260323.002355
- Create a release from branch release-20260316.002412
- Create a release from branch release-20260309.002048
- Create a release from branch release-20260302.002017
- Create a release from branch release-20260223.112802
- Create a release from branch release-20260119.001810
- Expand `GetOptions` with network control fields (#5549) by @ThetaSinner
- Create a release from branch release-20260112.002450
- Add `GetStrategy` support to `Anchor` (#5548) by @ThetaSinner
- Create a release from branch release-20260105.001833
- Add GetStrategy configuration to TypedPath  (#5547) by @ThetaSinner
- Create a release from branch release-20251229.001810
- Create a release from branch release-20251222.001720
- Create a release from branch release-20251201.001926
- Create a release from branch release-20251124.001659
- Create a release from branch release-20251119.115654
- Create a release from branch release-20251105.193427
- Create a release from branch release-20251105.001147
- Create a release from branch release-20251103.100646
- Create a release from branch release-20251029.001150
- Create a release from branch release-20251015.001251
- Create a release from branch release-20251008.002548
- Chore/rm unused types fns (#5325) by @mattyg
- Create a release from branch release-20251002.102845
- Create a release from branch release-20251001.001313
- Rustdoc formatting fixes (#5298) by @pdaoust
- Create a release from branch release-20250924.001733
- Warrant validation follow up (#5280) by @ThetaSinner
- Create a release from branch release-20250917.001123
- Create a release from branch release-20250910.001116
- Push Release commits from release-20250827.001211 branch (#5248) by @cdunster
- Create a release from branch release-20250820.001144
- Create a release from branch release-20250813.001217
- Add `recv_remote_signal` callback documentation try 2 (#5177) by @pdaoust
  - Feat: Add documentation for  callback in HDK module-level doc
  - Fix: properly alphabetise callbacks
  - Fix: improve language
- Create a release from branch release-20250806.001301
- Create a release from branch release-20250723.001249
- Create a release from branch release-20250716.001240
- Create a release from branch release-20250709.001228
- Create a release from branch release-20250702.001217
- Create a release from branch release-20250625.001242
- Create a release from branch release-20250611.001856
- Create a release from branch release-20250528.001153
- Create a release from branch release-20250514.001133
- Create a release from branch release-20250507.001140
- Remove unused get request kinds & GetOptions (#4956) by @jost-s
- Create a release from branch release-20250430.001138
- Create a release from branch release-20250423.001128
- Create a release from branch release-20250416.001135
- Create a release from branch release-20250404.120841
- Remove lineage field (#4842) by @matthme
- Create a release from branch release-20250403.155950
- Remove agent_latest_pubkey (#4815) by @matthme
- Create a release from branch release-20250224.160528
- Minor typo in rust docs (#4737) by @mattyg
- Create a release from branch release-20250219.004534
- Create a release from branch release-20250212.005142
- Chore/todo comments out of cargodocs (#4700) by @mattyg
- Change all enums exposed in the conductor API to use serde `tag = "type"` and `content = "value"` (#4616) by @matthme
- Separate kitsune_p2p_timestamp from holochain (#4686) by @ThetaSinner
- Create a release from branch release-20250205.005133
- Capability grants info for an app (#4647) by @nphias
- Create a release from branch release-20250129.004417
- Create a release from branch release-20250122.005106
- Remove arbitrary from tests (#4640) by @ThetaSinner
- Create a release from branch release-20250115.130248
- Create a release from branch release-20250108.004547
- Create a release from branch release-20241225.004431
- Create a release from branch release-20241218.004735
- More clippy fixes for Arbitrary (#4536) by @ThetaSinner
- Create a release from branch release-20241211.005655
- Holo_hash/encoding feature by default in hdk #4427 (#4523) by @nphias
- Create a release from branch release-20241204.005132
- Create a release from branch release-20241127.004859
- Create a release from branch release-20241120.005400
- Create a release from branch release-20241113.005114
- Put some hdk functions behind unstable flag (#4371) by @maackle
- Create a release from branch release-20241106.004411
- Create a release from branch release-20241030.004523
- Deprecate precompiled wasm bundling for ios (#4376) by @mattyg
- Create a release from branch release-20241023.004427
- Transaction type generic over DbKindT (#4295) by @maackle
- Create a release from branch release-20241016.174505
- Add some fields to the DB; add disposable event sourcing experiment (#4265) by @maackle
- Create a release from branch release-20241009.004404
- Prepare 0.5 dev (#4316) by @ThetaSinner
- Create a release from branch release-20240926.150323
- Create a release from branch release-20240925.004452
- Slim down release builds part 1 (#4282) by @ThetaSinner
- Prepare upgrade to rust 1.81 (#4275) by @ThetaSinner
- Create a release from branch release-20240918.004220
- Feat/wasmer interpreter integration (#4206) by @mattyg
- Create a release from branch release-20240904.004824
- Adapt Open and CloseChain to allow for Agent migration as well (#4142) by @maackle
- Get agent activity full (#4221) by @ThetaSinner
- Add `get_agent_key_lineage` to mock hdk (#4225) by @ThetaSinner
- Remove unused `migrate_agent` call (#4141) by @maackle
- Create a release from branch release-20240828.004055
- Create a release from branch release-20240823.100115
- Dynamic db encryption (#4198) by @neonphog
- This pin should not have been removed by @maackle
- Absorb deepkey sdk and types crates (#4201) by @maackle
- Release 20240819.140303 (#4199) by @ThetaSinner
- Always initialise the agent arc on network join (#4197) by @ThetaSinner
- Deepkey integration (#3463) by @maackle
- Create a release from branch release-20240807.004831
- UseExisting provisioning strategy (#4075) by @maackle
- Create a release from branch release-20240717.004654
- Expose validation receipt info for actions (#4049) by @ThetaSinner
- Create a release from branch release-20240710.004628
- Add lineage to DNA manifest and add function to return installed cells by lineage (#4058) by @maackle
- Warrant-friendly consistency awaiting (#4071) by @maackle
- Create a release from branch release-20240626.004529
- Refactor Warrant to include author and timestamp (#4020) by @maackle
- Create a release from branch release-20240619.004549
- Create a release from branch release-20240612.004512
- Warrants pt 3 -- authoring and integrating warrants (#3875) by @maackle
- Update dependencies (#3991) by @ThetaSinner
- Create a release from branch release-20240605.200925
- Run unit tests on github's free runners (#3894) by @neonphog
- Deferred memproofs, pt 1 (#3928) by @maackle
- Create a release from branch release-20240529.004721
- Create a release from branch release-20240523.125153
- Bump wasmer to 0.0.94 (#3907) by @ThetaSinner
- Update rust and remove crate2nix (#3857) by @ThetaSinner
- Warrants (so far) (#3865) by @maackle
- Create a release from branch release-20240515.004441
- Create a release from branch release-20240508.003809
- Make open/close HDK callable (#3804) by @ThetaSinner
- Create a release from branch release-20240501.152652
- Fixup bad 0.3.0 holochain release (#3799) by @ThetaSinner
- Create a release from branch release-20240501.004555
- Prepare for 0.4 dev releases (#3786) by @ThetaSinner
- Check toml formatting in ci (#3781) by @ThetaSinner
- Create a release from branch release-20240425.115844
- Create a release from branch release-20240424.004413
- Dependency updates (#3657) by @ThetaSinner
- Create a release from branch release-20240417.004246
- Non-blocking wasm compilation (#3590) by @maackle
- Create a release from branch release-20240410.004338
- Update HDK documentation for `get_links` (#3561) by @ThetaSinner
- Create a release from branch release-20240403.160133
- :Topology` dimension split (#3482) by @maackle
  - Split dimension into space and time  * WIP  * Finish split  * Clippy
- Create a release from branch release-20240327.004300
- Revert workspace deps and touch all Cargo.toml to trigger re-dep (#3505) by @neonphog
- Create a release from branch release-20240320.003423
- Create a release from branch release-20240313.004312
- Hdk functions for encryption / decryption by *signing* agent keypairs (#3412) by @neonphog
- Merge branch 'release-20240306.004209' into develop by @holochain-release-automation2
- Create a release from branch release-20240306.004209
- Make it possible to `cargo check --all-features` (#3392) by @maackle
- Create a release from branch release-20240228.004140
- Fix HDK doc warnings (#3339) by @ThetaSinner
- Correct docs for call_info (#3338) by @ThetaSinner
- Clone management from the HDK (#3275) by @ThetaSinner
- Create a release from branch release-20240207.003254
- Create a release from branch release-20240201.115513
- Bump Rust version (#3267) by @ThetaSinner
- Create a release from branch release-20240124.004605
- Create a release from branch release-20240117.004514
- Chore/rename hdk entry defs (#2979) by @mattyg
- Create a release from branch release-20240112.112002
- Sync main to develop 2024 01 11 (#3196) by @ThetaSinner
- Create a release from branch release-20240110.003625 (#3191) by @ThetaSinner
- Update copyright dates to 2024 (#3185) by @joshuavial
- Create a release from branch release-20231222.142916
- Merge branch 'develop' by @ThetaSinner
- Merge release branch release-20231213.003542 (#3165) by @ThetaSinner
- Move conductor services to own crate (#3097) by @maackle
- Bump wasmer to 0.0.91 (#3136) by @thedavidmeister
- Create a release from branch release-20231213.003542
- Rename `remote_signal` to `send_remote_signal` (#3113) by @ThetaSinner
- Wasmer dir (#3025) by @thedavidmeister
- Hide Timestamp::now() from hdk (#3116) by @ThetaSinner
- Create a release from branch release-20231206.112234
- Feat/dna properties macro (#2880) by @mattyg
- Create a release from branch release-20231129.004341
- Refactor/move path to hdi (#2980) by @mattyg
- Create a release from branch release-20231122.004553
- Clean up mixed license usage (#2989) by @ThetaSinner
- Create a release from branch release-20231115.003452
- 2023 10 11 wasmer (#2951) by @thedavidmeister
- Create a release from branch release-20231101.003619
- Fix warnings (#2936) by @maackle
- Create a release from branch release-20231011.004956
- Refactor/avoid importing kitsune to client (#2857) by @mattyg
- Create a release from branch release-20231004.005318
- Add proptest::Arbitrary impls everywhere (#2727) by @maackle
- Unit tests for validation receipts (#2843) by @ThetaSinner
- Bump to rust-1.71.1 (#2660) by @neonphog
- Bump holochain-serialization and holochain-wasmer deps (#2844) by @maackle
- Create a release from branch release-20230920.004520
- Create a release from branch release-20230913.003318
- Holo-ready CHC API updates (#2698) by @maackle
- Create a release from branch release-20230823.003418
- Create a release from branch release-20230809.004243
- Create a release from branch release-20230802.003955
- Correct deleted vs updated in update_entry docs (#2621) by @ThetaSinner
- Create a release from branch release-20230726.004038
- Use `GetLinksInput` with `get_links` and expose filtering on author and timestamp (#2444) by @ThetaSinner
- Create a release from branch release-20230719.011122
- Update CHC API for Holo DO implementation (#2502) by @maackle
- Create a release from branch release-20230712.004839
- Point to current getrandom (#2556) by @ThetaSinner
- Create a release from branch release-20230705.005229
- Updates for contrafact 0.2.0 (#2535) by @maackle
- Feat/path tryinto string (#2544) by @mattyg
- Create a release from branch release-20230703.184956
- [DOCS] HDK callbacks should accept an argument which can be deserialized (#2282) by @ThetaSinner
- Create a release from branch release-20230621.004233
- Move entry def check to app validation and rewrite private entry check (#2059) by @maackle
- Wip on dna info 2 (#2366) by @thedavidmeister
- Create a release from branch release-20230614.004108
- Create a release from branch release-20230607.004739
- Create a release from branch release-20230531.004233
- Count links (#2430) by @ThetaSinner
- Create a release from branch release-20230524.003830
- Merge remote-tracking branch 'upstream/main' into develop by @steveej
- Create a release from branch release-20230503.003735
- Support multiple action or entry types in query filters (#2302) by @ThetaSinner
- Create a release from branch release-20230427.171927
- Create a release from branch release-20230426.003734
- Create a new hc command to run a holochain webrtc signal server (#2265) by @neonphog
- Preserialization dylib, Connor's work (#2218) by @maackle
- Create a release from branch release-20230420.162535
- Feature consistency for sqlite (#2248) by @ThetaSinner
- Create a release from branch release-20230413.232054
- 2023 03 27 node is blocked (#2148) by @thedavidmeister
- Create a release from branch release-20230412.003659
- 2023 03 20 node id kitsune (#2113) by @thedavidmeister
- Create a release from branch release-20230405.003224
- Tx5 - hc demo-cli (#2030) by @neonphog
- AppManifest `version` becomes `installed_hash` (#2157) by @maackle
- Merge remote-tracking branch 'upstream/main' into pr_merge_main_into_develop by @steveej
- Create a release from branch release-20230322.003727
- Improve HoloHash::from_raw_32 and use it (#2162) by @maackle
- Take `name` out of DnaDefHash (#2099) by @maackle
- Create a release from branch release-20230315.183209
- Tracing Fixes (#2079) by @neonphog
- Enforce blocks on remote call (#1988) by @thedavidmeister
- Block rejected validation receipts 4eva (#1987) by @thedavidmeister
- Changes to existing crates for diagnostic testing (#2003) by @maackle
- 2023 02 13 block kitsune (#1970) by @thedavidmeister
- 2023 01 24 block sketch (#1828) by @thedavidmeister
- Improve docs on recent changes (#1724) by @zippy
- OpType -> FlatOp + some reorg (#1909) by @maackle
- Create a release from branch release-20230126.223635
- Fix mock hdk tests (#1792) by @maackle
- Remove more ValidationPackage code (#1739) by @maackle
- Create a release from branch release-20230120.225800
- Added author to link (#1782) by @guillemcordoba
- 2023 01 19 wasmer version (#1781) by @thedavidmeister
- Update copyright year 2023 (#1740) by @Seb33300
- 2023 01 16 callback doc (#1766) by @thedavidmeister
- Remove docs saying links are to entries (#1767) by @thedavidmeister
- Document call_info in hdk (#1765) by @thedavidmeister
- Countersigning top level docs (#1768) by @thedavidmeister
- Create a release from branch release-20230117.165308
- Rollup of rust related PRs (#1735) by @steveej
- Create a release from branch release-20221223.034701
- Create a release from branch release-20221215.173657
- Apply develop versions to changed crates
- Create a release from branch release-20221130.011217
- Apply develop versions to changed crates
- Merge pull request #1670 from holochain/secure-primitive-dedupe by @maackle
- Remove duplicate secure_primitives! macro by @maackle
- Merge pull request #1667 from holochain/2022-11-23-ergo by @thedavidmeister
- Merge branch 'develop' into 2022-11-23-ergo by @thedavidmeister
- Merge pull request #1664 from holochain/2022-11-22-schedule by @thedavidmeister
- Merge branch 'develop' into 2022-11-22-schedule by @thedavidmeister
- Update time.rs by @thedavidmeister
- Fmt by @thedavidmeister
- Rename more stuff by @thedavidmeister
- Wip on renaming role id by @thedavidmeister
- Wip on renaming by @thedavidmeister
- Renaming zome id to zome index by @thedavidmeister
- Rename ZomeId to ZomeIndex by @thedavidmeister
- Create a release from branch release-20221123.011302
- Apply develop versions to changed crates
- Merge pull request #1663 from holochain/conductor-api-11-17 by @neonphog
- Serde_yaml v0.9 by @zo-el
- Create a release from branch release-20221116.012050
- Apply develop versions to changed crates
- Merge pull request #1650 from holochain/quantum-time-modifier by @maackle
- Add another feature flag by @maackle
- Add feature flag by @maackle
- Merge remote-tracking branch 'origin/develop' into quantum-time-modifier by @maackle
- Create a release from branch release-20221109.012313
- Apply develop versions to changed crates
- Add quantum_time as DnaModifier by @maackle
- Merge pull request #1620 from holochain/2022-10-10-wasmer by @thedavidmeister
- Lint by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-10-10-wasmer by @thedavidmeister
- Create a release from branch release-20221103.145333
- Apply develop versions to changed crates
- Merge pull request #1642 from holochain/fix/properties-not-set by @maackle
- Merge remote-tracking branch 'origin/develop' into fix/properties-not-set by @maackle
- Create a release from branch release-20221102.014648
- Apply develop versions to changed crates
- Add a test and make some other tests more expressive for no apparent reason by @maackle
- Create a release from branch release-20221026.192152
- Apply develop versions to changed crates
- Merge pull request #1605 from holochain/pr_pin_local_crates by @steveej
- Merge pull request #1609 from holochain/kitsune-diagnostics by @maackle
- Merge remote-tracking branch 'origin/develop' into kitsune-diagnostics by @maackle
- Merge pull request #1583 from holochain/2022-09-20-scheduler-debug by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-09-20-scheduler-debug by @thedavidmeister
- Scheduler debug by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-09-20-scheduler-debug by @thedavidmeister
- More scheduler debug by @thedavidmeister
- Merge pull request #1610 from holochain/more-metrics by @maackle
- Merge pull request #1633 from holochain/diagnostic-groundwork by @maackle
- Many small changes made during the course of diagnostic testing spike by @maackle
- Create a release from branch release-20221019.014538
- Apply develop versions to changed crates
- Merge pull request #1622 from holochain/remove-dylib-config by @maackle
- Remove dylib config in Cargo.tomls by @maackle
- Bump wasmer by @thedavidmeister
- Create a release from branch release-20221005.164304
- Apply develop versions to changed crates
- Merge pull request #1598 from holochain/less-inline-zome-sets by @maackle
- Use single InlineZome instead of set where possible by @maackle
- Create a release from branch release-20220928.014801
- Apply develop versions to changed crates
- Merge pull request #1584 from holochain/add-breaking-change by @maackle
- Add changelog about changelog revision by @maackle
- Merge remote-tracking branch 'origin/develop' into add-breaking-change by @maackle
- Create a release from branch release-20220921.145054
- Apply develop versions to changed crates
- Retroactively add a breaking change by @maackle
- Merge pull request #1575 from holochain/sweet-inline-zomes by @maackle
- Add aliases for old names by @maackle
- Apply suggestions from code review by @maackle
- :{callback -> function} by @maackle
- Create a release from branch release-20220914.013149
- Apply develop versions to changed crates
- Merge pull request #1565 from holochain/chore/dna-phenotype by @maackle
- Merge branch 'develop' into chore/dna-phenotype by @maackle
- Merge pull request #1571 from holochain/fix-lair-config by @neonphog
- Merge branch 'develop' into fix-lair-config by @neonphog
- Checkpoint by @neonphog
- Merge branch 'develop' into chore/dna-phenotype by @maackle
- Merge pull request #1538 from holochain/chain-graft by @maackle
- Merge remote-tracking branch 'origin/develop' into chain-graft by @maackle
- Merge pull request #1566 from holochain/remove-validation-package by @maackle
- Comment updates by @maackle
- Merge remote-tracking branch 'origin/develop' into chain-graft by @maackle
- Merge branch 'chain-query-order' into chain-graft by @maackle
- Merge remote-tracking branch 'origin/develop' into chain-graft by @maackle
- Clippy by @maackle
- Sweeping use of DnaPhenotypeOpt where applicable by @maackle
- Don't assign a random network seed if not set by @maackle
- Merge pull request #1547 from holochain/feat/dna-cloning by @maackle
- Merge branch 'develop' of https://github.com/holochain/holochain into feat/dna-cloning
- Merge pull request #1539 from holochain/chain-query-order by @maackle
- Merge remote-tracking branch 'origin/develop' into chain-query-order by @maackle
- Add proper test of chain query filter ordering by @maackle
- Rename ChainQueryFilter field by @maackle
- CHANGELOG by @maackle
- Allow descending order of chain query filter by @maackle
- Remove internal changes from changelogs
- Merge branch 'develop' of https://github.com/holochain/holochain into feat/dna-cloning
- Create a release from branch release-20220908.155008
- Apply develop versions to changed crates
- Introduce dna phenotype option struct
- Merge branch 'develop' of https://github.com/holochain/holochain into feat/dna-cloning
- Create a release from branch release-20220907.100911
- Apply develop versions to changed crates
- Merge pull request #1559 from holochain/pr-hdi-0.1-rollup by @steveej
- Create a release from branch release-20220907.014838
- Apply develop versions to changed crates
- Merge branch 'develop' of https://github.com/holochain/holochain into feat/dna-cloning
- Merge pull request #1501 from holochain/chain-item-generic by @maackle
- Merge pull request #1524 from holochain/chain-item-generic-isotest by @maackle
- Merge pull request #1458 from holochain/pub-hdk-crate by @thedavidmeister
- Merge branch 'develop' into pub-hdk-crate by @thedavidmeister
- Merge pull request #1457 from holochain/pub-hzt-crate by @thedavidmeister
- Merge branch 'develop' into pub-hzt-crate by @abe-njama
- Update zome.rs by @abe-njama
- Update inline_zome.rs by @abe-njama
- Very basic struct to hold strings that can be written to by the tracing crate and sent to the host when complete. by @abe-njama
- Merge branch 'develop' into pub-hdk-crate by @abe-njama
- Very basic struct to hold strings that can be written to by the traci…  8ec5960 …ng crate and sent to the host when complete by @abe-njama
- Include cloned cells in app_info
- Impl destroy clone cell
- Add conversion from app role id to clone id
- Set DNA name for clone cell
- Merge branch 'develop' of https://github.com/holochain/holochain into feat/dna-cloning
- Create a release from branch release-20220831.015922
- Apply develop versions to changed crates
- Merge pull request #1530 from holochain/must-get-agent-activity-cache-entry by @thedavidmeister
- Merge branch 'develop' into must-get-agent-activity-cache-entry by @thedavidmeister
- Add entry def to cache entry by @freesig
- Add origin time to DnaPhenotype
-  replace single fields by struct in DnaDef
- Combine dna phenotype props in new struct
- Merge branch 'develop' of https://github.com/holochain/holochain into feat/dna-cloning
- Merge pull request #1502 from holochain/must-get-agent-activity-host-fn by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into must-get-agent-activity-host-fn by @thedavidmeister
- Create a release from branch release-20220820.111904
- Apply develop versions to changed crates
- Merge pull request #1514 from holochain/test-wasm-memory by @thedavidmeister
- Merge branch 'develop' into test-wasm-memory by @thedavidmeister
- Merge pull request #1523 from holochain/crate-ver-info by @neonphog
- Merge branch 'crate-ver-info' of github.com:holochain/holochain into crate-ver-info by @neonphog
- Merge branch 'develop' into crate-ver-info by @neonphog
- Merge branch 'develop' into crate-ver-info by @neonphog
- Add hdi_version_req to BUILD_INFO by @neonphog
- Rollback wasmer by @thedavidmeister
- Wip on debugging wasm by @thedavidmeister
- Working must_get_agent_activity by @freesig
- Add interface and coacade test by @freesig
- Move clone id gen fn to app
- Merge branch 'develop' of https://github.com/holochain/holochain into feat/dna-cloning by @jost-s
- Create a release from branch release-20220817.013233
- Apply develop versions to changed crates
- Merge pull request #1483 from holochain/must-get-agent-activity by @freesig
- Merge branch 'develop' into must-get-agent-activity by @freesig
- Create a release from branch release-20220810.012252
- Apply develop versions to changed crates
- Merge branch 'develop' of github.com:/holochain/holochain into must-get-agent-activity by @freesig
- Merge branch 'develop' of github.com:holochain/holochain into must-get-agent-activity by @freesig
- Merge branch 'develop' into must-get-agent-activity by @freesig
- Merge branch 'add-chain-filter' of github.com:/holochain/holochain into must-get-agent-activity by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into must-get-agent-activity by @freesig
- Calc clone ids & create clone cells by @jost-s
- Merge branch 'develop' of https://github.com/holochain/holochain into feat/dna-cloning by @jost-s
- Create a release from branch release-20220803.124141
- Apply develop versions to changed crates
- Add types for clone cells to cell module by @jost-s
- Merge pull request #1470 from holochain/guillemcordoba-patch-2 by @guillemcordoba
- Added changelog item by @guillemcordoba
- Merge branch 'develop' into guillemcordoba-patch-2 by @guillemcordoba
- Create a release from branch release-20220728.122329
- Apply develop versions to changed crates
- Merge pull request #1463 from holochain/add-chain-filter by @freesig
- Merge branch 'develop' into add-chain-filter by @freesig
- Add ChainFilter and ChainFilterIter by @freesig
- Merge pull request #1488 from holochain/validation-match-helpers by @freesig
- Merge branch 'validation-match-helpers' of github.com:/holochain/holochain into validation-match-helpers by @freesig
- Merge branch 'develop' into validation-match-helpers by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into validation-match-helpers by @freesig
- Merge pull request #1461 from holochain/get-links-return by @freesig
- Merge branch 'develop' into get-links-return by @freesig
- Merge branch 'get-links-return' of github.com:/holochain/holochain into get-links-return by @freesig
- Merge branch 'develop' into get-links-return by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into get-links-return by @freesig
- Add zome id and link type to links that are returned by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into validation-match-helpers by @freesig
- WIP by @freesig
- Merge branch 'develop' into guillemcordoba-patch-2 by @guillemcordoba
- Merge pull request #1402 from holochain/fix-query by @maackle
- Update crates/hdk/CHANGELOG.md by @maackle
- Merge remote-tracking branch 'origin/develop' into fix-query by @maackle
- Create a release from branch release-20220713.013021
- Apply develop versions to changed crates
- Merge pull request #1485 from holochain/2022-07-12-role-id by @maackle
- Merge remote-tracking branch 'origin/develop' into 2022-07-12-role-id by @maackle
- Role call dispatch by @thedavidmeister
- Merge pull request #1473 from holochain/rename_hdi by @steveej
- Create a release from branch release-20220710.155915
- Apply develop versions to changed crates
- Merge remote-tracking branch 'origin/develop' into fix-query by @maackle
- Merge branch 'develop' into fix-query by @maackle
- Changelog by @maackle
- Make test pass by emitting error for unsupported filter combinations by @maackle
- Add failing test for SourceChain::query by @maackle
- Expose the TypedPath type in prelude by @guillemcordoba
- Create a release from branch release-20220701.181019
- Apply develop versions to changed crates
- Merge pull request #1466 from holochain/quantized-gossip by @maackle
- Merge remote-tracking branch 'origin/develop' into quantized-gossip by @maackle
- Create a release from branch release-20220629.012044
- Apply develop versions to changed crates
- Merge pull request #1453 from holochain/add-zome-id by @freesig
- Changelogs by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into add-zome-id by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into add-zome-id by @freesig
- Make scoped zome types easier to use by @freesig
- Add zome id back into headers by @freesig
- Merge pull request #1459 from holochain/quantized-gossip-5 by @maackle
- Fix missing import by @maackle
- Fix warning by @maackle
- WIP configurable recent gossip cutoff by @maackle
- Merge remote-tracking branch 'origin/develop' into HEAD by @maackle
- Merge pull request #1446 from holochain/integrate-shared-secret by @neonphog
- Merge branch 'develop' into integrate-shared-secret by @neonphog
- Merge pull request #1430 from holochain/weight-hdk01 by @maackle
- Merge branch 'develop' into weight-hdk01 by @maackle
- Create a release from branch release-20220622.133046
- Apply develop versions to changed crates
- Merge remote-tracking branch 'origin/develop' into weight-hdk01 by @maackle
- Merge remote-tracking branch 'origin/develop' into weight-1 by @maackle
- Merge branch 'weight' into weight-1 by @maackle
- Merge branch 'weight' into weight-1 by @maackle
- Add Rate types. Make weighted and unweighted header types. by @maackle
- Changelog by @neonphog
- Merge branch 'develop' into integrate-shared-secret by @neonphog
- Merge branch 'rand-update' into integrate-shared-secret by @neonphog
- Checkpoint shared secret by @neonphog
- Checkpoint shared secret by @neonphog
- Merge remote-tracking branch 'origin/develop' into quantized-gossip-5 by @maackle
- Merge pull request #1394 from holochain/2022-05-23-countersigning by @thedavidmeister
- Merge branch 'develop' into 2022-05-23-countersigning by @thedavidmeister
- Merge pull request #1445 from holochain/rand-update by @neonphog
- Changelog by @neonphog
- Merge branch 'develop' into rand-update by @neonphog
- Bump rand version && required associated fixes by @neonphog
- Merge branch 'develop' of github.com:holochain/holochain into 2022-05-23-countersigning by @thedavidmeister
- Merge pull request #1441 from holochain/record-action-entry by @maackle
- Grammar by @maackle
- Element -> Record by @maackle
- Grammar by @maackle
- Header -> Action by @maackle
- Merge branch 'develop' of github.com:holochain/holochain into 2022-05-23-countersigning by @thedavidmeister
- Create a release from branch release-20220616.084359
- Apply develop versions to changed crates
- Merge pull request #1436 from holochain/pr_backfill_changelog_pr1386 by @steveej
- Merge pull request #1386 from holochain/2022-05-17-wasm-metering by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-05-17-wasm-metering by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-05-17-wasm-metering by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-05-23-countersigning by @thedavidmeister
- Merge pull request #1434 from holochain/integrity-dna-def-changelog by @freesig
- Changelog for hdk by @freesig
- Changelog for holochain zome types by @freesig
- Merge pull request #1325 from holochain/integrity-dna-def by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into integrity-dna-def by @freesig
- Merge pull request #1410 from holochain/hdk-shared-secret by @neonphog
- Changelog by @neonphog
- Code review & merge fixes by @neonphog
- Merge branch 'develop' into hdk-shared-secret by @neonphog
- Merge branch 'develop' into hdk-shared-secret by @neonphog
- Missed some glue by @neonphog
- Cleanup by @neonphog
- Continue shared secret plumbing by @neonphog
- Use bytes for key ref by @neonphog
- Merge branch 'develop' into hdk-shared-secret by @neonphog
- Hdk shared secret api by @neonphog
- Renames by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into integrity-dna-def by @freesig
- Merge pull request #1380 from holochain/2022-05-04-wasmer-bump by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-05-04-wasmer-bump by @thedavidmeister
- Debug by @thedavidmeister
- Changelog by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-05-04-wasmer-bump by @thedavidmeister
- Merge branch 'develop' of github.com:/holochain/holochain into integrity-dna-def by @freesig
- Docs/bring out init fn (#1418) by @jost-s
- Remove zome from source chain and fix post commit by @freesig
- Add examples to hdi by @freesig
- Change path to use root by @freesig
- Merge branch 'develop' of github.com:holochain/holochain into integrity-dna-def by @freesig
- Create a release from branch release-20220608.011447
- Apply develop versions to changed crates
- Merge branch 'develop' of github.com:holochain/holochain into integrity-dna-def by @freesig
- Merge pull request #1392 from holochain/feat/readme/cargo-readme-workflow by @jost-s
- Document hdk by @freesig
- Document HDI types and holochain_zome_types by @freesig
- Add docs and remove require_validation_type by @freesig
- Remove unused types by @freesig
- Clippy by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into integrity-dna-def by @freesig
- Create a release from branch release-20220601.012853
- Apply develop versions to changed crates
- Merge pull request #1400 from Seb33300/develop by @steveej
- Update copyright year by @Seb33300
- Create a release from branch release-20220525.012131
- Apply develop versions to changed crates
- Merge pull request #1389 from holochain/fix-trace-feature-again by @freesig
- Actually test tracing is working and fix feature flag by @freesig
- WIP by @freesig
- WIP by @freesig
- WIP by @freesig
- WIP by @freesig
- WIP by @freesig
- Only commit to integrity zomes by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into integrity-dna-def by @freesig
- Adds integrity zomes to dna def by @freesig
- WIP by @freesig
- Basic metering by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-05-04-wasmer-bump by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-05-04-wasmer-bump by @thedavidmeister
- Fmt by @thedavidmeister
- Compiling wasmer bump by @thedavidmeister
- Merge branch 'quantized-gossip-2' into quantized-gossip-3 by @maackle
- Merge remote-tracking branch 'origin/quantized-gossip' into quantized-gossip-2 by @maackle
- Create a release from branch release-20220518.010753
- Apply develop versions to changed crates
- Merge branch 'quantized-gossip-2' into quantized-gossip-3 by @maackle
- Merge remote-tracking branch 'origin/develop' into quantized-gossip-2 by @maackle
- Merge pull request #1384 from holochain/remove-dev-dep-versions by @maackle
- Remove all versions from all dev deps by @maackle
- Merge branch 'quantized-gossip-2' into quantized-gossip-3 by @maackle
- Merge remote-tracking branch 'origin/develop' into quantized-gossip-2 by @maackle
- Merge remote-tracking branch 'origin/develop' into quantized-gossip-3 by @maackle
- Create a release from branch release-20220511.012519
- Apply develop versions to changed crates
- Revert "Revert all changes since 3e27aa0ad0c, except to dht crate" by @maackle
- Revert all changes since 3e27aa0ad0c, except to dht crate by @maackle
- Merge branch 'quantized-gossip-1' into quantized-gossip by @maackle
- Merge pull request #1365 from holochain/hdk-ergonomics by @maackle
- More info in the changelog by @maackle
- Update crates/hdk/CHANGELOG.md by @maackle
- Merge remote-tracking branch 'origin/develop' into hdk-ergonomics by @maackle
- Create a release from branch release-20220429.205522
- Apply develop versions to changed crates
- Use Into for hdk link hash params by @maackle
- Merge remote-tracking branch 'origin/develop' into quantized-gossip by @maackle
- Merge pull request #1348 from holochain/release-20220421.145237 by @neonphog
- Merge remote-tracking branch 'upstream/develop' into release-20220421.145237 by @steveej
- Merge pull request #1323 from holochain/docs/fix-item-links by @jost-s
- Inline env var with command by @jost-s
- Undo intra-doc error by @jost-s
- Introduce doc warning to see ci reaction by @jost-s
- Merge branch 'develop' of https://github.com/holochain/holochain into docs/fix-item-links by @jost-s
- Merge branch 'develop' of https://github.com/holochain/holochain into docs/fix-item-links by @jost-s
- Update all changelogs by @jost-s
- Create a release from branch release-20220421.145237
- Apply develop versions to changed crates
- Merge branch 'quantized-gossip-1' into quantized-gossip by @maackle
- Merge pull request #1335 from holochain/fix-trace-feature by @maackle
- Merge branch 'develop' into fix-trace-feature by @maackle
- Revert "Use Topology in DnaDef" by @maackle
- Merge remote-tracking branch 'origin/fix-trace-feature' into quantized-gossip by @maackle
- Fixes bad trace feature by @freesig
- Use Topology in DnaDef by @maackle
- Merge remote-tracking branch 'origin/develop' into quantized-gossip by @maackle
- Create a release from branch release-20220414.075333
- Apply develop versions to changed crates
- Merge pull request #1341 from holochain/pr_merge_main_to_develop by @steveej
- Add notes for unpublished release
- Create a release from branch release-20220413.011152
- Apply develop versions to changed crates
- Merge pull request #1330 from holochain/pr_hdk_changelog_fixup by @steveej
- Insert missing 0.0.127 section
- Remove link from UNRELEASED heading
- Merge pull request #1324 from holochain/integrity by @freesig
- Change idk to hdi by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into integrity by @freesig
- Merge pull request #1329 from holochain/main by @steveej
- Create a release from branch release-20220406.010602
- Apply develop versions to changed crates
- Merge pull request #1268 from holochain/2022-03-01-validate-deps-async by @thedavidmeister
- Changelog by @thedavidmeister
- Merge branch 'develop' into 2022-03-01-validate-deps-async by @thedavidmeister
- Merge pull request #1308 from holochain/2022-03-21-compound-external by @thedavidmeister
- Merge branch '2022-03-21-compound-external' of github.com:holochain/holochain into 2022-03-21-compound-external by @thedavidmeister
- Merge branch 'develop' into 2022-03-21-compound-external by @thedavidmeister
- Changelog by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-03-21-compound-external by @thedavidmeister
- Merge pull request #1316 from holochain/2022-03-29-compound-external-doc-update by @thedavidmeister
- Added `External` to table of multiformat prefixes by @pdaoust
- Merge branch 'develop' of github.com:holochain/holochain into 2022-03-21-compound-external by @thedavidmeister
- Wip on linkable hash by @thedavidmeister
- Wip on AnyLinkableHash by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-03-01-validate-deps-async by @thedavidmeister
- Validation deps async by @thedavidmeister
- Update docs by @freesig
- Change genesis self check to take dna info so it can run on the integrity zome by @freesig
- WIP by @freesig
- Merge pull request #1305 from holochain/stable-val-crates by @freesig
- Merge branch 'develop' into stable-val-crates by @freesig
- Merge pull request #1298 from holochain/2022-03-17-external-hash by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-03-17-external-hash by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-03-17-external-hash by @thedavidmeister
- Remove External from HDK HashInput by @thedavidmeister
- Changelog by @thedavidmeister
- Add external hash type by @thedavidmeister
- Fmt by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into stable-val-crates by @thedavidmeister
- Merge pull request #1311 from holochain/todo-sweep-2-maackle by @maackle
- Merge remote-tracking branch 'origin/develop' into todo-sweep-2-maackle by @maackle
- Create a release from branch release-20220330.010719
- Apply develop versions to changed crates
- Fixes, add serde "rc" feature by @maackle
- Merge branch 'develop' into todo-sweep-2-maackle by @maackle
- Remove Clone of MembraneProof data by @maackle
- Move zome and links by @freesig
- Test of fetch_op_regions works well by @maackle
- WIP need to test region query by @maackle
- Split quantum module into parts by @maackle
- Merge remote-tracking branch 'origin/quantized-gossip-1' into quantized-gossip by @maackle
- Merge pull request #1283 from holochain/unidirectional-arcs-2 by @maackle
- Merge remote-tracking branch 'origin/develop' into unidirectional-arcs-2 by @maackle
- Create a release from branch release-20220323.023956
- Apply develop versions to changed crates
- Merge pull request #1299 from holochain/docs/hdk-hash-module by @jost-s
- Update crates/hdk/src/hash.rs by @jost-s
- Update crates/hdk/src/hash.rs by @jost-s
- Correct location description by @jost-s
- Format by @jost-s
- Add section to clarify digest and location bytes by @jost-s
- Hard wrap doc comments at 80 chars by @jost-s
- Merge branch 'develop' of https://github.com/holochain/holochain into docs/hdk-hash-module by @jost-s
- Merge pull request #1292 from holochain/pr_merge_main_to_develop_again by @steveej
- Merge remote-tracking branch 'upstream/main' into pr_merge_main_to_develop_again
- Create a release from branch release-20220316.022611
- Apply develop versions to changed crates
- Add pr ref to changelog by @jost-s
- Add hash doc update to hdk changelog by @jost-s
- Explain holochain hashes in hdk::hash by @jost-s
- Swap paragraphs on hApps in HDK docs for more logical sequence by @jost-s
- Merge release-20220316.022611 back into develop (#1289) by @holochain-release-automation2
- Merge pull request #1266 from holochain/2022-02-27-baseless by @thedavidmeister
- Hdk changelog by @thedavidmeister
- Fmt by @thedavidmeister
- Merge branch 'develop' into 2022-02-27-baseless by @thedavidmeister
- Enum for hdk link types by @thedavidmeister
- Wip on link type by @thedavidmeister
- Merge branch 'unidirectional-arcs-2' into quantized-gossip-more-topo by @maackle
- Make DhtArc unidirectional -- defined by left edge rather than center by @maackle
- Merge remote-tracking branch 'origin/develop' into quantized-gossip-more-topo by @maackle
- Merge pull request #1260 from holochain/conductor-refactor by @maackle
- Merge branch 'conductor-refactor' into quantized-gossip by @maackle
- Merge remote-tracking branch 'origin/develop' into conductor-refactor by @maackle
- Merge pull request #1272 from holochain/release-20220303.215755 by @steveej
- Create a release from branch release-20220303.215755
- Apply develop versions to changed crates
- Merge pull request #1269 from holochain/conductor-refactor-2 by @maackle
- Merge pull request #1267 from holochain/kitsune-host-logic by @maackle
- Remove DnaStore from Spaces by @maackle
- Merge branch 'kitsune-host-logic' into quantized-gossip by @maackle
- Update op.rs by @freesig
- Merge remote-tracking branch 'origin/develop' into quantized-gossip by @maackle
- Merge pull request #1253 from holochain/rust-2021 by @steveej
- Merge branch 'develop' into rust-2021 by @maackle
- Merge pull request #1255 from holochain/release-20220223.090000 by @steveej
- Create a release from branch release-20220223.090000
- Apply develop versions to changed crates
- Merge branch 'optimized-gossip' into quantized-gossip by @maackle
- Merge remote-tracking branch 'origin/develop' into rust-2021 by @maackle
- Merge pull request #1252 from holochain/dna-origin-time-fixes by @maackle
- Use HOLOCHAIN_EPOCH for test origin_time by @maackle
- Update Rust edition to 2021 by @maackle
- Merge pull request #1212 from holochain/op-validation by @freesig
- Merge branch 'develop' into op-validation by @freesig
- Merge pull request #1230 from holochain/2022-02-14-keccak256 by @thedavidmeister
- Merge branch 'develop' into 2022-02-14-keccak256 by @thedavidmeister
- Merge pull request #1228 from holochain/2022-02-11-blake2b by @thedavidmeister
- Lint by @thedavidmeister
- Add blake2b to hdk by @thedavidmeister
- Merge pull request #1227 from holochain/2022-02-11-header-hash by @thedavidmeister
- Debug by @thedavidmeister
- Merge branch 'develop' into 2022-02-11-header-hash by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2022-02-11-header-hash by @thedavidmeister
- Convenience function for header hashing in hdk by @thedavidmeister
- Wip on header hash in hdk by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into op-validation by @freesig
- Merge pull request #1237 from holochain/default-sharding-2 by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into default-sharding-2 by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into develop by @freesig
- Changelog by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into op-validation by @freesig
- Merge pull request #1211 from holochain/signed-hashed by @freesig
- Merge branch 'signed-hashed' of github.com:/holochain/holochain into op-validation by @freesig
- Changelog by @freesig
- Fmt by @freesig
- Clippy by @freesig
- Merge branch 'develop' into signed-hashed by @maackle
- Merge branch 'develop' of github.com:/holochain/holochain into signed-hashed by @freesig
- Comments by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into signed-hashed by @freesig
- Pr fixes by @freesig
- Merge branch 'develop' into signed-hashed by @freesig
- Pr fixes by @freesig
- Change app validation to be via op instead of header by @freesig
- Add SignedHashed type by @freesig
- Merge remote-tracking branch 'origin/develop' into quantized-gossip by @maackle
- Merge pull request #1224 from holochain/dna-origin-time by @maackle
- Merge remote-tracking branch 'origin/develop' into dna-origin-time by @maackle
- Merge pull request #1186 from holochain/init-can-call by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into init-can-call by @freesig
- Merge branch 'init-can-call' of github.com:/holochain/holochain into init-can-call by @freesig
- Merge branch 'develop' into dna-origin-time by @maackle
- Use zero timestamp for DnaDefFixturator by @maackle
- Merge branch 'no-hashes-in-op-gossip' into quantized-gossip by @maackle
- Merge pull request #1222 from holochain/2022-02-10-hash-hdk by @thedavidmeister
- Add partialeq to hashinput by @thedavidmeister
- Merge branch 'develop' into 2022-02-10-hash-hdk by @thedavidmeister
- Update crates/holochain_zome_types/CHANGELOG.md by @thedavidmeister
- Fmt by @thedavidmeister
- Changelog by @thedavidmeister
- Extend hash interface in hdk by @thedavidmeister
- Merge branch 'dna-origin-time' of github.com:holochain/holochain into dna-origin-time by @maackle
- Merge branch 'develop' into dna-origin-time by @maackle
- Merge remote-tracking branch 'origin/develop' into dna-origin-time by @maackle
- Merge pull request #1226 from holochain/hdk-docs/add-deleteinput-to-delete-entry by @jost-s
- Merge branch 'develop' into hdk-docs/add-deleteinput-to-delete-entry by @jost-s
- Add accidentally deleted ? back in by @jost-s
- Clarify code example for delete_entry by @jost-s
- Add DeleteInput to docs for delete_cap_grant params by @jost-s
- Fmt using cargo by @jost-s
- Remove double doc quote by @jost-s
- Remove spaces between link [ and ` by @jost-s
- Simplify get_details first line description by @jost-s
- Add details about DeleteInput to fn delete_entry by @jost-s
- Unify input types in zome_types::entry by @jost-s
- Unify update description by @jost-s
- Link delete_entry to delete by @jost-s
- Merge remote-tracking branch 'origin/develop' into dna-origin-time by @maackle
- Merge pull request #1229 from holochain/release-20220211.091841 by @steveej
- Create a release from branch release-20220211.091841
- Apply develop versions to changed crates
- Add sys validation for DnaDef::origin_time by @maackle
- Fix another test by @maackle
- Update comment, fix test by @maackle
- Add origin_time to DnaDef by @maackle
- Merge pull request #1213 from holochain/chain-top-ordering-for-delete by @maackle
- Fix a warning, and some inadvertent formatting by @maackle
- Merge branch 'develop' into chain-top-ordering-for-delete by @maackle
- Merge pull request #1200 from holochain/flaky by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into flaky by @freesig
- Add test and update CHANGELOG by @maackle
- Allow backward-compatible DeleteInput for delete hdk functions by @maackle
- Merge pull request #1204 from holochain/pr/bump-serde-via-holochain-wasmer by @steveej
- Bump holochain-wasmer by @steveej
- Merge pull request #1207 from holochain/release-20220202.112225 by @steveej
- Create a release from branch release-20220202.112225
- Apply develop versions to changed crates
- Merge pull request #1138 from holochain/2021-11-22-properties-examples by @thedavidmeister
- Merge branch 'develop' into 2021-11-22-properties-examples by @thedavidmeister
- Merge pull request #1185 from holochain/clippy-stuff by @freesig
- Merge branch 'develop' into clippy-stuff by @freesig
- Merge branch 'develop' into 2021-11-22-properties-examples by @thedavidmeister
- Merge pull request #1196 from holochain/release-20220126.200716 by @steveej
- Create a release from branch release-20220126.200716
- Merge pull request #1194 from holochain/pr/bump-holochain-wasmer by @steveej
- Bump holochain-wasmer to fix a compilation issue by @steveej
- Properties feature in hdk by @thedavidmeister
- Merge pull request #1177 from holochain/2022-01-05-holo-hash-encoding by @thedavidmeister
- Merge branch 'develop' into 2022-01-05-holo-hash-encoding by @thedavidmeister
- Merge pull request #1172 from holochain/hdk-docs/add-introduction-and-restructure by @jost-s
- Merge branch 'develop' into hdk-docs/add-introduction-and-restructure by @jost-s
- Merge pull request #1193 from holochain/release-20220120.093525 by @jost-s
- Create a release from branch release-20220120.093525
- Incorporate further suggestions by @jost-s
- Update HDK changelog by @jost-s
- Merge branch 'develop' of https://github.com/holochain/holochain into hdk-docs/add-introduction-and-restructure by @jost-s
- Merge branch 'develop' into hdk-docs/add-introduction-and-restructure by @jost-s
- Get terminology straight & add link to app architecture by @jost-s
- Update HDK changelog by @jost-s
- Merge branch 'hdk-docs/add-introduction-and-restructure' of https://github.com/holochain/holochain into hdk-docs/add-introduction-and-restructure by @jost-s
- Merge branch 'develop' into hdk-docs/add-introduction-and-restructure by @jost-s
- Format by @jost-s
- Merge branch 'develop' of https://github.com/holochain/holochain into hdk-docs/add-introduction-and-restructure by @jost-s
- Add links to WASM details & specs
- Add introduction & move example section up
- Merge branch 'develop' of github.com:holochain/holochain into 2022-01-05-holo-hash-encoding by @thedavidmeister
- Merge pull request #1180 from holochain/2022-01-10-call by @thedavidmeister
- Merge branch 'develop' into 2022-01-10-call by @thedavidmeister
- Rename calltarget variants by @thedavidmeister
- Update crates/holochain_zome_types/src/call.rs by @thedavidmeister
- Changelog by @thedavidmeister
- Wip on call merge by @thedavidmeister
- Merge branch 'develop' into 2022-01-05-holo-hash-encoding by @thedavidmeister
- Merge pull request #1179 from holochain/release-20220106.093622 by @jost-s
- Create a release from branch release-20220106.093622
- Apply develop versions to changed crates
- Changelog by @thedavidmeister
- Holo_hash encoding in hdk features by @thedavidmeister
- Merge pull request #1171 from holochain/release-20211222.094252 by @steveej
- Merge remote-tracking branch 'upstream/develop' into release-20211222.094252 by @steveej
- Merge pull request #1156 from holochain/2021-12-01-hash-path by @thedavidmeister
- Merge branch '2021-12-01-hash-path' of github.com:holochain/holochain into 2021-12-01-hash-path by @thedavidmeister
- Merge branch 'develop' into 2021-12-01-hash-path by @thedavidmeister
- Merge pull request #1142 from holochain/2021-11-25-query-entry by @thedavidmeister
- Lint by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-11-25-query-entry by @thedavidmeister
- Remove header clone by @thedavidmeister
- Update docs by @thedavidmeister
- Merge branch '2020-11-25-query-entry' of github.com:holochain/holochain into 2021-11-25-query-entry by @thedavidmeister
- Update crates/holochain_zome_types/src/query.rs by @thedavidmeister
- Rename ChainQueryFilterSequenceRange to ChainQueryFilterRange by @thedavidmeister
- Clippy by @thedavidmeister
- Better tests for entry hash queries by @thedavidmeister
- Hash filter query tests by @thedavidmeister
- Hash bounded local queries by @thedavidmeister
- Changelog by @thedavidmeister
- Update crates/hdk/src/hash_path/path.rs by @thedavidmeister
- Leaner DHT_PREFIX by @thedavidmeister
- Merge branch 'develop' into 2021-12-01-hash-path by @thedavidmeister
- Merge branch 'develop' into 2021-12-01-hash-path by @thedavidmeister
- Wip on efficient path by @thedavidmeister
- Add leaf to path by @thedavidmeister
- Wip on single component hash paths by @thedavidmeister
- Create a release from branch release-20211222.094252
- Merge pull request #1108 from holochain/2021-11-10-remove-app-info by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-11-10-remove-app-info by @thedavidmeister
- Merge pull request #1159 from holochain/release-20211208.091009 by @steveej
- Create a release from branch release-20211208.091009
- Apply develop versions to changed crates
- Merge pull request #1117 from holochain/release-20211110.083530 by @steveej
- Create a release from branch release-20211110.083530
- Apply develop versions to changed crates
- Merge pull request #1104 from holochain/fix-private-entries by @freesig
- Merge pull request #1110 from holochain/fix-private-entries-maackle by @freesig
- Add `Element::privatized` by @maackle
- Remove app info by @thedavidmeister
- Merge pull request #1079 from holochain/2021-10-29-agent-chain-top by @thedavidmeister
- Merge branch 'develop' into 2021-10-29-agent-chain-top by @thedavidmeister
- Merge pull request #1081 from holochain/2021-10-31-extern-fns-zome-info by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-10-31-extern-fns-zome-info by @thedavidmeister
- Merge branch 'develop' into 2021-10-31-extern-fns-zome-info by @thedavidmeister
- Changelog by @thedavidmeister
- More tests for zome info by @thedavidmeister
- Extern fns in zome info by @thedavidmeister
- Merge branch 'develop' into 2021-10-29-agent-chain-top by @thedavidmeister
- Merge pull request #1080 from holochain/2021-10-31-zome-info-properties by @thedavidmeister
- Merge branch 'develop' into 2021-10-31-zome-info-properties by @thedavidmeister
- Lint changelog by @thedavidmeister
- Changelog by @thedavidmeister
- Add a placeholder for properties in zome info by @thedavidmeister
- Merge branch 'develop' into 2021-10-29-agent-chain-top by @thedavidmeister
- Merge pull request #1088 from holochain/release-20211103.094627 by @steveej
- Create a release from branch release-20211103.094627
- Apply develop versions to changed crates
- Merge branch 'develop' into 2021-10-29-agent-chain-top by @thedavidmeister
- Merge pull request #1078 from holochain/2021-10-29-call-info-function-name by @thedavidmeister
- Fmt by @thedavidmeister
- Merge branch 'develop' into 2021-10-29-call-info-function-name by @thedavidmeister
- Add function name to call info by @thedavidmeister
- Add chain top to agent info by @thedavidmeister
- Merge pull request #1055 from holochain/2021-10-19-entry-defs-zome-info by @thedavidmeister
- Fmt by @thedavidmeister
- Merge branch 'develop' into 2021-10-19-entry-defs-zome-info by @thedavidmeister
- Merge pull request #1063 from holochain/2021-10-21-cap-claim by @thedavidmeister
- Merge branch 'develop' into 2021-10-21-cap-claim by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-10-21-cap-claim by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-10-21-cap-claim by @thedavidmeister
- Provenance and cap grant on call info by @thedavidmeister
- Wip on cap claim in call info by @thedavidmeister
- Merge branch 'develop' into 2021-10-19-entry-defs-zome-info by @thedavidmeister
- Merge pull request #1052 from holochain/2021-10-18-zomes-on-dna by @thedavidmeister
- Merge branch 'develop' into 2021-10-18-zomes-on-dna by @thedavidmeister
- Merge branch 'develop' into 2021-10-18-zomes-on-dna by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-10-18-zomes-on-dna by @thedavidmeister
- Update crates/hdk/src/info.rs by @thedavidmeister
- Merge branch 'develop' into 2021-10-19-entry-defs-zome-info by @thedavidmeister
- Merge pull request #1074 from holochain/release-20211027.100746 by @steveej
- Merge remote-tracking branch 'upstream/develop' into release-20211027.100746 by @steveej
- Merge pull request #1004 from holochain/sourcechain-order by @freesig
- Merge branch 'develop' into sourcechain-order by @freesig
- Merge branch 'develop' into sourcechain-order by @freesig
- Merge branch 'develop' into sourcechain-order by @freesig
- Merge branch 'develop' into sourcechain-order by @neonphog
- Enforce source chain query ordering by @freesig
- Create a release from branch release-20211027.100746
- Apply develop versions to changed crates
- Merge pull request #1062 from holochain/release-20211021.140006 by @steveej
- Merge remote-tracking branch 'upstream/develop' into release-20211021.140006 by @steveej
- Create a release from branch release-20211021.140006
- Apply develop versions to changed crates
- Fmt by @thedavidmeister
- Merge branch 'develop' into 2021-10-19-entry-defs-zome-info by @thedavidmeister
- Merge pull request #1049 from holochain/2021-10-17-verbose-type by @thedavidmeister
- Merge branch 'develop' into 2021-10-17-verbose-type by @thedavidmeister
- Merge pull request #1058 from holochain/release-20211020.171211 by @steveej
- Create a release from branch release-20211020.171211
- Apply develop versions to changed crates
- Merge branch 'develop' into 2021-10-17-verbose-type by @thedavidmeister
- Debug by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-10-17-verbose-type by @thedavidmeister
- Lint by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-10-17-verbose-type by @thedavidmeister
- Update changelogs by @thedavidmeister
- Remove verbose types by @thedavidmeister
- Merge branch 'develop' into 2021-10-19-entry-defs-zome-info by @thedavidmeister
- Merge pull request #1047 from holochain/2021-10-15-call-info-as-at by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-10-15-call-info-as-at by @thedavidmeister
- Merge pull request #1044 from holochain/2021-10-14-no-zome-info-properties by @thedavidmeister
- Merge branch 'develop' into 2021-10-15-call-info-as-at by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-10-15-call-info-as-at by @thedavidmeister
- Merge branch 'develop' into 2021-10-15-call-info-as-at by @thedavidmeister
- Lint by @thedavidmeister
- Changelog by @thedavidmeister
- Add as at to call info by @thedavidmeister
- Fmg by @thedavidmeister
- Add entry_defs to zome info by @thedavidmeister
- Changelog by @thedavidmeister
- Add zome names to dna info by @thedavidmeister
- Merge branch '2021-10-14-no-zome-info-properties' of github.com:holochain/holochain into 2021-10-14-no-zome-info-properties by @thedavidmeister
- Merge branch 'develop' into 2021-10-14-no-zome-info-properties by @thedavidmeister
- Merge pull request #1051 from harlantwood/doc-fix-formatting by @neonphog
- Fix bullet points in generated HTML docs by @harlantwood
- Lint by @thedavidmeister
- Merge branch 'develop' into 2021-10-14-no-zome-info-properties by @thedavidmeister
- Merge pull request #1012 from holochain/2021-09-29-verbose-type by @thedavidmeister
- Lint by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-09-29-verbose-type by @thedavidmeister
- Merge pull request #1000 from holochain/2021-09-21-post-commit by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-09-21-post-commit by @thedavidmeister
- Merge pull request #1040 from holochain/new-lair-3 by @neonphog
- Merge branch 'develop' into new-lair-3 by @neonphog
- Merge pull request #1028 from holochain/pr/rust-1.55.0 by @steveej
- Merge branch 'develop' into new-lair-3 by @neonphog
- Merge branch 'develop' into new-lair-3 by @neonphog
- Checkpoint normalizing test keystores by @neonphog
- Merge branch 'develop' of github.com:holochain/holochain into 2021-09-21-post-commit by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-09-21-post-commit by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-09-21-post-commit by @thedavidmeister
- Merge branch 'develop' into 2021-09-21-post-commit by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-09-21-post-commit by @thedavidmeister
- Wip on post commit callback by @thedavidmeister
- Merge branch 'develop' into 2021-09-29-verbose-type by @freesig
- Changelog by @thedavidmeister
- Remove Links by @thedavidmeister
- Dna info by @thedavidmeister
- Merge pull request #1043 from holochain/release-20211013.091723 by @steveej
- Create a release from branch release-20211013.091723
- Apply develop versions to changed crates
- Merge pull request #1034 from holochain/mock-network-kitsune by @freesig
- Changes to kitsune to allow network mocking by @freesig
- Merge pull request #1026 from holochain/release-20211006.105406 by @steveej
- Create a release from branch release-20211006.105406
- Apply develop versions to changed crates
- Merge pull request #1023 from holochain/bump-rusqlite by @neonphog
- Bump rusqlite to 0.26.0 by @neonphog
- Merge pull request #1011 from holochain/release-20210929.090317 by @steveej
- Create a release from branch release-20210929.090317
- Merge pull request #984 from holochain/2021-09-06-schedule-do-something by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-09-06-schedule-do-something by @thedavidmeister
- Merge pull request #998 from holochain/release-20210922.083906 by @steveej
- Create a release from branch release-20210922.083906
- Merge pull request #997 from holochain/pr/fixup-documentation-urls by @steveej
- Update crates/holochain_zome_types/src/schedule.rs by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-09-06-schedule-do-something by @thedavidmeister
- Merge pull request #960 from holochain/2021-09-03-schedule-data by @thedavidmeister
- Merge branch 'develop' into 2021-09-03-schedule-data by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2020-09-03-schedule-data by @thedavidmeister
- Update crates/hdk/CHANGELOG.md by @thedavidmeister
- Merge branch '2020-09-06-schedule-do-something' of github.com:holochain/holochain into 2021-09-06-schedule-do-something by @thedavidmeister
- Merge branch 'develop' into 2021-09-06-schedule-do-something by @thedavidmeister
- Merge pull request #985 from holochain/2021-08-20-chain-top-ordering by @thedavidmeister
- Wip on controlled schedule by @thedavidmeister
- Merge branch '2021-08-20-chain-top-ordering' of github.com:holochain/holochain into 2021-09-06-schedule-do-something by @thedavidmeister
- Merge branch 'develop' into 2021-08-20-chain-top-ordering by @thedavidmeister
- Merge pull request #941 from holochain/2021-08-22-reorder-chain by @thedavidmeister
- Wip on async apply by @thedavidmeister
- Wip on rebase headers by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-09-06-schedule-do-something by @thedavidmeister
- Merge pull request #983 from holochain/release-20210916.085414 by @steveej
- Create a release from branch release-20210916.085414
- Merge pull request #981 from holochain/holo-hash-no-default-features by @steveej
- HDK builds by @maackle
- Merge pull request #962 from holochain/timestamp-millis by @maackle
- Merge pull request #970 from holochain/timestamp-new-crate by @maackle
- Merge pull request #971 from holochain/timestamp-standardization by @maackle
- Merge remote-tracking branch 'origin/develop' into timestamp-standardization by @maackle
- Merge pull request #972 from holochain/release-20210901.105419 by @steveej
- Applying develop versions to unpublished crates
- Merge remote-tracking branch 'upstream/develop' into release-20210901.105419 by @steveej
- Create a release from branch release-20210901.105419
- Put Timestamp::now() behind "now" feature; "full" includes "rusqlite" by @maackle
- Change all raw integer timestamps to use `Timestamp` by @maackle
- Switch Timestamp repr to microseconds by @maackle
- Move Timestamp into its own crate by @maackle
- Remove all unit-specific accessors for Timestamp by @maackle
- Standardize on microseconds in database by @maackle
- Standardize on milliseconds for ChainLock table expiry field by @maackle
- Clippy fix by @maackle
- Replace a few more instances of raw Timestamp member access by @maackle
- Standardize on sql millisecond representation; add some guardrails by @maackle
- Scheduler compiles by @thedavidmeister
- Wip on scheduling by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2020-09-06-schedule-do-something by @thedavidmeister
- Merge pull request #967 from holochain/bump-ghost-actor by @neonphog
- Work around the dry-run release automation... by @neonphog
- Update changelog by @thedavidmeister
- Add schedule wasm by @thedavidmeister
- Infallible externs by @thedavidmeister
- Changelog by @thedavidmeister
- Wip on schedule data by @thedavidmeister
- Merge pull request #950 from holochain/no-default-features-for-holo-hash by @maackle
- Merge branch 'develop' into no-default-features-for-holo-hash by @maackle
- Merge pull request #949 from holochain/pr/holochain-fixup-cargo.toml by @steveej
- Add API and crate badges by @steveej
- Extra README for holochain_types by @maackle
- Holochain_zome_types holo_hash dep with no-default-features by @maackle
- Merge pull request #948 from holochain/release-20210825.101130 by @steveej
- Create a release from branch release-20210825.101130
- Merge pull request #939 from holochain/2021-08-20-chain-top-ordering by @thedavidmeister
- Lint by @thedavidmeister
- Chain top ordering params by @thedavidmeister
- Wip on scratch chain top ordering by @thedavidmeister
- Wip on scratch chain top ordering by @thedavidmeister
- Merge pull request #931 from holochain/release-20210817.185301 by @steveej
- Create a release from branch release-20210817.185301
- Merge pull request #927 from holochain/2021-08-14-countersign-fail by @thedavidmeister
- Merge branch 'develop' into 2021-08-14-countersign-fail by @thedavidmeister
- Merge pull request #919 from holochain/2021-08-12-redundant-clones by @thedavidmeister
- Merge branch 'develop' into 2021-08-12-redundant-clones by @thedavidmeister
- Merge branch 'develop' into 2021-08-12-redundant-clones by @thedavidmeister
- Merge branch 'develop' into 2021-08-12-redundant-clones by @thedavidmeister
- Remove redundant clones by @thedavidmeister
- Changelog by @thedavidmeister
- Fmt by @thedavidmeister
- Fmt by @thedavidmeister
- Use correct entry hash by @thedavidmeister
- Merge pull request #923 from holochain/countersigning-success by @freesig
- Merge branch 'develop' of github.com:/holochain/holochain into countersigning-success by @freesig
- Merge pull request #903 from holochain/countersigning-workflow by @freesig
- Merge branch 'countersigning-response' of github.com:/holochain/holochain into countersigning-success by @freesig
- Merge branch 'countersigning-response' of github.com:/holochain/holochain into countersigning-success by @freesig
- Merge branch 'develop' into countersigning-workflow by @freesig
- Merge pull request #770 from holochain/handy-holohash-ext by @maackle
- Merge remote-tracking branch 'origin/develop' into handy-holohash-ext by @maackle
- Merge remote-tracking branch 'origin/develop' into handy-holohash-ext by @maackle
- Merge branch 'develop' into handy-holohash-ext by @maackle
- Merge remote-tracking branch 'origin/develop' into handy-holohash-ext by @maackle
- Merge remote-tracking branch 'origin/develop' into handy-holohash-ext by @maackle
- Merge branch 'develop' into handy-holohash-ext by @maackle
- Merge remote-tracking branch 'origin/develop' into handy-holohash-ext by @maackle
- Re-add test_utils feature to holo_hash by @maackle
- Successfully finish a countersigning session by @freesig
- Checks on incoming success by @freesig
- Merge branch 'develop' into countersigning-workflow by @freesig
- Merge pull request #916 from holochain/2021-07-07-countersigning-integrity-check by @thedavidmeister
- Merge branch 'develop' into 2021-07-07-countersigning-integrity-check by @thedavidmeister
- Merge pull request #868 from holochain/2021-07-07-countersigning-integrity-check by @thedavidmeister
- Merge pull request #895 from holochain/2021-07-28-accept-preflight by @thedavidmeister
- Merge branch '2021-07-07-countersigning-integrity-check' into 2021-07-28-accept-preflight by @thedavidmeister
- Merge branch 'develop' into 2021-07-07-countersigning-integrity-check by @thedavidmeister
- Merge pull request #781 from holochain/hdk-uses-featureless-zome-types by @maackle
- Fix lint issues by @maackle
- Fix application of features by @maackle
- Merge remote-tracking branch 'origin/develop' into hdk-uses-featureless-zome-types by @maackle
- Merge remote-tracking branch 'origin/develop' into hdk-uses-featureless-zome-types by @maackle
- Properly use fixturator feature in test wasms by @maackle
- Use no default-features for zome_types in hdk by @maackle
- Wip on moving countersigning integrity checks to methods by @thedavidmeister
- Merge branch 'develop' into 2021-07-07-countersigning-integrity-check by @thedavidmeister
- Update changelogs by @thedavidmeister
- Improve docs by @thedavidmeister
- Able to create and update countersigned entries by @thedavidmeister
- Merge branch '2021-07-07-countersigning-integrity-check' into 2021-07-28-accept-preflight by @thedavidmeister
- Merge branch 'develop' into 2021-07-07-countersigning-integrity-check by @thedavidmeister
- Compiling chain lock by @thedavidmeister
- Merge branch 'develop' into 2021-07-28-accept-preflight by @thedavidmeister
- Wip on locking preflights by @thedavidmeister
- Merge branch 'develop' into 2021-07-07-countersigning-integrity-check by @thedavidmeister
- Merge branch 'develop' into 2021-07-07-countersigning-integrity-check by @thedavidmeister
- Merge branch 'develop' into 2021-07-07-countersigning-integrity-check by @thedavidmeister
- Compiling countersigning integrity checks by @thedavidmeister
- Wip on validation by @thedavidmeister
- Merge branch '2021-06-16-countersigning' of github.com:holochain/holochain into 2021-07-07-countersigning-integrity-check by @thedavidmeister
- Wip on integrity check by @thedavidmeister
- Countersigning in memory workspace and workflow, still needs message back to signers by @freesig
- Merge pull request #834 from holochain/kitsune-gossip by @freesig
- Bump mockall by @freesig
- Merge pull request #893 from holochain/release-20210722.172107 by @steveej
- Merge remote-tracking branch 'upstream/develop' into release-20210722.172107 by @steveej
- Merge pull request #886 from holochain/2021-07-20-remote-call-multi by @thedavidmeister
- Merge branch 'develop' into 2021-07-20-remote-call-multi by @freesig
- Merge pull request #733 from holochain/subtle-encoding by @maackle
- Merge remote-tracking branch 'origin/develop' into subtle-encoding by @maackle
- Update crates/holochain_zome_types/src/lib.rs by @thedavidmeister
- Use subtle-encoding for Signature by @maackle
- Bulk link inputs by @thedavidmeister
- Vector inputs by @thedavidmeister
- Merge branch 'develop' into 2021-07-20-remote-call-multi by @thedavidmeister
- Multi get links and get link details by @thedavidmeister
- Get and get_details multi by @thedavidmeister
- Call multi by @thedavidmeister
- Remote call multi by @thedavidmeister
- Multi remote call by @thedavidmeister
- Create a release from branch release-20210722.172107
- Merge pull request #885 from holochain/experiment/release-debug by @steveej
- Applying develop versions to changed crates
  - The following crates changed since their most recent release and are therefore increased to a develop version:
  - Holochain-0.0.102-dev.0 - holochain_wasm_test_utils-0.0.2-dev.0 - holochain_cli-0.0.3-dev.0 - holochain_util-0.0.2-dev.0 - mr_bundle-0.0.2-dev.0 - holochain_zome_types-0.0.4-dev.0 - holochain_test_wasm_common-0.0.2-dev.0 - hdk_derive-0.0.4-dev.0 - fixt-0.0.4-dev.0 - holochain_keystore-0.0.2-dev.0 - hdk-0.0.102-dev.0 - holochain_sqlite-0.0.2-dev.0 - holochain_cli_bundle-0.0.2-dev.0 - holo_hash-0.0.4-dev.0 - holochain_state-0.0.2-dev.0 - holochain_p2p-0.0.2-dev.0 - holochain_cli_sandbox-0.0.3-dev.0 - kitsune_p2p-0.0.2-dev.0 - holochain_websocket-0.0.2-dev.0 - holochain_types-0.0.2-dev.0 - holochain_cascade-0.0.2-dev.0 - holochain_conductor_api-0.0.2-dev.0 - kitsune_p2p_types-0.0.2-dev.0 - kitsune_p2p_proxy-0.0.2-dev.0 - kitsune_p2p_transport_quic-0.0.2-dev.0
- Revert "setting develop versions to conclude 'release-20210624.155736'" by @steveej
- Merge pull request #877 from holochain/facts by @maackle
- Use new holochain_wasmer release by @maackle
- Use new holochain_serialized_bytes release by @maackle
- Merge branch 'develop' into facts by @maackle
- Merge pull request #880 from holochain/damien-windows by @ddd-mtl
- Merge branch 'develop' into damien-windows by @ddd-mtl
- Fixes for windows: by @ddd-mtl
- Fix bad fact by @maackle
- Clippy by @maackle
- Use contrafact release for deps by @maackle
- Update for latest contrafact, add missing Arbitrary impls by @maackle
- Merge remote-tracking branch 'origin/develop' into facts by @maackle
- Merge pull request #842 from holochain/2021-06-16-countersigning by @thedavidmeister
- Lint by @thedavidmeister
- Merge branch 'develop' into 2021-06-16-countersigning by @thedavidmeister
- Merge pull request #874 from holochain/2021-07-13-encrypt-docs by @thedavidmeister
- Update changelog by @thedavidmeister
- Reverse recipient and sender in x25519 decrypt new by @thedavidmeister
- Merge pull request #812 from holochain/2021-05-26-must by @thedavidmeister
- Merge branch 'develop' into 2021-05-26-must by @thedavidmeister
- Merge from develop by @thedavidmeister
- Merge remote-tracking branch 'origin/develop' into 2021-05-26-must by @thedavidmeister
- Merge branch 'develop' into 2021-05-26-must by @thedavidmeister
- Enumerate all outcomes for wasmerror mapping by @thedavidmeister
- Fmt by @thedavidmeister
- Update crates/hdk/src/entry.rs by @thedavidmeister
- Lint by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-05-26-must by @thedavidmeister
- Lint by @thedavidmeister
- Lint by @thedavidmeister
- Use holochain-wasmer v0.0.70 by @thedavidmeister
- Move error to callback result mapping to host by @thedavidmeister
- Wip on callback mapping by @thedavidmeister
- FQ name in map extern by @thedavidmeister
- Wip on try into for ValidateCallbackResult by @thedavidmeister
- Must get in hdk by @thedavidmeister
- Compiling must gets by @thedavidmeister
- Wip on must_get_header by @thedavidmeister
- Merge branch '2020-06-16-countersigning' of github.com:holochain/holochain into 2021-06-16-countersigning by @thedavidmeister
- Merge branch 'develop' into 2021-06-16-countersigning by @thedavidmeister
- Merge pull request #858 from holochain/app-status by @maackle
- Merge pull request #866 from holochain/cell-status-connected-to-app-status by @maackle
- Merge remote-tracking branch 'origin/develop' into cell-status-connected-to-app-status by @maackle
- Test of app status influencing cell status and vice versa by @maackle
- Review feedback by @thedavidmeister
- Fmt by @thedavidmeister
- Base data structures for countersigning by @thedavidmeister
- Merge branch 'develop' into 2021-06-16-countersigning by @thedavidmeister
- Merge pull request #854 from holochain/instance_cache_2 by @thedavidmeister
- Point to latest wasmer release by @thedavidmeister
- Merge remote-tracking branch 'origin/develop' into instance_cache_2 by @thedavidmeister
- Merge pull request #865 from holochain/release-20210624.155736 by @steveej
- Setting develop versions to conclude 'release-20210624.155736'
- Release-20210624.155736
- Add performance notes for wasmer upgrade by @steveej
- Merge branch 'develop' into fix-element-tuple-docs-typo
- Merge branch 'develop' into fix-element-tuple-docs-typo by @thedavidmeister
- Element as struct, with proper doc comments.
- Literally made Element a tuple. No breaking-changes.
- Typo fix for element's docs: triple to tuple
- Merge branch 'develop' of github.com:/holochain/holochain into instance_cache_2 by @freesig
- Merge pull request #848 from holochain/instance_cache by @steveej
- Working instancd cache by @freesig
- Trying to use wasmer cache by @freesig
- Merge branch 'develop' into instance_cache by @freesig
- Merge pull request #853 from holochain/pr/bump-holonix-20210627.003600 by @steveej
- Wip on countersigning structures by @thedavidmeister
- Merge branch 'develop' into 2021-06-16-countersigning by @thedavidmeister
- Merge pull request #836 from holochain/workaround_leak by @steveej
- Bump holochain-wasmer dependency to 0.0.71 by @steveej
- Use wasmer workaround by @freesig
- Wip on countersigning by @thedavidmeister
- Wip on countersigning as entry type by @thedavidmeister
- Merge pull request #822 from holochain/2021-06-11-hdk-sweep by @steveej
- Lint by @thedavidmeister
- Lint docs by @thedavidmeister
- Merge branch 'develop' into sqlite by @neonphog
- Merge pull request #807 from holochain/pr/merge-main-to-develop by @maackle
- Merge remote-tracking branch 'upstream/main' into pr/merge-main-to-develop by @steveej
- Merge pull request #676 from holochain/release-20210304.120604 by @steveej
- Multi-crate release performed on 2021-03-04 by @steveej
- Remove even more mentions of "third" by @steveej
  - I think this is obsolete because we're publishing this at the same crate as the previous iterations.
- Change fn result types to include `Callback` by @steveej
- Fix types in docstrings by @steveej
- Concluding multi-crate release on 2021-02-27 by @steveej
- Gossip hashes indexes by @freesig
- Merge remote-tracking branch 'origin/develop' into sqlite by @maackle
- Merge pull request #801 from holochain/2021-05-31-wasmer by @maackle
- Wasmer v0.0.69 by @thedavidmeister
- Updated wasmer by @thedavidmeister
- Merge remote-tracking branch 'origin/develop' into sqlite by @maackle
- Merge pull request #785 from holochain/init-failure-actually-fails by @maackle
- Merge branch 'develop' into init-failure-actually-fails by @neonphog
- Merge branch 'develop' into init-failure-actually-fails by @zippy
- Fix bug where failed init still allowed commits by @maackle
- Merge branch 'develop' of github.com:holochain/holochain into sqlite by @thedavidmeister
- Merge pull request #773 from holochain/2021-05-17-wasmer by @thedavidmeister
- Tidy up wasmer by @thedavidmeister
- Merge branch 'develop' into 2021-05-17-wasmer by @thedavidmeister
- Merge branch 'develop' into 2021-05-17-wasmer by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2020-05-17-wasmer by @thedavidmeister
- Clippy by @maackle
- Merge remote-tracking branch 'origin/develop' into sqlite by @maackle
- Merge pull request #783 from holochain/add-dna-def-to-genesis-self-check by @maackle
- Merge branch 'develop' into add-dna-def-to-genesis-self-check by @maackle
- Merge pull request #755 from holochain/experiment/macos-unstable-nixpkgs by @thedavidmeister
- Clips by @thedavidmeister
- Merge pull request #723 from holochain/pr/release-automation-milestone-1 by @steveej
- Add missing changelogs mostly with `unreleasable: true`, adjust version numbers by @steveej
  - The version change is necessary to adhere to our defined restriction of only wanting to release 0.0.X versions until we're ready for more stability promising version numbers.
- Merge branch 'develop' into add-dna-def-to-genesis-self-check by @maackle
- Merge pull request #779 from holochain/pr/holochain_util_and_apache_licenses by @steveej
- Change some licenses to Apache-2.0 by @steveej
- Merge pull request #780 from holochain/dna-def-as-zome-type by @maackle
- Add DnaDef to GenesisSelfCheckData by @maackle
- Move DnaDef fixturators to zome_types by @maackle
- Feature flag tweaks by @maackle
- Move DnaDef et al to zome_types, with feature flag by @maackle
- Merge branch 'develop' of github.com:holochain/holochain into sqlite by @freesig
- Merge pull request #708 from holochain/2021-03-22-deepkey-tweaks by @thedavidmeister
- Merge branch 'develop' into 2021-03-22-deepkey-tweaks by @thedavidmeister
- Update crates/holochain_zome_types/src/lib.rs by @thedavidmeister
- Mut_ref to mut for Element by @thedavidmeister
- Use test_utils feature for mut elements by @thedavidmeister
- Unpub fields by @thedavidmeister
- Mutable references to element internals for testing by @thedavidmeister
- Fmt by @thedavidmeister
- Merge branch '2021-03-22-deepkey-tweaks' of github.com:holochain/holochain into 2021-03-22-deepkey-tweaks by @thedavidmeister
- Merge branch 'develop' into 2021-03-22-deepkey-tweaks by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2021-03-22-deepkey-tweaks by @thedavidmeister
- Wip on deepkey tweaks by @thedavidmeister
- Links testing by @freesig
- Merge pull request #767 from holochain/sqlite-integrating by @freesig
- Merge branch 'develop' of github.com:holochain/holochain into sqlite by @freesig
- Merge branch 'develop' of github.com:holochain/holochain into sqlite by @freesig
- Merge remote-tracking branch 'origin/develop' into sqlite by @maackle
- More test fixes by @freesig
- Spike on publish and sys validation by @freesig
- Merge branch 'sqlite-cascade2' of github.com:holochain/holochain into sqlite-cascade2 by @maackle
- Move wire types into holochain_types by @freesig
- Clippy by @freesig
- More uniform naming by @maackle
- Refining agent activity query by @maackle
- Proper query for agent activity, but no results come back in test by @maackle
- Get details by @freesig
- Fix problems with temp vars by @maackle
- WIP GetAgentActivityQuery by @maackle
- Working get entry for cascade by @freesig
- Adds test for ChainHeadQuery by @maackle
- Add trait to queries by @freesig
- Takes the queries futher adding: Query, Union, Resolve and Render by @freesig
- Make insert_op insert everything by @maackle
- Merge remote-tracking branch 'origin/develop' into sqlite by @maackle
- Merge remote-tracking branch 'origin/develop' into sqlite by @maackle
- Remove all references to "lmdb" except those that speak to its legacy by @maackle
- Silly tinyid idea by @maackle
- Merge remote-tracking branch 'origin/develop' into facts by @maackle
- Merge pull request #758 from holochain/hdk-doc-fixes by @thedavidmeister
- Update README.md
- Update README.md
- Update README.md
- Update README.md
- Update README.md
- Update README.md
- Create ON-WASM.md
- Update README.md
- Rev 1
- Add some contrafact::Facts by @maackle
- Impl Arbitrary for Element by @maackle
- Merge remote-tracking branch 'origin/develop' into dhtop-arbitrary by @maackle
- Merge pull request #744 from holochain/graceful-genesis-fail by @maackle
- Merge pull request #754 from holochain/sync-genesis by @maackle
- Merge branch 'graceful-genesis-fail' into sync-genesis by @maackle
- Merge branch 'develop' into graceful-genesis-fail by @maackle
- Merge pull request #753 from holochain/pr/restructure-changelogs by @steveej
- Manually prepare all CHANGELOG files for automation by @steveej
- Merge branch 'develop' into graceful-genesis-fail by @maackle
- Include self-check in genesis workflow by @maackle
- Add genesis_self_check callback by @maackle
- Merge remote-tracking branch 'origin/develop' into sync-genesis by @maackle
- Merge pull request #731 from holochain/tx2-kitsune by @neonphog
- Merge pull request #735 from holochain/tx2-errors by @neonphog
- Merge branch 'tx2-kitsune' into tx2-errors by @neonphog
- Additional tracing by @neonphog
- Reworked genesis check, let's give this a shot by @maackle
- Impl Arbitrary for DhtOp by @maackle
- Merge pull request #734 from holochain/fix_caps by @maackle
- More HashSet -> BTreeSet conversions by @maackle
- CapGrants use BTreeSet by @freesig
- Merge pull request #643 from holochain/more_test_utils by @freesig
- Add more testing diagnostics by @freesig
- Merge pull request #689 from holochain/2021-03-11-ephemeral-sign by @thedavidmeister
- Clips by @thedavidmeister
- Merge branch '2020-03-11-ephemeral-sign' of github.com:holochain/holochain into 2021-03-11-ephemeral-sign by @thedavidmeister
- Merge branch 'develop' into 2021-03-11-ephemeral-sign by @thedavidmeister
- Merge pull request #674 from holochain/2021-03-03-mock-hdk by @thedavidmeister
- Move hosthdk init into hdk and out of extern by @thedavidmeister
- Conductor sign ephemeral by @thedavidmeister
- Hdk ephemeral sign by @thedavidmeister
- Clips by @thedavidmeister
- Lint by @thedavidmeister
- Use thread local for HDK by @thedavidmeister
- Fmt by @thedavidmeister
- Use standard wasmer by @thedavidmeister
- Use parking lot for hdk locking by @thedavidmeister
- Lint by @thedavidmeister
- Merge branch '2021-03-03-mock-hdk' of github.com:holochain/holochain into 2021-03-03-mock-hdk by @thedavidmeister
- Merge branch 'develop' into 2021-03-03-mock-hdk by @thedavidmeister
- Merge pull request #677 from holochain/release-20210304.120604 by @steveej
- (cargo-release) start next development iteration {{next_version}} by @steveej
- Multi-crate release performed on 2021-03-04 by @steveej
- Concurrent mocking by @thedavidmeister
- Fmt by @thedavidmeister
- Some unit tests for wasms by @thedavidmeister
- Wip on testing mock by @thedavidmeister
- Wip on wasm unit tests by @thedavidmeister
- Clippy by @thedavidmeister
- Fmt by @thedavidmeister
- Lint by @thedavidmeister
- Mockable hdk by @thedavidmeister
- Hdk mock wip by @thedavidmeister
- Merge pull request #673 from holochain/pr/hdk-docs-fixup by @steveej
- Change fn result types to include `Callback` by @steveej
- Fix types in docstrings by @steveej
- Merge pull request #670 from holochain/pr/hdk-remove-mentions-of-third by @steveej
- Remove even more mentions of "third" by @steveej
  - I think this is obsolete because we're publishing this at the same crate as the previous iterations.
- Merge pull request #669 from holochain/release-20210226.155101 by @steveej
- Concluding multi-crate release on 2021-02-27 by @steveej
- Specify holo_hash and holochain_zome_types version by @steveej
- Specify holo_hash version by @steveej
- Specify fixt version by @steveej
- Merge pull request #667 from holochain/release-20210226.155101 by @steveej
- Merge branch 'main' into release-20210226.155101 by @steveej
- Merge branch 'develop' of https://github.com/holochain/holochain into main
- Merge pull request #355 from holochain/develop by @zippy
- Multi-crate release performed on 2021-02-26 by @steveej
- Fix regex for version replacement by @steveej
- Merge pull request #662 from holochain/pr/prepare-release-hdk-0.0.100-alpha.0 by @steveej
- Fake version to hdk-v0.0.99 by @thedavidmeister
  - This fake version is put in place to enable `cargo release` bump to `0.0.100-alpha.1` with the next invocation.
  - Manual steps taken here: * Change version to 0.0.99 * Update all dependencies * docs: remove all hdk3 occurences * Cargo.toml: more accurate homepage and documentation links * Add changelog * Add release.toml for the workspace and the hdk
- Merge pull request #661 from holochain/pr/cal-license-everywhere by @steveej
- Use CAL-1.0 license file in all crates by @thedavidmeister
- Merge pull request #651 from holochain/happ-bundles-3 by @maackle
- Unpatch wasmer and serialization by @thedavidmeister
- Correct dep versions, add binary integration roundtrip test by @maackle
- Update copyright year by @maackle
- Merge remote-tracking branch 'origin/develop' into happ-bundles-3 by @maackle
- Docs (#639) by @thedavidmeister
- Merge remote-tracking branch 'origin/develop' into happ-bundles-3 by @maackle
- Add tracing to map_extern for deserialization fail (#649) by @thedavidmeister
- Merge remote-tracking branch 'origin/develop' into happ-bundles-2 by @maackle
- Rename EntryDef registration macro (#641) by @pospi
- Merge branch 'happ-bundles-1-point-5' into happ-bundles-2 by @maackle
- Merge branch 'mr-bundle' into happ-bundles-1-point-5 by @maackle
- Merge branch 'happ-bundles-1' into mr-bundle by @maackle
- Merge branch 'yaml-dna' into happ-bundles-1 by @maackle
- Merge remote-tracking branch 'origin/develop' into yaml-dna by @maackle
- Merge pull request #575 from holochain/inline-zome-validation by @maackle
- Write test using a validation rule in an inline zome by @maackle
- Merge branch 'develop' into yaml-dna by @freesig
- Split app entry trait impl macro out of entry_def macro (#616) by @pospi
- 2021 02 08 entry type (#633) by @thedavidmeister
- Merge branch 'happ-bundles-cli' into happ-bundles-2 by @maackle
- Merge remote-tracking branch 'origin/develop' into happ-bundles-1-point-5 by @maackle
- Merge remote-tracking branch 'origin/develop' into happ-bundles-2 by @maackle
- TK-06669 2021 02 08 sign vec (#631) by @thedavidmeister
- TK-06677 2021 02 08 prelude (#630) by @thedavidmeister
- Merge pull request #629 from holochain/2021-02-04-wasm-return by @thedavidmeister
- Merge branch '2021-02-04-wasm-return' of github.com:holochain/holochain into 2021-02-04-wasm-return by @thedavidmeister
- Merge branch 'develop' into 2021-02-04-wasm-return by @thedavidmeister
- Merge pull request #635 from holochain/feature-timestamp by @neonphog
- Fmt+clippy by @neonphog
- Remove some unnecessary comments by @pjkundert
- Improve compatibility of holochain types with Zome WASM by @pjkundert
- Bump to holochain wasmer 66 by @thedavidmeister
- Return host errors to the guest on host_fn by @thedavidmeister
- Merge pull request #624 from holochain/2021-02-02-tracing by @thedavidmeister
- Lint by @thedavidmeister
- Working wasm tracing by @thedavidmeister
- Tracing in wasm by @thedavidmeister
- Wip on tracing from wasm by @thedavidmeister
- Merge branch 'mr-bundle' into happ-bundles-2 by @maackle
- Merge branch 'happ-bundles-1' into mr-bundle by @maackle
- Merge branch 'happ-bundles-1' into happ-bundles-2 by @maackle
- Merge remote-tracking branch 'origin/develop' into happ-bundles-1 by @maackle
- Merge remote-tracking branch 'origin/happ-bundles' into bundle-refactor by @maackle
- Dna.json -> dna.yaml by @maackle
- Merge remote-tracking branch 'origin/develop' into mr-bundle by @maackle
- Merge pull request #576 from holochain/2021-01-06-into-inner by @thedavidmeister
- Rename fields to as_ref and set absolute paths to types in map_extern by @thedavidmeister
- Tighter scope on map extern by @thedavidmeister
- Rename all InputInner to Input by @thedavidmeister
- Lint by @thedavidmeister
- Update to latest wasmer by @thedavidmeister
- Wip on fixing call_remote by @thedavidmeister
- Compiling by @thedavidmeister
- Wip on new wasmer by @thedavidmeister
- Wip on EntryWithDefId by @thedavidmeister
- Use newer wasmer guest by @thedavidmeister
- Relax serde deps by @maackle
- Merge pull request #572 from holochain/2021-01-04-hash-entry-consistent by @thedavidmeister
- Better docs by @thedavidmeister
- Remove more lifetimes from hdk by @thedavidmeister
- Hdk entry conversion by @thedavidmeister
- Hdk entry conversion by @thedavidmeister
- Wip on TryFrom Entry hdk by @thedavidmeister
- Merge pull request #567 from holochain/conductor_read_limit by @freesig
- Merge pull request #530 from holochain/crate-reorg by @maackle
- Bump serialization and wasmer by @maackle
- Merge remote-tracking branch 'origin/develop' into crate-reorg by @maackle
- Merge pull request #559 from holochain/2020-12-15-crypto-box by @thedavidmeister
- Lint by @thedavidmeister
- Lit by @thedavidmeister
- Lint tests by @thedavidmeister
- Better docs by @thedavidmeister
- Merge by @thedavidmeister
- Working round trip for crypto box in lair and hdk by @thedavidmeister
- Merge branch '2020-12-01-secretbox' of github.com:holochain/holochain into 2020-12-15-crypto-box by @thedavidmeister
- Merge branch 'develop' into 2020-12-01-secretbox by @thedavidmeister
- Lint docs by @thedavidmeister
- Merge cleanup by @thedavidmeister
- Wip on xsalsa20_poly1305 cleanup by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2020-12-01-secretbox by @thedavidmeister
- Working roundtrip for secretbox by @thedavidmeister
- Wip on secretbox by @thedavidmeister
- Crypto box wip by @thedavidmeister
- Merge remote-tracking branch 'origin/develop' into crate-reorg by @maackle
- Merge pull request #553 from holochain/cool-conductor by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into cool-conductor by @freesig
- Merge pull request #546 from holochain/remote_signal by @freesig
- Bump versions by @freesig
- Merge remote-tracking branch 'origin/remote_signal' into cool-conductor by @maackle
- Change recv_remote_signal to take serialized bytes by @freesig
- Bump versions by @freesig
- Merge branch 'develop' into remote_signal by @neonphog
- Merge remote-tracking branch 'origin/develop' into cool-conductor by @maackle
- Add docs by @freesig
- Remote signal working with zome by @freesig
- Working with inline zome by @freesig
- Almost almost by @maackle
- Comprehensive glob import for zome_types by @maackle
- Merge branch 'crate-reorg-merge' into crate-reorg by @maackle
- Merge remote-tracking branch 'origin/develop' into crate-reorg by @maackle
- Merge pull request #551 from holochain/pr/nix-bump-rust-1.48.0 by @steveej
- Respect clippy lints for rust 1.48.0 by @steveej
- Fix doc tests by @maackle
- Remove monolith, get tests running by @maackle
- Fix up unused deps and feature flag alignment for zome_types by @maackle
- Merge branch 'inline-zome-errors' of https://github.com/holochain/holochain into crate-reorg by @freesig
- Merge branch 'develop' into inline-zome-errors by @freesig
- Merge pull request #536 from holochain/test_dup_paths by @freesig
- Merge branch 'develop' into test_dup_paths by @freesig
- Merge branch 'fixing_stuff_wip' of https://github.com/holochain/holochain into test_dup_paths by @freesig
- Merge branch 'fixing_stuff_wip' of https://github.com/holochain/holochain into test_dup_paths by @freesig
- Sort links, remove dup paths, fix shutdown issue by @freesig
- Merge branch 'develop' into inline-zome-errors by @maackle
- Merge pull request #518 from holochain/fixing_stuff_wip by @freesig
- Pr fixes by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into fixing_stuff_wip by @freesig
- Merge branch 'develop' into fixing_stuff_wip by @neonphog
- Merge branch 'develop' of https://github.com/holochain/holochain into fixing_stuff_wip by @freesig
- Add new get calls blocking and content by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into fixing_stuff_wip by @freesig
- Change cascade to not wait of fetches by default by @freesig
- Obsolete need for Dna and Ribosome to live together, by @maackle
- Merge pull request #521 from holochain/test-conductor by @maackle
- Merge remote-tracking branch 'origin/develop' into test-conductor by @maackle
- Multi conductor test with local network (too verbose, needs work) by @maackle
- Hdk3 + hdk3_derive check out by @maackle
- Holochain_lmdb checks out by @maackle
- Merge remote-tracking branch 'origin/develop' into crate-reorg by @maackle
- Merge remote-tracking branch 'origin/develop' into fix_diag by @maackle
- Use * import for zome_type in many places by @maackle
- Put fixturators in proper crates by @maackle
- Move zome_types to own crate, but with fixturator errors by @maackle
- All tests check out, but not all pass (see detail) by @maackle
- Monolith checks out by @maackle
- : -> crate:: by @maackle
- Revert "Remove all Cargo.toml from moved crates" by @maackle
- Naive prefix all crate:: with monolith::{cratename} by @maackle
- Rampant naive "monolith::" prefix by @maackle
- Move wasm_workspace; all lib.rs become mod.rs by @maackle
- Remove all Cargo.toml from moved crates by @maackle
- Move hdk_derive by @maackle
- Move all holochain crates into monolith crate by @maackle
- Cargo fmt -- --config=flatten_imports=true (using rustfmt fork) by @maackle
- Merge pull request #512 from holochain/inline-zome by @maackle
- Merge branch 'develop' into inline-zome by @maackle
- Merge pull request #520 from holochain/guillemcordoba-patch-2 by @guillemcordoba
- Added note by @guillemcordoba
- Fixed format by @guillemcordoba
- Merge branch 'develop' into guillemcordoba-patch-2 by @guillemcordoba
- Fixed call_remote documentation by @guillemcordoba
- Merge branch 'develop' into inline-zome by @maackle
- Merge pull request #511 from holochain/host-fn-api-struct by @maackle
- Merge branch 'host-fn-api-struct' into inline-zome by @maackle
- Merge remote-tracking branch 'origin/develop' into host-fn-api-struct by @maackle
- Merge remote-tracking branch 'origin/develop' into inline-zome by @maackle
- Merge pull request #501 from holochain/admin_peer_peer_inject by @freesig
- Merge branch 'admin_peer_peer_inject' of https://github.com/holochain/holochain into admin_peer_peer_inject by @freesig
- Merge branch 'develop' into admin_peer_peer_inject by @maackle
- Pr fixes by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into admin_peer_peer_inject by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into admin_peer_peer_inject by @freesig
- Merge branch 'admin_peer_peer_inject_fixing' into admin_peer_peer_inject by @freesig
- Trying to fix by @freesig
- Cargo fmt -- --config=merge_imports=true (unstable rustfmt) by @maackle
- Use Infallible errors for final hookup by @maackle
- Merge branch 'host-fn-api-struct' into inline-zome by @maackle
- Unify zome_io with host_fns under HostFnApi by @maackle
- WIP HostFnApi trait by @maackle
- Wrap EmitSignalInput in AppSignal newtype by @maackle
- New wasm_io macro structure to link input and output by @maackle
- Merge pull request #489 from holochain/2020-11-16-export-hdk-externs by @thedavidmeister
- Merge branch 'develop' into 2020-11-16-export-hdk-externs by @thedavidmeister
- Fmt by @thedavidmeister
- Merge by @thedavidmeister
- Merge branch 'develop' into 2020-11-16-export-hdk-externs by @thedavidmeister
- Merge branch 'develop' into 2020-11-16-export-hdk-externs by @thedavidmeister
- Merge branch 'develop' into 2020-11-16-export-hdk-externs by @thedavidmeister
- Merge branch 'develop' into 2020-11-16-export-hdk-externs by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2020-11-16-export-hdk-externs by @thedavidmeister
- Lint by @thedavidmeister
- Mod based extern mapping by @thedavidmeister
- Lint by @thedavidmeister
- Mod based extern mapping by @thedavidmeister
- Squelch warnings for CI by @maackle
- Fix call() zome mismatch problem by @maackle
- WIP Fixing bridge_call test by @maackle
- Make ZomeCallInvocation take Zome instead of ZomeName by @maackle
- Introduce ZomeDef/Zome; pass Zomes around where needed by @maackle
- Merge remote-tracking branch 'origin/develop' into inline-ribosome-zome-enum by @maackle
- Merge pull request #505 from holochain/get-links-scaling-bug by @thedavidmeister
- Fmt by @thedavidmeister
- Merge pull request #500 from pospi/fix-entrydef-macro-scoping by @thedavidmeister
- Merge branch 'develop' into fix-entrydef-macro-scoping by @thedavidmeister
- Merge pull request #502 from holochain/2020-11-17-references-hdk by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2020-11-17-references-hdk by @thedavidmeister
- Merge pull request #503 from holochain/hotfix by @freesig
- Merge pull request #490 from holochain/inject_agent_info by @freesig
- Merge branch 'develop' into inject_agent_info by @freesig
- Local network tests by @freesig
- Apply suggestions from code review by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2020-11-17-references-hdk by @thedavidmeister
- Merge branch 'develop' of github.com:holochain/holochain into 2020-11-17-references-hdk by @thedavidmeister
- Lint by @thedavidmeister
- Hdk references by @thedavidmeister
- Merge branch 'develop' into fix-entrydef-macro-scoping by @thedavidmeister
- Merge pull request #487 from holochain/2020-11-16-infallible-debug by @thedavidmeister
- Merge branch 'develop' into 2020-11-16-infallible-debug by @thedavidmeister
- Merge pull request #486 from holochain/2020-11-16-host-externs by @thedavidmeister
- Merge branch 'develop' into 2020-11-16-host-externs by @thedavidmeister
- Merge branch 'develop' into 2020-11-16-host-externs by @thedavidmeister
- Bump upstream wasmer and remove host_externs! by @thedavidmeister
- Merge branch 'develop' into 2020-11-16-infallible-debug by @thedavidmeister
- Merge branch 'develop' into 2020-11-16-infallible-debug by @thedavidmeister
- Make debug infallible in hdk by @thedavidmeister
- Fix missing prefix in entry def macro by @pospi
- Merge pull request #494 from holochain/2020-11-17/clone-hdkerror by @neonphog
- Satisfy clone trait for HdkError by @zo-el
- Stub RibosomeT impl for InlineDna by @maackle
- Add POC InlineDna by @maackle
- Add zome_types::prelude by @maackle
- Merge pull request #488 from holochain/2020-11-16-thiserror-again by @zippy
- Thiserror bump missed places by @zippy
- Merge pull request #485 from holochain/2020-11-13-thiserror by @thedavidmeister
- Merge branch 'develop' into 2020-11-13-thiserror by @thedavidmeister
- Merge pull request #478 from holochain/2020-11-12-functionize-hdk by @thedavidmeister
- Lint by @thedavidmeister
- Functionize more hdk by @thedavidmeister
- Functionize get! by @thedavidmeister
- Functionize get_link_details! by @thedavidmeister
- Functionize get_details! by @thedavidmeister
- Functionize get_details! by @thedavidmeister
- Functionize emit_signal! by @thedavidmeister
- Functionize delete! by @thedavidmeister
- Functionize delete_link! by @thedavidmeister
- Functionize delete_link! by @thedavidmeister
- Functionize create! by @thedavidmeister
- Functionize create_link by @thedavidmeister
- Functionize call_remote by @thedavidmeister
- Funtionize call_remote by @thedavidmeister
- Funtionize agent_info by @thedavidmeister
- Functionize delete_entry by @thedavidmeister
- Functionize update cap grant by @thedavidmeister
- Functionize generate_cap_secret by @thedavidmeister
- Functionize generate_cap_secret by @thedavidmeister
- Functionize delete_cap_grant by @thedavidmeister
- Wip on crud functions for hdk by @thedavidmeister
- Functionise create cap grant by @thedavidmeister
- Functionize create_cap_claim by @thedavidmeister
- Bump thiserror by @zippy
- Merge pull request #462 from holochain/get_details_rejected by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into get_details_rejected by @freesig
- Merge pull request #459 from holochain/holohash-39 by @maackle
- Merge remote-tracking branch 'origin/develop' into holohash-39 by @maackle
- Add doc diagram. Full -> Untyped, Raw -> Full by @maackle
- More usage of consts by @maackle
- Retype agent hash as entry hash; fix KeystoreSender slice problem by @maackle
- Big rename, hunting down remaining failure causes by @maackle
- Using 39 bytes where appropriate by @maackle
- Merge branch 'develop' of https://github.com/holochain/holochain into get_details_rejected by @freesig
- Merge pull request #444 from holochain/get_agent_activity_host_fn by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into get_agent_activity_host_fn by @freesig
- Merge pull request #453 from holochain/call by @freesig
- Add cell id to allow bridge calls in call by @freesig
- Pr fixes, take dna hash by @freesig
- Merge branch 'develop' into call by @freesig
- Merge branch 'develop' into call by @freesig
- Working call by @freesig
- Pr fixes by @freesig
- Host func connected and store rejected activity by @freesig
- Slow lmdb reader by @freesig
- Working by @freesig
- Host func connected and store rejected activity by @freesig
- Get details returns rejected by @freesig
- Merge pull request #437 from holochain/create_link_hash_on_links by @freesig
- Merge branch 'develop' into create_link_hash_on_links by @freesig
- Merge pull request #434 from holochain/damien-minor by @maackle
- Merge branch 'develop' into damien-minor by @zippy
- Merge branch 'develop' into damien-minor by @Connoropolous
- Update crates/zome_types/src/capability/secret.rs by @freesig
- Merge branch 'develop' into damien-minor by @ddd-mtl
- Made error message actionable by @ddd-mtl
- Merge branch 'develop' into create_link_hash_on_links by @freesig
- Merge pull request #408 from holochain/cache_val_pack by @freesig
- Merge branch 'develop' into cache_val_pack by @freesig
- Merge remote-tracking branch 'origin/develop' into unignore by @maackle
- Merge branch 'develop' into unignore by @maackle
- Unignore some tests by @maackle
- Merge branch 'develop' into cache_val_pack by @freesig
- Merge branch 'get_agent_activity_cascade' of https://github.com/holochain/holochain into cache_val_pack by @freesig
- Broken by @freesig
- Merge pull request #414 from holochain/custom_val_pack by @freesig
- Merge branch 'cache_val_pack' of https://github.com/holochain/holochain into custom_val_pack by @freesig
- Custom val package by @freesig
- Adds the hash of the create link to links by @freesig
- Merge pull request #407 from holochain/get_agent_activity_cascade by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into get_agent_activity_cascade by @freesig
- Merge pull request #428 from holochain/send_updates_to_header_auth by @freesig
- Updates send to element authority by @freesig
- Simplify get agent activity by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into get_agent_activity_cascade by @freesig
- Merge pull request #404 from holochain/get_agent_activity by @freesig
- Remove activity by @freesig
- Debugging by @freesig
- Clean up and add highest_observed by @freesig
- Get agent activity returns status by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into get_agent_activity by @freesig
- Merge pull request #399 from holochain/get_validation_package by @freesig
- Added network call by @freesig
- Merge branch 'develop' into get_validation_package by @freesig
- Merge pull request #394 from holochain/authored_prefix by @freesig
- Merge pull request #396 from holochain/write_cache by @freesig
- Merge branch 'authored_prefix' of https://github.com/holochain/holochain into write_cache by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into authored_prefix by @freesig
- Merge branch 'develop' into write_cache by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into authored_prefix by @freesig
- Merge branch 'details_hashes' of https://github.com/holochain/holochain into authored_prefix by @freesig
- Merge branch 'details_hashes' of https://github.com/holochain/holochain into authored_prefix by @freesig
- Merge branch 'develop' into get_validation_package by @freesig
- Merge pull request #344 from holochain/val_package by @freesig
- Merge branch 'val_package' of https://github.com/holochain/holochain into get_validation_package by @freesig
- Name change by @freesig
- Get validation package from author by @freesig
- Merge branch 'validate_entry_id' of https://github.com/holochain/holochain into val_package by @freesig
- Merge pull request #390 from holochain/details_hashes by @thedavidmeister
- Merge branch 'develop' into details_hashes by @freesig
- Merge pull request #393 from holochain/small-fries by @maackle
- Merge branch 'develop' into small-fries by @maackle
- Some comments by @maackle
- Merge branch 'develop' into details_hashes by @freesig
- Merge pull request #376 from Connoropolous/develop by @thedavidmeister
- Merge branch 'develop' into develop by @Connoropolous
- Fmt by @Connoropolous
- Add specific Error type to to_app_option TryFrom trait constraint by @Connoropolous
- Add comments and zome id to zome info by @freesig
- Merge branch 'develop' into details_hashes by @freesig
- Merge pull request #371 from holochain/signals by @thedavidmeister
- Merge remote-tracking branch 'origin/develop' into signals by @maackle
- Failing e2e test by @maackle
- Get_details and get_link_details return SignedHeaderHashed by @freesig
- Merge pull request #330 from holochain/app_val by @freesig
- Change to chain to sub chain by @freesig
- Merge branch 'validate_entry_id' of https://github.com/holochain/holochain into val_package by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into app_val by @freesig
- Merge pull request #367 from holochain/2020-09-18-sign by @thedavidmeister
- Verify signature in wasm by @thedavidmeister
- Working sign in wasm by @thedavidmeister
- Add sign to wasm ribosome by @thedavidmeister
- Move all signature types into zome types by @thedavidmeister
- Merge branch 'app_val' of https://github.com/holochain/holochain into val_package by @freesig
- Merge pull request #319 from holochain/lair-mock-keystore by @neonphog
- Merge branch 'develop' into lair-mock-keystore by @neonphog
- Merge pull request #350 from holochain/optional-cap-secret by @thedavidmeister
- Merge branch 'develop' into optional-cap-secret by @thedavidmeister
- Merge pull request #351 from holochain/refactor-element-with-element-entry by @thedavidmeister
- Merge branch 'develop' into refactor-element-with-element-entry by @maackle
- Clippy by @maackle
- Use ElementEntry instead of Option<Entry> for Element by @maackle
- Delete From<()> for CapSecret by @thedavidmeister
- Merge by @thedavidmeister
- Merge pull request #347 from holochain/query-returns-elements by @maackle
- Include elements as an option for query by @freesig
- Merge branch 'develop' of https://github.com/holochain/holochain into query-returns-elements by @freesig
- Query returns a vector of elements by @guillemcordoba
- Use Option<CapSecret> instead of requiring a dummy one when not needed by @maackle
- Merge branch 'list-cells' into lair-mock-keystore by @neonphog
- Merge pull request #343 from holochain/2020-09-13-hdk-sweep by @thedavidmeister
- Merge branch '2020-09-13-hdk-sweep' of github.com:Holo-Host/holochain into 2020-09-13-hdk-sweep by @thedavidmeister
- Update crates/hdk/src/host_fn/create_link.rs by @thedavidmeister
- Update crates/hdk/src/guest_callback/entry_defs.rs by @thedavidmeister
- Update crates/hdk/src/host_fn/create_link.rs by @thedavidmeister
- Update crates/hdk/src/hash_path/path.rs by @thedavidmeister
- Update crates/hdk/src/host_fn/get_link_details.rs by @thedavidmeister
- Update crates/hdk/src/host_fn/get_links.rs by @thedavidmeister
- Update crates/hdk/src/host_fn/get_links.rs by @thedavidmeister
- Update crates/zome_types/src/metadata.rs by @thedavidmeister
- Remove dangerous From impl for cap secret by @thedavidmeister
- Merge by @thedavidmeister
- Merge branch 'develop' into rsm-release by @zippy
- Update urls by @zippy
- Fmt by @thedavidmeister
- Query docs by @thedavidmeister
- Update hdk docs by @thedavidmeister
- Sweep hdk comments by @thedavidmeister
- Renames and comments for hdk by @thedavidmeister
- Lint by @thedavidmeister
- Merge branch '2020-08-25-hdk-docs' of github.com:Holo-Host/holochain into 2020-09-13-hdk-sweep by @thedavidmeister
- Wip on hdk docs by @thedavidmeister
- Simplify crud names and structure in hdk by @thedavidmeister
- Rename CreateEntry to Create by @thedavidmeister
- Rename LinkRemove to DeleteLink by @thedavidmeister
- Rename link_entries to create_link by @thedavidmeister
- Rename ElementDelete to DeleteElement by @thedavidmeister
- Rename EntryUpdate to UpdateEntry by @thedavidmeister
- Rename entry_hash to hash_entry by @thedavidmeister
- Rename commit to create by @thedavidmeister
- Merge branch 'develop' into lair-mock-keystore by @neonphog
- Use lair test keystore by @neonphog
- Merge branch 'develop' into lair-mock-keystore by @neonphog
- Mostly use test keystore by @neonphog
- First pass at validation package by @freesig
- Merge pull request #323 from Holo-Host/2020-09-03-cap-claim-crud by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain into 2020-09-03-cap-claim-crud by @thedavidmeister
- Merge pull request #337 from Holo-Host/2020-09-10-holonix-stable by @maackle
- Merge branch 'develop' of github.com:Holo-Host/holochain into 2020-09-10-holonix-stable by @thedavidmeister
- Merge pull request #335 from Holo-Host/query-chain by @maackle
- Actually hook up the query host fn by @maackle
- Finish multi filter test by @maackle
- Merge remote-tracking branch 'origin/develop' into query-chain by @maackle
- WIP test for multi filter by @maackle
- Test for EntryType filter, and much Fixturator refactoring to enable it by @maackle
- Create unimplemented SourceChain::query by @maackle
- Basic types and test wasm in place by @maackle
- Stable rust from holonix by @thedavidmeister
- Merge pull request #317 from Holo-Host/2020-08-30-cap-claim by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain into 2020-08-30-cap-claim by @thedavidmeister
- Merge pull request #294 from Holo-Host/sys_val by @freesig
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into sys_val by @freesig
- Merge pull request #309 from Holo-Host/sys_val_reader by @freesig
- Merge branch 'remove_async' of https://github.com/Holo-Host/holochain into sys_val_reader by @freesig
- Merge branch 'remove_async' of https://github.com/Holo-Host/holochain into sys_val_reader by @freesig
- Merge branch 'reading-rainbow-nolock' of https://github.com/Holo-Host/holochain into sys_val_reader by @freesig
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into sys_val by @freesig
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into sys_val by @freesig
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into sys_val by @freesig
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into sys_val by @freesig
- Sys validation unit tests by @freesig
- Grant crud macros by @thedavidmeister
- Lint by @thedavidmeister
- Merge branch 'from_agent' of github.com:Holo-Host/holochain into 2020-08-30-cap-claim by @thedavidmeister
- Merge pull request #314 from Holo-Host/entry-size-limit by @maackle
- Merge remote-tracking branch 'origin/develop' into entry-size-limit by @maackle
- Enforce Entry size limit via SerializedBytes newtype by @maackle
- Remove simple init by @thedavidmeister
- Merge branch 'develop' into 2020-08-30-cap-claim by @thedavidmeister
- Merge pull request #313 from Holo-Host/2020-08-28-cap-grant by @thedavidmeister
- Remove default for cap access by @thedavidmeister
- Checking cap claims against grants by @thedavidmeister
- Wip cap claims by @thedavidmeister
- Merge branch 'develop' into 2020-08-28-cap-grant by @thedavidmeister
- Merge pull request #295 from Holo-Host/reading-rainbow by @maackle
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into reading-rainbow by @freesig
- Update to serialized_bytes 0.0.43 by @maackle
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into reading-rainbow by @freesig
- More massive refactor by @maackle
- Working cap grant commits by @thedavidmeister
- Can commit cap grants by @thedavidmeister
- Merge pull request #312 from Holo-Host/2020-08-26-cap-commit by @thedavidmeister
- Remove nanoid by @thedavidmeister
- Lint by @thedavidmeister
- Lint by @thedavidmeister
- Cap secret generation by @thedavidmeister
- Merge pull request #300 from Holo-Host/2020-08-23-hdk-sweep by @thedavidmeister
- Merge pull request #303 from Holo-Host/2020-08-24-hdk-sweep by @thedavidmeister
- Rename EntryDefInt to EntryDefIndex by @thedavidmeister
- Merge pull request #302 from Holo-Host/2020-08-24-hdk-sweep by @thedavidmeister
- Tweak prelude for hdk by @thedavidmeister
- Merge branch 'develop' into 2020-08-23-hdk-sweep by @thedavidmeister
- Merge pull request #298 from Holo-Host/2020-08-20-hdk-sweep by @thedavidmeister
- Sys_time and zome_info macros by @thedavidmeister
- Macro for random bytes by @thedavidmeister
- Get details tests by @thedavidmeister
- Add crud wasm by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain into 2020-08-20-hdk-sweep by @thedavidmeister
- Merge pull request #296 from Holo-Host/holohash-primitive-entry-hashes by @maackle
- Make Entry a primitive hash type by @maackle
- Wip on hdk_entry macro by @thedavidmeister
- Merge pull request #293 from Holo-Host/2020-08-14-hdk-review by @thedavidmeister
- Lint by @thedavidmeister
- Lint by @thedavidmeister
- Lint by @thedavidmeister
- Hdk wip by @thedavidmeister
- Wip on hdk review by @thedavidmeister
- Split all the hdk fns out to match ribosome by @thedavidmeister
- Merge pull request #284 from Holo-Host/docs_for_hdk by @freesig
- Api call and built tutorial by @freesig
- Add docs for hdk api by @freesig
- Merge pull request #271 from Holo-Host/get_details by @freesig
- Merge branch 'get_details' of https://github.com/Holo-Host/holochain into get_details by @freesig
- Merge branch 'develop' into get_details by @thedavidmeister
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into get_details by @freesig
- Merge pull request #272 from Holo-Host/2020-08-03-validate-links by @thedavidmeister
- Merge branch 'develop' into 2020-08-03-validate-links by @thedavidmeister
- Merge branch 'develop' into 2020-08-03-validate-links by @neonphog
- Lint by @thedavidmeister
- Merge branch '2020-08-03-validate-links' of github.com:Holo-Host/holochain into 2020-08-03-validate-links by @thedavidmeister
- Merge branch 'develop' into 2020-08-03-validate-links by @thedavidmeister
- Merge branch 'develop' into 2020-08-03-validate-links by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain into 2020-08-03-validate-links by @thedavidmeister
- Working tests for validate link add by @thedavidmeister
- Merge branch '2020-08-05-entry-defs-macro' of github.com:Holo-Host/holochain into 2020-08-03-validate-links by @thedavidmeister
- Merge branch 'develop' into 2020-08-03-validate-links by @thedavidmeister
- Merge branch 'develop' into 2020-08-03-validate-links by @thedavidmeister
- Wip on validate links callback by @thedavidmeister
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into get_details by @freesig
- Merge pull request #277 from Holo-Host/2020-08-05-get-link-details by @freesig
- Lint by @thedavidmeister
- Update crates/hdk/src/api.rs by @thedavidmeister
- Merge branch 'develop' into 2020-08-05-get-link-details by @neonphog
- Merge pull request #276 from Holo-Host/2020-08-05-crate-prefix-hdk by @thedavidmeister
- Merge by @thedavidmeister
- Move deps into hdk prelude for wasms by @thedavidmeister
- Merge by @thedavidmeister
- Merge pull request #275 from Holo-Host/hashtype-serde-weirdness by @thedavidmeister
- Bump crates by @thedavidmeister
- Update to new published dep versions by @maackle
- Wip on get link details by @thedavidmeister
- Merge pull request #274 from Holo-Host/2020-08-05-entry-defs-macro by @thedavidmeister
- Merge branch '2020-08-05-entry-defs-macro' of github.com:Holo-Host/holochain into 2020-08-05-entry-defs-macro by @thedavidmeister
- Merge branch 'develop' into 2020-08-05-entry-defs-macro by @thedavidmeister
- Entry defs macro by @thedavidmeister
- Entry defs macro by @thedavidmeister
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into get_details by @freesig
- Merge pull request #267 from Holo-Host/get_links_details by @maackle
- Merge branch 'develop' into get_links_details by @freesig
- Add get_links detail by @freesig
- Change authority to optionaly return metadata by @freesig
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into get_details by @freesig
- Merge pull request #270 from Holo-Host/update_headers by @freesig
- Seperate entry update header by @freesig
- Get details by @freesig
- Merge pull request #263 from Holo-Host/update_hostfn by @freesig
- Merge branch 'develop' of github.com:Holo-Host/holochain into update_hostfn by @thedavidmeister
- Merge pull request #236 from Holo-Host/cascade2 by @freesig
- Bump wasmer crate by @thedavidmeister
- Wip update by @thedavidmeister
- Change host fn names by @freesig
- Merge branch 'develop' into cascade2 by @freesig
- Merge pull request #259 from Holo-Host/refactor_integrate by @freesig
- Merge pull request #261 from Holo-Host/cascade2_rf by @freesig
- Avoid cloning on op lights by @freesig
- Working by @freesig
- Merge branch 'refactor_integrate' of https://github.com/Holo-Host/holochain into cascade2_rf by @freesig
- Merge branch 'develop' into refactor_integrate by @freesig
- Refactor integration by @freesig
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into cascade2 by @freesig
- Merge pull request #257 from Holo-Host/basis_in_op by @thedavidmeister
- Move basis to dht op by @freesig
- Merge pull request #250 from Holo-Host/2020-07-24-remove-links by @thedavidmeister
- Merge branch 'develop' into 2020-07-24-remove-links by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain into 2020-07-24-remove-links by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain into 2020-07-24-remove-links by @thedavidmeister
- Wip on remove link by @thedavidmeister
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into cascade2 by @freesig
- Merge pull request #246 from Holo-Host/move_element by @freesig
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into move_element by @freesig
- Moved elements by @freesig
- Partial by @freesig
- Added links and refactor of get by @freesig
- Working dht_get by @freesig
- Merge pull request #248 from Holo-Host/housekeeping by @maackle
- HeaderAddress -> HeaderHash by @maackle
- Merge pull request #241 from Holo-Host/holohash-refactor by @maackle
- Merge remote-tracking branch 'origin/develop' into holohash-refactor by @maackle
- Merge pull request #240 from Holo-Host/get_meta by @freesig
- Merge branch 'develop' into get_meta by @freesig
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into get_meta by @freesig
- TimeHeaderHash -> TimedHeaderHash by @maackle
- Add get meta and add entry status by @freesig
- Merge branch 'develop' into holohash-refactor by @freesig
- Merge pull request #243 from Holo-Host/2020-07-21-call-remote-outgoing by @thedavidmeister
- Merge branch 'develop' into 2020-07-21-call-remote-outgoing by @thedavidmeister
- Wip on call remote from wasm by @thedavidmeister
- Rename remote call to call remote by @thedavidmeister
- Hook up remote call to ribosome by @thedavidmeister
- Merge remote-tracking branch 'origin/develop' into holohash-refactor by @maackle
- Merge pull request #245 from Holo-Host/move_headers by @thedavidmeister
- All headers moved by @freesig
- Partial by @freesig
- Holo_hash -> holo_hash_ext; holo_hash_core -> holo_hash by @maackle
- Merge pull request #238 from Holo-Host/holohash-refactor by @thedavidmeister
- Lint by @thedavidmeister
- Merge remote-tracking branch 'origin/develop' into holohash-refactor by @maackle
- Merge pull request #235 from Holo-Host/ribosome_network by @freesig
- Wip on host fn access by @thedavidmeister
- Fix warnings by @maackle
- HashableContent can be prehashed by @maackle
- Simplified HashableContent bound by @maackle
- Need to special-case HashableContent for Agent within Entry by @maackle
- WIP by @maackle
- Need to refactor SignedHeaderHashed now, so it fits into the CasBuf by @maackle
- Test wasms clean by @maackle
- Holochain_types is clean by @maackle
- Uh oh! Orphan rule! by @maackle
- Merge pull request #232 from Holo-Host/2020-07-09-link-tag-paths by @thedavidmeister
- Merge branch '2020-07-09-link-tag-paths' of github.com:Holo-Host/holochain into 2020-07-09-link-tag-paths by @thedavidmeister
- Merge branch 'develop' into 2020-07-09-link-tag-paths by @thedavidmeister
- Merge pull request #230 from Holo-Host/entry_def_buffer by @freesig
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into entry_def_buffer by @freesig
- Bump wasmer by @freesig
- Return zome name by @freesig
- Added buffer and split globals by @freesig
- Lint by @thedavidmeister
- Lint by @thedavidmeister
- Lint by @thedavidmeister
- Merge branch '2020-07-09-link-tag-paths' of github.com:Holo-Host/holochain into 2020-07-09-link-tag-paths by @thedavidmeister
- Merge branch 'develop' into 2020-07-09-link-tag-paths by @thedavidmeister
- Merge pull request #227 from Holo-Host/anchors_freesig by @thedavidmeister
- Serialize paths straight into their link tags by @thedavidmeister
- Wip on tag paths by @thedavidmeister
- Lint by @thedavidmeister
- Anchor and path tests at ribosome level by @thedavidmeister
- Remove path API wrapper from anchor by @thedavidmeister
- List_anchor_tags building by @thedavidmeister
- List_anchor_addresses building by @thedavidmeister
- Anchor and get_anchor in anchor wasm by @thedavidmeister
- Merge remote-tracking branch 'origin/develop' into demoable by @maackle
- Merge pull request #225 from Holo-Host/rm-2020 by @neonphog
- Rm 2020 by @neonphog
- Merge branch 'develop' of https://github.com/Holo-Host/holochain into 2020-06-22-anchor by @freesig
- Merge pull request #216 from Holo-Host/integrate_host_fn by @freesig
- String dsl for sharding text anchors by @thedavidmeister
- Binary based sharding by @thedavidmeister
- Wip on sharding by @thedavidmeister
- Anchors by @thedavidmeister
- Wip on anchors by @thedavidmeister
- Wip on path anchors by @thedavidmeister
- Merge branch 'integrate_host_fn' of github.com:Holo-Host/holochain-2020 into 2020-06-22-anchor by @thedavidmeister
- Merge pull request #218 from Holo-Host/pr/freesig/216 by @freesig
- Clean up by @freesig
- Passing test by @freesig
- Impl get links by @freesig
- Impl get links by @freesig
- Impl host funcs by @freesig
- Lint by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain-2020 into 2020-06-22-anchor by @thedavidmeister
- Merge pull request #203 from Holo-Host/2020-06-23-commit-entry by @thedavidmeister
- Commit entry in ribosome by @thedavidmeister
- Merge pull request #201 from Holo-Host/2020-06-23-get-entry-io by @thedavidmeister
- Lint docs by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain-2020 into 2020-06-23-get-entry-io by @thedavidmeister
- Merge pull request #196 from Holo-Host/2020-06-19-random-bytes by @thedavidmeister
- Merge branch 'develop' into 2020-06-19-random-bytes by @thedavidmeister
- Alias bytebuf for random bytes output by @thedavidmeister
- Merge branch '2020-06-19-random-bytes' of github.com:Holo-Host/holochain-2020 into 2020-06-19-random-bytes by @thedavidmeister
- Merge branch 'develop' into 2020-06-19-random-bytes by @thedavidmeister
- Wip on random bytes by @thedavidmeister
- Wip on get entry ribosome io by @thedavidmeister
- Merge pull request #199 from Holo-Host/2020-06-22-entry-hash by @thedavidmeister
- Entry hash ribosome implementation by @thedavidmeister
- Wip on anchor by @thedavidmeister
- Wip anchor by @thedavidmeister
- Merge pull request #177 from Holo-Host/2020-06-04-call-remote-db by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain-2020 into 2020-06-04-call-remote-db by @thedavidmeister
- Merge pull request #189 from Holo-Host/2020-06-12-wasm-perf by @thedavidmeister
- Merge branch 'develop' into 2020-06-12-wasm-perf by @thedavidmeister
- Wasm security fixes by @thedavidmeister
- Wip on wasm perf by @thedavidmeister
- Wip on removing send by @thedavidmeister
- Merge branch '2020-06-04-call-remote-db' of github.com:Holo-Host/holochain-2020 into 2020-06-04-call-remote-db by @thedavidmeister
- Merge branch 'develop' into 2020-06-04-call-remote-db by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain-2020 into 2020-06-04-call-remote-db by @thedavidmeister
- Merge pull request #186 from Holo-Host/docs by @maackle
- Remove TODO by @maackle
- Merge remote-tracking branch 'origin/develop' into docs by @maackle
- Merge pull request #188 from Holo-Host/readmes by @timotree3
- Holochain lib readme plus cargo toml fixes by @zippy
- Merge branch 'develop' into readmes by @zippy
- Merge pull request #170 from Holo-Host/2020-06-03-entry-defs by @thedavidmeister
- Constructor for EntryDef by @thedavidmeister
- Merge branch 'develop' into 2020-06-03-entry-defs by @zippy
- Entry defs callback tests by @thedavidmeister
- Wip on entry defs callback by @thedavidmeister
- Move readme docs into crate lib.rs file and regenrate from `cargo readme` by @zippy
- Adds some hand-crafted READMEs by @maackle
- Some more zome_types docs by @maackle
- Add call_remote types to holochain_p2p actor and event api by @neonphog
- Merge pull request #164 from Holo-Host/2020-05-27-hdk-mvp by @thedavidmeister
- Update crates/hdk/README.md by @thedavidmeister
- Merge branch 'develop' into 2020-05-27-hdk-mvp by @thedavidmeister
- Merge pull request #156 from Holo-Host/capability-lookup by @maackle
- Comments, cleanup by @maackle
- Tests pass by @maackle
- Add failing tests by @maackle
- Update hdk docs by @thedavidmeister
- Hdk docs wip by @thedavidmeister
- Merge pull request #158 from Holo-Host/sorted-deps by @neonphog
- Merge branch 'develop' into sorted-deps by @neonphog
- Merge pull request #153 from Holo-Host/2020-05-21-ribosome-tests by @thedavidmeister
- Merge branch 'develop' into 2020-05-21-ribosome-tests by @maackle
- Validation package callback tests by @thedavidmeister
- Post commit callback test by @thedavidmeister
- Migrate agent callback tests by @thedavidmeister
- Init callback tests by @thedavidmeister
- Sort Cargo.toml deps using cargo-sort-ck by @neonphog
- Merge pull request #150 from Holo-Host/address-refactor by @maackle
- Re-insitute the renamings by @maackle
- Merge remote-tracking branch 'origin/develop' into address-refactor by @maackle
- Merge pull request #108 from Holo-Host/2020-04-30-wasm-callbacks by @thedavidmeister
- ZomeInvocation -> ZomeCallInvocation by @maackle
- Merge remote-tracking branch 'origin/develop' into 2020-04-30-wasm-callbacks by @maackle
- Wip on moving cap tokens into zome types by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain-2020 into 2020-04-30-wasm-callbacks by @thedavidmeister
- Move workspace off invocation and cleanup by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain-2020 into 2020-04-30-wasm-callbacks by @thedavidmeister
- Return callbacks early if we have a definitive result by @thedavidmeister
- Noop disallowed side effects by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain-2020 into 2020-04-30-wasm-callbacks by @thedavidmeister
- Migrate agent callback tests by @thedavidmeister
- It compiles by @thedavidmeister
- Callback refactor by @thedavidmeister
- Wip on callback refactor by @thedavidmeister
- Working validation round trips from commit by @thedavidmeister
- Wip on testing callbacks by @thedavidmeister
- Fmt by @thedavidmeister
- Wip on callbacks cleanup by @thedavidmeister
- Wip on breaking callback types out of shims by @thedavidmeister
- Wip on validaton callback by @thedavidmeister
- EntryAddress -> EntryHash by @maackle
- Merge pull request #125 from Holo-Host/2020-05-12-bump-wasmer by @neonphog
- Bump wasmer by @thedavidmeister
- Merge pull request #93 from Holo-Host/sx-rename by @zippy
- Merge branch 'develop' into sx-rename by @zippy
- Merge pull request #86 from Holo-Host/no-legacy-2 by @maackle
- Rename sx_ crates to holochain_ by @zippy
- Merge remote-tracking branch 'origin/develop' into no-legacy-2 by @maackle
- Merge pull request #56 from Holo-Host/TK-01224 by @maackle
- Bump serialized-bytes dep by @maackle
- WIP by @maackle
- Merge pull request #58 from Holo-Host/2020-04-08-wasm-imports by @thedavidmeister
- Update definition of `wasm_io_types!` to follow API guidelines by @timotree3
- Hash: Replace `write!` with `.fmt(...)` call by @timotree3
- Add TODOs for future work by @timotree3
- Private-ify `mod zome_io` because it's all re-exported by @timotree3
- Debug: Take `String` as argument to maximize caller flexibility by @timotree3
- Update crates/zome_types/src/zome_io.rs by @thedavidmeister
- Remove dbg from debug_msg in zome types by @thedavidmeister
- Remove sleep and schedule for ribosome by @thedavidmeister
- Lint by @thedavidmeister
- Debug works with line numbers by @thedavidmeister
- Merge branch 'develop' of github.com:Holo-Host/holochain-2020 into 2020-04-08-wasm-imports by @thedavidmeister
- Merge pull request #48 from Holo-Host/api-interface by @zippy
- Merge branch 'develop' into api-interface by @neonphog
- Remove hash tests by @thedavidmeister
- Imports generally seem to be working in a basic way by @thedavidmeister
- Wip on wasm imports by @thedavidmeister
- Fmt by @thedavidmeister
- Import debug function working in ribosome by @thedavidmeister
- Passing tests by @thedavidmeister
- Initial commit by @holochain-release-automation2

### First-time Contributors

- @ made their first contribution in [#1](https://github.com/holochain/holochain-hdk/pull/1)
- @veeso made their first contribution in [#3](https://github.com/holochain/holochain-hdk/pull/3)
- @holochain-release-automation2 made their first contribution
- @ThetaSinner made their first contribution
- @cdunster made their first contribution
- @jost-s made their first contribution
- @mattyg made their first contribution
- @pdaoust made their first contribution
- @ddd-mtl made their first contribution
- @matthme made their first contribution
- @bytetigers made their first contribution
- @c12i made their first contribution
- @nphias made their first contribution
- @maackle made their first contribution
- @neonphog made their first contribution
- @steveej made their first contribution
- @joshuavial made their first contribution
- @thedavidmeister made their first contribution
- @zippy made their first contribution
- @guillemcordoba made their first contribution
- @Seb33300 made their first contribution
- @github-actions[bot] made their first contribution
- @zo-el made their first contribution
- @abe-njama made their first contribution
- @freesig made their first contribution
- @harlantwood made their first contribution
- @pospi made their first contribution
- @pjkundert made their first contribution
- @Connoropolous made their first contribution
- @timotree3 made their first contribution

Changes made before these crates moved out of
[holochain/holochain](https://github.com/holochain/holochain) are listed in
that repository's `CHANGELOG.md` up to the split.
