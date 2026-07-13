/// Simulated device property to spoof. Used with [`crate::MumuCli::simulate`].
///
/// Pass `None` as the value to clear back to the generated default.
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
