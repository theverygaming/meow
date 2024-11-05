CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE brainlog_entry_type (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    name varchar(255) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE brainlog_entry (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    time timestamp without time zone NOT NULL,
    type_id uuid references brainlog_entry_type(id) NOT NULL,
    body TEXT NOT NULL
);
