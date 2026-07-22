# mumu_cli

Async Rust wrapper around the `MuMuManager.exe` CLI (`mumu` on PATH). Controls MuMu Player emulator slots without shelling out manually. This file is the wrapper's own Rust API — for the underlying `MuMuManager.exe` CLI itself (raw subcommands/args this wrapper shells out to), see `mumu_cli_docs.md`.

## Construction

```rust
let cli = MumuCli::from_path();          // uses "mumu" from PATH
let cli = MumuCli::new("C:/path/to/MuMuManager.exe");
```

## Slot info

```rust
let s0 = SlotIndex::new(0); // typed slot index — the API takes no bare u32
let all: BTreeMap<SlotIndex, PlayerInfo> = cli.info_all().await?;
let slot: PlayerInfo = cli.info_one(s0).await?;
let some = cli.info(VmIndex::Many(vec![s0, SlotIndex::new(1)])).await?; // generic: any VmIndex → BTreeMap
let raw: String = cli.info_raw(s0).await?;  // unparsed, diagnostics only
let ver: String = cli.version().await?;    // MuMu player version string
slot.is_running()       // VM process alive
slot.is_android_ready() // Android has booted
```

Vital fact (verified on real hardware): MuMu's `info` output changes *shape*
with the result count — an index-keyed map for multiple instances, but one
bare object when the result is exactly one instance (including
`--vmindex all` on a PC with a single instance). `parse_info_output` accepts
both; anything else parsing MuMu `info` JSON must too.

## Control

```rust
cli.launch(s0).await?;
cli.shutdown(s0).await?;
cli.restart(s0).await?;
cli.show_window(s0).await?;
cli.hide_window(s0).await?;
cli.control(VmIndex::Many(slots), ControlAction::Launch).await?;
```

## Settings

Typed API — use `Setting` variants, pass a slice to `setting_apply`:

```rust
cli.setting_apply(s0, &[
    Setting::MaxFrameRate(15),
    Setting::RendererMode(RendererMode::Vulkan),
    Setting::PerformanceMode(PerformanceMode::Custom),
    Setting::PerformanceCpuCustom(2),
    Setting::PerformanceMemCustom(2.0),
    Setting::SystemDiskReadonly(true),
    Setting::ResolutionMode(ResolutionMode::Tablet),
]).await?;
```

Raw string pairs (escape hatch):

```rust
cli.setting_set(s0, &[("max_frame_rate", "15")]).await?;
```

From a JSON file (same format as `--all_writable` output):

```rust
cli.setting_from_file(s0, Path::new("my_settings.json")).await?;
```

Inspect current values:

```rust
let json: String = cli.setting_all_writable(s0).await?;
```

### Setting enum variants

| Variant | Wire key | Type | Notes |
|---|---|---|---|
| `PlayerName` | `player_name` | `String` | |
| `RendererMode` | `renderer_mode` | `RendererMode` | `Vulkan` / `DirectX` |
| `RendererStrategy` | `renderer_strategy` | `RendererStrategy` | `Auto` / `Perf` / `Dis` |
| `ForceDiscreteGraphics` | `force_discrete_graphics` | `bool` | |
| `ScreenBrightness` | `screen_brightness` | `u8` | 0–100 |
| `MaxFrameRate` | `max_frame_rate` | `u32` | fps cap |
| `DynamicAdjustFrameRate` | `dynamic_adjust_frame_rate` | `bool` | |
| `DynamicLowFrameRateLimit` | `dynamic_low_frame_rate_limit` | `u32` | valid: 5,10,…,60 |
| `ShowFrameRate` | `show_frame_rate` | `bool` | |
| `VerticalSync` | `vertical_sync` | `bool` | |
| `WindowAutoRotate` | `window_auto_rotate` | `bool` | |
| `WindowSaveRect` | `window_save_rect` | `bool` | |
| `WindowSizeFixed` | `window_size_fixed` | `bool` | |
| `SystemDiskReadonly` | `system_disk_readonly` | `bool` | |
| `SystemVolumeClose` | `system_volume_close` | `bool` | |
| `AppKeptlive` | `app_keptlive` | `bool` | |
| `RootPermission` | `root_permission` | `bool` | |
| `MiniDisk` | `mini_disk` | `bool` | |
| `MouseStyle` | `mouse_style` | `bool` | cursor visibility |
| `JoystickAutoConnect` | `joystick_auto_connect` | `bool` | |
| `ApkAssociation` | `apk_asscciation` | `bool` | wire key has MuMu typo |
| `QuitConfirm` | `quit_confirm` | `bool` | |
| `PhoneBrand` | `phone_brand` | `String` | e.g. `"Samsung"` |
| `PhoneModel` | `phone_model` | `String` | e.g. `"Galaxy Z Flip4"` |
| `PhoneMiit` | `phone_miit` | `String` | model ID e.g. `"SM-F721N"` |
| `PhoneImei` | `phone_imei` | `String` | |
| `PhoneNumber` | `phone_number` | `String` | |
| `NetBridgeOpen` | `net_bridge_open` | `bool` | |
| `NetBridgeIpMode` | `net_bridge_ip_mode` | `NetBridgeIpMode` | `Dhcp` / `Static` |
| `NetBridgeIpAddr` | `net_bridge_ip_addr` | `String` | |
| `NetBridgeGateway` | `net_bridge_gateway` | `String` | |
| `NetBridgeSubnetMask` | `net_bridge_subnet_mask` | `String` | |
| `NetBridgeDns1` | `net_bridge_dns1` | `String` | |
| `NetBridgeDns2` | `net_bridge_dns2` | `String` | |
| `NetBridgeCard` | `net_bridge_card` | `String` | |
| `PerformanceMode` | `performance_mode` | `PerformanceMode` | `Low`/`Middle`/`High`/`Custom` |
| `PerformanceCpuCustom` | `performance_cpu.custom` | `u32` | valid: 1,2,3,4 |
| `PerformanceMemCustom` | `performance_mem.custom` | `f64` | GB; valid: 0.75,1.0,…,16.0 |
| `GpuMode` | `gpu_mode` | `GpuMode` | `Low`=530 / `Middle`=640 / `High`=740 / `Custom` |
| `GpuModelCustom` | `gpu_model.custom` | `String` | e.g. `"Adreno (TM) 640"` |
| `ResolutionMode` | `resolution_mode` | `ResolutionMode` | `Tablet`/`Phone`/`Widescreen`/`Custom` |
| `ResolutionWidthCustom` | `resolution_width.custom` | `f64` | px, range 380–4096 |
| `ResolutionHeightCustom` | `resolution_height.custom` | `f64` | px, range 380–4096 |
| `ResolutionDpiCustom` | `resolution_dpi.custom` | `f64` | range 10–960 |

## Other operations

```rust
// Create / clone / delete / rename
cli.create(2, false).await?;              // counts stay u32
cli.clone_player(s0, 3).await?;
cli.delete(SlotIndex::new(5)).await?;
cli.rename(s0, "MySlot").await?;

// Shell / ADB
cli.sh(s0, "getprop ro.build.version.release").await?;
cli.adb(s0, "connect").await?;
cli.write_file(s0, "/sdcard/script.lua", bytes).await?;
// `adb --cmd` cannot take quoted compound commands (tokenization breaks, exit 127).
// NemuShell `sh` accepts pipes/redirects but swallows their stdout (verify via a follow-up read).

// Install an APK / pull a file or directory off the device — both shell
// out directly to the bundled adb.exe (not MuMuManager's own adb/sh
// subcommands), since a multi-hundred-MB APK or a whole directory doesn't
// fit write_file's base64-through-shell pipeline. Both need find_adb() to
// have resolved (constructed with a full exe path). Both wait for the
// daemon to report the slot's endpoint as `device` first (up to 20s,
// redialing a stuck `offline` entry once) — a freshly-booted instance
// flaps `offline` for a few seconds.
cli.install_apk(s0, Path::new("app.apk")).await?;
cli.pull(s0, "/storage/emulated/0/Download/logs", Path::new("./cache")).await?;
// Vital fact (verified on real hardware): pulling a directory nests the
// source's basename under the destination — the call above lands at
// ./cache/logs/..., not ./cache/... directly. `pull` itself does no
// flattening; callers that want a flat destination handle it themselves.

// Spoof device identity (pass None to clear)
cli.simulate(s0, SimuKey::Imei, Some("123456789012345")).await?;
cli.simulate(s0, SimuKey::AndroidId, None).await?;

// Import / export .mumudata backups.
// import semantics: see mumu_cli_docs.md's `import` section. Diff
// info_all before/after for the new instance's index. The MuMu UI's
// restore-into-existing-slot flow has no CLI equivalent.
cli.import(Path::new("backup.mumudata")).await?;
cli.export(s0, Path::new("./backups"), Some("slot0"), true).await?;

// Tile all windows
cli.sort().await?;

// Find bundled adb.exe (only works when constructed with full exe path)
if let Some(adb) = cli.find_adb() { /* ... */ }
```

## SlotIndex / VmIndex

`SlotIndex` (= `Idx<Slot>`, a phantom-typed u32) is the only way slot numbers enter or leave the API — no bare `u32` indices. `SlotIndex::new(n)` / `.get()` at the edges; serializes as the bare number. Future index kinds get their own `Idx<Marker>` alias.

Methods accepting `impl Into<VmIndex>` take a `SlotIndex` or `Vec<SlotIndex>` directly:

```rust
cli.launch(SlotIndex::new(0)).await?;           // SlotIndex → VmIndex::One
cli.control(VmIndex::All, ControlAction::Shutdown).await?;
cli.control(slots, ControlAction::Restart).await?; // Vec<SlotIndex> → VmIndex::Many
```
