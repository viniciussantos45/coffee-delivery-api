-- Your SQL goes here
CREATE TABLE order_items(
    id TEXT PRIMARY KEY NOT NULL,
    order_id TEXT NOT NULL,
    coffee_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price DECIMAL NOT NULL,
    FOREIGN KEY(order_id) REFERENCES orders(id)
    FOREIGN KEY(coffee_id) REFERENCES coffees(id)
);