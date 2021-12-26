table! {
    history (message_id) {
        message_id -> Int8,
        channel_id -> Int8,
    }
}

table! {
    history_window_config (channel_id) {
        channel_id -> Int8,
        window_size -> Int4,
    }
}

joinable!(history -> history_window_config (channel_id));

allow_tables_to_appear_in_same_query!(
    history,
    history_window_config,
);
