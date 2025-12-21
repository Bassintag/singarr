#!/bin/sh

set -e

DB_PATH="/data/db.sqlite"
MIGRATIONS_DIR="/app/migrations"

echo "▶ init sqlite db"

mkdir -p "$(dirname "$DB_PATH")"

sqlite3 "$DB_PATH" <<'EOF'
CREATE TABLE IF NOT EXISTS _migrations (
  id TEXT PRIMARY KEY,
  applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
EOF

for file in "$MIGRATIONS_DIR"/*.sql; do
  [ -f "$file" ] || continue
  id="$(basename "$file")"

  applied=$(sqlite3 "$DB_PATH" \
    "SELECT 1 FROM _migrations WHERE id = '$id' LIMIT 1;")

  if [ -z "$applied" ]; then
    echo "▶ applying migration $id"
    sqlite3 "$DB_PATH" < "$file"
    sqlite3 "$DB_PATH" \
      "INSERT INTO _migrations (id) VALUES ('$id');"
  else
    echo "✓ migration $id already applied"
  fi
done

echo "▶ starting api"
singarr-api &

echo "▶ starting nginx"
exec nginx -g "daemon off;"