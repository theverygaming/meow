CREATE TABLE finance_account (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    name varchar(255) NOT NULL,
    type integer NOT NULL,
    initial_balance double precision NOT NULL,
    opening_time timestamp without time zone NOT NULL
);

CREATE TABLE finance_transaction (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    name varchar(255) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE finance_transaction_item (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    transaction_id uuid references finance_transaction(id) NOT NULL,
    description TEXT NOT NULL,
    account_id uuid references finance_account(id) NOT NULL,
    debit double precision NOT NULL,
    credit double precision NOT NULL
);
