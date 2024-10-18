FROM rust:latest

WORKDIR /usr/src/app

RUN rustup component add clippy

COPY . .

RUN cargo build --release

CMD ["cargo","run"]