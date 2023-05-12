-- Add migration script here
ALTER TABLE tw_queries
ADD COLUMN open_date  datetime AFTER diff_percentage,
ADD COLUMN open_value decimal(20, 8) AFTER open_date,
ADD COLUMN close_date datetime AFTER open_value,
ADD COLUMN close_value decimal(20, 8) AFTER close_date
;
