FROM ekidd/rust-musl-builder:nightly-2020-06-23 AS builder

RUN sudo chown -R rust:rust /home

RUN USER=root cargo new /home/sse --bin
WORKDIR /home/sse
## Caching dependencies
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml
RUN cargo build --bins --release --target x86_64-unknown-linux-musl
RUN rm src/*.rs

COPY src ./src
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/sse*
RUN cargo build --bins --release --target x86_64-unknown-linux-musl

FROM alpine:latest
COPY --from=builder /home/sse/target/x86_64-unknown-linux-musl/release/sse /usr/local/bin/
EXPOSE 8000
CMD /usr/local/bin/sse
