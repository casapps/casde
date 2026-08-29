//! Build/version metadata embedded at compile time (AI.md PART 6).

pub const VERSION: &str = match option_env!("APP_VERSION") {
    Some(v) => v,
    None => env!("CARGO_PKG_VERSION"),
};
pub const OFFICIAL_SITE: &str = match option_env!("APP_OFFICIAL_SITE") {
    Some(v) => v,
    None => "",
};
pub const COMMIT_ID: &str = match option_env!("APP_COMMIT_ID") {
    Some(v) => v,
    None => "N/A",
};
pub const BUILD_EPOCH: &str = match option_env!("APP_BUILD_EPOCH") {
    Some(v) => v,
    None => "0",
};

/// Build date derived from `BUILD_EPOCH` (RFC 3339 UTC); "N/A" when unset.
pub fn build_date() -> String {
    match BUILD_EPOCH.parse::<i64>() {
        Ok(n) if n > 0 => chrono::DateTime::from_timestamp(n, 0)
            .map(|t| t.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .unwrap_or_else(|| "N/A".into()),
        _ => "N/A".into(),
    }
}
