pub(crate) fn map_err(error: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(error.to_string())
}
