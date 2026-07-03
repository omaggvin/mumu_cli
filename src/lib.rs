mod error;
mod types;

pub use error::{MumuError, Result};
pub use types::{
    ControlAction, GpuMode, NetBridgeIpMode, PerformanceMode, PlayerInfo, RendererMode,
    RendererStrategy, ResolutionMode, Setting, SimuKey, VmIndex,
};

use std::{collections::BTreeMap, path::{Path, PathBuf}};
use tokio::process::Command;

/// Async wrapper around the `MuMuManager.exe` CLI (`mumu` on PATH).
///
/// Construct with [`MumuCli::from_path`] when `mumu` is on `PATH`, or
/// [`MumuCli::new`] with the full path to the executable.
///
/// All methods are async and require a Tokio runtime.
#[derive(Clone)]
pub struct MumuCli {
    exe: PathBuf,
}

impl MumuCli {
    /// Create a client pointing at the given executable path.
    pub fn new(exe: impl Into<PathBuf>) -> Self {
        Self { exe: exe.into() }
    }

    /// Create a client using `mumu` from `PATH`.
    pub fn from_path() -> Self {
        Self::new("mumu")
    }

    async fn run(&self, args: &[&str]) -> Result<Vec<u8>> {
        let out = Command::new(&self.exe).args(args).output().await?;
        if !out.status.success() {
            return Err(MumuError::NonZeroExit {
                code: out.status.code(),
                stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            });
        }
        Ok(out.stdout)
    }

    async fn run_text(&self, args: &[&str]) -> Result<String> {
        Ok(String::from_utf8(self.run(args).await?)?)
    }

    // ── version ───────────────────────────────────────────────────────────────

    /// Returns the MuMu player version string.
    pub async fn version(&self) -> Result<String> {
        Ok(self.run_text(&["version"]).await?.trim().to_string())
    }

    // ── info ──────────────────────────────────────────────────────────────────

    /// Returns a map of slot index → [`PlayerInfo`] for the given `vmindex`.
    pub async fn info(&self, vmindex: impl Into<VmIndex>) -> Result<BTreeMap<u32, PlayerInfo>> {
        let idx = vmindex.into().to_arg();
        let raw = self.run_text(&["info", "--vmindex", &idx]).await?;
        let map: std::collections::HashMap<String, PlayerInfo> = serde_json::from_str(&raw)?;
        Ok(map
            .into_iter()
            .filter_map(|(k, v)| k.parse::<u32>().ok().map(|i| (i, v)))
            .collect())
    }

    /// Returns info for every slot.
    pub async fn info_all(&self) -> Result<BTreeMap<u32, PlayerInfo>> {
        self.info(VmIndex::All).await
    }

    /// Returns info for a single slot. Errors if the slot is not found in the response.
    pub async fn info_one(&self, index: u32) -> Result<PlayerInfo> {
        self.info(VmIndex::One(index))
            .await?
            .remove(&index)
            .ok_or_else(|| MumuError::NonZeroExit {
                code: None,
                stderr: format!("no info returned for slot {index}"),
            })
    }

    // ── control ───────────────────────────────────────────────────────────────

    /// Run a [`ControlAction`] on one or more slots.
    pub async fn control(&self, vmindex: impl Into<VmIndex>, action: ControlAction) -> Result<()> {
        let idx = vmindex.into().to_arg();
        self.run(&["control", "--vmindex", &idx, action.as_str()]).await?;
        Ok(())
    }

    /// Launch the VM for slot `index`.
    pub async fn launch(&self, index: u32) -> Result<()> {
        self.control(index, ControlAction::Launch).await
    }

    /// Shut down the VM for slot `index`.
    pub async fn shutdown(&self, index: u32) -> Result<()> {
        self.control(index, ControlAction::Shutdown).await
    }

    /// Restart the VM for slot `index`.
    pub async fn restart(&self, index: u32) -> Result<()> {
        self.control(index, ControlAction::Restart).await
    }

    /// Show the player window for slot `index`.
    pub async fn show_window(&self, index: u32) -> Result<()> {
        self.control(index, ControlAction::ShowWindow).await
    }

    /// Hide the player window for slot `index`.
    pub async fn hide_window(&self, index: u32) -> Result<()> {
        self.control(index, ControlAction::HideWindow).await
    }

    // ── create / clone / delete / rename ──────────────────────────────────────

    /// Create `number` new VM slots. Pass `mini = true` to use the mini-disk data partition.
    pub async fn create(&self, number: u32, mini: bool) -> Result<()> {
        let n = number.to_string();
        let mut args = vec!["create", "--number", &n];
        if mini {
            args.push("--mini");
        }
        self.run(&args).await?;
        Ok(())
    }

    /// Clone one or more slots, creating `number` copies.
    pub async fn clone_player(&self, vmindex: impl Into<VmIndex>, number: u32) -> Result<()> {
        let idx = vmindex.into().to_arg();
        let n = number.to_string();
        self.run(&["clone", "--vmindex", &idx, "--number", &n]).await?;
        Ok(())
    }

    /// Delete one or more slots.
    pub async fn delete(&self, vmindex: impl Into<VmIndex>) -> Result<()> {
        let idx = vmindex.into().to_arg();
        self.run(&["delete", "--vmindex", &idx]).await?;
        Ok(())
    }

    /// Rename slot `index` to `name`.
    pub async fn rename(&self, index: u32, name: &str) -> Result<()> {
        let i = index.to_string();
        self.run(&["rename", "--vmindex", &i, "--name", name]).await?;
        Ok(())
    }

    // ── sh / adb ──────────────────────────────────────────────────────────────

    /// Run a shell command inside slot `index` and return stdout.
    pub async fn sh(&self, index: u32, cmd: &str) -> Result<String> {
        let i = index.to_string();
        self.run_text(&["sh", "--vmindex", &i, "--cmd", cmd]).await
    }

    /// Run an ADB command against slot `index` and return stdout.
    pub async fn adb(&self, index: u32, cmd: &str) -> Result<String> {
        let i = index.to_string();
        self.run_text(&["adb", "--vmindex", &i, "--cmd", cmd]).await
    }

    /// Write arbitrary bytes to `remote_path` on the Android filesystem of slot `index`.
    ///
    /// Uses base64 encoding to avoid backslash-stripping in the `sh --cmd` pipeline.
    pub async fn write_file(&self, index: u32, remote_path: &str, content: &[u8]) -> Result<()> {
        use base64::prelude::*;
        let encoded = BASE64_STANDARD.encode(content);
        self.sh(index, &format!("echo {encoded} | base64 -d > {remote_path}")).await?;
        Ok(())
    }

    /// Find the `adb.exe` bundled with this MuMu installation.
    ///
    /// Walks up from the executable to the MuMu root and searches `nx_device/*/shell/adb.exe`.
    /// Only meaningful when [`MumuCli::new`] was used with a full path.
    pub fn find_adb(&self) -> Option<PathBuf> {
        let mumu_dir = self.exe.parent()?.parent()?;
        let nx_device = mumu_dir.join("nx_device");
        for entry in std::fs::read_dir(&nx_device).ok()?.flatten() {
            let candidate = entry.path().join("shell").join("adb.exe");
            if candidate.exists() {
                return Some(candidate);
            }
        }
        None
    }

    // ── simulation ────────────────────────────────────────────────────────────

    /// Spoof a simulated device property on one or more slots.
    ///
    /// Pass `value = None` to clear back to the generated default (`__null__`).
    pub async fn simulate(
        &self,
        vmindex: impl Into<VmIndex>,
        key: SimuKey,
        value: Option<&str>,
    ) -> Result<()> {
        let idx = vmindex.into().to_arg();
        let v = value.unwrap_or("__null__");
        self.run(&[
            "simulation",
            "--vmindex", &idx,
            "--simu_key", key.as_str(),
            "--simu_value", v,
        ])
        .await?;
        Ok(())
    }

    // ── sort ──────────────────────────────────────────────────────────────────

    /// Tile all player windows on screen.
    pub async fn sort(&self) -> Result<()> {
        self.run(&["sort"]).await?;
        Ok(())
    }

    // ── import / export ───────────────────────────────────────────────────────

    /// Import a `.mumudata` file as ONE new instance.
    ///
    /// MuMuManager's `import` has no target-slot concept — it always
    /// creates a new instance, and `--number` is a repeat count ("Number of
    /// action run"), NOT an index. Verified against the real CLI 2026-07-02
    /// the hard way: passing a slot index of 10 here imported ten copies.
    /// Callers needing the new instance's index must diff `info_all`
    /// before/after.
    pub async fn import(&self, path: &Path) -> Result<()> {
        let p = path.to_string_lossy();
        self.run(&["import", "--path", &p, "--number", "1"]).await?;
        Ok(())
    }

    /// Export one or more slots as `.mumudata` files into `dir`.
    ///
    /// `name` overrides the output filename (without extension).
    /// `zip` uses the compressed format.
    pub async fn export(
        &self,
        vmindex: impl Into<VmIndex>,
        dir: &Path,
        name: Option<&str>,
        zip: bool,
    ) -> Result<()> {
        let idx = vmindex.into().to_arg();
        let d = dir.to_string_lossy();
        let mut args = vec!["export", "--vmindex", &idx, "--dir", &d];
        if let Some(n) = name {
            args.extend_from_slice(&["--name", n]);
        }
        if zip {
            args.push("--zip");
        }
        self.run(&args).await?;
        Ok(())
    }

    // ── setting ───────────────────────────────────────────────────────────────

    /// Apply one or more raw key-value setting pairs to a slot.
    ///
    /// Prefer [`MumuCli::setting_apply`] for type-safe usage.
    pub async fn setting_set(
        &self,
        vmindex: impl Into<VmIndex>,
        pairs: &[(&str, &str)],
    ) -> Result<()> {
        let idx = vmindex.into().to_arg();
        let mut args = vec!["setting", "--vmindex", &idx];
        for (k, v) in pairs {
            args.extend_from_slice(&["--key", k, "--value", v]);
        }
        self.run(&args).await?;
        Ok(())
    }

    /// Apply settings from a UTF-8 JSON file (same format as `--all_writable` output).
    pub async fn setting_from_file(
        &self,
        vmindex: impl Into<VmIndex>,
        path: &Path,
    ) -> Result<()> {
        let idx = vmindex.into().to_arg();
        let p = path.to_string_lossy();
        self.run(&["setting", "--vmindex", &idx, "--path", &p]).await?;
        Ok(())
    }

    /// Return all writable setting keys and their current values as raw JSON.
    ///
    /// Useful for inspecting what a slot currently has before applying changes.
    pub async fn setting_all_writable(&self, vmindex: impl Into<VmIndex>) -> Result<String> {
        let idx = vmindex.into().to_arg();
        self.run_text(&["setting", "--vmindex", &idx, "--all_writable"]).await
    }

    /// Apply typed [`Setting`] values to a slot in a single invocation.
    pub async fn setting_apply(
        &self,
        vmindex: impl Into<VmIndex>,
        settings: &[Setting],
    ) -> Result<()> {
        let pairs: Vec<(&'static str, String)> = settings.iter().map(|s| s.as_pair()).collect();
        let str_pairs: Vec<(&str, &str)> =
            pairs.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.setting_set(vmindex, &str_pairs).await
    }
}
