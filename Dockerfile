FROM rust:latest as builder

RUN mkdir /app

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .

ADD . .

RUN cargo build --release

FROM debian:latest
ARG APP=/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=fs

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /app/target/release/file-sharing ${APP}/fs

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["/app/fs"]
