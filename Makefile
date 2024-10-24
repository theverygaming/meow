.PHONY: all
all: precommit

.PHONY: typecheck
typecheck:
	mypy ./meow --strict

.PHONY: lint
lint:
	pylint ./meow --disable=missing-module-docstring

.PHONY: precommit
precommit: test typecheck lint

.PHONY: test
test:
	coverage run --source meow/ -m pytest -vv --tb=long tests/
	coverage html --omit="tests/*"
	coverage report -m --omit="tests/*"

.PHONY: postgrescontainer
postgrescontainer:
	docker run --rm -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres

.PHONY: psql
psql:
	PGPASSWORD="postgres" psql -d postgres -U postgres -h 127.0.0.1 $@

.PHONY: format
format: 
	black meow tests \
    --line-length 100 \
    --preview \
    --enable-unstable-feature string_processing
