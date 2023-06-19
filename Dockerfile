FROM rust:1.65

RUN cargo install diesel_cli --no-default-features --features sqlite

COPY . .

RUN diesel setup --database-url="sqlite://db.sqlite3"

RUN cargo install --path .

EXPOSE 10000

CMD ["coffee-delivery-api"]
