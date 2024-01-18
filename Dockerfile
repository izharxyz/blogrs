ARG RUST_VERSION=1.75.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/blogrs /

FROM debian:bookworm-slim AS final
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser
COPY --from=builder /blogrs /usr/local/bin
RUN chown appuser /usr/local/bin/blogrs
USER appuser
ENV RUST_LOG="blogrs=debug,info"
WORKDIR /app
ENTRYPOINT ["blogrs"]
EXPOSE 3000/tcp