//! Shared domain/application logic. UI adapters (GUI/CLI) sit on top of
//! this layer; behavior lives here, never duplicated per-surface
//! (AI.md PART 2 -> "Architectural Rule").

/// Runtime UI surface. TUI is intentionally absent: IDEA.md "App surfaces"
/// declares TUI out of scope for a desktop environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiMode {
    /// The DE session itself: compositor, panel, window management. Also
    /// the IPC main server (IDEA.md "Client/server (IPC) split").
    Gui,
    /// `casdectl`-style automation/scripting client.
    Cli,
}

/// Detect which surface to run, honoring the override priority in
/// AI.md PART 3: explicit flag > config > environment > auto-detect.
pub fn detect_ui_mode(env: &crate::support::env::Env, forced: Option<UiMode>) -> UiMode {
    if let Some(mode) = forced {
        return mode;
    }

    let remote_shell = env.has_any(&[
        "SSH_CONNECTION",
        "SSH_CLIENT",
        "SSH_TTY",
        "MOSH_IP",
        "MOSH_KEY",
    ]);
    let display_available = env.has("WAYLAND_DISPLAY") || env.has("DISPLAY");

    if !remote_shell && display_available {
        UiMode::Gui
    } else {
        UiMode::Cli
    }
}
