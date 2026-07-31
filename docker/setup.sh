#!/bin/bash
set -e

echo "[tsdm-setup] waiting for Discuz X5 to be ready..."
for i in $(seq 1 30); do
    if curl -sf http://app:80/ > /dev/null 2>&1; then
        echo "[tsdm-setup] Discuz is ready."
        break
    fi
    echo "[tsdm-setup] waiting... ($i/30)"
    sleep 2
done

echo "[tsdm-setup] registering tsdm_newWing theme style..."
mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASSWORD" "$DB_NAME" < /theme.sql 2>/dev/null || true

for f in /migrations/from-x3/*.sql; do
    if [ -f "$f" ]; then
        echo "[tsdm-setup] applying migration: $(basename "$f")"
        mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASSWORD" "$DB_NAME" < "$f" 2>/dev/null || true
    fi
done

echo "[tsdm-setup] plugin installation complete."
