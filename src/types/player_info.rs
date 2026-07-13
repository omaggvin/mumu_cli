use serde::{Deserialize, Serialize};

use super::idx::SlotIndex;

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
    /// Parses `index` as a [`SlotIndex`]. Returns slot `0` if the string is malformed.
    pub fn slot_index(&self) -> SlotIndex {
        self.index.parse().unwrap_or(SlotIndex::new(0))
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
