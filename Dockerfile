FROM rust:1.57 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

From debian:buster-slim
RUN apt-get update

COPY --from=builder /usr/local/cargo/bin/rusty-rumbuckle /usr/local/bin/rusty-rumbuckle

CMD ["rusty-rumbuckle"]
