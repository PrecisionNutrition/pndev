#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
  CREATE USER es WITH PASSWORD 'es';
  ALTER USER es WITH SUPERUSER;
EOSQL
