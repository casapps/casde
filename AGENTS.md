# casde

Read `AI.md` and `IDEA.md` before acting on this project.

`AI.md` is the read-only implementation spec (HOW) — the CasjaysDev
standard Rust application template (single static binary, GUI/CLI, no
TUI, Docker-only toolchain, X11+Wayland parity, MIT license).

`IDEA.md` is the product definition (WHAT) — CASDE, a full desktop
environment. It carries the actual feature decisions (compositor,
workspaces, keybindings, panel, theming, system integration, trust
boundaries) that AI.md's placeholders and generic rules resolve
against. Never invent product behavior not already decided there.

`SPEC.md`, if non-empty, overrides AI.md wherever the two conflict.

See `TODO.AI.md` for the current implementation backlog.
