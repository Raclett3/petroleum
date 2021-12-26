-- Your SQL goes here
CREATE TABLE history_window_config(
    channel_id BIGINT NOT NULL PRIMARY KEY,
    window_size INTEGER NOT NULL
);

CREATE TABLE history(
    message_id BIGINT NOT NULL PRIMARY KEY,
    channel_id BIGINT NOT NULL,
    FOREIGN KEY (channel_id) REFERENCES history_window_config ON DELETE CASCADE
);
