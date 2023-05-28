
ARG RUST_VERSION=1.65

FROM rust:${RUST_VERSION} as builder

WORKDIR /src

COPY . .

RUN cargo fetch --locked

RUN cargo build  --release

FROM debian:buster-slim as runner

RUN apt update -y

RUN apt install libssl-dev -y 

COPY --from=builder /src/target/release/dust-mail-server /usr/local/bin

RUN addgroup --gid 1000 rust 
RUN adduser --uid 1000 --gid 1000 dust-mail 

RUN install -d -m 755 -o dust-mail -g rust /config

USER dust-mail

ENV CONFIG_LOCATION "/config"

EXPOSE 8080

CMD ["/usr/local/bin/dust-mail-server"]