FROM debian:bookworm-slim AS builder

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends clang curl libssl-dev build-essential pkg-config ca-certificates

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

WORKDIR /work
COPY . .

RUN cargo install sqlx-cli

ENV DATABASE_URL=sqlite:/verification.db
RUN cargo sqlx database create
RUN cargo sqlx migrate run
RUN cargo leptos build --release -vvv

FROM debian:bookworm-slim AS runner

WORKDIR /app

COPY --from=builder /work/target/x86_64-unknown-linux-gnu/release/kotiboksi /app/kotiboksi
COPY --from=builder /work/target/site /app/site

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_SITE_ROOT=./site
ENV DATABASE_URL=sqlite:guestbook.db
EXPOSE 3000

CMD ["/app/kotiboksi"]
