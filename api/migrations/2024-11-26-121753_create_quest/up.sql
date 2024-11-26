CREATE TABLE quest (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    name varchar(255) NOT NULL
);

CREATE TABLE quest_item (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    quest_id uuid references quest(id) NOT NULL,
    attributes jsonb NOT NULL DEFAULT '{}'::jsonb,
    name varchar(255) NOT NULL,
    body TEXT NOT NULL DEFAULT ''
);
