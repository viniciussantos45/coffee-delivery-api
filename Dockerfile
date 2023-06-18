FROM rust:1.65

COPY . .

RUN cargo install diesel_cli --no-default-features --features sqlite

RUN diesel setup --database-url=$DATABASE_URL

EXPOSE 3333

CMD ["cargo", "run", "--release"]
