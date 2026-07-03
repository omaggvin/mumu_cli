use serde::{Deserialize, Serialize};

// ── Setting enums ─────────────────────────────────────────────────────────────

/// Graphics rendering backend. Corresponds to `renderer_mode`.
#[derive(Debug, Clone, Copy)]
pub enum RendererMode {
    Vulkan,
    DirectX,
}

impl RendererMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Vulkan => "vk",
            Self::DirectX => "dx",
        }
    }
}

/// Rendering optimisation strategy. Corresponds to `renderer_strategy`.
#[derive(Debug, Clone, Copy)]
pub enum RendererStrategy {
    Auto,
    Perf,
    /// Power-saving / display-quality mode (`"dis"`).
    Dis,
}

impl RendererStrategy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Perf => "perf",
            Self::Dis => "dis",
        }
    }
}

/// CPU/RAM performance tier. Corresponds to `performance_mode`.
///
/// `Low`, `Middle`, and `High` use fixed hardware profiles.
/// `Custom` reads `performance_cpu.custom` and `performance_mem.custom`.
#[derive(Debug, Clone, Copy)]
pub enum PerformanceMode {
    Low,
    Middle,
    High,
    Custom,
}

impl PerformanceMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Middle => "middle",
            Self::High => "high",
            Self::Custom => "custom",
        }
    }
}

/// GPU emulation tier. Corresponds to `gpu_mode`.
///
/// `Low`, `Middle`, and `High` use fixed Adreno models; `Custom` reads `gpu_model.custom`.
#[derive(Debug, Clone, Copy)]
pub enum GpuMode {
    /// Fixed model: Adreno (TM) 530.
    Low,
    /// Fixed model: Adreno (TM) 640.
    Middle,
    /// Fixed model: Adreno (TM) 740.
    High,
    /// Uses the string set via [`Setting::GpuModelCustom`].
    Custom,
}

impl GpuMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Middle => "middle",
            Self::High => "high",
            Self::Custom => "custom",
        }
    }
}

/// Preset resolution category. Corresponds to `resolution_mode`.
///
/// `Custom` uses the values set via [`Setting::ResolutionWidthCustom`],
/// [`Setting::ResolutionHeightCustom`], and [`Setting::ResolutionDpiCustom`].
#[derive(Debug, Clone, Copy)]
pub enum ResolutionMode {
    Tablet,
    Phone,
    Widescreen,
    Custom,
}

impl ResolutionMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Tablet => "tablet",
            Self::Phone => "phone",
            Self::Widescreen => "widescreen",
            Self::Custom => "custom",
        }
    }
}

/// Network bridge IP assignment mode. Corresponds to `net_bridge_ip_mode`.
#[derive(Debug, Clone, Copy)]
pub enum NetBridgeIpMode {
    Dhcp,
    Static,
}

impl NetBridgeIpMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Dhcp => "dhcp",
            Self::Static => "static",
        }
    }
}

// ── Setting ───────────────────────────────────────────────────────────────────

/// A typed MuMu VM setting, covering every key exposed by `mumu setting --all_writable`.
///
/// Build a slice and pass it to [`crate::MumuCli::setting_apply`]:
///
/// ```ignore
/// cli.setting_apply(0, &[
///     Setting::MaxFrameRate(15),
///     Setting::RendererMode(RendererMode::Vulkan),
///     Setting::SystemDiskReadonly(true),
/// ]).await?;
/// ```
///
/// For raw string pairs use [`crate::MumuCli::setting_set`] instead.
#[derive(Debug, Clone)]
pub enum Setting {
    /// Display name shown in the MuMu player list.
    PlayerName(String),
    /// Graphics backend (`renderer_mode`).
    RendererMode(RendererMode),
    /// Rendering optimisation strategy (`renderer_strategy`).
    RendererStrategy(RendererStrategy),
    /// Force use of the discrete GPU (`force_discrete_graphics`).
    ForceDiscreteGraphics(bool),
    /// Screen brightness 0–100 (`screen_brightness`).
    ScreenBrightness(u8),
    /// Frame rate cap in fps (`max_frame_rate`).
    MaxFrameRate(u32),
    /// Automatically lower frame rate when the window is idle (`dynamic_adjust_frame_rate`).
    DynamicAdjustFrameRate(bool),
    /// Floor fps when dynamic adjustment is active (`dynamic_low_frame_rate_limit`).
    /// Known valid values: 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60.
    DynamicLowFrameRateLimit(u32),
    /// Show the fps counter overlay (`show_frame_rate`).
    ShowFrameRate(bool),
    /// Enable vertical sync (`vertical_sync`).
    VerticalSync(bool),
    /// Allow the window to auto-rotate with the Android orientation (`window_auto_rotate`).
    WindowAutoRotate(bool),
    /// Persist the window position and size between sessions (`window_save_rect`).
    WindowSaveRect(bool),
    /// Prevent the user from resizing the window (`window_size_fixed`).
    WindowSizeFixed(bool),
    /// Mount the Android system partition read-only (`system_disk_readonly`).
    SystemDiskReadonly(bool),
    /// Mute Android system audio (`system_volume_close`).
    SystemVolumeClose(bool),
    /// Keep the app alive when the player window is closed (`app_keptlive`).
    AppKeptlive(bool),
    /// Grant root permissions inside the VM (`root_permission`).
    RootPermission(bool),
    /// Use the mini-disk data partition (`mini_disk`).
    MiniDisk(bool),
    /// Show the mouse cursor inside the player window (`mouse_style`).
    MouseStyle(bool),
    /// Auto-connect joystick devices (`joystick_auto_connect`).
    JoystickAutoConnect(bool),
    /// Register MuMu as the default handler for APK files (`apk_asscciation`).
    /// Note: the wire key preserves MuMu's own typo (`apk_asscciation`).
    ApkAssociation(bool),
    /// Show a confirmation dialog when quitting (`quit_confirm`).
    QuitConfirm(bool),
    /// Spoofed phone manufacturer (`phone_brand`).
    PhoneBrand(String),
    /// Spoofed phone model name (`phone_model`).
    PhoneModel(String),
    /// Spoofed phone model ID, e.g. `"SM-F721N"` (`phone_miit`).
    PhoneMiit(String),
    /// Spoofed IMEI (`phone_imei`).
    PhoneImei(String),
    /// Spoofed phone number (`phone_number`).
    PhoneNumber(String),
    /// Enable the network bridge (`net_bridge_open`).
    NetBridgeOpen(bool),
    /// IP assignment mode for the network bridge (`net_bridge_ip_mode`).
    NetBridgeIpMode(NetBridgeIpMode),
    /// Static IP address for the bridge (`net_bridge_ip_addr`).
    NetBridgeIpAddr(String),
    /// Gateway for the bridge (`net_bridge_gateway`).
    NetBridgeGateway(String),
    /// Subnet mask for the bridge (`net_bridge_subnet_mask`).
    NetBridgeSubnetMask(String),
    /// Primary DNS for the bridge (`net_bridge_dns1`).
    NetBridgeDns1(String),
    /// Secondary DNS for the bridge (`net_bridge_dns2`).
    NetBridgeDns2(String),
    /// Network adapter to bridge (`net_bridge_card`).
    NetBridgeCard(String),
    /// CPU/RAM performance tier (`performance_mode`).
    PerformanceMode(PerformanceMode),
    /// CPU core count when `PerformanceMode::Custom` is active (`performance_cpu.custom`).
    /// Known valid values: 1, 2, 3, 4.
    PerformanceCpuCustom(u32),
    /// RAM in GB when `PerformanceMode::Custom` is active (`performance_mem.custom`).
    /// Known valid values: 0.75, 1.0, 1.5, 1.75, 2.0, 3.0, 4.0 … 16.0.
    PerformanceMemCustom(f64),
    /// GPU emulation tier (`gpu_mode`).
    GpuMode(GpuMode),
    /// GPU model string when `GpuMode::Custom` is active (`gpu_model.custom`).
    /// e.g. `"Adreno (TM) 640"`.
    GpuModelCustom(String),
    /// Preset resolution category (`resolution_mode`).
    ResolutionMode(ResolutionMode),
    /// Width in pixels when `ResolutionMode::Custom` is active (`resolution_width.custom`).
    ResolutionWidthCustom(f64),
    /// Height in pixels when `ResolutionMode::Custom` is active (`resolution_height.custom`).
    ResolutionHeightCustom(f64),
    /// DPI when `ResolutionMode::Custom` is active (`resolution_dpi.custom`). Range: 10–960.
    ResolutionDpiCustom(f64),
}

impl Setting {
    /// Returns the MuMu wire key for this setting.
    pub fn key(&self) -> &'static str {
        match self {
            Self::PlayerName(_) => "player_name",
            Self::RendererMode(_) => "renderer_mode",
            Self::RendererStrategy(_) => "renderer_strategy",
            Self::ForceDiscreteGraphics(_) => "force_discrete_graphics",
            Self::ScreenBrightness(_) => "screen_brightness",
            Self::MaxFrameRate(_) => "max_frame_rate",
            Self::DynamicAdjustFrameRate(_) => "dynamic_adjust_frame_rate",
            Self::DynamicLowFrameRateLimit(_) => "dynamic_low_frame_rate_limit",
            Self::ShowFrameRate(_) => "show_frame_rate",
            Self::VerticalSync(_) => "vertical_sync",
            Self::WindowAutoRotate(_) => "window_auto_rotate",
            Self::WindowSaveRect(_) => "window_save_rect",
            Self::WindowSizeFixed(_) => "window_size_fixed",
            Self::SystemDiskReadonly(_) => "system_disk_readonly",
            Self::SystemVolumeClose(_) => "system_volume_close",
            Self::AppKeptlive(_) => "app_keptlive",
            Self::RootPermission(_) => "root_permission",
            Self::MiniDisk(_) => "mini_disk",
            Self::MouseStyle(_) => "mouse_style",
            Self::JoystickAutoConnect(_) => "joystick_auto_connect",
            Self::ApkAssociation(_) => "apk_asscciation",
            Self::QuitConfirm(_) => "quit_confirm",
            Self::PhoneBrand(_) => "phone_brand",
            Self::PhoneModel(_) => "phone_model",
            Self::PhoneMiit(_) => "phone_miit",
            Self::PhoneImei(_) => "phone_imei",
            Self::PhoneNumber(_) => "phone_number",
            Self::NetBridgeOpen(_) => "net_bridge_open",
            Self::NetBridgeIpMode(_) => "net_bridge_ip_mode",
            Self::NetBridgeIpAddr(_) => "net_bridge_ip_addr",
            Self::NetBridgeGateway(_) => "net_bridge_gateway",
            Self::NetBridgeSubnetMask(_) => "net_bridge_subnet_mask",
            Self::NetBridgeDns1(_) => "net_bridge_dns1",
            Self::NetBridgeDns2(_) => "net_bridge_dns2",
            Self::NetBridgeCard(_) => "net_bridge_card",
            Self::PerformanceMode(_) => "performance_mode",
            Self::PerformanceCpuCustom(_) => "performance_cpu.custom",
            Self::PerformanceMemCustom(_) => "performance_mem.custom",
            Self::GpuMode(_) => "gpu_mode",
            Self::GpuModelCustom(_) => "gpu_model.custom",
            Self::ResolutionMode(_) => "resolution_mode",
            Self::ResolutionWidthCustom(_) => "resolution_width.custom",
            Self::ResolutionHeightCustom(_) => "resolution_height.custom",
            Self::ResolutionDpiCustom(_) => "resolution_dpi.custom",
        }
    }

    /// Returns the MuMu wire value for this setting.
    pub fn value(&self) -> String {
        match self {
            Self::PlayerName(s) => s.clone(),
            Self::RendererMode(v) => v.as_str().to_string(),
            Self::RendererStrategy(v) => v.as_str().to_string(),
            Self::ForceDiscreteGraphics(b) => bool_str(*b),
            Self::ScreenBrightness(n) => n.to_string(),
            Self::MaxFrameRate(n) => n.to_string(),
            Self::DynamicAdjustFrameRate(b) => bool_str(*b),
            Self::DynamicLowFrameRateLimit(n) => n.to_string(),
            Self::ShowFrameRate(b) => bool_str(*b),
            Self::VerticalSync(b) => bool_str(*b),
            Self::WindowAutoRotate(b) => bool_str(*b),
            Self::WindowSaveRect(b) => bool_str(*b),
            Self::WindowSizeFixed(b) => bool_str(*b),
            Self::SystemDiskReadonly(b) => bool_str(*b),
            Self::SystemVolumeClose(b) => bool_str(*b),
            Self::AppKeptlive(b) => bool_str(*b),
            Self::RootPermission(b) => bool_str(*b),
            Self::MiniDisk(b) => bool_str(*b),
            Self::MouseStyle(b) => bool_str(*b),
            Self::JoystickAutoConnect(b) => bool_str(*b),
            Self::ApkAssociation(b) => bool_str(*b),
            Self::QuitConfirm(b) => bool_str(*b),
            Self::PhoneBrand(s) => s.clone(),
            Self::PhoneModel(s) => s.clone(),
            Self::PhoneMiit(s) => s.clone(),
            Self::PhoneImei(s) => s.clone(),
            Self::PhoneNumber(s) => s.clone(),
            Self::NetBridgeOpen(b) => bool_str(*b),
            Self::NetBridgeIpMode(v) => v.as_str().to_string(),
            Self::NetBridgeIpAddr(s) => s.clone(),
            Self::NetBridgeGateway(s) => s.clone(),
            Self::NetBridgeSubnetMask(s) => s.clone(),
            Self::NetBridgeDns1(s) => s.clone(),
            Self::NetBridgeDns2(s) => s.clone(),
            Self::NetBridgeCard(s) => s.clone(),
            Self::PerformanceMode(v) => v.as_str().to_string(),
            Self::PerformanceCpuCustom(n) => n.to_string(),
            Self::PerformanceMemCustom(f) => format!("{f:.6}"),
            Self::GpuMode(v) => v.as_str().to_string(),
            Self::GpuModelCustom(s) => s.clone(),
            Self::ResolutionMode(v) => v.as_str().to_string(),
            Self::ResolutionWidthCustom(f) => format!("{f:.6}"),
            Self::ResolutionHeightCustom(f) => format!("{f:.6}"),
            Self::ResolutionDpiCustom(f) => format!("{f:.6}"),
        }
    }

    /// Returns `(key, owned_value)` — convenience for building CLI arg pairs.
    pub fn as_pair(&self) -> (&'static str, String) {
        (self.key(), self.value())
    }
}

fn bool_str(b: bool) -> String {
    if b { "true" } else { "false" }.to_string()
}

// ── VmIndex ───────────────────────────────────────────────────────────────────

/// Selects which VM slot(s) a command targets.
///
/// Most methods accept `impl Into<VmIndex>`, so you can pass a bare `u32` for a single slot.
#[derive(Debug, Clone)]
pub enum VmIndex {
    /// Every slot (`--vmindex all`).
    All,
    /// A single slot by index.
    One(u32),
    /// An explicit list of slots (`--vmindex 0,2,3`).
    Many(Vec<u32>),
}

impl VmIndex {
    pub(crate) fn to_arg(&self) -> String {
        match self {
            VmIndex::All => "all".to_string(),
            VmIndex::One(i) => i.to_string(),
            VmIndex::Many(v) => v.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","),
        }
    }
}

impl From<u32> for VmIndex {
    fn from(i: u32) -> Self { VmIndex::One(i) }
}

impl From<Vec<u32>> for VmIndex {
    fn from(v: Vec<u32>) -> Self { VmIndex::Many(v) }
}

// ── ControlAction ─────────────────────────────────────────────────────────────

/// Action passed to `mumu control`. Used with [`crate::MumuCli::control`] and the
/// convenience wrappers [`crate::MumuCli::launch`], [`crate::MumuCli::shutdown`], etc.
#[derive(Debug, Clone, Copy)]
pub enum ControlAction {
    Launch,
    Shutdown,
    Restart,
    ShowWindow,
    HideWindow,
}

impl ControlAction {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Launch => "launch",
            Self::Shutdown => "shutdown",
            Self::Restart => "restart",
            Self::ShowWindow => "show_window",
            Self::HideWindow => "hide_window",
        }
    }
}

// ── SimuKey ───────────────────────────────────────────────────────────────────

/// Simulated device property to spoof. Used with [`crate::MumuCli::simulate`].
///
/// Pass `None` as the value to clear back to the generated default.
#[derive(Debug, Clone, Copy)]
pub enum SimuKey {
    AndroidId,
    MacAddress,
    Imei,
}

impl SimuKey {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::AndroidId => "android_id",
            Self::MacAddress => "mac_address",
            Self::Imei => "imei",
        }
    }
}

// ── PlayerInfo ────────────────────────────────────────────────────────────────

/// Info returned by `mumu info` for a single VM slot.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerInfo {
    /// Slot index as a string (as returned by mumu).
    pub index: String,
    pub name: String,
    #[serde(default)]
    pub is_process_started: bool,
    #[serde(default)]
    pub is_android_started: bool,
    #[serde(default)]
    pub is_main: bool,
    #[serde(default)]
    pub hyperv_enabled: bool,
    pub vt_enabled: Option<bool>,
    pub pid: Option<u32>,
    pub headless_pid: Option<u32>,
    pub adb_host_ip: Option<String>,
    pub adb_port: Option<u32>,
    pub player_state: Option<String>,
    pub disk_size_bytes: Option<u64>,
    pub created_timestamp: Option<u64>,
    pub launch_time: Option<u64>,
    pub error_code: Option<i32>,
    pub launch_err_code: Option<i32>,
    pub launch_err_msg: Option<String>,
}

impl PlayerInfo {
    /// Parses `index` as a `u32`. Returns `0` if the string is malformed.
    pub fn slot_index(&self) -> u32 {
        self.index.parse().unwrap_or(0)
    }

    /// `true` when the MuMu process for this slot is running.
    pub fn is_running(&self) -> bool {
        self.is_process_started
    }

    /// `true` when Android has finished booting inside the VM.
    pub fn is_android_ready(&self) -> bool {
        self.is_android_started
    }
}
