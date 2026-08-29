# TODO.AI.md — CASDE implementation backlog

`AI.md` is truth about WHAT must be done overall; this file only tracks
it. PART 0–6 scaffolding (Cargo workspace, `src/` skeleton, Docker
toolchain wiring, Makefile, license/metadata files, loaders) is done —
see the bootstrap completion report. Everything below is feature
implementation this bootstrap pass intentionally did not build.

## Blockers (need a user decision before the dependent items can start)

## [ ] Decide exact BSD variant(s) in scope
Read: IDEA.md "Open questions" (line 583), AI.md PART 0 "Single Static Binary"
OpenBSD only, or also FreeBSD/NetBSD? Also need the concrete BSD
equivalents for `systemd-logind` session management and polkit
privilege elevation. Blocks: any BSD-target work in "Compositor" and
"System integration" below (Linux/amd64+arm64 work is unblocked and
should proceed first regardless).

## [ ] Decide the phased implementation roadmap
Read: IDEA.md "Open questions" (line 587), IDEA.md "Technology foundation" (lines 48-98)
Which backend/milestone ships first, given full Wayland/X11 effect
parity and full multi-monitor support are both required for v1.
IDEA.md's own reasoning leans toward X11 first (`x11rb` and the X11
protocol surface are simpler/more stable than Smithay's Wayland
surface) but this is explicitly still open. The dependency order below
assumes that lean; confirm or correct it before starting "Display
backends".

## Dependency order (topological, PART number as tiebreaker)

1. IPC / session-state protocol (foundation everything else talks to)
2. Display backends: X11 first, then Wayland (per the roadmap
   assumption above — reconfirm against the blocker before starting)
3. Window management
4. Virtual desktops / workspaces
5. Keybindings
6. Panel / taskbar / app menu
7. Config system + settings UI
8. Multi-monitor
9. Accessibility
10. System integration (screenshot, clipboard, audio, notifications,
    keyring, xdg-desktop-portal)
11. Theming
12. Autostart / session startup
13. `casdectl` CLI completeness (depends on 1-12 having something to
    query/control)
14. Cross-cutting AI.md PARTs 7-11 (runtime polish, testing depth,
    security/privacy hardening, CI/CD, doc/license generation)

## [ ] IPC / session-state protocol
Read: AI.md PART 2 "Local Client/Server (IPC) Mode", IDEA.md "App surfaces" (line 124)
Implement the main-socket listener at `$XDG_RUNTIME_DIR/casde/casde.sock`
(`src/ipc/mod.rs` already has the path helper), the wire protocol for
`casdectl`/settings-UI clients, and per-monitor session-state tracking
(active workspace/tag set, window list with geometry/workspace-
membership/focus). Peer-credential check (`SO_PEERCRED`) per IDEA.md
"Trust boundaries & security exceptions" (line 459).

## [ ] X11 backend
Read: IDEA.md "Technology foundation" (lines 59-66, 84-87), IDEA.md "Display backends" (line 155)
Build on `x11rb` for the X11 protocol layer; CASDE's own window
management/policy/panel layered on top as original code. No link-time
`libX11` dependency (AI.md PART 0 "X11 AND Wayland Are Both Required").

## [ ] Wayland backend (compositor)
Read: IDEA.md "Technology foundation" (lines 59-63), IDEA.md "Compositor" (line 166)
Build on `smithay` for protocol plumbing, DRM/KMS, input, and rendering
integration; CASDE's own compositing/effects, panel, window management,
and policy layered on top. Full effect parity with the X11 backend is
required for v1 (IDEA.md "Display backends").

## [ ] Window management
Read: IDEA.md "Window management" (line 218)

## [ ] Virtual desktops / workspaces
Read: IDEA.md "Virtual desktops / workspaces" (line 175)
Multi-tag model; 9 default workspaces: dev/web/sys/office/virt/comm/
audio/video/gfx (XFCE4 config in `desktopmgr` is the accuracy baseline).

## [ ] Keybindings
Read: IDEA.md "Keybindings" (line 234)
Super-based bindings plus XF86 media keys.

## [ ] Panel / taskbar / app menu
Read: IDEA.md "Panel / taskbar / app menu" (line 305)
Built-in widget catalog only — no third-party plugin system.

## [ ] Config system + settings UI
Read: IDEA.md "Config system" (line 363), AI.md PART 4 "Privilege Escalation & System Integration"
YAML config (`src/config/mod.rs` has the load/default skeleton) plus a
full settings UI; polkit elevation only for the specific privileged
actions IDEA.md names, never for normal app launch.

## [ ] Multi-monitor
Read: IDEA.md "Multi-monitor" (line 377)
Full v1 support, not a follow-on.

## [ ] Accessibility
Read: IDEA.md "Accessibility" (line 393)
Full a11y from day one, not retrofitted.

## [ ] System integration
Read: IDEA.md "System integration" (line 403), IDEA.md "Trust boundaries & security exceptions" (line 459)
Screenshot tool, clipboard manager, PipeWire+PulseAudio integration, a
notification daemon implementing `org.freedesktop.Notifications`, a
native keyring implementing the full Secret Service API, and an
`xdg-desktop-portal` compat provider with auto-approved consent (a
deliberate, documented divergence from upstream portal behavior — keep
the trust-boundary writeup in sync as this lands).

## [ ] Theming
Read: IDEA.md "Theming" (line 492)
50 bundled themes (25 dark + 25 light, curated well-known palettes),
each covering GTK3/4 + Qt5/6 + decorations + panel. Icon themes:
N.I.B. default, Obsidian optional. Fonts: Hack Nerd Font default, Noto
Color Emoji. Embed via `src/assets.rs` (`rust-embed`, already wired to
`assets/themes|icons|fonts`) — never fetched at runtime (AI.md PART 0
"Self-Contained Assets").

## [ ] Autostart / session startup
Read: IDEA.md "Autostart / session startup" (line 571)

## [ ] `casdectl` CLI completeness
Read: IDEA.md "App surfaces" (line 117), AI.md PART 2 "Binary Model"
Extend `src/ui/cli/mod.rs` beyond the `--licenses`/`servers` stubs:
workspace management, keybinding queries, panel/theme changes — same
actions the settings UI exposes, over the same IPC socket.

## [ ] Runtime detection, configuration, and output polish
Read: AI.md PART 7

## [ ] Testing, quality, and debugging depth
Read: AI.md PART 8
Add integration tests exercising both X11 and Wayland forwarding once
those backends exist (self-validation table in AI.md PART 1 already
defines what "verified" means per change type).

## [ ] Security & privacy hardening
Read: AI.md PART 9
Dependency governance (cargo-audit/cargo-deny already scaffolded via
`deny.toml`), plus IPC/portal/keyring-specific hardening once those
land.

## [ ] CI/CD, releases, and automation
Read: AI.md PART 10
GitHub Actions workflows (security-only workflows first, `ci.yml`/
`release.yml` last per the global workflow-gate rule), release artifact
publishing per AI.md PART 5 "Release Artifacts", SBOM generation.

## [ ] Documentation & license generation
Read: AI.md PART 11
First real `cargo about generate about.hbs` run once dependencies are
locked, to replace the "pending" placeholder in `LICENSE.md`'s
generated region. Expand README as GUI/CLI behavior solidifies.

## Not applicable

- **AI.md PART 14 (RFC protocol server / daemon mode)** — conditional
  on IDEA.md declaring a network listener. IDEA.md explicitly rules
  this out ("no TCP listener, ever" — "App surfaces", line 124); the
  IPC socket above is a Unix-domain client/server split, which PART 14
  does not govern.
- **AI.md PART 12 (Checklists) / PART 13 (IDEA.md reference)** — these
  are verification/reference material, not implementation work; no
  backlog item needed.
