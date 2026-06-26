mod error;
mod types;

pub use error::{MumuError, Result};
pub use types::{ControlAction, PlayerInfo, SimuKey, VmIndex};

use std::{collections::BTreeMap, path::{Path, PathBuf}};
use tokio::process::Command;

#[derive(Clone)]
pub struct MumuCli {
    exe: PathBuf,
}

impl MumuCli {
    pub fn new(exe: impl Into<PathBuf>) -> Self {
        Self { exe: exe.into() }
    }

    /// Use "mumu" from PATH.
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

    pub async fn version(&self) -> Result<String> {
        Ok(self.run_text(&["version"]).await?.trim().to_string())
    }

    // ── info ──────────────────────────────────────────────────────────────────

    pub async fn info(&self, vmindex: impl Into<VmIndex>) -> Result<BTreeMap<u32, PlayerInfo>> {
        let idx = vmindex.into().to_arg();
        let raw = self.run_text(&["info", "--vmindex", &idx]).await?;
        let map: std::collections::HashMap<String, PlayerInfo> = serde_json::from_str(&raw)?;
        Ok(map
            .into_iter()
            .filter_map(|(k, v)| k.parse::<u32>().ok().map(|i| (i, v)))
            .collect())
    }

    pub async fn info_all(&self) -> Result<BTreeMap<u32, PlayerInfo>> {
        self.info(VmIndex::All).await
    }

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

    pub async fn control(&self, vmindex: impl Into<VmIndex>, action: ControlAction) -> Result<()> {
        let idx = vmindex.into().to_arg();
        self.run(&["control", "--vmindex", &idx, action.as_str()]).await?;
        Ok(())
    }

    pub async fn launch(&self, index: u32) -> Result<()> {
        self.control(index, ControlAction::Launch).await
    }

    pub async fn shutdown(&self, index: u32) -> Result<()> {
        self.control(index, ControlAction::Shutdown).await
    }

    pub async fn restart(&self, index: u32) -> Result<()> {
        self.control(index, ControlAction::Restart).await
    }

    pub async fn show_window(&self, index: u32) -> Result<()> {
        self.control(index, ControlAction::ShowWindow).await
    }

    pub async fn hide_window(&self, index: u32) -> Result<()> {
        self.control(index, ControlAction::HideWindow).await
    }

    // ── create / clone / delete / rename ──────────────────────────────────────

    pub async fn create(&self, number: u32, mini: bool) -> Result<()> {
        let n = number.to_string();
        let mut args = vec!["create", "--number", &n];
        if mini {
            args.push("--mini");
        }
        self.run(&args).await?;
        Ok(())
    }

    pub async fn clone_player(&self, vmindex: impl Into<VmIndex>, number: u32) -> Result<()> {
        let idx = vmindex.into().to_arg();
        let n = number.to_string();
        self.run(&["clone", "--vmindex", &idx, "--number", &n]).await?;
        Ok(())
    }

    pub async fn delete(&self, vmindex: impl Into<VmIndex>) -> Result<()> {
        let idx = vmindex.into().to_arg();
        self.run(&["delete", "--vmindex", &idx]).await?;
        Ok(())
    }

    pub async fn rename(&self, index: u32, name: &str) -> Result<()> {
        let i = index.to_string();
        self.run(&["rename", "--vmindex", &i, "--name", name]).await?;
        Ok(())
    }

    // ── sh / adb ──────────────────────────────────────────────────────────────

    pub async fn sh(&self, index: u32, cmd: &str) -> Result<String> {
        let i = index.to_string();
        self.run_text(&["sh", "--vmindex", &i, "--cmd", cmd]).await
    }

    pub async fn adb(&self, index: u32, cmd: &str) -> Result<String> {
        let i = index.to_string();
        self.run_text(&["adb", "--vmindex", &i, "--cmd", cmd]).await
    }

    /// Write arbitrary bytes to a path on the Android filesystem.
    /// Uses base64 to avoid backslash-stripping in the sh --cmd pipeline.
    pub async fn write_file(&self, index: u32, remote_path: &str, content: &[u8]) -> Result<()> {
        use base64::prelude::*;
        let encoded = BASE64_STANDARD.encode(content);
        self.sh(index, &format!("echo {encoded} | base64 -d > {remote_path}")).await?;
        Ok(())
    }

    /// Find the adb.exe bundled with this MuMu installation.
    /// Only meaningful when MumuCli was constructed with a full exe path.
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

    pub async fn sort(&self) -> Result<()> {
        self.run(&["sort"]).await?;
        Ok(())
    }

    // ── import / export ───────────────────────────────────────────────────────

    /// Import a `.mumudata` file into slot `slot`.
    /// Corresponds to: `mumu import -p <path> -n <slot>`
    pub async fn import(&self, path: &Path, slot: u32) -> Result<()> {
        let p = path.to_string_lossy();
        let n = slot.to_string();
        self.run(&["import", "--path", &p, "--number", &n]).await?;
        Ok(())
    }

    /// Export slot `vmindex` as a `.mumudata` file into `dir`.
    /// `name` sets the output filename (without extension); `zip` uses compressed format.
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

    /// Apply one or more key-value settings to a slot in a single invocation.
    /// Corresponds to: `mumu setting -v <vmindex> -k k1 -val v1 [-k k2 -val v2 ...]`
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

    /// Apply settings from a UTF-8 JSON file.
    /// Corresponds to: `mumu setting -v <vmindex> -p <path>`
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

    /// List all writable setting keys for a slot (returns raw JSON output).
    /// Corresponds to: `mumu setting -v <vmindex> -aw`
    pub async fn setting_all_writable(&self, vmindex: impl Into<VmIndex>) -> Result<String> {
        let idx = vmindex.into().to_arg();
        self.run_text(&["setting", "--vmindex", &idx, "--all_writable"]).await
    }
}
