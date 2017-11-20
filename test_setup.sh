#!/bin/bash

echo "DATABASE_URL=postgres://postgres@localhost/entropyicons" > .env
source ./.env

cargo install diesel_cli
cargo install clippy --force

diesel database reset
if [ -f "./db_setup.sql" ]; then
    psql -U postgres entropyicons < db_setup.sql
fi
