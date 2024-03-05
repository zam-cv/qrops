// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Integer,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 150]
        password -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 150]
        password -> Varchar,
        balance_cash -> Integer,
        balance_verqor -> Integer,
        balance_coyote -> Integer,
        current_day -> Timestamp,
        max_sections -> Integer,
    }
}

diesel::table! {
    statistics (id) {
        user_id -> Integer,
        id -> Integer,
        date -> Timestamp,
        punctuation -> Integer,
    }
}

diesel::table! {
    crop_types (id) {
        id -> Integer,
        #[max_length = 50]
        name -> Varchar,
        price -> Integer,
    }
}

diesel::table! {
    crop_sections (id) {
        id -> Integer,
        user_id -> Integer,
        crop_type_id -> Nullable<Integer>,
        units -> Integer,
    }
}

diesel::joinable!(crop_sections -> users (user_id));
diesel::joinable!(crop_sections -> crop_types (crop_type_id));
diesel::joinable!(statistics -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    crop_sections,
    crop_types,
    statistics,
    users,
);
