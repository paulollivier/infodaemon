FROM rust:latest as builder
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/infodeamon
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
RUN addgroup -g 1000 infodaemon
RUN adduser -D -s /bin/sh -u 1000 -G infodaemon infodaemon
COPY --from=builder /usr/src/infodaemon/target/x86_64-unknown-linux-musl/release/infodaemon /usr/local/bin/infodaemon
RUN chown infodaemon:infodaemon /usr/local/bin/infodaemon
USER infodaemon
EXPOSE 3030
CMD ["infodaemon"]