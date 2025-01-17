# nightly 1.86
FROM rustlang/rust:nightly@sha256:1dc6ec0840b90688acfc3cbf5dc5c9297b630a4903837ac772c78ac1f5bb2584 AS base

RUN cargo install sccache --version ^0.7
RUN cargo install cargo-chef --version ^0.1

ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM base AS planner
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

FROM base AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release

FROM base AS runner
WORKDIR /app
COPY --from=builder /app/target/release/sms-forwarder-bot /app/sms-forwarder-bot

CMD [ "/app/sms-forwarder-bot" ]
