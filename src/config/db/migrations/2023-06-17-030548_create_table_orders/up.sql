-- Your SQL goes here
CREATE TABLE orders(
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    street TEXT NOT NULL,
    number TEXT NOT NULL,
    neighborhood TEXT NOT NULL,
    city TEXT NOT NULL,
    state TEXT NOT NULL,
    zip_code TEXT NOT NULL,
    country TEXT NOT NULL,
    complement TEXT NOT NULL,

    payment_method TEXT NOT NULL,
    total_price DOUBLE PRECISION NOT NULL,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);