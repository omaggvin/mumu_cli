use serde::{Deserialize, Serialize};

// ── VmIndex ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum VmIndex {
    All,
    One(u32),
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
    pub fn slot_index(&self) -> u32 {
        self.index.parse().unwrap_or(0)
    }

    pub fn is_running(&self) -> bool {
        self.is_process_started
    }

    pub fn is_android_ready(&self) -> bool {
        self.is_android_started
    }
}
