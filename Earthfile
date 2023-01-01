VERSION 0.6
FROM rustlang/rust:nightly-buster-slim
WORKDIR /rustexample

all:
    BUILD +build
    BUILD +check

install-chef:
   ENV CARGO_UNSTABLE_SPARSE_REGISTRY=true
   RUN cargo install --debug cargo-chef

prepare-cache:
    FROM +install-chef
    COPY --dir src Cargo.lock Cargo.toml .
    RUN cargo chef prepare
    SAVE ARTIFACT recipe.json

# Using cutoff-optimization to ensure cache hit (see examples/cutoff-optimization)
build-cache:
    FROM +install-chef
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook --release
    RUN cargo chef cook
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home

build:
    COPY --dir src Cargo.lock Cargo.toml .
    COPY +build-cache/cargo_home $CARGO_HOME
    ENV CARGO_UNSTABLE_SPARSE_REGISTRY=true
    RUN cargo build --release
	RUN strip target/release/highlight-assertions 
    SAVE ARTIFACT target/release/highlight-assertions AS LOCAL earthly-artifacts/highlight-assertions

check:
    COPY --dir src Cargo.lock Cargo.toml .
    COPY +build-cache/cargo_home $CARGO_HOME
    ENV CARGO_UNSTABLE_SPARSE_REGISTRY=true
    RUN cargo fmt --check
    RUN cargo clippy
#docker:
    #FROM debian:buster-slim
    #COPY +build/example-rust example-rust
    #EXPOSE 9091
    #ENTRYPOINT ["./highlight-assertions"]
    #SAVE IMAGE --push earthly/examples:rust
