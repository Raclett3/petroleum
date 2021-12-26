use crate::bot::{CommandHandler, Context, IncomingMessage, MessageHandler};
use crate::models::{History, HistoryWindowConfig};
use crate::schema::{history::dsl as history, history_window_config::dsl as history_window_config};
use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;

pub struct HistoryWindow;

#[async_trait]
impl MessageHandler for HistoryWindow {
    async fn on_message(
        &mut self,
        (message_id, message): &IncomingMessage,
        context: &Context,
    ) -> Result<(), Box<dyn Error>> {
        let channel_id = message.channel_id;

        let messages_to_delete = {
            let conn = context.db_conn.lock().unwrap();
            let window_size = history_window_config::history_window_config
                .select(history_window_config::window_size)
                .find(channel_id as i64)
                .first::<i32>(&*conn)
                .optional()?;

            let window_size = if let Some(window_size) = window_size {
                window_size as i64
            } else {
                return Ok(());
            };

            diesel::insert_into(history::history)
                .values(History {
                    channel_id: channel_id as i64,
                    message_id: *message_id as i64,
                })
                .execute(&*conn)?;

            let messages_to_delete = history::history
                .filter(history::channel_id.eq(channel_id as i64))
                .order(history::message_id.desc())
                .offset(window_size)
                .load::<History>(&*conn)?;

            for message in &messages_to_delete {
                diesel::delete(history::history)
                    .filter(history::message_id.eq(message.message_id as i64))
                    .execute(&*conn)?;
            }

            messages_to_delete
        };

        for message in messages_to_delete {
            let _ = context
                .callbacks
                .delete_message(message.channel_id as u64, message.message_id as u64)
                .await;
        }

        Ok(())
    }
}

pub struct HistoryWindowConfigurator;

#[async_trait]
impl CommandHandler for HistoryWindowConfigurator {
    fn accepts(&self, command_name: &str) -> bool {
        command_name == "meslimit"
    }

    async fn handler(
        &mut self,
        args: &[&str],
        (_, message): &IncomingMessage,
        context: &Context,
    ) -> Result<(), Box<dyn Error>> {
        let reply = {
            let conn = context.db_conn.lock().unwrap();
            match args {
                &["enable", size] => {
                    if let Ok(size) = size.parse::<i32>() {
                        if (1..=10000).contains(&size) {
                            diesel::insert_into(history_window_config::history_window_config)
                                .values(HistoryWindowConfig {
                                    channel_id: message.channel_id as i64,
                                    window_size: size,
                                })
                                .on_conflict(history_window_config::channel_id)
                                .do_update()
                                .set(history_window_config::window_size.eq(size))
                                .execute(&*conn)?;
                            "有効化しました。"
                        } else {
                            "1以上10000以下の範囲で指定してください。"
                        }
                    } else {
                        "不正なパラメータです。"
                    }
                }
                &["disable"] => {
                    diesel::delete(history_window_config::history_window_config)
                        .filter(history_window_config::channel_id.eq(message.channel_id as i64))
                        .execute(&*conn)?;
                    "無効化しました。"
                }
                _ => "不正なコマンドです。",
            }
        };

        context.callbacks.send_message(message.reply(reply)).await
    }
}
