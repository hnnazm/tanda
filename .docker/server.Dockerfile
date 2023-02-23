FROM rust:1.67

WORKDIR /server

COPY ../Cargo.toml /server

RUN cargo install cargo-watch

COPY ../ /server
