FROM node:22-alpine AS portal-builder

WORKDIR /build/portal

COPY portal/package.json portal/pnpm-lock.yaml ./
RUN corepack enable && pnpm install --frozen-lockfile

COPY portal/ ./
RUN pnpm build

FROM ghcr.io/rust-cross/cargo-zigbuild:latest AS builder
ARG VERSION
ENV VERSION=${VERSION}

RUN apt-get update && apt-get install -y \
    build-essential \
    wget \
    pkg-config \
    ca-certificates \
    flex \
    bison \
 && rm -rf /var/lib/apt/lists/*


WORKDIR /tmp


ENV PKG_CONFIG_ALL_STATIC=1

SHELL ["/bin/bash", "-c"]

WORKDIR /build

COPY Cargo.toml Cargo.toml

RUN mkdir src && echo "fn main(){}" > src/main.rs

ENV ac_cv_func_malloc_0_nonnull=yes

ENV PKG_CONFIG_SYSROOT_DIR=/


RUN cargo zigbuild --release --target x86_64-unknown-linux-musl || true


COPY ./src ./src
COPY ./build.rs ./build.rs
COPY --from=portal-builder /build/portal/build ./portal/build

RUN mkdir /out


RUN cargo zigbuild --release --target x86_64-unknown-linux-musl \
 && cp target/x86_64-unknown-linux-musl/release/dockless /out/dockless-${VERSION}-linux-amd64


RUN rustup target add aarch64-unknown-linux-musl
RUN cargo zigbuild --release --target aarch64-unknown-linux-musl \
&& cp target/aarch64-unknown-linux-musl/release/dockless /out/dockless-${VERSION}-linux-arm64


RUN rustup target add armv7-unknown-linux-musleabihf
RUN cargo zigbuild --release --target armv7-unknown-linux-musleabihf \
&& cp target/armv7-unknown-linux-musleabihf/release/dockless /out/dockless-${VERSION}-linux-armv7


FROM scratch

COPY --from=builder /out /