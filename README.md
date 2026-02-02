# Terminal 1 - to run the server.

cargo watch -q -c -w src/ -w .cargo/ -x run

# Terminal 2 - to run the quick_dev.

cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

# Start the Postgres server docker image:

docker run --rm --name pg -p 5432:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=welcome postgres:15

# (Optional) To have a psql terminal on pg.

# In another terminal (tab) run psql:

docker exec -it -u postgres pg psql

# Install the SQLx command-line tool, but only with PostgreSQL support and native TLS—nothing extra.

cargo install sqlx-cli --no-default-features --features native-tls,postgres
sqlx database create

# Docker command to create a postgres DB

docker exec -it pg psql -U postgres -c "CREATE DATABASE axum_auth;"

# Docker command to check if the postgres user and database exist:

docker exec -it pg psql -U postgres -l

# Postgres migration command with sqlx

sqlx migrate add -r users

# Execute migration with sqlx

sqlx migrate run
