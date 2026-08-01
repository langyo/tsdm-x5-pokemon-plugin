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
    pip install "git+https://github.com/celestia-island/celestia-devtools.git"
    celestia-devtools init --no-hooks
    @echo "celestia-devtools installed."

commit-msg-hook-install *ARGS='':
    celestia-devtools hook install {{ARGS}}

# ── Formatting ───────────────────────────────────────────────

markdown-fmt *ARGS='':
    celestia-devtools format-markdown . {{ARGS}}

markdown-fmt-check:
    celestia-devtools format-markdown . --check

fmt: markdown-fmt
    just --evaluate _devtools > /dev/null

fmt-check: markdown-fmt-check

# ── Rust / WASM ──────────────────────────────────────────────

build *ARGS='':
    cargo build {{ARGS}}

build-release *ARGS='':
    cargo build --release {{ARGS}}

test *ARGS='':
    cargo test {{ARGS}}

lint:
    cargo clippy --workspace --all-targets -- -D warnings

# ── Commit message ───────────────────────────────────────────

commit-msg-lint FILE:
    celestia-devtools commit-msg-lint check {{FILE}}

# ── Docker / Podman ──────────────────────────────────────────

# Start the full dev stack (Discuz X5 + MariaDB + plugin auto-install).
up *FLAGS='':
    @python scripts/docker/dev.py up {{FLAGS}}

# Stop services.
down *FLAGS='':
    @python scripts/docker/dev.py down {{FLAGS}}

# Restart the app container.
restart *FLAGS='':
    @python scripts/docker/dev.py restart {{FLAGS}}

# Follow logs.
logs *FLAGS='':
    @python scripts/docker/dev.py logs {{FLAGS}}

# Show container status.
status:
    @python scripts/docker/dev.py status

# Full teardown including volumes (wipes database).
clean:
    @python scripts/docker/dev.py clean

# Build WASM frontend and restart app container.
rebuild:
    cargo build --release
    just restart

# ── Publish ─────────────────────────────────────────────────

# Run static tests (schema consistency + stale column scan + seed data validation).
test-php:
    @python3 scripts/test/run.py

# Rebuild the WASM frontends into plugin/wasm.
build-wasm *ARGS='':
    @python3 scripts/publish/build.py --build-wasm {{ARGS}}

# Package the plugin into an X5-installable zip under dist/.
publish *ARGS='':
    @python3 scripts/publish/build.py {{ARGS}}

# Verify a published plugin zip.
publish-verify ZIP:
    @python3 scripts/publish/build.py --verify {{ZIP}}

# ── Cache ────────────────────────────────────────────────────

cache-guard *ARGS='':
    celestia-devtools cache-guard . {{ARGS}}

clean-incremental:
    celestia-devtools cache-guard . --clean-incremental
