FROM rust:1.86-slim AS builder

WORKDIR /app
COPY brasileirao-api ./brasileirao-api
WORKDIR /app/brasileirao-api
RUN cargo build --release

FROM debian:stable-slim
COPY --from=builder /app/brasileirao-api/target/release/brasileirao-api /usr/local/bin/brasileirao-api
COPY run.sh /run.sh

RUN chmod +x /run.sh

CMD [ "/run.sh" ]