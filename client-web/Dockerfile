# select build image
FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown

# create a new empty shell project
RUN USER=root cargo new --bin client-web
WORKDIR /client-web

# copy over your manifests
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

RUN rm src/*.rs

# copy your source tree
COPY src ./src

# this build step will cache your dependencies
RUN cargo build --target wasm32-unknown-unknown --release

# build for release
RUN rm ./target/wasm32-unknown-unknown/release/deps/*
RUN cargo build --target wasm32-unknown-unknown --release

# our final base
FROM nginx:stable-alpine

# copy the build artifact from the build stage
COPY --from=build /client-web/target/wasm32-unknown-unknown/release/deps/client_web.wasm /usr/share/nginx/html/wasm/

# copy the static file from static
COPY static/* /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]