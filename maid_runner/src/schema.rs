// @generated automatically by Diesel CLI.

diesel::table! {
    logs (id) {
        id -> Integer,
        session -> Text,
        session_description -> Text,
        source_from -> Text,
        source_command -> Text,
        source_detail -> Text,
        source_description -> Text,
        timestemp -> Text,
        returned_status -> Text,
        formated_stdout -> Text,
        formated_stderr -> Text,
        debug -> Text,
    }
}
