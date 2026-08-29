# Makefile — thin wrappers around the Docker-wrapped cargo commands
# (AI.md PART 5 -> "Cargo Commands"). Never called from CI `run:` steps.

PROJECT_NAME ?= casde
PROJECT_ORG  ?= $(shell git remote get-url origin 2>/dev/null | sed -E 's#.*[:/]([^/]+)/[^/]+(\.git)?$$#\1#')
PROJECT_IMAGE ?= casjaysdev/rust:latest
COMPOSE_DEV := docker compose -f docker/docker-compose.dev.yml
COMPOSE_TEST := docker compose -f docker/docker-compose.test.yml

# casjaysdev/rust:latest's own host triple is musl (`rustc -vV` -> host:
# x86_64-unknown-linux-musl). Without an explicit --target, cargo has no
# host/target split and applies .cargo/config.toml's crt-static rustflags
# to proc-macro/build-script compilation too, which rustc rejects. Always
# pass --target explicitly so cargo separates host tool builds from the
# statically-linked target build (AI.md PART 5 "Build Rules" still governs
# the rustflags content; this only selects which target profile applies).
TARGET ?= x86_64-unknown-linux-musl

# Captured ONCE per build; every other time value derives from this
BUILD_EPOCH := $(shell date -u +%s)
# ISO 8601 UTC, derived from BUILD_EPOCH (docker build-arg / OCI label use only)
BUILD_DATE := $(shell date -u -d @$(BUILD_EPOCH) +"%Y-%m-%dT%H:%M:%SZ")
COMMIT_ID := $(shell git rev-parse HEAD 2>/dev/null || echo unknown)

.PHONY: fmt fmt-check lint test build build-release run-gui docker-build docker-test clean

fmt:
	$(COMPOSE_DEV) run --rm dev cargo fmt --all

fmt-check:
	$(COMPOSE_DEV) run --rm dev cargo fmt --all --check

lint:
	$(COMPOSE_DEV) run --rm dev cargo clippy --workspace --all-targets --all-features --target $(TARGET) -- -D warnings

test:
	$(COMPOSE_DEV) run --rm dev cargo test --workspace --all-features --target $(TARGET)

build:
	$(COMPOSE_DEV) run --rm dev cargo build --target $(TARGET)

build-release:
	$(COMPOSE_DEV) run --rm dev cargo build --release --target $(TARGET)

run-gui:
	$(COMPOSE_DEV) run --rm gui cargo run --target $(TARGET) -- --ui gui

docker-build:
	docker build -f docker/Dockerfile \
	  --build-arg BUILD_EPOCH=$(BUILD_EPOCH) \
	  --build-arg BUILD_DATE=$(BUILD_DATE) \
	  --build-arg COMMIT_ID=$(COMMIT_ID) \
	  -t $(PROJECT_NAME):latest .

docker-test:
	$(COMPOSE_TEST) run --rm test

clean:
	$(COMPOSE_DEV) run --rm dev cargo clean
