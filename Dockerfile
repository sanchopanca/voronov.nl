FROM rust:1.55

RUN USER=root cargo new --bin voronov
WORKDIR /voronov

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/voronov*
RUN cargo build --release

EXPOSE 3030
CMD ["./target/release/voronov"]
