use crate::schema::{history, history_window_config};

#[derive(Queryable, Insertable)]
#[table_name = "history"]
pub struct History {
    pub message_id: i64,
    pub channel_id: i64,
}

#[derive(Queryable, Insertable)]
#[table_name = "history_window_config"]
pub struct HistoryWindowConfig {
    pub channel_id: i64,
    pub window_size: i32,
}
