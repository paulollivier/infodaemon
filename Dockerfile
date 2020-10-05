FROM rust:latest as builder
WORKDIR /usr/src/infodaemon
COPY Cargo.toml Cargo.toml
COPY src src
RUN cargo install --path .

FROM debian:buster-slim
# this fails. Why?
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/infodaemon /usr/local/bin/infodaemon
EXPOSE 3030
CMD ["infodaemon"]