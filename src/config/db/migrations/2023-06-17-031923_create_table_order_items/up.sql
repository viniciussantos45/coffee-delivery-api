-- Your SQL goes here
CREATE TABLE order_items(
    id TEXT PRIMARY KEY,
    order_id TEXT,
    coffee_id TEXT,
    quantity INTEGER,
    unit_price REAL,
    FOREIGN KEY(order_id) REFERENCES orders(id)
    FOREIGN KEY(coffee_id) REFERENCES coffees(id)
);