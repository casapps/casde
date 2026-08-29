// Several modules are stubs awaiting the TODO.AI.md feature backlog
// (compositor, config, IPC, theming) — allow unused scaffolding until
// those items wire the pieces together.
#![allow(dead_code)]

mod app;
mod assets;
mod config;
mod ipc;
mod platform;
mod state;
mod support;
mod ui;

use app::UiMode;
use clap::{Parser, ValueEnum};

/// CASDE — a full, single-binary desktop environment for Linux (Wayland + X11).
#[derive(Debug, Parser)]
#[command(name = "casde", version = support::meta::VERSION)]
struct Cli {
    /// Force a UI surface instead of auto-detecting (AI.md PART 3).
    #[arg(long, value_enum)]
    ui: Option<UiSurface>,

    #[command(subcommand)]
    command: Option<ui::cli::Command>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum UiSurface {
    Gui,
    Cli,
}

impl From<UiSurface> for UiMode {
    fn from(v: UiSurface) -> Self {
        match v {
            UiSurface::Gui => UiMode::Gui,
            UiSurface::Cli => UiMode::Cli,
        }
    }
}

fn main() -> anyhow::Result<()> {
    support::init_logging();

    let cli = Cli::parse();
    let env = support::env::Env;

    // A subcommand always implies the CLI surface, even without --ui cli.
    let forced = cli
        .ui
        .map(UiMode::from)
        .or(cli.command.is_some().then_some(UiMode::Cli));
    let mode = app::detect_ui_mode(&env, forced);

    match mode {
        UiMode::Gui => ui::gui::run(),
        UiMode::Cli => ui::cli::run(cli.command),
    }
}
