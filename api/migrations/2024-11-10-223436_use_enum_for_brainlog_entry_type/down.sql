ALTER TABLE brainlog_entry DROP COLUMN log_type;

CREATE TABLE brainlog_entry_type (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    name varchar(255) NOT NULL,
    description TEXT NOT NULL
);

ALTER TABLE brainlog_entry ADD type_id uuid references brainlog_entry_type(id) NOT NULL;
