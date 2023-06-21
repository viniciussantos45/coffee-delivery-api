FROM rust:1.65

COPY . .

RUN cargo install --path .

EXPOSE 10000

CMD ["coffee-delivery-api"]
