# Docker

CASDE never builds or runs on the host toolchain (AI.md PART 0 -> "No Host
Toolchain or Binary Execution"). Every `cargo` command and the compiled
binary itself run inside Docker.

## Build the release image

```bash
BUILD_EPOCH="$(date -u +%s)"
BUILD_DATE="$(date -u -d "@${BUILD_EPOCH}" +%Y-%m-%dT%H:%M:%SZ)"
docker build -f docker/Dockerfile \
  --build-arg BUILD_EPOCH="$BUILD_EPOCH" \
  --build-arg BUILD_DATE="$BUILD_DATE" \
  -t casde:latest .
```

## Toolchain shell (fmt/clippy/test/build)

```bash
docker compose -f docker/docker-compose.dev.yml run --rm dev cargo fmt --all --check
docker compose -f docker/docker-compose.dev.yml run --rm dev cargo clippy --workspace --all-targets --all-features -- -D warnings
docker compose -f docker/docker-compose.dev.yml run --rm dev cargo test --workspace --all-features
docker compose -f docker/docker-compose.dev.yml run --rm dev cargo build --release
```

## Run the GUI with display forwarding

X11:

```bash
xhost +SI:localuser:$(id -un)
docker compose -f docker/docker-compose.dev.yml run --rm gui cargo run -- --ui gui
xhost -SI:localuser:$(id -un)
```

Wayland: same command — `docker-compose.dev.yml`'s `gui` service forwards
both `DISPLAY` and `WAYLAND_DISPLAY`; the app picks Wayland when
`WAYLAND_DISPLAY` is set (AI.md PART 3 -> "Backend selection").

## Automated tests

```bash
docker compose -f docker/docker-compose.test.yml run --rm test
```

`tests/` scripts are the preferred interface for this compose file; avoid
invoking it directly outside of automation.
