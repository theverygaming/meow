ALTER TABLE brainlog_entry DROP COLUMN type_id;
DROP TABLE brainlog_entry_type;

ALTER TABLE brainlog_entry ADD log_type varchar(255) NOT NULL;
