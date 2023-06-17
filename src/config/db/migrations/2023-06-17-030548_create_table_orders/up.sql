-- Your SQL goes here
CREATE TABLE orders(
    id TEXT PRIMARY KEY,
    user_id TEXT,
    street TEXT,
    number TEXT,
    neighborhood TEXT,
    city TEXT,
    state TEXT,
    zip_code TEXT,
    country TEXT,
    complement TEXT,

    payment_method TEXT,
    total_price REAL,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(user_id) REFERENCES users(id)
);