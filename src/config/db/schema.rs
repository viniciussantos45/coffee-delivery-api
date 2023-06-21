// @generated automatically by Diesel CLI.

diesel::table! {
    coffees (id) {
        id -> Uuid,
        coffee_name -> Text,
        additions -> Array<Nullable<Text>>,
        description -> Text,
        price -> Float8,
        image_path -> Text,
    }
}

diesel::table! {
    order_items (id) {
        id -> Text,
        order_id -> Text,
        coffee_id -> Uuid,
        quantity -> Int8,
        unit_price -> Float8,
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
        total_price -> Float8,
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
