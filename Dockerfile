FROM rust:1.69.0
WORKDIR /usr/src/app
COPY . .
RUN SQLX_OFFLINE=true cargo build --release
CMD ["cargo", "run", "--release"]
