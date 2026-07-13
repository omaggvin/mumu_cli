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
