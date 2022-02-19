#!/bin/sh
# wait-for-postgres.sh

set -e

host="$1"
password="$2"
user="$3"
shift 3

echo "Pass: $password || psql -h $host -U $user -c '\q'"

until PGPASSWORD=$password psql -h "$host" -U $user -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up - executing command"
exec "$@"
