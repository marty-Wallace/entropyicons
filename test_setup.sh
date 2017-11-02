#!/bin/bash

echo "DATABASE_URL=postgres://postgres@localhost/rust_360" > .env
source ./.env
cargo install diesel_cli
if [[ "$1" == nightly* ]]; then
    cargo install clippy
fi
diesel database reset
if [ -f "./db_setup" ]; then
    psql -U postgres rust_360 < db_setup
fi
