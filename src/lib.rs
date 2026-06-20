mod error;
mod types;

pub use error::{MumuError, Result};
pub use types::{ControlAction, PlayerInfo, SimuKey, VmIndex};

use std::{collections::BTreeMap, path::PathBuf};
use tokio::process::Command;

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
}
