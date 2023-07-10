FROM rust:slim-bookworm AS binary-builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

COPY --from=binary-builder /app/target/release/dash /runtime/

CMD [ "./runtime/dash" ]