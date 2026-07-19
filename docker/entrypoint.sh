#!/bin/bash
set -e

DISCUZ_ROOT="/app/public"

if [ -d "/plugin" ] && [ -f "/plugin/discuz_plugin_pokemon.json" ]; then
    PLUGIN_DIR="$DISCUZ_ROOT/source/plugin/pokemon"
    if [ ! -d "$PLUGIN_DIR" ]; then
        echo "[tsdm] installing Pokemon plugin..."
        cp -r /plugin "$PLUGIN_DIR"
        chown -R www-data:www-data "$PLUGIN_DIR" 2>/dev/null || true
        echo "[tsdm] plugin installed."
    fi
fi

if [ -d "/template" ] && [ -f "/template/discuz_style_new_wing.json" ]; then
    TEMPLATE_DIR="$DISCUZ_ROOT/template/re_tsdm_newWing"
    if [ ! -d "$TEMPLATE_DIR" ]; then
        echo "[tsdm] installing TSDM New Wing template..."
        cp -r /template "$TEMPLATE_DIR"
        chown -R www-data:www-data "$TEMPLATE_DIR" 2>/dev/null || true
        echo "[tsdm] template installed."
    fi
fi

echo "[tsdm] starting..."
exec "$@"
