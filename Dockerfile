FROM rust:latest

COPY . .

RUN cargo install --path .

EXPOSE 10000

CMD ["coffee-delivery-api"]
