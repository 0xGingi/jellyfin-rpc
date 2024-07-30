FROM rust:latest AS builder

RUN apt-get update && apt-get install -y git

RUN git clone https://github.com/Radiicall/jellyfin-rpc.git /app

WORKDIR /app

RUN export RUSTUP_TOOLCHAIN=stable && \
    cargo fetch --locked --target "$(uname -m)-unknown-linux-gnu"

RUN export RUSTUP_TOOLCHAIN=stable && \
    export CARGO_TARGET_DIR=target && \
    cargo build --frozen --release --all-features --no-default-features

FROM alpine:latest

RUN apk add --no-cache libc6-compat

COPY --from=builder /app/target/release/jellyfin-rpc /usr/local/bin/jellyfin-rpc

RUN mkdir -p /etc/jellyfin-rpc

ENTRYPOINT ["/usr/local/bin/jellyfin-rpc"]

CMD ["/etc/jellyfin-rpc/config.json"]