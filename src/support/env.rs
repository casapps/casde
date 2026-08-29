//! Environment/runtime-signal helpers for UI mode detection (AI.md PART 3).

pub struct Env;

impl Env {
    pub fn has(&self, key: &str) -> bool {
        std::env::var_os(key).is_some()
    }

    pub fn has_any(&self, keys: &[&str]) -> bool {
        keys.iter().any(|k| self.has(k))
    }

    pub fn truthy(&self, key: &str) -> bool {
        matches!(
            std::env::var(key).ok().as_deref(),
            Some("1") | Some("true") | Some("TRUE") | Some("yes")
        )
    }

    pub fn is(&self, key: &str, value: &str) -> bool {
        std::env::var(key).ok().as_deref() == Some(value)
    }
}
