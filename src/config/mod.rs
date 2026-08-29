//! Configuration loading and defaults (YAML — IDEA.md "Config system").
//!
//! Real schema and settings-UI wiring are implementation-PART work
//! (see TODO.AI.md "Config system"); this module only establishes the
//! load path and default-on-empty-config guarantee for now.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Frozen on-disk identifiers (AI.md PART 4 -> "Path Rule"). Never derive
/// these from the mutable project_name/project_org.
pub const INTERNAL_ORG: &str = "casapps";
pub const INTERNAL_NAME: &str = "casde";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    // Populated as DE feature PARTs land (workspaces, keybindings,
    // panel, theme, multi-monitor, accessibility — see TODO.AI.md).
}

pub fn config_dir() -> PathBuf {
    dirs_path("XDG_CONFIG_HOME", ".config")
}

pub fn data_dir() -> PathBuf {
    dirs_path("XDG_DATA_HOME", ".local/share")
}

pub fn cache_dir() -> PathBuf {
    dirs_path("XDG_CACHE_HOME", ".cache")
}

pub fn state_dir() -> PathBuf {
    dirs_path("XDG_STATE_HOME", ".local/state")
}

fn dirs_path(xdg_var: &str, fallback_rel: &str) -> PathBuf {
    let base = std::env::var_os(xdg_var)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(fallback_rel)
        });
    base.join(INTERNAL_ORG).join(INTERNAL_NAME)
}

/// Load config from disk, falling back to defaults when absent — the app
/// must run with full default functionality when config is empty
/// (AI.md PART 0 -> "Self-Contained Assets").
pub fn load() -> anyhow::Result<Config> {
    let path = config_dir().join("config.yaml");
    if !path.exists() {
        return Ok(Config::default());
    }
    let raw = std::fs::read_to_string(&path)?;
    Ok(serde_yaml::from_str(&raw)?)
}
