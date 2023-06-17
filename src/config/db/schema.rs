// @generated automatically by Diesel CLI.

diesel::table! {
    coffees (id) {
        id -> Text,
        coffee_name -> Text,
        image_path -> Text,
        price -> Double,
    }
}

diesel::table! {
    order_items (id) {
        id -> Text,
        order_id -> Text,
        coffee_id -> Text,
        quantity -> Integer,
        unit_price -> Double,
    }
}

diesel::table! {
    orders (id) {
        id -> Text,
        user_id -> Text,
        street -> Text,
        number -> Text,
        neighborhood -> Text,
        city -> Text,
        state -> Text,
        zip_code -> Text,
        country -> Text,
        complement -> Text,
        payment_method -> Text,
        total_price -> Double,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(order_items -> coffees (coffee_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(coffees, order_items, orders, users,);
