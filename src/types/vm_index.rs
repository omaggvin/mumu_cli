use super::idx::SlotIndex;

/// Selects which VM slot(s) a command targets.
///
/// Most methods accept `impl Into<VmIndex>`, so you can pass a bare [`SlotIndex`]
/// (or a `Vec` of them) for the common cases.
#[derive(Debug, Clone)]
pub enum VmIndex {
    /// Every slot (`--vmindex all`).
    All,
    /// A single slot by index.
    One(SlotIndex),
    /// An explicit list of slots (`--vmindex 0,2,3`).
    Many(Vec<SlotIndex>),
}

impl VmIndex {
    pub(crate) fn to_arg(&self) -> String {
        match self {
            VmIndex::All => "all".to_string(),
            VmIndex::One(i) => i.to_string(),
            VmIndex::Many(v) => v
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(","),
        }
    }
}

impl From<SlotIndex> for VmIndex {
    fn from(i: SlotIndex) -> Self {
        VmIndex::One(i)
    }
}

impl From<Vec<SlotIndex>> for VmIndex {
    fn from(v: Vec<SlotIndex>) -> Self {
        VmIndex::Many(v)
    }
}
