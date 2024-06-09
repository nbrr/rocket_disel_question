// @generated automatically by Diesel CLI.

diesel::table! {
    my_elements (id) {
        id -> Uuid,
        field -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(my_elements,);
