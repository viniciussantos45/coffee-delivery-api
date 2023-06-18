FROM rust:1.65

RUN cargo install diesel_cli --no-default-features --features sqlite

COPY . .

RUN diesel setup --database-url=$DATABASE_URL

RUN cargo install --path .

EXPOSE 443

CMD ["coffee-delivery-api"]
