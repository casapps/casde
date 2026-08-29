# =============================================================================
# Build Stage - Compile casde (static musl binary)
# =============================================================================
FROM casjaysdev/rust:latest AS builder

ARG TARGETARCH
ARG BUILD_EPOCH
ARG COMMIT_ID

WORKDIR /work
COPY . .

RUN case "${TARGETARCH}" in \
      amd64) export CASDE_TARGET=x86_64-unknown-linux-musl ;; \
      arm64) export CASDE_TARGET=aarch64-unknown-linux-musl ;; \
      *) echo "unsupported TARGETARCH: ${TARGETARCH}" >&2; exit 1 ;; \
    esac \
    && rustup target add "${CASDE_TARGET}" \
    && GOFLAGS=-buildvcs=false BUILD_EPOCH="${BUILD_EPOCH}" COMMIT_ID="${COMMIT_ID}" \
       cargo build --release --target "${CASDE_TARGET}" \
    && cp "target/${CASDE_TARGET}/release/casde" /casde

# =============================================================================
# Runtime Stage - Minimal Alpine
# =============================================================================
FROM alpine:latest

ARG VERSION=dev
ARG BUILD_DATE
ARG VCS_REF
ARG LICENSE=MIT

RUN apk add --no-cache tini ca-certificates

COPY --from=builder /casde /usr/local/bin/casde
COPY docker/rootfs/ /
COPY docker/Dockerfile /usr/local/share/casde/Dockerfile

RUN chmod 755 /usr/local/bin/* \
    && chmod 755 /usr/local/bin/casde

STOPSIGNAL SIGTERM

HEALTHCHECK --start-period=10s --interval=1m --timeout=5s --retries=3 \
    CMD /usr/local/bin/casde --version || exit 1

ENTRYPOINT [ "tini", "-p", "SIGTERM", "--", "/usr/local/bin/entrypoint.sh" ]
CMD [ "casde" ]
