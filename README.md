# TSDM Pokemon Plugin for Discuz X5

天使动漫论坛宠物小精灵插件 — 从 Discuz X3 迁移至 X5，Docker 打包部署。

## Quick Start

```bash
# 安装 deps (仅首次)
pip install podman-compose

# 启动 Podman machine (Windows/macOS)
podman machine start

# 一键启动
python scripts/docker/dev.py up
```

打开浏览器访问:

| URL | 说明 |
|---|---|
| `https://localhost:8443/` | 论坛首页 (HTTPS，自签证书) |
| `http://localhost:8080/` | 自动重定向至 HTTPS |

首次访问需接受自签证书警告（浏览器点「高级 → 继续前往」）。

默认管理员: `admin` / `admin123`

首次启动时 `entrypoint.sh` 会自动完成：从镜像内置源码引导 Discuz、按
`DB_*` 环境变量生成 `config/config_global.php` 与独立模式
`config/config_ucenter.php`、落 `data/install.lock`、生成自签 TLS 证书
（写入 `caddy_certs` 卷，不进仓库）、安装插件与模板。数据库结构及种子
由 `docker/init.d/` 在 db 卷首次初始化时导入。

## Troubleshooting（Windows + Podman/WSL）

- **起栈报 netavark/nftables 错误**（`nft did not return successfully`）：
  WSL 内核不支持 netavark 生成的 nft 规则。执行
  `podman machine ssh "sudo mkdir -p /etc/containers && printf '[network]\nfirewall_driver = \"iptables\"\n' | sudo tee -a /etc/containers/containers.conf"`
  后 `podman machine stop && podman machine start` 再重试。
- **容器内 bash/Caddy 报 `\r` 或语法错误**：检出时行尾被转成 CRLF，
  仓库已通过 `.gitattributes` 强制 `*.sh`/`Caddyfile`/`justfile` 保持
  LF；旧检出需重新 checkout 或手动转换行尾。
- **端口占用**：用 `APP_HTTP_PORT=8080 APP_HTTPS_PORT=8443
  python scripts/docker/dev.py up` 换端口。

## Just Recipes

```bash
just install       # 安装 celestia-devtools (仅首次)
just fmt           # 格式化 Markdown + Rust
just fmt-check     # 检查格式（不写入）
just lint          # clippy 检查 Rust 代码
just test          # 运行 Rust 测试
just test-php      # 静态 PHP 测试（schema 一致性 + php -l）
just build         # 构建 Rust (本机目标)
just build-wasm    # 重新构建 admin/game 的 WASM 前端到 plugin/wasm
just publish       # 打包为 X5 可安装的插件 zip (dist/tsdm-pokemon-<version>.zip)
just publish-verify <zip>  # 校验已生成的插件包结构
just up            # 启动 Docker/Podman 开发环境
just down          # 停止
just restart       # 重启
just logs          # 查看日志
```

## Publishing

`just publish` 会依次：手动构建 WASM 前端（`cargo build --target wasm32-unknown-unknown` +
`wasm-bindgen-cli`，无需 wasm-pack），整合 DB 迁移脚本，剔除开发期文件后组装插件目录，
生成安装包，然后自动 bump patch 版本号、打 git tag 并推送：

```
dist/
└── tsdm-pokemon-<version>.zip
    └── pokemon/                  # 插件目录（与 discuz_plugin_pokemon.json 的 directory 一致）
        ├── discuz_plugin_pokemon.json
        ├── install.php / uninstall.php
        ├── admincp.inc.php / game.inc.php
        ├── api/ admin/ table/ i18n/ images/
        ├── migrations/           # X3→X5 迁移脚本（供老站升级用）
        └── wasm/                 # 运行时 WASM 资产 + snippets
```

版本号由根目录 `VERSION` 文件管理（格式 `MAJOR.MINOR.PATCH`）。每次 `just publish` 后
patch 自增，同时更新 `rust/admin/Cargo.toml` 和 `rust/game/Cargo.toml`，提交
`🔖 Bump version to X.Y.Z.` 并打 tag `vX.Y.Z` 推送。注意 zip 文件名用的是 bump 之前的
版本号（打包在前、bump 在后），发布时以 git tag 为准。

可用标志：`--no-bump`（跳过版本 bump）、`--skip-build`（跳过 WASM 重建）、
`--with-theme`（额外打包主题）、`--bump-only`（仅 bump+tag 不构建）、
`--clean`（打包前清空 `dist/`——注意这会连带删掉手工放在 dist 里的说明文件）。

在 X5 论坛安装：

1. 解压 zip，将 `pokemon/` 文件夹上传到 `source/plugin/pokemon`
2. 后台 → 应用 → 插件，找到「TSDM 宠物小精灵」，点击安装（执行 `install.php`）
3. 启用插件并确认 `api/index.php` 可被 Caddy/Nginx 访问（伪静态需放行）

插件自带头像接口（`endpoint=avatar`）与徽章图片接口（`endpoint=badge`），
不依赖 X5 已移除的 `uc_server`，覆盖插件目录即可用，无需改动论坛本体文件。

主题包（可选）：`just publish --with-theme` 额外生成 `dist/re_tsdm_newWing-<version>.zip`，
解压到 `template/` 后在后台上传 `discuz_style_new_wing.json` 导入样式。

## Project Structure

```
├── docker/                   Docker 配置
│   ├── docker-compose.yml    编排文件 (app + db + setup)
│   ├── Dockerfile            生产镜像 (FROM izumiko/discuz)
│   ├── entrypoint.sh         容器入口（自动安装插件+模板）
│   ├── config/Caddyfile      Caddy 服务器配置 (HTTPS + 伪静态)
│   └── init.d/               MariaDB 初始化 SQL (预置库)
├── plugin/                   插件主体
│   ├── api/                  PHP API 端点
│   ├── admin/                管理后台路由
│   ├── table/                X5 数据表类
│   ├── i18n/                 语言包 (SC_UTF8 / TC_UTF8)
│   └── images/               精灵/道具/地图素材
├── rust/                     Rust 前端 (Dioxus WASM)
│   ├── admin/                管理后台
│   ├── game/                 游戏前端
│   └── utils/                共享类型与工具
├── template/                 tsdm_newWing 模板
├── migrations/               X3 → X5 数据库迁移脚本
└── scripts/                  开发脚本
    └── docker/               Docker/Podman 编排 (Python)
```

## Database Migration

迁移脚本位于 `migrations/from-x3/`：

1. `001_x3_to_x5_migration.sql` — 引擎升级 MyISAM→InnoDB，字符集 utf8mb3→utf8mb4，
   数据表结构归一（含后续重整，脚本可重复执行）

```bash
# 生产环境手动执行
mysql -u discuz -p discuz < migrations/from-x3/001_x3_to_x5_migration.sql
```

## Commit Convention

遵循 [gitmoji](https://gitmoji.dev) 规范:

```
<gitmoji> <Capitalized English summary.>
```

详见 [CONTRIBUTING.md](CONTRIBUTING.md)。

## License

Proprietary — TSDM.net
