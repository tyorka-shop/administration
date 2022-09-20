FROM rust:1.61 AS builder
COPY . /build
WORKDIR /build

RUN apt-get update \ 
  && DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends --assume-yes protobuf-compiler

ENV SQLX_OFFLINE=true

RUN cargo build --release --workspace \
  && mv /build/target/release/tyorka-admin /build/tyorka-admin \ 
  && mv /build/target/release/insta_sync /build/insta_sync \ 
  && rm -rf /build/target


FROM node:14-bullseye
COPY --from=builder /build/tyorka-admin /build/insta_sync /usr/local/bin/

EXPOSE 3000

CMD ["tyorka-admin"]