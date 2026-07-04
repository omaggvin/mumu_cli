use thiserror::Error;

#[derive(Debug, Error)]
pub enum MumuError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("utf8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("mumu exited {code:?}: {stderr}")]
    NonZeroExit { code: Option<i32>, stderr: String },
    #[error("bundled adb.exe not found under the mumu install dir")]
    AdbExeNotFound,
    #[error("slot has no adb_host_ip/adb_port yet (not booted?)")]
    AdbEndpointUnavailable,
    #[error("adb install failed: {0}")]
    InstallFailed(String),
}

pub type Result<T> = std::result::Result<T, MumuError>;
