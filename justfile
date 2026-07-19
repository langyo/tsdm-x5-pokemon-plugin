# TSDM Pokemon Plugin for Discuz X5 — build + dev recipes.
# Gitmoji commit convention: <emoji> <Capitalized English summary.>

set shell := ["bash", "-c"]
set windows-shell := ["bash.exe", "-c"]
set unstable
set lists

default:
    @just --list

# ── Bootstrap ────────────────────────────────────────────────

install:
    pnpm install

commit-msg-hook-install *ARGS='':
    pip install "git+https://github.com/celestia-island/celestia-devtools.git"
    celestia-devtools hook install {{ARGS}}

# ── Formatting ───────────────────────────────────────────────

markdown-fmt *ARGS='':
    pip install -q "git+https://github.com/celestia-island/celestia-devtools.git"
    celestia-devtools format-markdown . {{ARGS}}

fmt: markdown-fmt

fmt-check: markdown-fmt

# ── Frontend (Vue 3 + Vite) ─────────────────────────────────

dev:
    pnpm --filter @tsdm/webui dev

build:
    pnpm build

test:
    pnpm test

lint:
    pnpm lint

typecheck:
    pnpm typecheck

# CI pass locally:
ci: lint typecheck test build

# ── Docker / Podman ──────────────────────────────────────────

up *FLAGS='':
    @python scripts/docker/dev.py up {{FLAGS}}

down *FLAGS='':
    @python scripts/docker/dev.py down {{FLAGS}}

restart *FLAGS='':
    @python scripts/docker/dev.py restart {{FLAGS}}

logs *FLAGS='':
    @python scripts/docker/dev.py logs {{FLAGS}}

status:
    @python scripts/docker/dev.py status

clean:
    @python scripts/docker/dev.py clean

# ── Commit message ───────────────────────────────────────────

commit-msg-lint FILE:
    pip install -q "git+https://github.com/celestia-island/celestia-devtools.git"
    celestia-devtools commit-msg-lint check {{FILE}}
