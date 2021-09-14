FROM rust

WORKDIR /source
COPY . ./

RUN ["cargo", "build"]

ENTRYPOINT ["/source/target/debug/rust-web-service"]