---
dv_VmIndex:
  - Many(vec![0,1,2]), ControlAction::Launch
  - One(5)
  - All, ControlAction::Shutdown
  - Many(vec![1,3]), ControlAction::Restart
dv_Setting:
  - MaxFrameRate(15),
  - PerformanceCpuCustom(2),
  - PerformanceMemCustom(2.0),
  - SystemDiskReadonly(true),
dv_RendererMode: Vulkan
dv_PerformanceMode: Custom
dv_ResolutionMode: Tablet
dv_0, Path: new("my_settings.json")
dv_0, SimuKey:
  - Imei, Some("123456789012345")
  - AndroidId, None
dv_Path: new("backup.mumudata")
dv_0u32, Path: new("./backups"), Some("slot0"), true
dv_0-path: new("my_settings.json")
dv_0-simukey:
  - Imei, Some("123456789012345")
  - AndroidId, None
dv_0u32-path: new("./backups"), Some("slot0"), true
---
# mumu_cli

Async Rust wrapper around the `MuMuManager.exe` CLI (`mumu` on PATH). Lets server-side code control MuMu Player emulator slots without shelling out manually.

## Construction

```rust
let cli = MumuCli::from_path();          // uses "mumu" from PATH
let cli = MumuCli::new("C:/path/to/MuMuManager.exe");
```

## Slot info

```rust
let all: BTreeMap<u32, PlayerInfo> = cli.info_all().await?;
let slot: PlayerInfo = cli.info_one(0).await?;
slot.is_running()       // VM process alive
slot.is_android_ready() // Android has booted
```

## Control

```rust
cli.launch(0).await?;
cli.shutdown(0).await?;
cli.restart(0).await?;
cli.show_window(0).await?;
cli.hide_window(0).await?;
cli.control(VmIndex::Many(vec![0,1,2]), ControlAction::Launch).await?;
```

## Settings

Typed API — use `Setting` variants, pass a slice to `setting_apply`:

```rust
cli.setting_apply(0, &[
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
cli.setting_set(0, &[("max_frame_rate", "15")]).await?;
```

From a JSON file (same format as `--all_writable` output):

```rust
cli.setting_from_file(0, Path::new("my_settings.json")).await?;
```

Inspect current values:

```rust
let json: String = cli.setting_all_writable(0).await?;
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
cli.create(2, false).await?;
cli.clone_player(0u32, 3).await?;
cli.delete(VmIndex::One(5)).await?;
cli.rename(0, "MySlot").await?;

// Shell / ADB
cli.sh(0, "getprop ro.build.version.release").await?;
cli.adb(0, "connect").await?;
cli.write_file(0, "/sdcard/script.lua", bytes).await?;

// Spoof device identity (pass None to clear)
cli.simulate(0, SimuKey::Imei, Some("123456789012345")).await?;
cli.simulate(0, SimuKey::AndroidId, None).await?;

// Import / export .mumudata backups.
// import always creates ONE NEW instance — MuMuManager has no
// import-into-slot concept. Diff info_all before/after for the new index.
cli.import(Path::new("backup.mumudata")).await?;
cli.export(0u32, Path::new("./backups"), Some("slot0"), true).await?;

// Tile all windows
cli.sort().await?;

// Find bundled adb.exe (only works when constructed with full exe path)
if let Some(adb) = cli.find_adb() { /* ... */ }
```

## VmIndex

Methods accepting `impl Into<VmIndex>` take a bare `u32` for a single slot:

```rust
cli.launch(0).await?;                           // u32 → VmIndex::One
cli.control(VmIndex::All, ControlAction::Shutdown).await?;
cli.control(VmIndex::Many(vec![1,3]), ControlAction::Restart).await?;
```
