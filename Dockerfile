# Builder
FROM rust:latest as builder

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/talk-better

COPY Cargo.toml Cargo.toml

COPY Cargo.lock Cargo.lock

RUN mkdir src/

COPY ./src/ src/

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/talk-better*

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# Production
FROM alpine:latest as prod

RUN addgroup -g 1000 talk-better

RUN adduser -D -s /bin/sh -u 1000 -G talk-better talk-better

WORKDIR /home/talk-better/bin/

COPY --from=builder /usr/src/talk-better/target/x86_64-unknown-linux-musl/release/talk-better .

RUN chown talk-better:talk-better talk-better

USER talk-better

CMD ["./talk-better"]