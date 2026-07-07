# syntax=docker/dockerfile:1
ARG BINARY_NAME_DEFAULT=kphis-backend
ARG BASE_IMAGE=clux/muslrust:stable

# Mapping ARM64 / AMD64 naming
FROM ${BASE_IMAGE} AS base-amd64
ENV DOCKER_TARGET_ARCH=x86_64
FROM ${BASE_IMAGE} AS base-arm64
ENV DOCKER_TARGET_ARCH=aarch64

FROM base-${TARGETARCH} AS builder
ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT
ENV RUSTFLAGS="--cfg unsound_local_offset"
RUN apt-get update && \
    apt-get install -yq tzdata && \
    ln -fs /usr/share/zoneinfo/Asia/Bankok /etc/localtime && \
    dpkg-reconfigure -f noninteractive tzdata
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser

# Build dummy main in order to cached dependencies and reused when the source code changes
COPY .cargo ./.cargo
COPY Cargo.toml ./
COPY crates/kphis-api-core ./crates/kphis-api-core
COPY crates/kphis-api-handler ./crates/kphis-api-handler
COPY crates/kphis-api-pacs ./crates/kphis-api-pacs
COPY crates/kphis-api-pdf ./crates/kphis-api-pdf
COPY crates/kphis-api-query ./crates/kphis-api-query
COPY crates/kphis-api-router ./crates/kphis-api-router
COPY crates/kphis-model ./crates/kphis-model
COPY crates/kphis-sql ./crates/kphis-sql
COPY crates/kphis-util ./crates/kphis-util
# needed by workspace
COPY crates/wasm-tests ./crates/wasm-tests
# needed by kphis-api-*
COPY crates/kphis-sqlx-tester ./crates/kphis-sqlx-tester
# needed by wasm-tests
COPY crates/kphis-ui-core ./crates/kphis-ui-core
# add only Cargo.toml
COPY crates/$BINARY_NAME/Cargo.toml ./crates/$BINARY_NAME/
COPY ENTITY ./ENTITY

# Build dummy
RUN mkdir crates/$BINARY_NAME/src \
    && echo "fn main() {print!(\"Dummy main\");}" > crates/$BINARY_NAME/src/main.rs
RUN set -x && cargo build --bin $BINARY_NAME --target $DOCKER_TARGET_ARCH-unknown-linux-musl --release
RUN ["/bin/bash", "-c", "set -x && rm target/$DOCKER_TARGET_ARCH-unknown-linux-musl/release/deps/${BINARY_NAME//-/_}*"]
RUN rm -rf ./crates/$BINARY_NAME/src

# Now add the rest of the project and build the real main
COPY crates/$BINARY_NAME/src ./crates/$BINARY_NAME/src
RUN set -x && cargo build --bin $BINARY_NAME --target $DOCKER_TARGET_ARCH-unknown-linux-musl --release
RUN mkdir -p /build-out
RUN set -x && cp target/$DOCKER_TARGET_ARCH-unknown-linux-musl/release/$BINARY_NAME /build-out/

# Create a minimal docker image
FROM scratch
COPY --from=builder /etc/passwd /etc/passwd
USER dockeruser
ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT
ENV TZ=Asia/Bangkok
COPY --from=builder /build-out/$BINARY_NAME /
COPY --from=builder /usr/share/zoneinfo /usr/share/zoneinfo
COPY --from=builder /usr/share/zoneinfo/Asia/Bangkok /etc/localtime
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
# Start with an execution list (there is no sh in a scratch image)
# No shell => no variable expansion, |, <, >, etc
# Hard coded start command
ENTRYPOINT ["/kphis-backend"]