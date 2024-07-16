FROM rust:latest

WORKDIR /app/

COPY . .

RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

EXPOSE 8000

CMD ["cargo", "run", "--release", "--", "0.0.0.0:8000"]