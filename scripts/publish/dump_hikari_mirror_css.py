#!/usr/bin/env python3
"""把 hikari Rust 镜像自带、但 Vue 那半边没有的组件样式补成一份 CSS。

hikari 的样式有两条路：

1. Vue 侧的 SCSS 表（`components/src/styles/index.scss` 引的那些），由
   `build_tairitsu_admin.py` 的 `compile_hikari_css()` 编译进
   `admin-hikari.css`；
2. Rust 镜像自己的 `StyledComponent::styles()`，运行时经
   `StyleRegistry::register_all()` + `css_bundle()` 注入。

只走第 1 条会漏掉「只有 Rust 镜像、没有 Vue 表」的组件——`FormField` 就是，
它在页面里是个没有标签/帮助文字样式的裸 div。

整份 bundle 不能直接拼进去：它与 Vue 表大面积重叠（同一批 `.hk-*` 类名），
拼进去等于把每个组件定义两遍，同权重规则谁生效取决于顺序。所以这里的做法是
**只补缺的**：拿 `admin-hikari.css` 里已有的 `.hk-*` 类名做底，逐块问 helper
要那些点名的前缀，产出一份 `admin-mirror.css`。

用法：python -I scripts/publish/dump_hikari_mirror_css.py [--kernel rust/admin2]
"""

import argparse
import re
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OUT = ROOT / "plugin" / "wasm2"

# 需要补齐的组件表：它们只有 Rust 镜像，Vue 侧没有对应 SCSS。
# 新增「只用 Rust 镜像」的组件时，把它的类名前缀加在这里。
MIRROR_ONLY_PREFIXES = (
    "hk-form-field",
)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--kernel", default="rust/admin2")
    args = parser.parse_args()

    kernel = ROOT / args.kernel
    if not (kernel / "Cargo.toml").is_file():
        sys.exit(f"kernel crate not found: {kernel}")
    helper = kernel / "examples" / "dump_hikari_mirror_css.rs"
    if not helper.is_file():
        sys.exit(f"helper example missing: {helper}")

    compiled = OUT / "admin-hikari.css"
    if not compiled.is_file():
        sys.exit(f"{compiled.name} not built yet (run build_tairitsu_admin.py)")
    present = set(re.findall(r"\.(hk-[a-z0-9-]+)", compiled.read_text(encoding="utf-8")))

    cargo = shutil.which("cargo")
    if not cargo:
        sys.exit("cargo not found on PATH")

    chunks: list[str] = []
    missing: list[str] = []
    for prefix in MIRROR_ONLY_PREFIXES:
        if any(name == prefix or name.startswith(prefix + "-") for name in present):
            continue
        missing.append(prefix)
        run = subprocess.run(
            [cargo, "run", "--quiet", "--example", "dump_hikari_mirror_css", "--",
             f".{prefix}"],
            cwd=kernel, check=False, capture_output=True, text=True,
        )
        if run.returncode != 0:
            sys.exit(run.stderr.strip() or f"helper failed for {prefix}")
        chunk = run.stdout.strip()
        if not chunk:
            sys.exit(f"helper produced nothing for {prefix}")
        chunks.append(f"/* hikari mirror-only component: {prefix} */\n{chunk}")

    if not chunks:
        print("[admin] mirror.css: nothing to add (every mirror-only sheet is "
              "already covered by the Vue tables)")
        return

    # 直接追加进 admin-hikari.css 的尾部：页面只引这一份样式表，PHP 不必为
    # 「有没有补丁样式」多一个条件分支；顺序上它落在全部 Vue 表之后、宿主
    # 的 admin.css 之前（后者是另一个 <link>）。
    with compiled.open("a", encoding="utf-8") as handle:
        handle.write("\n".join(chunks) + "\n")
    print(
        f"[admin] mirror.css: {len(missing)} mirror-only sheet(s) "
        f"({', '.join(missing)}) appended to {compiled.name} "
        f"({sum(len(c) for c in chunks)} bytes)"
    )


if __name__ == "__main__":
    main()
