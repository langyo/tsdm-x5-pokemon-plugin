#!/usr/bin/env python3
"""构建 tairitsu 版管理后台（_admin2）到 plugin/wasm2/。

复用 build_tairitsu_poc.py 的 vendor/glue/shim 装配（同一份共享资产），
只生成 admin 自己的组件产物与启动脚本：

  1. cargo build -p _admin2 --target wasm32-wasip2 --release
  2. jco transpile 到临时目录，仅拷贝 _admin2* 到 component-wrapper/
     （与 _game2* 共存，互不覆盖）
  3. 写 admin-boot.js / admin-manifest.json（入口页 admin_v2.inc.php 用）

用法与 poc 脚本一致：--skip-build / --skip-vendor。
"""

import argparse
import importlib.util
import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]

# 复用 poc 脚本的共享装配与常量（OUT / INTERFACE_MAP / ...）。
_spec = importlib.util.spec_from_file_location("poc_build", HERE / "build_tairitsu_poc.py")
poc = importlib.util.module_from_spec(_spec)
sys.modules["poc_build"] = poc
_spec.loader.exec_module(poc)

OUT = poc.OUT
OUT_REL = poc.OUT_REL
OUT_REL_URL = poc.OUT_REL_URL
INTERFACE_MAP = poc.INTERFACE_MAP

ADMIN_DIR = ROOT / "rust" / "admin2"
WASM_ARTIFACT = ADMIN_DIR / "target" / "wasm32-wasip2" / "release" / "_admin2.wasm"
COMPONENT = "_admin2"

# hikari-components 的 styles/index.scss 里一行 vue-mirror 组件表导入。
MIRROR_IMPORT_RE = re.compile(r"^\s*@import\s+'(\./vue-mirror/[^']+)'\s*;", re.MULTILINE)


def cargo_build(skip: bool) -> Path:
    wasm = WASM_ARTIFACT
    if skip and wasm.exists():
        print("[admin] skip cargo build")
        return wasm
    result = subprocess.run(
        [
            "cargo", "build",
            "--manifest-path", str(ADMIN_DIR / "Cargo.toml"),
            "--target", "wasm32-wasip2",
            "--release",
        ],
        cwd=str(ADMIN_DIR),
        check=False,
        shell=False,
    )
    if result.returncode != 0:
        sys.exit(result.returncode)
    if not wasm.exists():
        sys.exit("admin wasm artifact missing after build")
    return wasm


def jco_transpile_admin(wasm: Path) -> None:
    """转译到临时目录后只拷贝 _admin2*（component-wrapper 与 _game2 共存）。"""
    wrapper = OUT / "component-wrapper"
    wrapper.mkdir(parents=True, exist_ok=True)
    npx = shutil.which("npx") or "npx.cmd"
    with tempfile.TemporaryDirectory() as td:
        tmp_out = Path(td) / "wrapper"
        result = subprocess.run(
            [
                npx, "--yes", "@bytecodealliance/jco",
                "transpile", str(wasm), "-o", str(tmp_out),
            ],
            check=False,
            shell=False,
        )
        if result.returncode != 0:
            sys.exit(result.returncode)
        for src in tmp_out.iterdir():
            if src.name.startswith(COMPONENT):
                shutil.copy2(src, wrapper / src.name)


def bust_wrapper_cache(build_id: str) -> None:
    wrapper = OUT / "component-wrapper" / f"{COMPONENT}.js"
    js = wrapper.read_text(encoding="utf-8")
    js = js.replace(
        f"./{COMPONENT}.core.wasm", f"./{COMPONENT}.core.wasm?v={build_id}"
    )
    wrapper.write_text(js, encoding="utf-8")


# 导入对账用 poc 脚本的同一份（逐函数、三份产物全查，见其 verify_imports）：
# admin2 独有的导入（如 location.setHref）以前不在接口级对账的视野里。


def write_boot_assets(build_id: str) -> None:
    """admin-boot.js：以 boot.js 为模板，仅替换组件名与产物清单。

    boot.js 的装配（importmap / MessageChannel rAF / 同步 glue-core /
    findStart / pm-flush 观察器）已在本仓库验证可靠，直接复用最稳。
    """
    boot = (OUT / "boot.js").read_text(encoding="utf-8")
    boot = boot.replace(
        "由 scripts/publish/build_tairitsu_poc.py 生成",
        "由 scripts/publish/build_tairitsu_admin.py 生成（模板：boot.js）",
    )
    boot = boot.replace(
        "import(BASE + '/component-wrapper/_game2.js?b=' + BUILD_ID)",
        "import(BASE + '/component-wrapper/_admin2.js?b=' + BUILD_ID)",
    )
    (OUT / "admin-boot.js").write_text(boot, encoding="utf-8")

    manifest = {
        "build_id": build_id,
        "base": OUT_REL_URL,
        "entry": f"{OUT_REL_URL}/admin-boot.js?v={build_id}",
        "component": f"{OUT_REL_URL}/component-wrapper/{COMPONENT}.js?b={build_id}",
        "interfaces": sorted(INTERFACE_MAP),
    }
    (OUT / "admin-manifest.json").write_text(
        json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )


def compile_admin_css() -> None:
    """编译 rust/admin2/styles/main.scss → plugin/wasm2/admin.css。

    后台样式归前端：以前这段 CSS 内联在 admin_v2.inc.php 里，PHP 改样式、
    `<style>` 还直接落在论坛页面上。现在由这里产出，入口页只挂 `<link>`。
    """
    source = ADMIN_DIR / "styles" / "main.scss"
    output = OUT / "admin.css"
    sass = shutil.which("sass")
    if not sass:
        sys.exit("sass not found on PATH (needed for rust/admin2/styles/main.scss)")
    result = subprocess.run(
        [sass, "--style=compressed", "--no-source-map", "--quiet", str(source), str(output)],
        check=False,
        shell=False,
    )
    if result.returncode != 0:
        sys.exit(result.returncode)


def hikari_packages_dir() -> Path:
    """hikari 检出的 packages/ 目录。

    从 cargo metadata 读 hikari-components 的 manifest 位置：依赖来自 git 检出
    还是本地 path patch，答案都是同一个目录，所以 CI 与本地开发共用这一条路径。
    """
    result = subprocess.run(
        [
            "cargo", "metadata",
            "--format-version", "1",
            "--manifest-path", str(ADMIN_DIR / "Cargo.toml"),
        ],
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=False,
        shell=False,
        cwd=str(ADMIN_DIR),
    )
    if result.returncode != 0:
        sys.exit(result.stderr)
    for pkg in json.loads(result.stdout)["packages"]:
        if pkg["name"] == "hikari-components":
            return Path(pkg["manifest_path"]).resolve().parent.parent
    sys.exit("hikari-components not found in cargo metadata")


def hikari_mirror_imports(packages: Path) -> list[str]:
    """hikari-components 全局表里全部 vue-mirror 组件表，路径改写成相对
    `<hikari>/packages`（编译时的 --load-path）。

    清单从 hikari 自己的 styles/index.scss 里取：锁定哪个版本的 hikari，
    入口就跟到哪个版本，不在本仓库手抄一份会过期的列表。
    """
    index = packages / "components" / "src" / "styles" / "index.scss"
    found = MIRROR_IMPORT_RE.findall(index.read_text(encoding="utf-8"))
    if not found:
        sys.exit(f"no vue-mirror sheet imports found in {index}")
    return [f"components/src/styles{path[1:]}" for path in found]


def compile_hikari_css() -> None:
    """编译 hikari 样式入口 → plugin/wasm2/admin-hikari.css。

    后台整壳用 hikari 标准件（AdminShell / AdminHeader / NavSidebar / Modal …），
    组件样式不再手写皮肤。入口分两段：

    - 头部：rust/admin2/styles/hikari.scss 里的静态 @use（Vue 侧的规范基础层）；
    - 尾部：hikari 的全部 vue-mirror 组件表（见 hikari_mirror_imports）。

    有意不走 hikari-components 自己的 styles/index.scss 整表——那张表还带着
    Gen-1 主题基座，其中的元素级按钮基线会把镜像壳里 s-* 类的原生控件重画
    （AdminHeader 的头像按钮被挤成 0 宽）；Vue 的规范组合不含这一层。
    """
    source = ADMIN_DIR / "styles" / "hikari.scss"
    output = OUT / "admin-hikari.css"
    sass = shutil.which("sass")
    if not sass:
        sys.exit("sass not found on PATH (needed for rust/admin2/styles/hikari.scss)")
    packages = hikari_packages_dir()
    # 只取入口里的 @use / @import 行；注释留在源文件里给读的人。
    head = [
        line
        for line in source.read_text(encoding="utf-8").splitlines()
        if line.startswith("@use ") or line.startswith("@import ")
    ]
    mirror = [f'@import "{path}";' for path in hikari_mirror_imports(packages)]
    with tempfile.TemporaryDirectory() as td:
        entry = Path(td) / "hikari-entry.scss"
        entry.write_text("\n".join(head + mirror) + "\n", encoding="utf-8")
        result = subprocess.run(
            [
                sass, "--style=compressed", "--no-source-map", "--quiet",
                "--load-path", str(packages),
                # 组件表里的主题 partial 以裸名 @use（'variables' / 'mixins'）。
                "--load-path", str(packages / "theme" / "styles"),
                str(entry), str(output),
            ],
            check=False,
            shell=False,
        )
    if result.returncode != 0:
        sys.exit(result.returncode)
    print(
        f"[admin] hikari.css: {len(head)} base sheets + {len(mirror)} vue-mirror "
        f"sheets → {output.name} ({output.stat().st_size} bytes)"
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--skip-vendor", action="store_true")
    args = parser.parse_args()

    OUT.mkdir(parents=True, exist_ok=True)
    compile_hikari_css()
    compile_admin_css()
    wasm = cargo_build(args.skip_build)
    jco_transpile_admin(wasm)
    build_id = str(int(wasm.stat().st_mtime))
    bust_wrapper_cache(build_id)
    poc.verify_imports()
    if not args.skip_vendor:
        poc.vendor_copy()
    elif not (OUT / "glue").is_dir() or not (OUT / "wasi").is_dir():
        sys.exit("--skip-vendor requires existing plugin/wasm2/glue and plugin/wasm2/wasi")
    # glue getParentElement 补丁已随上游 #124 的合入退役（见 poc 脚本主流程注释）。
    poc.verify_glue_exports()
    poc.write_boot_assets()  # 刷新 boot.js 模板（importmap 随 INTERFACE_MAP 变化）
    poc.shims_from_glue(build_id)
    poc.glue_core()
    write_boot_assets(build_id)
    print("[admin] done →", OUT)


if __name__ == "__main__":
    main()
