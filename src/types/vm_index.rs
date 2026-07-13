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
