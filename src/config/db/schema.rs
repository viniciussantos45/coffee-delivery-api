// @generated automatically by Diesel CLI.

diesel::table! {
    coffees (id) {
        id -> Text,
        coffee_name -> Text,
        image_path -> Text,
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

diesel::allow_tables_to_appear_in_same_query!(coffees, users,);
