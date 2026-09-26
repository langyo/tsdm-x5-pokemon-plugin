#!/bin/bash
set -e

# 首次启动引导脚本。所有步骤幂等：已就位的内容直接跳过。
#
# 设计约束：
# - podman 不会像 docker 那样把镜像内容隐式复制进空命名卷，因此
#   /app/public 的源码引导必须显式做（docker 下卷已有内容时同样跳过）。
# - Discuz 配置（config_global/config_ucenter/install.lock）由本脚本按
#   环境变量生成；数据库本体依赖 init.d 种子（docker-compose 的 db 服务），
#   本脚本不负责建库。

DISCUZ_ROOT="/app/public"
DB_HOST="${DB_HOST:-tsdm-db}"
DB_USER="${DB_USER:-root}"
DB_PASSWORD="${DB_PASSWORD:-root}"
DB_NAME="${DB_NAME:-discuz}"

# --- 1. 引导 Discuz 源码到 /app/public ---
if [ ! -f "$DISCUZ_ROOT/index.php" ] && [ -f /usr/src/discuz/index.php ]; then
    echo "[tsdm] bootstrapping Discuz source into /app/public..."
    cp -a /usr/src/discuz/. "$DISCUZ_ROOT/"
fi

# --- 2. 生成自签 TLS 证书（Caddyfile 固定引用该路径） ---
if [ ! -f /etc/caddy/certs/localhost.crt ]; then
    echo "[tsdm] generating self-signed TLS certificate..."
    mkdir -p /etc/caddy/certs
    openssl req -x509 -newkey rsa:2048 -nodes -days 3650 \
        -keyout /etc/caddy/certs/localhost.key \
        -out /etc/caddy/certs/localhost.crt \
        -subj "/CN=localhost" >/dev/null 2>&1
fi

# --- 3. 生成 Discuz 配置（仅首次） ---
if [ ! -f "$DISCUZ_ROOT/config/config_global.php" ]; then
    AUTHKEY="$(openssl rand -hex 24)"
    COOKIEPRE="$(openssl rand -hex 2)_"
    MEMPRE="$(openssl rand -hex 2)_"
    echo "[tsdm] provisioning Discuz config (db: $DB_HOST/$DB_NAME)..."

    sed -e "s|@@DB_HOST@@|$DB_HOST|g" \
        -e "s|@@DB_USER@@|$DB_USER|g" \
        -e "s|@@DB_PASSWORD@@|$DB_PASSWORD|g" \
        -e "s|@@DB_NAME@@|$DB_NAME|g" \
        -e "s|@@AUTHKEY@@|$AUTHKEY|g" \
        -e "s|@@COOKIEPRE@@|$COOKIEPRE|g" \
        -e "s|@@MEMPRE@@|$MEMPRE|g" \
        /docker/provision/config_global.php.tpl > "$DISCUZ_ROOT/config/config_global.php"

    cp /docker/provision/config_ucenter.php "$DISCUZ_ROOT/config/config_ucenter.php"

    # 数据库由 init.d 种子导入，无需安装向导；落锁避免跳转 /install/
    touch "$DISCUZ_ROOT/data/install.lock"
    echo "[tsdm] Discuz config provisioned."
fi

# --- 4. 安装插件与模板 ---
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
