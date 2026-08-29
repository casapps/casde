//! `casdectl`-style CLI surface — automation fallback that talks to the
//! running session over the IPC socket (IDEA.md "App surfaces").

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print embedded LICENSE.md and exit (AI.md PART 11 -> "User-Visible
    /// Attribution Surface").
    Licenses,
    /// List running CASDE servers (main + any named servers).
    Servers,
}

pub fn run(cmd: Option<Command>) -> anyhow::Result<()> {
    match cmd {
        Some(Command::Licenses) => {
            print!("{}", include_str!("../../../LICENSE.md"));
            Ok(())
        }
        Some(Command::Servers) => {
            anyhow::bail!("server registry is not implemented yet — see TODO.AI.md")
        }
        None => {
            println!(
                "casde {} — no subcommand given; try --help",
                crate::support::meta::VERSION
            );
            Ok(())
        }
    }
}
