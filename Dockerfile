FROM rust:1.67.0
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
CMD ["cargo", "run", "--release"]
