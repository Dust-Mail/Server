
ARG RUST_VERSION=1.65

FROM rust:${RUST_VERSION} as builder

WORKDIR /src

COPY . .

RUN cargo fetch --locked

RUN cargo build --release

FROM rust:${RUST_VERSION} as runner

WORKDIR /app

COPY --from=builder /src/target/release/dust-mail-server ./bin

RUN addgroup --gid 1001 rust 
RUN adduser --uid 1001 --gid 1001 dust-mail 

RUN mkdir -m 755 /config

RUN chown dust-mail /config

USER dust-mail

ENV CONFIG_LOCATION "/config"

CMD ["/app/bin"]