mod error;
mod types;

pub use error::{MumuError, Result};
pub use types::{
    ControlAction, GpuMode, NetBridgeIpMode, PerformanceMode, PlayerInfo, RendererMode,
    RendererStrategy, ResolutionMode, Setting, SimuKey, VmIndex,
};

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    time::Duration,
};
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
    ///
    /// MuMu's `info` output changes *shape* with the result count
    /// (real-hardware fact, 2026-07-03): an index-keyed map when it covers
    /// multiple instances, but a single **bare object** when there is
    /// exactly one — including `--vmindex all` on a PC with one instance.
    /// Both shapes are accepted here; the bare object's own `index` field
    /// supplies its key.
    pub async fn info(&self, vmindex: impl Into<VmIndex>) -> Result<BTreeMap<u32, PlayerInfo>> {
        let idx = vmindex.into().to_arg();
        let raw = self.run_text(&["info", "--vmindex", &idx]).await?;
        parse_info_output(&raw)
    }

    /// Returns info for every slot.
    pub async fn info_all(&self) -> Result<BTreeMap<u32, PlayerInfo>> {
        self.info(VmIndex::All).await
    }

    /// Raw `info --vmindex <index>` output, unparsed — diagnostics only
    /// (pcc's `mumu_diag`), so field/shape surprises are visible verbatim.
    pub async fn info_raw(&self, index: u32) -> Result<String> {
        let i = index.to_string();
        self.run_text(&["info", "--vmindex", &i]).await
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
        self.run(&["control", "--vmindex", &idx, action.as_str()])
            .await?;
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
        self.run(&["clone", "--vmindex", &idx, "--number", &n])
            .await?;
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
        self.run(&["rename", "--vmindex", &i, "--name", name])
            .await?;
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
    ///
    /// Windows caps a process command line near 32 KB, and `MuMuManager.exe`
    /// fails with "error 206" once `sh --cmd <one-shot>` crosses it — about
    /// 24 KB of content after base64's 4/3 inflation. Below the cap the file
    /// goes in the original single `echo … | base64 -d` command, byte for
    /// byte as before. Above it the base64 is streamed to a temp file in
    /// safe-sized chunks (`echo >`/`>>`) and decoded once, so an oversized
    /// write that used to fail now succeeds without changing the path any
    /// currently-working write takes. base64's alphabet has no shell-special
    /// chars, and `base64 -d` ignores the newlines `echo` inserts between
    /// chunks.
    pub async fn write_file(&self, index: u32, remote_path: &str, content: &[u8]) -> Result<()> {
        // Conservative headroom under the ~32 KB Windows command-line ceiling;
        // every real write today (loader, config, licence, worker Lua) is a
        // few KB and stays on the single-command path.
        const CMD_BUDGET: usize = 28_000;
        for cmd in Self::write_file_cmds(remote_path, content, CMD_BUDGET) {
            self.sh(index, &cmd).await?;
        }
        Ok(())
    }

    /// Builds the `sh` command(s) [`Self::write_file`] runs. Pure (no device
    /// I/O) so the chunk boundaries and base64 round-trip stay unit-testable.
    /// `budget` bounds the length of any single command.
    fn write_file_cmds(remote_path: &str, content: &[u8], budget: usize) -> Vec<String> {
        use base64::prelude::*;
        let encoded = BASE64_STANDARD.encode(content);
        let one_shot = format!("echo {encoded} | base64 -d > {remote_path}");
        if one_shot.len() <= budget {
            return vec![one_shot];
        }
        let tmp = format!("{remote_path}.omna_b64");
        // Room for `echo `, ` >> `, and the temp path in each chunk command.
        let chunk_len = budget.saturating_sub(tmp.len() + 16).max(1);
        let mut cmds = Vec::new();
        let mut first = true;
        for chunk in encoded.as_bytes().chunks(chunk_len) {
            let chunk = std::str::from_utf8(chunk).expect("base64 output is ascii");
            let redir = if first { ">" } else { ">>" };
            cmds.push(format!("echo {chunk} {redir} {tmp}"));
            first = false;
        }
        cmds.push(format!("base64 -d {tmp} > {remote_path} && rm -f {tmp}"));
        cmds
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
            "--vmindex",
            &idx,
            "--simu_key",
            key.as_str(),
            "--simu_value",
            v,
        ])
        .await?;
        Ok(())
    }

    /// [`Self::install_apk`]/[`Self::pull`] talk to the slot's TCP adb
    /// endpoint directly, and a freshly-booted instance regularly sits as
    /// `offline` in the daemon's device list for a few seconds (a chain's
    /// `install_roblox` right after `start_placement` failed with "adb.exe:
    /// device offline" on i9tjnr, 2026-07-08 — the same fresh-boot flap
    /// `kill_roblox`/`write_delta_file` were moved to NemuShell for, but
    /// there's no shell path for streaming an APK or pulling a directory).
    /// Connects the endpoint (idempotent) and polls `get-state` until the
    /// daemon reports `device`, redialing once on a stuck `offline` entry.
    async fn ensure_adb_ready(&self, adb: &Path, serial: &str) -> Result<()> {
        const READY_TIMEOUT: Duration = Duration::from_secs(20);
        const POLL: Duration = Duration::from_secs(1);

        let _ = Command::new(adb).args(["connect", serial]).output().await?;
        let deadline = tokio::time::Instant::now() + READY_TIMEOUT;
        let mut redialed = false;
        loop {
            let out = Command::new(adb)
                .args(["-s", serial, "get-state"])
                .output()
                .await?;
            let last = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout).trim(),
                String::from_utf8_lossy(&out.stderr).trim()
            );
            if last == "device" {
                return Ok(());
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(MumuError::AdbDeviceNotReady {
                    serial: serial.to_string(),
                    secs: READY_TIMEOUT.as_secs(),
                    last,
                });
            }
            if !redialed && last.contains("offline") {
                let _ = Command::new(adb)
                    .args(["disconnect", serial])
                    .output()
                    .await;
                let _ = Command::new(adb).args(["connect", serial]).output().await;
                redialed = true;
            }
            tokio::time::sleep(POLL).await;
        }
    }

    // ── apk install ──────────────────────────────────────────────────────────

    /// Installs (or upgrades in place — same as dragging an APK onto the
    /// emulator window: existing app data/login survive) an APK onto slot
    /// `index`.
    ///
    /// `write_file`'s `sh`-through-base64 pipeline can't carry a
    /// multi-hundred-MB binary (it would blow past the shell command-length
    /// limit long before reaching an APK's size), so this shells out to the
    /// MuMu-bundled `adb.exe` directly against the slot's own adb endpoint
    /// (`info_one`'s `adb_host_ip`/`adb_port`) and runs a real
    /// `adb install -r`, which streams the file over the ADB wire protocol
    /// instead of a shell string.
    pub async fn install_apk(&self, index: u32, apk_path: &Path) -> Result<()> {
        let info = self.info_one(index).await?;
        let (Some(host), Some(port)) = (info.adb_host_ip, info.adb_port) else {
            return Err(MumuError::AdbEndpointUnavailable);
        };
        let adb = self.find_adb().ok_or(MumuError::AdbExeNotFound)?;
        let serial = format!("{host}:{port}");
        self.ensure_adb_ready(&adb, &serial).await?;
        let path_str = apk_path.to_string_lossy();

        let out = Command::new(&adb)
            .args(["-s", &serial, "install", "-r", &path_str])
            .output()
            .await?;
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        if out.status.success() && stdout.contains("Success") {
            Ok(())
        } else {
            Err(MumuError::InstallFailed(format!("{stdout}{stderr}")))
        }
    }

    /// Pulls a file or directory off slot `index` at `remote` (an absolute
    /// on-device path) down to `local`. Same "MuMu's own wrapper can't do
    /// this" rationale as [`Self::install_apk`] — shells directly to the
    /// bundled `adb.exe` for a real `adb pull`, which handles whole
    /// directories natively instead of a size-fragile write_file/base64/sh
    /// round trip.
    pub async fn pull(&self, index: u32, remote: &str, local: &Path) -> Result<()> {
        let info = self.info_one(index).await?;
        let (Some(host), Some(port)) = (info.adb_host_ip, info.adb_port) else {
            return Err(MumuError::AdbEndpointUnavailable);
        };
        let adb = self.find_adb().ok_or(MumuError::AdbExeNotFound)?;
        let serial = format!("{host}:{port}");
        self.ensure_adb_ready(&adb, &serial).await?;
        let local_str = local.to_string_lossy();

        let out = Command::new(&adb)
            .args(["-s", &serial, "pull", remote, &local_str])
            .output()
            .await?;
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        if out.status.success() {
            Ok(())
        } else {
            Err(MumuError::PullFailed(format!("{stdout}{stderr}")))
        }
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
    pub async fn setting_from_file(&self, vmindex: impl Into<VmIndex>, path: &Path) -> Result<()> {
        let idx = vmindex.into().to_arg();
        let p = path.to_string_lossy();
        self.run(&["setting", "--vmindex", &idx, "--path", &p])
            .await?;
        Ok(())
    }

    /// Return all writable setting keys and their current values as raw JSON.
    ///
    /// Useful for inspecting what a slot currently has before applying changes.
    pub async fn setting_all_writable(&self, vmindex: impl Into<VmIndex>) -> Result<String> {
        let idx = vmindex.into().to_arg();
        self.run_text(&["setting", "--vmindex", &idx, "--all_writable"])
            .await
    }

    /// Apply typed [`Setting`] values to a slot in a single invocation.
    pub async fn setting_apply(
        &self,
        vmindex: impl Into<VmIndex>,
        settings: &[Setting],
    ) -> Result<()> {
        let pairs: Vec<(&'static str, String)> = settings.iter().map(|s| s.as_pair()).collect();
        let str_pairs: Vec<(&str, &str)> = pairs.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.setting_set(vmindex, &str_pairs).await
    }
}

/// Parses `MuMuManager info` output into an index-keyed map, accepting both
/// output shapes (see [`MumuCli::info`]): the multi-instance index-keyed
/// map, and the bare single object MuMu emits when the result is exactly
/// one instance.
fn parse_info_output(raw: &str) -> Result<BTreeMap<u32, PlayerInfo>> {
    if let Ok(map) = serde_json::from_str::<std::collections::HashMap<String, PlayerInfo>>(raw) {
        return Ok(map
            .into_iter()
            .filter_map(|(k, v)| k.parse::<u32>().ok().map(|i| (i, v)))
            .collect());
    }
    let one: PlayerInfo = serde_json::from_str(raw)?;
    let Ok(i) = one.index.parse::<u32>() else {
        return Err(MumuError::NonZeroExit {
            code: None,
            stderr: format!("info returned unparseable index {:?}", one.index),
        });
    };
    Ok(BTreeMap::from([(i, one)]))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verbatim `MuMuManager info --vmindex all` output from i9tjnr with a
    /// single instance (2026-07-03) — a bare object, not an index-keyed map.
    const SINGLE_RAW: &str = r#"{
  "index": "0",
  "name": "omaggviw@o1",
  "is_main": true,
  "error_code": 0,
  "disk_size_bytes": 4067478846,
  "created_timestamp": 1782734046011375,
  "is_android_started": false,
  "is_process_started": false,
  "hyperv_enabled": true
}"#;

    #[test]
    fn single_instance_bare_object_parses() {
        let map = parse_info_output(SINGLE_RAW).expect("bare object must parse");
        assert_eq!(map.len(), 1);
        assert_eq!(map[&0].name, "omaggviw@o1");
    }

    #[test]
    fn multi_instance_keyed_map_parses() {
        let raw = format!(r#"{{ "0": {SINGLE_RAW}, "3": {SINGLE_RAW} }}"#);
        let map = parse_info_output(&raw).expect("keyed map must parse");
        assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![0, 3]);
    }

    #[test]
    fn garbage_is_an_error_not_a_panic() {
        assert!(parse_info_output("not json").is_err());
    }

    #[test]
    fn write_file_small_stays_single_command() {
        let cmds = MumuCli::write_file_cmds("/storage/emulated/0/Delta/x", b"hello world", 28_000);
        assert_eq!(cmds.len(), 1);
        assert!(cmds[0].starts_with("echo "));
        assert!(cmds[0].ends_with("| base64 -d > /storage/emulated/0/Delta/x"));
    }

    #[test]
    fn write_file_large_chunks_reconstruct_content() {
        use base64::prelude::*;
        // ~80 KB of non-trivial bytes forces the chunked path.
        let content: Vec<u8> = (0..80_000u32)
            .map(|i| (i.wrapping_mul(31) % 251) as u8)
            .collect();
        let budget = 28_000;
        let path = "/storage/emulated/0/Delta/big.bin";
        let cmds = MumuCli::write_file_cmds(path, &content, budget);

        assert!(cmds.len() > 2, "large content must chunk");
        for c in &cmds {
            assert!(
                c.len() <= budget,
                "command exceeds budget: {} > {budget}",
                c.len()
            );
        }

        // Replay the commands the way the device would: each chunk command is
        // `echo <chunk> >|>> <tmp>` with whitespace-free tokens, so split(' ')
        // recovers the base64; concatenating them and decoding must yield the
        // original bytes.
        let (decode_cmd, echo_cmds) = cmds.split_last().unwrap();
        let mut b64 = String::new();
        for (i, c) in echo_cmds.iter().enumerate() {
            let parts: Vec<&str> = c.split(' ').collect();
            assert_eq!(parts[0], "echo");
            assert_eq!(
                parts[2],
                if i == 0 { ">" } else { ">>" },
                "wrong redirect on chunk {i}"
            );
            b64.push_str(parts[1]);
        }
        assert!(decode_cmd.starts_with("base64 -d "));
        assert!(decode_cmd.contains(&format!("> {path}")));

        let decoded = BASE64_STANDARD
            .decode(b64.as_bytes())
            .expect("reassembled base64 must decode");
        assert_eq!(
            decoded, content,
            "chunked write must reconstruct the original bytes"
        );
    }
}
