//! GUI surface: the CASDE session itself (compositor, panel, window
//! management, settings UI) — IDEA.md "App surfaces" primary surface.
//! Compositor/panel/theming implementations land as TODO.AI.md feature
//! work; this module is the entry point the mode dispatcher calls into.

pub fn run() -> anyhow::Result<()> {
    tracing::info!("casde: starting GUI/session surface (not yet implemented)");
    anyhow::bail!("GUI surface is not implemented yet — see TODO.AI.md")
}
