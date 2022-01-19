
/// A module to be used with `#[serde(with = "ts_milliseconds_str")]` to allow
/// serialization/deserialization of string values containing a number of milliseconds since the
/// unix epoch in base 10. For example, the following json could have its `time` field deserialized
/// using this module:
/// ```json
/// {
///   time: "1640995200000"
/// }
/// ```
pub mod ts_milliseconds_str;
