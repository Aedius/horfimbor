# select build image
FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin lignee2
WORKDIR /lignee2

# copy over your manifests
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY src ./src

# build for release
RUN rm ./target/release/deps/lignee2*
RUN cargo build --release

# our final base
FROM gcr.io/distroless/cc

# copy the build artifact from the build stage
COPY --from=build /lignee2/target/release/lignee2 .

# set the startup command to run your binary
CMD ["./lignee2"]