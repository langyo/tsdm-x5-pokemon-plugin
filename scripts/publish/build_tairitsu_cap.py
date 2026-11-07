#!/usr/bin/env python3
"""构建 tairitsu 能力探针（_capprobe）到 plugin/wasm2/。

复用 build_tairitsu_poc.py 的 vendor/glue/shim 装配（同一份共享资产），
只生成 admin 自己的组件产物与启动脚本：

  1. cargo build -p _capprobe --target wasm32-wasip2 --release
  2. jco transpile 到临时目录，仅拷贝 _capprobe* 到 component-wrapper/
     （与 _game2* 共存，互不覆盖）
  3. 写 cap-boot.js / cap-manifest.json（入口页 admin_v2.inc.php 用）

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

CAP_DIR = ROOT / "rust" / "capprobe"
WASM_ARTIFACT = CAP_DIR / "target" / "wasm32-wasip2" / "release" / "_capprobe.wasm"
COMPONENT = "_capprobe"


def cargo_build(skip: bool) -> Path:
    wasm = WASM_ARTIFACT
    if skip and wasm.exists():
        print("[admin] skip cargo build")
        return wasm
    result = subprocess.run(
        [
            "cargo", "build",
            "--manifest-path", str(CAP_DIR / "Cargo.toml"),
            "--target", "wasm32-wasip2",
            "--release",
        ],
        cwd=str(CAP_DIR),
        check=False,
        shell=False,
    )
    if result.returncode != 0:
        sys.exit(result.returncode)
    if not wasm.exists():
        sys.exit("admin wasm artifact missing after build")
    return wasm


def jco_transpile_admin(wasm: Path) -> None:
    """转译到临时目录后只拷贝 _capprobe*（component-wrapper 与 _game2 共存）。"""
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


def verify_imports() -> None:
    wrapper = OUT / "component-wrapper" / f"{COMPONENT}.js"
    specifiers = sorted(set(re.findall(
        r"from ['\"]tairitsu-browser:full/([a-z-]+)['\"]",
        wrapper.read_text(encoding="utf-8"),
    )))
    unknown = [s for s in specifiers if s not in INTERFACE_MAP]
    if unknown:
        sys.exit(f"admin wrapper imports interfaces missing from INTERFACE_MAP: {unknown}")
    print(f"[admin] wrapper imports {len(specifiers)} interfaces — all mapped")


def write_boot_assets(build_id: str) -> None:
    """cap-boot.js：以 boot.js 为模板，仅替换组件名与产物清单。

    boot.js 的装配（importmap / MessageChannel rAF / 同步 glue-core /
    findStart / pm-flush 观察器）已在本仓库验证可靠，直接复用最稳。
    """
    boot = (OUT / "boot.js").read_text(encoding="utf-8")
    boot = boot.replace(
        "由 scripts/publish/build_tairitsu_poc.py 生成",
        "由 scripts/publish/build_tairitsu_cap.py 生成（模板：boot.js）",
    )
    boot = boot.replace(
        "import(BASE + '/component-wrapper/_game2.js?b=' + BUILD_ID)",
        "import(BASE + '/component-wrapper/_capprobe.js?b=' + BUILD_ID)",
    )
    (OUT / "cap-boot.js").write_text(boot, encoding="utf-8")

    manifest = {
        "build_id": build_id,
        "base": OUT_REL_URL,
        "entry": f"{OUT_REL_URL}/cap-boot.js?v={build_id}",
        "component": f"{OUT_REL_URL}/component-wrapper/{COMPONENT}.js?b={build_id}",
        "interfaces": sorted(INTERFACE_MAP),
    }
    (OUT / "cap-manifest.json").write_text(
        json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--skip-vendor", action="store_true")
    args = parser.parse_args()

    OUT.mkdir(parents=True, exist_ok=True)
    wasm = cargo_build(args.skip_build)
    jco_transpile_admin(wasm)
    build_id = str(int(wasm.stat().st_mtime))
    bust_wrapper_cache(build_id)
    verify_imports()
    if not args.skip_vendor:
        poc.vendor_copy()
    elif not (OUT / "glue").is_dir() or not (OUT / "wasi").is_dir():
        sys.exit("--skip-vendor requires existing plugin/wasm2/glue and plugin/wasm2/wasi")
    # glue getParentElement 补丁已随上游 #124 的合入退役（见 poc 脚本主流程注释）。
    poc.write_boot_assets()  # 刷新 boot.js（importmap 随 INTERFACE_MAP 变化）
    poc.shims_from_glue(build_id)
    poc.glue_core()
    write_boot_assets(build_id)
    print("[admin] done →", OUT)


if __name__ == "__main__":
    main()
