-- Add migration script here
CREATE TABLE IF NOT EXISTS tw_queries
(
    id                  BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    symbol              VARCHAR(64)    NOT NULL,
    start_period        datetime       NOT NULL,
    end_period          datetime       NOT NULL,
    diff_percentage     decimal(20, 8) NOT NULL,
    created_at          datetime default CURRENT_TIMESTAMP NOT NULL
);