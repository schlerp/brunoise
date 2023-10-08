// @generated automatically by Diesel CLI.

diesel::table! {
    event_handlers (name) {
        name -> Text,
        slug -> Text,
        python_code -> Text,
    }
}
