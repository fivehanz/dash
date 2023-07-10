### build binary
FROM rust:slim-bookworm AS binary-builder
WORKDIR /app
COPY . .
RUN cargo build --release

### runtime
FROM debian:bookworm-slim AS runtime
ENV APP_USER=admin 
ENV APP_DIR /runtime
RUN groupadd $APP_USER && useradd -g $APP_USER $APP_USER && mkdir /runtime

COPY --from=binary-builder /app/target/release/dash $APP_DIR/
COPY --from=binary-builder /app/dist $APP_DIR/dist
RUN chown -R $APP_USER:$APP_USER $APP_DIR

USER $APP_USER
WORKDIR $APP_DIR
EXPOSE 8080

ENTRYPOINT [ "./dash" ]