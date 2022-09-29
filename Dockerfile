FROM rust:1.61 AS builder
COPY . /build
WORKDIR /build

RUN apt-get update \ 
  && DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends --assume-yes protobuf-compiler

ENV SQLX_OFFLINE=true

RUN cargo build --release --workspace \
  && mv /build/target/release/tyorka-admin /build/tyorka-admin \ 
  && rm -rf /build/target


FROM node:14-bullseye
LABEL org.opencontainers.image.source https://github.com/tyorka-shop/administration
COPY --from=builder /build/tyorka-admin /usr/local/bin/
EXPOSE 3000

CMD ["tyorka-admin"]