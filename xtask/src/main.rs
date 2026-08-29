//! Build/release automation, executed inside Docker (AI.md PART 5).
//! Extend with real release/packaging tasks as TODO.AI.md "CI/CD,
//! releases, and automation" work lands.

use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let task = std::env::args().nth(1);
    match task.as_deref() {
        Some("help") | None => {
            println!("usage: cargo run -p xtask -- <task>");
            println!("tasks: help");
            Ok(())
        }
        Some(other) => bail!("unknown xtask task: {other}"),
    }
}
