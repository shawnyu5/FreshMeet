use chrono::{DateTime, Utc};

/// format a date into ISO8601 format
/// formats like "2001-07-08T00:34:60.026490+09:30"
///
/// * `st`: date to format
pub fn to_iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    return format!("{}", dt.format("%+"));
}
