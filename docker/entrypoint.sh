#!/bin/bash
set -e

echo "[tsdm] checking Discuz X5 installation..."
DISCUZ_ROOT="/app/public"

if [ ! -f "$DISCUZ_ROOT/index.php" ]; then
    echo "[tsdm] Discuz X5 not found, downloading..."
    curl -fsSL "https://download.discuz.vip/redirect/?X5.0" -o /tmp/discuz.zip
    unzip -q /tmp/discuz.zip -d "$DISCUZ_ROOT"
    rm /tmp/discuz.zip
    echo "[tsdm] Discuz X5 downloaded."
fi

echo "[tsdm] installing Pokemon plugin..."
PLUGIN_DIR="$DISCUZ_ROOT/source/plugin/pokemon"
if [ ! -d "$PLUGIN_DIR" ]; then
    cp -r /plugin "$PLUGIN_DIR"
    echo "[tsdm] plugin copied to $PLUGIN_DIR"
fi

echo "[tsdm] installing TSDM New Wing template..."
TEMPLATE_DIR="$DISCUZ_ROOT/template/re_tsdm_newWing"
if [ ! -d "$TEMPLATE_DIR" ]; then
    cp -r /template "$TEMPLATE_DIR"
    echo "[tsdm] template copied to $TEMPLATE_DIR"
fi

chown -R www-data:www-data "$DISCUZ_ROOT/source/plugin/pokemon" 2>/dev/null || true
chown -R www-data:www-data "$DISCUZ_ROOT/template/re_tsdm_newWing" 2>/dev/null || true

echo "[tsdm] starting FrankenPHP..."
exec "$@"
