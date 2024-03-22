FROM rust:latest

WORKDIR /usr/src/backend
COPY . .
EXPOSE 8765

RUN cargo build --release

CMD ["cargo", "run"]