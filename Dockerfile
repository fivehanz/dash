### build binary
FROM rust:1.67.1-slim-bookworm AS binary-builder

WORKDIR /app
COPY . .
RUN cargo build --release

### runtime
FROM debian:bookworm-slim AS runtime

ENV APP_USER admin 
ENV APP_DIR /runtime

ENV APP_PORT 8080
ENV APP_NAME dash
## DEBUG, DEV, PROD
ENV APP_MODE DEV


RUN groupadd $APP_USER && useradd -g $APP_USER $APP_USER && mkdir $APP_DIR
COPY --from=binary-builder /app/target/release/dash $APP_DIR/
COPY --from=binary-builder /app/dist $APP_DIR/dist
RUN chown -R $APP_USER:$APP_USER $APP_DIR

USER $APP_USER
WORKDIR $APP_DIR
EXPOSE $APP_PORT

ENTRYPOINT [ "./dash" ]
