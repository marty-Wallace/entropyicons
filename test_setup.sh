#!/bin/bash

echo "DATABASE_URL=postgres://postgres@localhost/rust_360" > .env
source ./.env
cargo install diesel_cli
cargo install clippy
diesel database reset
if [ -f "./db_setup" ]; then
    psql -U postgres rust_360 < db_setup
fi