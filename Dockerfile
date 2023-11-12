FROM rust:1.73 as build

# create a new empty shell project
RUN USER=root cargo new --bin flashies
WORKDIR /flashies

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
COPY ./index.html ./index.html

# build for release
RUN rm ./target/release/deps/flashies*
RUN cargo build --release

ENTRYPOINT ["./target/release/flashies"]
