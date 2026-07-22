---
tags: [context]
project: mumu_cli
---

# mumu_cli

Async Rust wrapper around the `MuMuManager.exe` CLI (`mumu` on PATH) — control MuMu Player emulator slots (create/clone/delete, launch/shutdown, settings, adb/sh, import/export, install/pull) without shelling out manually.

## Doc map

- `docs.md` — wrapper's own Rust API (this crate's public surface).
- `mumu_cli_docs.md` — the underlying `MuMuManager.exe` CLI itself (raw subcommands/args the wrapper shells out to).
- rustdoc (`cargo doc`) — inline on every public item, stands alone (docs.rs).

## Build / test

```sh
cargo build
cargo test
```

No external services required; tests cover pure parsing/chunking logic against captured real-hardware output, not a live MuMuManager.exe.

## Key MuMuManager quirks (verified on real hardware)

- **import**: no import-into-slot concept — always creates ONE new instance; `--number`/`-n` is a repeat count, not an index (passing 10 makes ten copies). Diff `info_all` before/after to find the new instance. Canonical statement: `mumu_cli_docs.md`'s `import` section.
- **info output shape-shifts** with result count; `parse_info_output` accepts both shapes. Canonical statement: `docs.md`'s "Slot info" section.
- **adb/sh subcommands vs bundled adb.exe**: `MumuManager adb`/`sh` go through a base64-through-shell pipeline (`write_file`) — fine for small payloads but can't stream a multi-hundred-MB APK or a whole directory. `install_apk`/`pull` instead shell directly to the MuMu-bundled `adb.exe` (found via `find_adb`) against the slot's own TCP adb endpoint, waiting for the daemon to report `device` (a fresh boot flaps `offline` for a few seconds).
- **write_file chunking**: Windows caps a command line near 32 KB; `sh --cmd` fails ("error 206") past it. Canonical statement: `write_file`'s rustdoc (`src/lib.rs`).
