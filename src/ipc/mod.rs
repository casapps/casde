//! Local client/server IPC (AI.md PART 2 -> "Local Client/Server (IPC)
//! Mode"; IDEA.md "App surfaces" -> client/server split).
//!
//! The running CASDE session is the long-lived main server; the settings
//! UI and `casdectl` are thin clients. No TCP listener, ever — see
//! IDEA.md "Trust boundaries & security exceptions". Wire protocol,
//! registry, and save/restore land as TODO.AI.md "IPC / session state".

use std::path::PathBuf;

use crate::config::INTERNAL_NAME;

/// Path to the main socket: `$XDG_RUNTIME_DIR/casde/casde.sock`
/// (`0700` dir, `0600` socket — enforced when the listener is created).
/// IDEA.md "App surfaces" pins this to a single `{internal_name}/`
/// segment, not the nested org/name layout used for config/data/cache.
pub fn main_socket_path() -> PathBuf {
    runtime_dir().join(format!("{INTERNAL_NAME}.sock"))
}

fn runtime_dir() -> PathBuf {
    let base = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(crate::config::state_dir);
    base.join(INTERNAL_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_name_matches_internal_name() {
        assert!(main_socket_path().ends_with(format!("{INTERNAL_NAME}.sock")));
    }
}
