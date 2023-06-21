-- Your SQL goes here
CREATE TABLE order_items(
    id TEXT PRIMARY KEY NOT NULL,
    order_id TEXT NOT NULL,
    coffee_id UUID NOT NULL,
    quantity BIGINT NOT NULL,
    unit_price DOUBLE PRECISION NOT NULL,
    FOREIGN KEY(order_id) REFERENCES orders(id),
    FOREIGN KEY(coffee_id) REFERENCES coffees(id)
);