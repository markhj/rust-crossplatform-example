FROM rust:latest
WORKDIR src
COPY . .
RUN cargo build --release
CMD ["./target/release/rust-crossplatform-example"]
