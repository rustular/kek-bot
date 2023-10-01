FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo fetch
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --workspace --recipe-path recipe.json
COPY . .
RUN cargo prisma 
RUN cargo prisma generate && cargo prisma db push
RUN cargo build --release --bin kek-bot

FROM ubuntu:latest AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/prisma-cli /usr/local/bin/prisma-cli
COPY --from=builder /app/target/release/kek-bot /usr/local/bin/kek-bot 
COPY --from=builder /app/migrate-and-start.sh /usr/local/bin/migrate-and-start.sh
CMD ["sh","/usr/local/bin/migrate-and-start.sh"]
