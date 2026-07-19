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

默认管理员: `admin` / `admin`

## Just Recipes

```bash
just fmt           # 格式化 Markdown
just lint          # clippy 检查 Rust 代码
just test          # 运行 Rust 测试
just build         # 构建 Rust/WASM
just up            # 启动 Docker/Podman 开发环境
just down          # 停止
just logs          # 查看日志
just status        # 容器状态
just clean         # 全量清理（含数据卷）
```

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

迁移脚本位于 `migrations/from-x3/`，按序执行：

1. `001_x3_to_x5_migration.sql` — 引擎升级 MyISAM→InnoDB，字符集 utf8mb3→utf8mb4
2. `002_cleanup_deprecated_fields.sql` — 清理废弃字段
3. `003_restructure_tables.sql` — 数据重整归一

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
