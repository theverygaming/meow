.PHONY: all
all:

.PHONY: postgrescontainer
postgrescontainer:
	docker run --rm -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres

.PHONY: psql
psql:
	PGPASSWORD="postgres" psql -d postgres -U postgres -h 127.0.0.1 $@
