# select build image
FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown

# create a new empty shell project
RUN USER=root cargo new --bin lignee
WORKDIR /lignee

# copy over your manifests
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml


# this build step will cache your dependencies

RUN echo 'fn main() { println!("dont see this"); }' > src/main.rs
RUN cargo build --target wasm32-unknown-unknown --release
RUN rm src/*.rs

# copy your source tree
COPY src ./src

# build for release
RUN rm ./target/wasm32-unknown-unknown/release/deps/*
RUN cargo build --target wasm32-unknown-unknown --release

# our final base
FROM nginx:stable-alpine

# copy the build artifact from the build stage
COPY --from=build /lignee/target/wasm32-unknown-unknown/release/deps/client_web.wasm /usr/share/nginx/html/wasm/

# copy the static file from static
COPY static/* /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]