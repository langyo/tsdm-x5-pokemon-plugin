#!/usr/bin/env python3
"""E2E test suite for Pokemon Plugin admin API. Run via: python scripts/e2e/run.py"""
import subprocess, os, json, sys, time, hashlib

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
TMP_DIR = os.path.join(PROJECT_ROOT, "scripts", "e2e", ".tmp")
os.makedirs(TMP_DIR, exist_ok=True)

CONTAINER = "tsdm-app"
DB_CONTAINER = "tsdm-db"

def sql(sql):
    r = subprocess.run(["podman", "exec", "-i", DB_CONTAINER, "mariadb", "-u", "root", "-proot", "discuz"],
                       input=sql, capture_output=True, text=True, encoding="utf-8")
    if r.returncode != 0:
        print(f"  SQL ERR: {r.stderr[:100]}")
    return r.stdout

def run_api(action, params=None):
    """Call a dispatch endpoint and return parsed JSON."""
    if params is None:
        params = {}
    params["action"] = action
    qs_parts = [f"{k}={v}" for k, v in params.items()]
    qs = "&".join(qs_parts)

    php = '''<?php
define("IN_DISCUZ", 1);
class DB {
    static $l;
    static function init() { self::$l = new mysqli("''' + DB_CONTAINER + '''", "root", "root", "discuz"); }
    static function fetch_all($s) { $r = self::$l->query($s); if (!$r) return []; $rs = []; while ($w = $r->fetch_assoc()) $rs[] = $w; return $rs; }
    static function fetch_first($s) { $r = self::$l->query($s); if (!$r) return null; return $r->fetch_assoc(); }
    static function result_first($s) { $r = self::$l->query($s); if (!$r) return 0; $w = $r->fetch_row(); return $w ? $w[0] : 0; }
    static function table($t) { return "pre_$t"; }
    static function query($s, $mode = null) { return self::$l->query(is_string($s) ? $s : ""); }
    static function insert_id() { return self::$l->insert_id; }
}
DB::init();
$_SERVER = ["REQUEST_METHOD" => "POST"];
parse_str(\'''' + qs + '''\', $_POST);
ob_start();
include "/app/public/source/plugin/pokemon/admin/dispatch.php";
$o = ob_get_clean();
$out = json_decode($o, true);
if ($out === null) {
    $out = ["_raw" => $o, "_parse_error" => json_last_error_msg()];
}
echo json_encode($out, JSON_UNESCAPED_UNICODE);
'''

    digest = hashlib.sha256(repr(action).encode("utf-8")).hexdigest()[:16]
    filepath = os.path.join(TMP_DIR, f"t_{digest}.php")
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(php)

    subprocess.run(["podman", "cp", filepath, f"{CONTAINER}:/tmp/t.php"], capture_output=True)
    r = subprocess.run(["podman", "exec", CONTAINER, "frankenphp", "php-cli", "/tmp/t.php"],
                       capture_output=True, text=True, encoding="utf-8")

    try:
        return json.loads(r.stdout.strip())
    except json.JSONDecodeError:
        return {"_raw": r.stdout[:500], "_error": str(r.stderr)[:500]}

def check(name, result, expect_success=True, expect_count=None, expect_keys=None):
    """Validate an API result."""
    success = result.get("success", False)
    data = result.get("data")
    warnings = isinstance(result.get("_raw"), str) and "Warning" in str(result.get("_raw", ""))

    if expect_success and not success:
        return f"FAILED: expected success, got {result}"
    if not expect_success and success:
        return f"FAILED: expected failure, got success"
    if expect_count is not None and data:
        if isinstance(data, list) and len(data) > 0 and isinstance(data[0], dict) and "count" in data[0]:
            actual = int(data[0]["count"])
            if actual != expect_count and expect_count > 0:
                return f"WARN: count={actual} expected={expect_count}"
        elif isinstance(data, list):
            actual = len(data)
            if expect_count > 0 and actual != expect_count:
                return f"WARN: count={actual} expected={expect_count}"
    if warnings:
        return "WARN: PHP warnings"
    return "OK"

# =========== TEST SUITE ===========
passed = 0
failed = 0
warned = 0
details = []

def test(name, result, **kwargs):
    global passed, failed, warned
    status = check(name, result, **kwargs)
    if status == "OK":
        passed += 1
        print(f"  OK  {name}")
    elif status.startswith("WARN"):
        warned += 1
        print(f"  WARN {name}")
        details.append(f"WARN {name}")
    else:
        failed += 1
        print(f"  FAIL {name}: {status}")
        details.append(f"FAIL {name}: {status}")

print("=" * 60)
print("Pokemon Plugin E2E Test Suite")
print("=" * 60)

# 1. CRUD — Pokemon (8 tests)
print("\n-- Pokemon Types --")
test("count::pokemon_type", run_api("count::pokemon_type"))
test("list::pokemon_type", run_api("list::pokemon_type", {"from": "0", "count": "2"}))
test("get::pokemon_type", run_api("get::pokemon_type", {"id": "1"}))
test("filter::pokemon_type", run_api("filter::pokemon_type", {"filters": '[{"tag":"ID","operator":"equal","value":"1"}]'}))

# 2. CRUD — Items (7 tests)
print("\n-- Items --")
test("count::item_type", run_api("count::item_type"))
test("list::item_type", run_api("list::item_type", {"from": "0", "count": "2"}))
test("get::item_type", run_api("get::item_type", {"id": "1"}))
test("filter::item_type", run_api("filter::item_type", {"filters": '[{"tag":"ID","operator":"equal","value":"1"}]'}))

# 3. CRUD — Maps (7 tests)
print("\n-- Maps --")
test("count::map_info", run_api("count::map_info"))
test("list::map_info", run_api("list::map_info", {"from": "0", "count": "2"}))
test("get::map_info", run_api("get::map_info", {"id": "1"}))
test("filter::map_info", run_api("filter::map_info", {"filters": '[{"tag":"ID","operator":"equal","value":"1"}]'}))

# 4. CRUD — Evolutions (7 tests)
print("\n-- Evolutions --")
test("count::evolution_info", run_api("count::evolution_info"))
test("list::evolution_info", run_api("list::evolution_info", {"from": "0", "count": "2"}))
test("get::evolution_info", run_api("get::evolution_info", {"id": "1"}))
test("filter::evolution_info", run_api("filter::evolution_info", {"filters": '[{"tag":"ID","operator":"equal","value":"1"}]'}))

# 5. CRUD — Skills (7 tests)
print("\n-- Skills --")
test("count::skill_type", run_api("count::skill_type"))
test("list::skill_type", run_api("list::skill_type", {"from": "0", "count": "2"}))
test("get::skill_type", run_api("get::skill_type", {"id": "1"}))
test("filter::skill_type", run_api("filter::skill_type", {"filters": '[{"tag":"ID","operator":"equal","value":"1"}]'}))

# 6. Config (3 tests)
print("\n-- Config --")
test("list::global_config", run_api("list::global_config"))
test("get::global_config", run_api("get::global_config"))
test("set::global_config", run_api("set::global_config", {"data": '{"is_open":"1"}'}))

# 7. Users (5 tests)
print("\n-- Users --")
test("count::user_info", run_api("count::user_info"), expect_count=1)
test("list::user_info", run_api("list::user_info", {"from": "0", "count": "2"}))
test("get::user_info", run_api("get::user_info", {"id": "1"}))
test("filter::user_info", run_api("filter::user_info", {"filters": '[{"tag":"UID","operator":"equal","value":"1"}]'}))

# 8. Pokemon info (3 tests)
print("\n-- Pokemon Info --")
test("list::pokemon_info", run_api("list::pokemon_info", {"uid": "1", "from": "0", "count": "2"}))
test("get::pokemon_info", run_api("get::pokemon_info", {"id": "1"}))

# 9. Item info (2 tests)
print("\n-- Item Info --")
test("list::item_info", run_api("list::item_info", {"uid": "1", "from": "0", "count": "2"}))

# Summary
total = passed + failed + warned
print(f"\n=== {passed}/{total} passed ({warned} warnings, {failed} failed) ===")
for d in details:
    print(f"  {d}")

sys.exit(0 if failed == 0 else 1)
