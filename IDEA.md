## Project description

CASDE is a full desktop environment for Linux with a built-in, feature-rich
compositor. It is a single, unified replacement for the fragmented WM/DE
landscape — IceWM, Hyprland, Sway, AwesomeWM, Xmonad, XFCE4, BSPWM, i3,
Openbox, MATE, and the rest — providing one coherent environment instead of
requiring users to hand-assemble a WM + compositor + panel + hotkey daemon +
notification daemon + applets stack themselves.

The feature set is a deliberate, deduplicated superset of what the existing
window managers/DEs in `desktopmgr` (i3, Sway, BSPWM, Awesome, LeftWM,
XMonad, Qtile, Herbstluftwm, Spectrwm, Openbox, CWM, XFCE4, MATE, Plasma,
Cutefish, TWM, DWM) already do individually. **XFCE4's config (in
`desktopmgr/xfce4`) is the accuracy baseline and default** — it is the
project author's actual daily-driver DE, so its panel layout, workspace
names, keybindings, theme, and autostart order are ported into CASDE's
built-in defaults, then extended with the features other WMs have that
XFCE4 lacks (native tiling, scratchpad/dropdown terminals, compositor
effects, tag-style workspaces, etc.), with true duplicates across the
source configs collapsed into one implementation.

Target users are Linux desktop users who want tiling-WM power (keybinding-
driven workflow, workspaces, window rules) without giving up a traditional
DE's completeness (panel, settings UI, tray, notifications, session
management) and without assembling it from a dozen separate daemons.

CASDE supports both native Wayland (not XWayland-shimmed) and native X11 as
first-class, fully-supported backends — not one primary and one
compatibility mode.

This is an early-stage plan: scope above is directional and will need
further discussion and research before implementation begins.

## Project variables

    project_name:  casde
    project_org:   casapps
    # FROZEN — set once at first-time setup, never edit
    internal_name: casde
    # FROZEN — set once at first-time setup, never edit
    internal_org:  casapps
    app_name:      CASDE
    reference_configs_dir: ../../desktopmgr
    default_reference_wm:  xfce4

## Business logic

### Technology foundation
- **Superseded decision** — the original plan below (fully from-scratch
  protocol implementation) is replaced: CASDE is implemented in **Rust**
  (per AI.md's single-Rust-application rule) and builds on existing,
  actively-maintained crates for protocol-level work wherever a suitable
  crate exists, rather than reimplementing Wayland/X11/DRM/input
  handling from zero. Reasoning: those protocol layers are exactly the
  kind of "already solved, extensively hardware-tested" problem that a
  from-scratch reimplementation would spend years re-discovering bugs
  in; reuse gets CASDE to a real working v1 in a realistic timeframe
  without changing what the finished DE looks or feels like to a user.
  - **Wayland backend**: build on `smithay` (the Rust Wayland
    compositor framework/crate ecosystem — the Rust-native equivalent
    of `wlroots`) for protocol plumbing, DRM/KMS, input, and rendering
    integration, with CASDE's own compositing/effects, panel, window
    management, and policy layered on top as original code.
  - **X11 backend**: build on `x11rb` (or `xcb`-equivalent Rust
    bindings) for the X11 protocol layer, again with CASDE's own window
    management/policy/panel layered on top.
  - **Everything CASDE-specific** (window rules engine, multi-tag
    workspaces, panel/widgets, theming/portal/keyring services,
    settings UI, CLI) remains original code — reuse applies to the
    protocol/hardware layer, not to CASDE's own DE behavior or feature
    set, which stays exactly as specified elsewhere in this document.
  - **General reuse principle**: prefer an existing, well-maintained
    crate over hand-rolled code anywhere one genuinely fits (e.g. the
    Secret Service D-Bus server, the notification-spec D-Bus server,
    fontconfig/freedesktop-icon-theme parsing, image decoding for
    wallpapers/icons) — reinvent only the parts that are CASDE's actual
    product differentiation.
- **Single static binary** — build mechanics (musl targets, static
  linking, toolchain image, artifact naming) are already fully governed
  by AI.md's "Single Static Binary" section; not restated here. The
  IDEA.md-level decision this drives: Linux (`amd64`/`arm64`, musl) is
  in scope by default per AI.md; BSD targets are opt-in per IDEA.md —
  which BSD variant(s) are in scope is still open, see Open Questions.
  Crate choices for the display layer follow AI.md's existing rule
  (`x11rb` for X11 — pure-Rust, no link-time `libX11`; `wayland-client`/
  `wayland-rs` with the `dlopen` feature for Wayland) so CASDE's binary
  has no link-time C dependency for display I/O on either backend.
- **Honest scope flag, revised**: switching to crate reuse substantially
  reduces (it does not eliminate) the multi-year-effort risk called out
  under the old from-scratch plan — protocol/hardware-layer bugs are
  now largely upstream's problem, not CASDE's. What remains genuinely
  large is still CASDE's own feature surface (window management, panel,
  theming/portal/keyring services, settings UI) and still needs a
  realistic phased roadmap (e.g. X11 backend first since `x11rb` and
  the X11 protocol surface are simpler/more stable than Smithay's
  Wayland surface) rather than building both backends to full parity
  simultaneously as a single monolithic v1.

### Platform targets
- Linux and BSD (not Linux-only). BSD support means avoiding
  Linux-specific assumptions where they'd break portability — e.g.
  `systemd-logind` session management, `/proc`-only introspection, and
  Linux-only DRM/KMS ioctls need a documented abstraction or a BSD
  equivalent (OpenBSD's `cwm` in `desktopmgr` is the existing proof this
  is viable, but scoped much smaller than a full DE).
- Exact BSD variant(s) targeted (OpenBSD only vs also FreeBSD/NetBSD) is
  still open — needs further discussion once the technology foundation
  above is underway.

### App surfaces
Per AI.md → "GUI/TUI/CLI Capability Rule", surfaces in scope are declared
explicitly here:
- **GUI** (primary, interactive): the desktop environment itself
  (compositor, panel/bar, app menu, taskbar, desktop icons, notifications,
  screenshot tool, clipboard manager) plus a full settings UI, similar in
  scope to `xfce4-settings-manager`.
- **CLI** (automation fallback): a `casdectl`-style companion tool that
  reads/writes the same YAML config and can trigger the same actions as
  the settings UI (workspace management, keybinding queries, panel/theme
  changes) — for scripting, dotfiles, and non-interactive use, matching
  AI.md's "CLI is the universal fallback... for automation" rule.
- **TUI**: not in scope. A desktop environment has no meaningful
  terminal-only interactive mode.
- No client/server (IPC) split is defined yet — needs further design.

### Display backends
- Native Wayland compositor (not an XWayland compatibility shim) — full
  support.
- Native X11 window manager — full support.
- **Full effect parity between backends is required for v1** — every
  compositor effect (transparency, shadows, animations, wobbly windows)
  must work identically on Wayland and X11 before v1 ships. Given the
  from-scratch technology foundation above, this is a significant driver
  of the v1 timeline and must be accounted for in the phased roadmap
  rather than treated as a stretch goal.

### Compositor
- Built-in, not pluggable/external (no picom/compton dependency).
- Must support standard desktop-user compositor effects: window
  transparency, shadows, animations, and effects such as "wobbly windows".
- Must support the effect/theming layers XFCE4 splits across
  `compton.conf` (compositor), `xsettingsd.conf` (X settings), and GTK
  theme files — CASDE unifies these into one native config surface instead
  of shelling out to separate tools.

### Virtual desktops / workspaces
- Built-in virtual desktops with custom, user-defined naming (not just
  numbered). This is native to Xmonad in `desktopmgr`
  (`XMonad.Actions.DynamicWorkspaces` — add/remove/rename workspaces at
  runtime, not just a fixed named list); XFCE4's `workspace_names` is a
  static config-time list by comparison, not a dynamic feature. CASDE
  adopts the Xmonad-style dynamic naming/add/remove capability, not just
  XFCE4's static list.
- Default naming is inspired by, but not bound to, Xmonad's 9-workspace
  set (`dev www sys doc vbox chat mus vid gfx`) and XFCE4's 8 (`DEV WWW
  SYS DOC VBOX CHAT MEDIA GFX`) — each name below is chosen independently
  for what a power user actually needs, not by mirroring either source:

      dev, web, sys, office, virt, comm, audio, video, gfx

  - `dev` first — highest-frequency context for a power user, natural
    workspace-1 (`Super+1`) muscle memory
  - `web` second — daily-driver browsing, secondary to dev
  - `sys` — system monitoring/settings/admin tools, unambiguous, kept
  - `office` — documents/spreadsheets/presentations/PDFs; chosen over
    `docs` (ambiguous with dev/API docs) and `edit`/`text` (too generic,
    overlaps with `dev`)
  - `virt` — virtualization/containers in general, not just VirtualBox
  - `comm` — chat, mail, IRC — communication broadly, not just chat
  - `audio`/`video` — spelled out, kept as two separate workspaces
    rather than a single merged `media`
  - `gfx` — kept as-is, already accurate and short
- Support window-rule-based auto-assignment of apps to a workspace by
  window class/instance/title (the pattern every tiling WM in
  `desktopmgr` implements, e.g. BSPWM's `bspc rule -a Firefox desktop='^2'`
  and Xmonad's `doShift (myWorkspaces !! n)`).
- Support workspace wrap/cycle behavior (wrap on cycle, scroll-to-switch)
  as configurable options, matching XFCE4 defaults (`wrap_cycle`,
  `scroll_workspaces` both on).
- **Workspace membership model: multi-tag** (Awesome/DWM-style) — a
  window can belong to multiple named workspaces/tags simultaneously,
  rather than exactly one at a time. Chosen because it's a strict
  superset: a window assigned to exactly one tag behaves identically to
  XFCE4's one-workspace-per-window model, so single-tag usage is fully
  preserved as the simple default while multi-tag membership is available
  for users who want it. The 9-workspace default list above are the
  default tag names.

### Window management
- Support both floating (stacking) and tiling layouts, switchable per
  workspace or per window — deduplicating XFCE4/MATE/Openbox's floating
  model with the tiling models of i3/Sway/BSPWM/Awesome/LeftWM/Xmonad/
  Qtile/Herbstluftwm/Spectrwm.
- Window rules engine matching on window class, instance, title, and role,
  with actions: assign workspace, set floating/tiled/fullscreen, set
  geometry, mark sticky (visible on all workspaces), focus-on-spawn — this
  is the common capability every tiling WM in `desktopmgr` already has
  under different syntax.
- Scratchpad/dropdown window support (show/hide toggle for a bound
  instance, e.g. a dropdown terminal), matching the i3/Sway scratchpad and
  BSPWM floating-toggle patterns.
- Window cycling/switching (Alt-Tab equivalent) with preview, matching
  XFCE4's `cycle_preview` behavior.

### Keybindings
- Full custom keybinding support: global hotkeys plus focused-app-specific
  (per-window-class) bindings.
- Default keymap ported from the XFCE4 baseline
  (`xfce4-keyboard-shortcuts.xml`), using `Super` as the primary modifier:
  - `Super+Return` — open terminal
  - `Super+f` — open file manager
  - `Super+b` — open browser
  - `Super+e` — open editor
  - `Super+1`..`Super+9` — switch to workspace 1–9 (9 default workspaces)
  - `Super+Tab` — switch window
  - `Super+p` — display settings
  - `Super+s` — screenshot/screenkey action
  - `Shift+Super+Escape` — restart/kill window manager
- Bare function keys (no modifier) follow the same scoping most platforms
  (Windows, macOS, GNOME, KDE) already use, rather than being free for
  arbitrary app launchers — `Super+F*` combos above/below are a separate,
  unaffected namespace:
  - `F1` — help / show keybinding cheatsheet
  - `F2` — rename (selected file/item, e.g. in the file manager)
  - `F3` — find/search
  - `F5` — refresh/reload current view
  - `F11` — toggle fullscreen for the focused window
  - `F12` — toggle quake-style dropdown terminal (Guake/Yakuake
    convention)
  - `Alt+F4` — close focused window (kept alongside `Super+Shift+q`
    below, matching the near-universal platform convention)
- **App-vs-WM binding precedence** — the focused app gets first claim on a
  key combo; CASDE's own global binding fires only when the app doesn't
  claim it. How that's achieved differs by backend, since the two
  protocols don't offer the same guarantees:
  - **Wayland**: implement the `xdg-desktop-portal` **GlobalShortcuts**
    protocol. Apps register/claim the shortcuts they handle, and CASDE's
    compositor gives a focused app's registered claim precedence over its
    own global binding for that same combo — this is real negotiation,
    not a heuristic.
  - **X11**: there is no equivalent negotiation channel in the protocol —
    a key grab is either global (WM intercepts before the app ever sees
    the event) or local (the app's toolkit grabs it and the WM never
    sees it); there's no "app declined, fall through to WM" signal.
    The achievable equivalent is policy, not negotiation: keep all
    CASDE global bindings in the `Super` namespace, which app-native
    shortcuts (Ctrl/Alt combos like Chromium's `Ctrl+T`) essentially
    never claim, so the conflict doesn't arise in the first place. A
    literal same-key override on X11 (e.g. binding `Ctrl+T` globally)
    is a known limitation, not a supported guarantee — document it as
    such rather than promising portal-style negotiation on X11.
- Vim-style directional focus/move bindings (`Super+h/j/k/l` focus,
  `Super+Shift+h/j/k/l` move), common across i3/Sway/BSPWM/Awesome/Qtile/
  LeftWM, are adopted as the tiling-mode default and coexist with the
  XFCE4-style bindings above.
- Common cross-WM system bindings to keep: `Super+Shift+q` kill window,
  `Super+Shift+space` toggle floating/tiling, `Super+d` or
  `Super+Shift+Return` app launcher.
- No hotkey daemon dependency (no sxhkd) — keybinding handling is native
  to CASDE, not delegated to an external process.
- **Media/hardware key namespace** — `dfmgr`'s sxhkd configs bind a large
  set of `XF86*` hardware keys (volume up/down/mute, brightness up/down,
  audio play/pause/next/prev, screen-lock) plus dedicated screenshot keys
  (`Print`, `Shift+Print` region capture). These occupy a separate
  namespace from `Super+*` (dedicated laptop/keyboard keys, not modifier
  combos) and don't conflict with the app-vs-WM precedence question
  above, so they're adopted as CASDE defaults as-is: `XF86Audio*` keys
  control volume/media via the native volume/media widgets, `Print`/
  `Shift+Print` trigger the built-in screenshot utility (full-screen /
  region respectively). sxhkd's remaining launcher-layer bindings
  (Alt+key app shortcuts, USB-mount, torrent, mail-sync triggers) are
  not adopted as defaults — they're user-specific launcher shortcuts,
  not WM-level bindings, and remain available to configure manually via
  CASDE's own keybinding config rather than being baked in.

### Panel / taskbar / app menu
- Built-in app menu, taskbar, and app bar (panel) — all "plugin"
  functionality that other DEs ship as separate loadable plugins
  (whiskermenu/app-menu, tasklist, pager, systray, clock, notification
  area, launchers, weather, media-player controls, etc.) ships natively in
  CASDE instead. **No third-party plugin system is supported.**
- Default panel layout ported from the XFCE4 baseline
  (`xfce4-panel.xml`): app menu → task list → workspace pager (2 rows) →
  system tray → notification area → clock → power/session actions → show-
  desktop → app launchers → media-player widget.
- Panel supports XFCE4-style autohide, dark mode, and configurable
  icon/panel size as defaults; multiple panels (top bar + dock-style
  bottom launcher bar) are supported, matching the XFCE4 baseline's
  two-panel layout (thin top bar + larger icon dock).
- **Third-party status content, without a plugin system**: one narrow
  built-in widget type — a script-output widget — runs a user-specified
  command/script on a configurable interval and renders its text/icon
  output in the panel. This gets conky-style flexibility for users
  without reopening a general third-party plugin architecture; it's a
  single native widget type, not a plugin loader.
- **Built-in widget catalog** — surveyed across all 19 `desktopmgr`
  WM/DE configs plus `dfmgr`'s Polybar/tint2/Plank/Lemonbar bars (78+
  distinct plugins/widgets found, heavily duplicated across bar
  frameworks — e.g. CPU/memory/battery/weather/volume/network each
  reimplemented 4-6 times as a lain widget, a vicious widget, an
  i3blocks script, an xmobar command, and a Polybar module). Rather than
  porting each source's implementation, CASDE ships one native widget
  per category, deduplicated:
  - App menu, task list, workspace pager/tag switcher, system tray,
    notification area, clock, session/power actions, show-desktop
    toggle, app launcher buttons, separators (already covered above,
    XFCE4-ported).
  - **System monitor widgets** — CPU, memory, disk usage, battery/power
    status, temperature: one built-in widget family (text + optional
    graph), replacing the lain/vicious/i3blocks/xmobar/conky
    reimplementations.
  - **Network widget** — interface speed/signal-strength display,
    replacing the per-WM net/wifi widgets and `networkmanager-dmenu`.
  - **Volume widget** — output/input level display and control,
    replacing `volumeicon`/pulsebar/alsabar/i3blocks-volume.
  - **Weather widget** — location-based forecast display, replacing the
    per-WM weather widgets and conky weather blocks.
  - **Media/music widget** — now-playing display with play/pause/skip,
    covering the xfce4-mpc-plugin/cmus/MPD-widget/lain-MPD spread (MPRIS-
    based, so it works with any MPRIS-compliant player, not just MPD).
  - **Clipboard indicator** — panel entry point into the built-in
    clipboard manager (see System integration), replacing
    `xfce4-clipman-plugin`.
  - Explicitly **not** built in as native widgets (out of scope, no
    equivalent planned): package-update-count, mail/IMAP, RSS/news,
    torrent-status, moonphase, and redshift/color-temperature displays —
    niche enough that the script-output widget above covers them for
    users who want them.
  - `conky`, `Eww`, and standalone bar frameworks (Polybar/tint2/Plank/
    Lemonbar/i3bar/swaybar/xmobar/wibox) are not adopted or supported —
    CASDE's panel is native and self-contained, consistent with "no
    third-party plugin system" above.

### Config system
- YAML-based primary config file.
- Full-featured settings UI, comparable in scope to
  `xfce4-settings-manager` (display, keyboard/shortcuts, panel, appearance,
  workspaces, power, session, notifications) — not YAML-only editing.
- Settings UI writes back to the same YAML config the user can hand-edit;
  no separate binary-format config store.
- **Privilege elevation**: per-user settings (theme, keybindings, panel
  layout, workspace names) apply directly with no elevation. System-level
  changes that affect all users or persist beyond the current session
  (system-wide display configuration, power profiles) require elevation
  via polkit (or the BSD equivalent, since platform targets include BSD —
  needs a concrete BSD polkit-equivalent decision) before applying.

### Multi-monitor
Full, "just works like every other OS" multi-monitor support is required
for v1 — not deferred to a later milestone. Beyond XFCE4's baseline
(per-monitor wallpaper/resolution/position, already in `displays.xml`),
this includes:
- **Per-monitor workspace sets** — each monitor has its own independent
  active workspace/tag set (i3/Sway-style), rather than one global
  workspace switched across every monitor at once.
- **Hotplug handling** — connecting/disconnecting a monitor at runtime
  automatically reflows panels, wallpaper, and window placement with no
  session restart required.
- **Mixed refresh-rate / mixed-DPI support** — monitors with different
  resolutions, DPI/scaling, and refresh rates active simultaneously (the
  common laptop + external-display case) must render correctly on both
  backends.

### Accessibility
- Full accessibility support is in scope from day one, not deferred:
  - Screen reader support via AT-SPI (compatible with Orca and similar
    assistive tools).
  - Full keyboard-only navigation — every panel, menu, and settings UI
    action must be reachable without a pointing device.
  - High-contrast theme option, in addition to the default dark theme.
  - Configurable UI scaling (panel, settings UI, fonts) independent of
    per-monitor display scaling.

### System integration
- Built-in screenshot utility (region/window/full-screen capture),
  replacing tools like `xfce4-screenshooter`.
- Full clipboard manager: history persisted to disk across restarts,
  matching `xfce4-clipman`'s default, **except** entries an app flags as
  sensitive (e.g. the `x-kde-passwordManagerHint`/no-store clipboard
  mimetype hint that password managers already set) are excluded from
  persistence and from history display.
- Full PipeWire support and full PulseAudio support (both audio backends,
  not PipeWire-only with a Pulse shim assumed away).
- Notification daemon built in (replacing `xfce4-notifyd`/`dunst`),
  implementing the freedesktop **Desktop Notification Specification**
  (`org.freedesktop.Notifications` D-Bus interface) in full — this is
  the actual interface every `libnotify`-using app already calls, so
  existing programs send notifications with zero app-side changes.
- **`xdg-desktop-portal` implemented as a compat provider, not skipped**
  — CASDE ships its own `org.freedesktop.impl.portal.*` backend
  (Screenshot, Clipboard, ScreenCast, GlobalShortcuts, Settings) rather
  than omitting portal support, for two reasons that both matter
  independent of the trust-boundary decision below:
  - **Sandboxed apps (Flatpak) have no other route to the screen or
    clipboard at all** — the sandbox blocks direct X11/Wayland access
    by design, so without a portal backend these apps simply can't
    screenshot, read/write clipboard, or register shortcuts, period.
  - **Wayland screen-sharing/recording has no direct protocol for any
    app, sandboxed or not** — `ScreenCast` is the only way Zoom,
    Discord, browser tab-share, and OBS capture a Wayland screen at
    all. Skipping this portal would mean screen-sharing simply doesn't
    work on CASDE's Wayland backend for anyone.
  - See **Trust boundaries** below for how consent is handled — the
    portal *interfaces* are fully implemented; what CASDE changes is
    the consent-prompt behavior, not whether the interface exists.
- Session management: login/logout/lock/suspend/reboot/shutdown actions
  built into the panel, matching `xfce4-session`.
- Installs a session file so display/login managers (LightDM, GDM, SDDM,
  etc.) can offer CASDE as a selectable session, for both the X11 and
  Wayland backends.
- **Keyring / secret storage** — built in natively rather than depending
  on `gnome-keyring` or `kwalletd`, since package availability for
  either varies significantly across distros/platforms (some minimal
  installs and BSD packaging lack both). CASDE implements the
  freedesktop **Secret Service API spec in full** (the
  `org.freedesktop.Secret.Service`/`.Collection`/`.Item` D-Bus
  interfaces under the `org.freedesktop.secrets` bus name) — not a
  partial/best-effort subset — so this is a **compatibility layer for
  existing programs**: any app that talks to `libsecret` (VSCode's
  built-in secret storage, Chrome/Chromium, most Electron apps via
  `keytar`, browsers' saved-password stores, NetworkManager's saved
  wifi/VPN credentials, email/IMAP clients, SSH/GPG agents) works
  against CASDE's keyring with zero app-side changes — the app has no
  way to tell it isn't talking to GNOME Keyring or KWallet. Encrypted at
  rest with a key derived from the user's login password, auto-unlocking
  at login via PAM integration — no separate
  keyring prompt for the common case, matching GNOME Keyring's default
  behavior without depending on the GNOME Keyring package itself.

### Trust boundaries & security exceptions
- **Deliberate exception, reconciled with portal compat**: the portal
  *interfaces* (Screenshot, Clipboard, ScreenCast, GlobalShortcuts) are
  fully implemented (see System integration) so every app — sandboxed
  or not — has a working, spec-compliant path to the screen, clipboard,
  and shortcuts. What CASDE changes from the GNOME/KDE model is the
  **consent behavior**, not the interface's existence: CASDE's portal
  backend **auto-approves** these requests instead of showing a
  per-request consent dialog. Non-sandboxed apps also retain direct
  access outside the portal (X11/Wayland-native calls) as before. This
  is still a known divergence from the modern Wayland security model
  (GNOME/KDE gate these behind interactive consent dialogs) — the
  accepted cost is the same as previously decided: a malicious or buggy
  app can screenshot the desktop, read clipboard contents, capture the
  screen, or steal a global shortcut without the user being asked, since
  the portal never actually prompts. Reasoning: direct, frictionless
  access without depending on interactive consent flows, while still
  being fully standards-compliant for apps (including sandboxed ones)
  that only know how to ask via the portal. Mitigations to revisit later
  if this proves too permissive: per-app allow/deny lists in the
  settings UI, or a per-app/per-portal opt-in consent mode.

### Theming
- Default theme ported from the XFCE4 baseline: `Arc-Pink-Dark` (dark
  base, pink accent, `#CE6CA2`-family highlight color), sourced from
  `thememgr` where it's a single multi-target package covering GTK-2.0,
  GTK-3.0, Cinnamon, GNOME Shell, Metacity-1, Openbox-3, Xfwm4, Plank,
  and Unity from one theme definition — confirming the "unified theming"
  approach below is achievable from a single upstream source, not a
  novel requirement. Button layout follows the XFCE4/Arc convention
  (`menu:minimize,maximize,close`). This is CASDE's out-of-the-box
  default — not a bare/unthemed first run.
- Unified theming: one CASDE theme setting drives window decorations,
  panel, compositor accents, and GTK app theming together, replacing
  XFCE4's split across `xfwm4.xml` theme, `xsettings.xml` `ThemeName`, and
  GTK config files.
- **Bundled theme set** — 25 dark + 25 light themes ship built in,
  curated from widely-recognized community palettes (e.g. Nord,
  Dracula, Gruvbox, Solarized, Catppuccin, One Dark/Light, Everforest,
  Tokyo Night, Arc, Adwaita), each a visually distinct palette rather
  than accent-color variants of one base. Every bundled theme covers
  all of: GTK3/4, Qt5/6, window decorations, and panel accent — full
  unified coverage per theme, not a GTK-only subset. `Arc-Pink-Dark`
  (above) is one of the 25 dark entries, kept as the out-of-the-box
  default.
- **System theme exposure — no third-party dependency.** CASDE
  implements the theming/dark-light mechanism itself rather than
  depending on `lxappearance`, `xfce4-settings`, or GTK's own daemon,
  while still adhering to the XDG standards apps actually expect:
  - **`xdg-desktop-portal` Settings namespace**
    (`org.freedesktop.appearance`, `color-scheme`: no-preference /
    prefer-dark / prefer-light) — CASDE's session provides this as a
    native `org.freedesktop.impl.portal.Settings` D-Bus service (CASDE
    is the desktop, so it's the *provider* of this portal interface,
    not a consumer), covering GTK4/Qt6/Electron/Flatpak apps that query
    it directly. Applies on both backends.
  - **XSETTINGS protocol (X11)** — CASDE runs its own XSETTINGS manager
    (replacing `xfsettingsd`/`xsettingsd`), publishing `Net/ThemeName`,
    `Gtk/ColorScheme`, and `Xft/*` font/DPI settings for GTK2/GTK3/Qt5
    apps on X11 that read this channel instead of the portal.
  - **Generated legacy config files** — CASDE writes
    `~/.config/gtk-3.0/settings.ini`, `~/.config/gtk-4.0/settings.ini`,
    and a Qt platform-theme config (`qt5ct`/`qt6ct`-compatible ini, with
    `QT_QPA_PLATFORMTHEME` set appropriately) from the active CASDE
    theme, so apps that only read files on startup (not live-updating
    via portal or XSETTINGS) still honor the current theme and
    dark/light mode on next launch.
  - Net effect: one CASDE theme/mode toggle fans out to all three
    channels automatically; apps that support any of the three honor
    it, apps that don't just keep their own default — CASDE never
    forces theming onto an app that doesn't opt in, matching how these
    standards are meant to work.
- **Icon themes** — from `iconmgr`, two themes surveyed:
  - **N.I.B.** (default, matches the XFCE4 baseline's
    `xsettings.xml` `IconThemeName`) — ~1,382 scalable/symbolic SVG
    icons, follows the system accent/color scheme rather than being a
    fixed palette. Shipped as CASDE's default icon theme, pairing with
    `Arc-Pink-Dark`.
  - **Obsidian** — 26,500 icons across 33 fixed color variants
    (mixed PNG/SVG, multiple fixed sizes). Included as an optional
    alternate icon theme for users who want a fixed-palette look instead
    of the accent-following default.
- **Fonts** — from `fontmgr`, 11 font collections surveyed; CASDE adopts
  a small default set rather than bundling all 11:
  - **Hack Nerd Font** as the default UI/monospace/terminal font — it's
    the largest collection (38 variants) and the de-facto common choice
    across every `dfmgr` panel/bar config surveyed, so it's the safest
    default for icon-glyph compatibility with panel widgets that render
    Nerd Font glyphs (workspace/tag icons, widget icons).
  - **Noto Color Emoji** as the default emoji fallback, with the same
    "strong" fontconfig priority binding `fontmgr` uses (forces itself
    into the sans-serif/serif/monospace fallback chain ahead of any
    other emoji font) so emoji render consistently regardless of the
    active UI font.
  - Other collections (FiraCode, CascadiaCode, MesloLGSNF, AnonymousPro,
    agave, and the icon-only fonts: all-the-icons, awesome-terminal,
    Font Awesome, LigatureSymbols) are not bundled as defaults but
    remain installable as alternate choices in the settings UI's font
    picker, consistent with "no feature gating" — CASDE doesn't lock
    users into the default font.

### Autostart / session startup
- Deterministic autostart ordering, generalized from the common sequence
  observed across every `desktopmgr` config: stop any stale instances of
  CASDE's own daemons → apply session/display setup → start the
  compositor → start the panel → start the notification daemon → start
  the clipboard manager and system tray/applets (network, volume, power)
  → restore wallpaper → launch user-configured autostart apps.
- Autostart entries configurable per-user (which apps launch with the
  session), matching the `~/.config/<session>/startup` `.desktop` pattern
  used across `desktopmgr` configs (e.g. XFCE4's `clipman`, `notifyd`
  autostart entries).

### Open questions (need further discussion/research before implementation)
- Exact BSD variant(s) to target (OpenBSD only vs also FreeBSD/NetBSD)
  and the concrete BSD equivalents for `systemd-logind` session
  management and polkit privilege elevation.
- Phased implementation roadmap for the technology foundation (which
  backend/milestone comes first) given full Wayland/X11 effect parity
  and full multi-monitor support are both required for v1.
- Client/server (IPC) split — not yet designed.
