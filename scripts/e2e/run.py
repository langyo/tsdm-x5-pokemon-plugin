#!/usr/bin/env python3
"""E2E test suite for Pokemon Plugin admin API.

Run via: python scripts/e2e/run.py           # dispatch-level tests (needs dev stack)
         python scripts/e2e/run.py --http    # additionally run HTTP-level tests

The dispatch-level suite drives admin/dispatch.php directly inside the app
container (no HTTP/auth); the HTTP-level suite logs in as admin over HTTPS and
exercises the same surface through plugin.php, including the boss endpoint and
unauthorized-access behavior.
"""
import subprocess, os, json, sys, hashlib
import urllib.request, urllib.parse, http.cookiejar, ssl

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(PROJECT_ROOT, "scripts", "docker"))
from _docker import docker_cmd

# 容器运行时与 dev.py 的探测保持一致（podman 存活则 podman，否则 docker），
# 避免栈由 docker compose 启动时 exec 找不到容器
RUNTIME = docker_cmd()[0]
TMP_DIR = os.path.join(PROJECT_ROOT, "scripts", "e2e", ".tmp")
os.makedirs(TMP_DIR, exist_ok=True)

CONTAINER = "tsdm-app"
DB_CONTAINER = "tsdm-db"
HTTP_BASE = os.environ.get("E2E_BASE_URL", "https://localhost:8443")

def sql(sql_str):
    r = subprocess.run([RUNTIME, "exec", "-i", DB_CONTAINER, "mariadb", "-u", "root", "-proot", "discuz"],
                       input=sql_str, capture_output=True, text=True, encoding="utf-8")
    if r.returncode != 0:
        print(f"  SQL ERR: {r.stderr[:100]}")
    return r.stdout

def sql_rows(sql_str):
    """Run SQL, return rows as lists of strings (batch format, no column names)."""
    r = subprocess.run([RUNTIME, "exec", "-i", DB_CONTAINER, "mariadb", "-u", "root", "-proot", "-N", "-B", "discuz"],
                       input=sql_str, capture_output=True, text=True, encoding="utf-8")
    if r.returncode != 0:
        print(f"  SQL ERR: {r.stderr[:100]}")
        return []
    return [line.split("\t") for line in r.stdout.strip().split("\n") if line.strip()]

def sql_scalar(sql_str):
    rows = sql_rows(sql_str)
    return rows[0][0] if rows else None

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
    static function affected_rows() { return self::$l->affected_rows; }
    static function errno() { return self::$l->errno; }
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

    subprocess.run([RUNTIME, "cp", filepath, f"{CONTAINER}:/tmp/t.php"], capture_output=True)
    r = subprocess.run([RUNTIME, "exec", CONTAINER, "frankenphp", "php-cli", "/tmp/t.php"],
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

def check_true(name, condition, detail=""):
    global passed, failed
    if condition:
        passed += 1
        print(f"  OK  {name}")
    else:
        failed += 1
        print(f"  FAIL {name}: {detail}")
        details.append(f"FAIL {name}: {detail}")

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

# ============ 10. Grant pet (发放宠物) — the core regression ============
print("\n-- Grant Pokemon (发放宠物) --")
grant_species_id = sql_scalar("SELECT id FROM pm_data ORDER BY id LIMIT 1")
grant_skill_id = sql_scalar("SELECT id FROM pm_skill ORDER BY id LIMIT 1")

grant_payload = {
    "id": 0,
    "type_id": int(grant_species_id or 1),
    "owner": 1,
    "name": "E2E_GRANT",
    "site": "store",
    "level": 10,
    "experience": 0,
    "intimacy": 50,
    "using_ball_id": 1,
    "is_shiny": False,
    "status": "normal",
    "sex": "male",
    "statistic": {"hit_points": 20, "attack": 20, "defense": 20,
                  "special_attack": 20, "special_defense": 20, "speed": 20},
    "base_points": {"hit_points": 0, "attack": 0, "defense": 0,
                    "special_attack": 0, "special_defense": 0, "speed": 0},
    "skills": [{"type_id": int(grant_skill_id or 1), "count": 10}] if grant_skill_id else [],
    "armor_slots_id": [None, None, None, None],
}
res = run_api("insert::pokemon_info", {"data": json.dumps(grant_payload, ensure_ascii=False)})
test("insert::pokemon_info (grant to uid 1)", res)
granted_id = None
if res.get("success") and res.get("data"):
    granted_id = int(res["data"][0]["id"])
    check_true("grant returns correct name", res["data"][0].get("name") == "E2E_GRANT", f'got {res["data"][0].get("name")}')
    check_true("grant returns status label", res["data"][0].get("status") == "normal", f'got {res["data"][0].get("status")}')
    check_true("grant returns skill list", len(res["data"][0].get("skills", [])) == len(grant_payload["skills"]),
               f'got {res["data"][0].get("skills")}')

if granted_id:
    row = sql_rows(f"SELECT pmname, nickname, sx, hp, initialuid, created_at, statetime FROM pm_mypm WHERE id={granted_id}")
    check_true("DB: pmname populated from species", bool(row) and row[0][0] != "", f"row={row}")
    check_true("DB: nickname stored", bool(row) and row[0][1] == "E2E_GRANT", f"row={row}")
    check_true("DB: nature (sx) populated", bool(row) and row[0][2] != "", f"row={row}")
    check_true("DB: hp computed > 0", bool(row) and int(row[0][3]) > 0, f"row={row}")
    check_true("DB: initialuid = owner", bool(row) and int(row[0][4]) == 1, f"row={row}")
    check_true("DB: timestamps set", bool(row) and int(row[0][5]) > 0 and int(row[0][6]) > 0, f"row={row}")
    if grant_skill_id:
        n = sql_scalar(f"SELECT COUNT(*) FROM pm_myskill WHERE petid={granted_id} AND uid=1")
        check_true("DB: skills inserted with correct petid/uid", n == "1", f"count={n}")

# negative grant paths
bad = dict(grant_payload, type_id=999999)
test("insert::pokemon_info (unknown species) fails", run_api("insert::pokemon_info", {"data": json.dumps(bad)}), expect_success=False)
bad = dict(grant_payload, owner=99999999)
test("insert::pokemon_info (unknown owner) fails", run_api("insert::pokemon_info", {"data": json.dumps(bad)}), expect_success=False)

# edit the granted pet (set path covers the same status translation fix)
if granted_id:
    edit = json.loads(json.dumps(grant_payload))
    edit.update({"id": granted_id, "name": "E2E_RENAME", "status": "happy1", "level": 11})
    res = run_api("set::pokemon_info", {"data": json.dumps(edit)})
    test("set::pokemon_info (rename + status happy1)", res)
    got = run_api("get::pokemon_info", {"id": str(granted_id)})
    if got.get("success") and got.get("data"):
        check_true("set: nickname updated", got["data"][0]["name"] == "E2E_RENAME", f'got {got["data"][0]["name"]}')
        check_true("set: status label roundtrip", got["data"][0]["status"] == "happy1", f'got {got["data"][0]["status"]}')
        check_true("set: level updated", int(got["data"][0]["level"]) == 11, f'got {got["data"][0]["level"]}')

# ============ 11. Grant money (发放宠物币) ============
print("\n-- Grant Money (发放宠物币) --")
res = run_api("set::user_info", {"data": '{"id":1,"money":12345}'})
test("set::user_info (money=12345)", res)
got = run_api("get::user_info", {"id": "1"})
money_ok = got.get("success") and got.get("data") and int(got["data"][0]["money"]) == 12345
check_true("get::user_info reflects new money", money_ok, f"got={got.get('data')}")
test("set::user_info (negative money) fails", run_api("set::user_info", {"data": '{"id":1,"money":-5}'}), expect_success=False)

# user search by nickname / uid string (regression for name:null)
flt = run_api("filter::user_info", {"filters": json.dumps([{"tag": "昵称", "operator": "equal", "value": "admin"}])})
test("filter::user_info by nickname", flt)
if flt.get("success") and flt.get("data"):
    check_true("filter by nickname: name not null", flt["data"][0].get("name") == "admin", f'got {flt["data"][0].get("name")}')
flt_uid = run_api("filter::user_info", {"filters": json.dumps([{"tag": "昵称", "operator": "equal", "value": "1"}])})
test("filter::user_info by numeric nickname (uid path)", flt_uid)
if flt_uid.get("success") and flt_uid.get("data"):
    check_true("filter by uid string: name not null", flt_uid["data"][0].get("name") == "admin", f'got {flt_uid["data"][0].get("name")}')

# ============ 12. Full grant flow for a brand-new user ============
print("\n-- New User Grant Flow --")
sql("INSERT INTO pre_common_member (username, loginname, password, email, groupid, regdate) "
    "VALUES ('e2e_grant_user', 'e2e_grant_user', '202cb962ac59075b964b07152d234b70', 'e2e@test.local', 10, UNIX_TIMESTAMP()) "
    "ON DUPLICATE KEY UPDATE username=username")
new_uid = sql_scalar("SELECT uid FROM pre_common_member WHERE username='e2e_grant_user'")
if new_uid:
    new_uid = int(new_uid)
    sql(f"DELETE FROM pm_usersdata WHERE uid={new_uid}")
    res = run_api("insert::user_info", {"data": json.dumps({"id": new_uid})})
    test("insert::user_info (create profile)", res)
    test("insert::user_info (duplicate) fails", run_api("insert::user_info", {"data": json.dumps({"id": new_uid})}), expect_success=False)

    # grant pet to the new user
    payload = json.loads(json.dumps(grant_payload))
    payload.update({"owner": new_uid, "name": "E2E_NEWBIE", "skills": []})
    res = run_api("insert::pokemon_info", {"data": json.dumps(payload)})
    test("insert::pokemon_info (new user)", res)
    new_pet_id = int(res["data"][0]["id"]) if res.get("success") and res.get("data") else None

    # grant coins to the new user
    res = run_api("set::user_info", {"data": json.dumps({"id": new_uid, "money": 777})})
    test("set::user_info (new user money=777)", res)
    check_true("new user money applied", sql_scalar(f"SELECT money FROM pm_usersdata WHERE uid={new_uid}") == "777",
               f"got {sql_scalar(f'SELECT money FROM pm_usersdata WHERE uid={new_uid}')}")

    # grant items + stacking
    item_type_id = sql_scalar("SELECT id FROM pm_itemdata ORDER BY id LIMIT 1")
    res = run_api("insert::item_info", {"data": json.dumps({"owner": new_uid, "type_id": int(item_type_id or 1), "count": 3})})
    test("insert::item_info (grant x3)", res)
    res = run_api("insert::item_info", {"data": json.dumps({"owner": new_uid, "type_id": int(item_type_id or 1), "count": 3})})
    test("insert::item_info (grant x3 again, stacking)", res)
    got = run_api("list::item_info", {"uid": str(new_uid), "from": "0", "count": "10"})
    qty = None
    if got.get("success") and got.get("data"):
        qtys = [int(i.get("count", 0)) for i in got["data"]]
        qty = sum(qtys)
    check_true("item stacking sums to 6", qty == 6, f"quantities={qty}")
    inv_id = None
    if got.get("data"):
        inv_id = int(got["data"][0]["id"])
    if inv_id:
        test("delete::item_info", run_api("delete::item_info", {"id": str(inv_id)}))

    # 单只宠物时放生应被拒绝（最后一只宠物不可放生的防护）
    if new_pet_id:
        test("delete::pokemon_info (last pet) correctly refused",
             run_api("delete::pokemon_info", {"id": str(new_pet_id)}), expect_success=False)
    sql(f"DELETE FROM pm_usersdata WHERE uid={new_uid}")
    sql(f"DELETE FROM pm_mypm WHERE uid={new_uid}")
    sql(f"DELETE FROM pm_myskill WHERE uid={new_uid}")
    sql(f"DELETE FROM pm_myitem WHERE uid={new_uid}")
    sql(f"DELETE FROM pre_common_member WHERE uid={new_uid}")

# release the pet granted to admin earlier
if granted_id:
    test("delete::pokemon_info (cleanup)", run_api("delete::pokemon_info", {"id": str(granted_id)}))
    check_true("DB: granted pet removed", sql_scalar(f"SELECT COUNT(*) FROM pm_mypm WHERE id={granted_id}") == "0",
               "row still present")

# ============ 13. Type-entity insert/set/delete roundtrips ============
print("\n-- Type Entity CRUD Roundtrips --")

def roundtrip(entity, get_id, mutate):
    res = run_api(f"get::{entity}", {"id": str(get_id)})
    if not (res.get("success") and res.get("data")):
        test(f"get::{entity} baseline", res)
        return
    obj = res["data"][0]
    obj.pop("_TYPE", None)
    obj.pop("map", None)
    obj.pop("evolution", None)
    obj.pop("evolutions", None)
    mutate(obj)
    obj["id"] = 0
    res = run_api(f"insert::{entity}", {"data": json.dumps(obj, ensure_ascii=False)})
    test(f"insert::{entity}", res)
    if not (res.get("success") and res.get("data")):
        return
    new_id = int(res["data"][0]["id"])
    obj["id"] = new_id
    mutate(obj)
    test(f"set::{entity}", run_api(f"set::{entity}", {"data": json.dumps(obj, ensure_ascii=False)}))
    test(f"delete::{entity}", run_api(f"delete::{entity}", {"id": str(new_id)}))
    return new_id

roundtrip("pokemon_type", 1, lambda o: o.update({"name": "E2E_TEST_SPECIES"}))
roundtrip("item_type", 1, lambda o: o.update({"name": "E2E_ITEM"}))
roundtrip("map_info", 1, lambda o: o.update({"name": "E2E_TEST_MAP"}))
roundtrip("skill_type", 1, lambda o: o.update({"name": "E2E_TEST_SKILL"}))

# evolution roundtrip needs existing source/target species
evo_src = sql_scalar("SELECT from_id FROM pm_evolution ORDER BY id LIMIT 1")
evo_dst = sql_scalar("SELECT to_id FROM pm_evolution ORDER BY id LIMIT 1")
if evo_src and evo_dst:
    res = run_api("insert::evolution_info", {"data": json.dumps({
        "source_id": int(evo_src), "target_id": int(evo_dst),
        "condition": {"min_level": 99}, "priority": 1})})
    test("insert::evolution_info", res)
    if res.get("success") and res.get("data"):
        evo_id = int(res["data"][0]["id"])
        test("set::evolution_info", run_api("set::evolution_info", {"data": json.dumps({
            "id": evo_id, "source_id": int(evo_src), "target_id": int(evo_dst),
            "condition": {"min_level": 98}, "priority": 2})}))
        test("delete::evolution_info", run_api("delete::evolution_info", {"id": str(evo_id)}))

# ============ 14. Map wild-pokemon direct actions ============
print("\n-- Map Wild Pokemon Actions --")
test("get_wild_pokemons_for_map", run_api("get_wild_pokemons_for_map", {"map_id": "1"}))
# scratch species to add/remove without polluting seed data
scratch = json.loads(json.dumps(grant_payload))
scratch_species_id = None
base = run_api("get::pokemon_type", {"id": "1"})
if base.get("success") and base.get("data"):
    obj = base["data"][0]
    for k in ("_TYPE", "map", "evolution", "evolutions"):
        obj.pop(k, None)
    obj["name"] = "E2E_MAP_SPECIES"
    obj["id"] = 0
    obj["map_ids"] = []
    res = run_api("insert::pokemon_type", {"data": json.dumps(obj, ensure_ascii=False)})
    test("insert scratch species for map test", res)
    if res.get("success") and res.get("data"):
        scratch_species_id = int(res["data"][0]["id"])

if scratch_species_id:
    test("add_pokemon_to_map", run_api("add_pokemon_to_map", {"map_id": "1", "pokemon_type_id": str(scratch_species_id)}))
    wild = run_api("get_wild_pokemons_for_map", {"map_id": "1"})
    in_map = any(int(w["id"]) == scratch_species_id for w in (wild.get("data") or []))
    check_true("added species appears in wild list", in_map, f"list={[w.get('id') for w in (wild.get('data') or [])]}")
    test("add_pokemon_to_map (duplicate) fails", run_api("add_pokemon_to_map", {"map_id": "1", "pokemon_type_id": str(scratch_species_id)}), expect_success=False)
    test("remove_pokemon_from_map", run_api("remove_pokemon_from_map", {"map_id": "1", "pokemon_type_id": str(scratch_species_id)}))
    wild = run_api("get_wild_pokemons_for_map", {"map_id": "1"})
    gone = all(int(w["id"]) != scratch_species_id for w in (wild.get("data") or []))
    check_true("removed species no longer in wild list", gone, f"list={[w.get('id') for w in (wild.get('data') or [])]}")
    test("remove_pokemon_from_map (not present) fails", run_api("remove_pokemon_from_map", {"map_id": "1", "pokemon_type_id": str(scratch_species_id)}), expect_success=False)
    run_api("delete::pokemon_type", {"id": str(scratch_species_id)})

# ============ 15. SQL console (removed) ============
print("\n-- SQL Console (removed) --")
test("run::sql_console is no longer dispatched", run_api("run::sql_console", {"sql": "SELECT 1 AS v"}), expect_success=False)

# ============ 16. Dispatch negatives ============
print("\n-- Dispatch Negatives --")
test("unknown entity fails", run_api("count::nonexistent_entity"), expect_success=False)
test("unknown op fails", run_api("dance::pokemon_type"), expect_success=False)

# ============ HTTP-level tests ============
def http_admin_tests():
    print("\n" + "=" * 60)
    print("HTTP-level Admin API Tests")
    print("=" * 60)

    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE
    cj = http.cookiejar.CookieJar()
    opener = urllib.request.build_opener(
        urllib.request.HTTPCookieProcessor(cj),
        urllib.request.HTTPSHandler(context=ctx))

    def req(url, data=None, headers=None):
        r = urllib.request.Request(url, data=data, headers=headers or {})
        try:
            resp = opener.open(r, timeout=30)
            return resp.status, resp.read().decode("utf-8", "replace")
        except urllib.error.HTTPError as e:
            return e.code, e.read().decode("utf-8", "replace")

    base = HTTP_BASE

    # login flow (full form with formhash; the legacy lssubmit fast-login
    # channel is rejected by current X5 builds)
    import re as _re
    import hashlib as _hashlib

    def _discuz_login(username, password, cookie_jar, label):
        opener2 = urllib.request.build_opener(
            urllib.request.HTTPCookieProcessor(cookie_jar),
            urllib.request.HTTPSHandler(context=ctx))
        def req2(url, data=None, headers=None):
            r = urllib.request.Request(url, data=data, headers=headers or {})
            try:
                resp = opener2.open(r, timeout=30)
                return resp.status, resp.read().decode("utf-8", "replace")
            except urllib.error.HTTPError as e:
                return e.code, e.read().decode("utf-8", "replace")
        _, text = req2(f"{base}/member.php?mod=logging&action=login")
        m = _re.search(r'name="formhash" value="([0-9a-f]+)"', text) or _re.search(r'formhash=([0-9a-f]+)', text)
        formhash = m.group(1) if m else ""
        body = urllib.parse.urlencode({
            "formhash": formhash,
            "username": username, "password": password,
            "questionid": 0, "answer": "", "cookietime": 2592000,
            "loginsubmit": "yes", "referer": f"{base}/"}).encode()
        _, text = req2(f"{base}/member.php?mod=logging&action=login", data=body)
        check_true(f"login as {label}", any(c.name.endswith("auth") for c in cookie_jar),
                   f"cookies={[c.name for c in cookie_jar]}")
        # 插件 API 与管理接口要求 X-Pm-Formhash（防 CSRF），取登录后的会话 formhash
        _, home = req2(f"{base}/plugin.php?id=pokemon:pokemon")
        m = _re.search(r"const formhash = '([0-9a-f]+)'", home) or _re.search(r'formhash=([0-9a-f]+)', home)
        return m.group(1) if m else ""

    admin_hash = _discuz_login("admin", "admin123", cj, "admin")

    def admin_post(action, extra=None, formhash=None):
        payload = {"action": action}
        payload.update(extra or {})
        status, text = req(f"{base}/plugin.php?id=pokemon:pokemon&index=admin",
                           data=json.dumps(payload).encode(),
                           headers={"Content-Type": "application/json",
                                    "X-Pm-Formhash": admin_hash if formhash is None else formhash})
        try:
            return status, json.loads(text)
        except json.JSONDecodeError:
            return status, {"_raw": text[:300]}

    # unauthorized access must not return dispatch JSON
    noauth = urllib.request.build_opener(urllib.request.HTTPSHandler(context=ctx))
    try:
        r = noauth.open(urllib.request.Request(
            f"{base}/plugin.php?id=pokemon:pokemon&index=admin",
            data=json.dumps({"action": "count::pokemon_type"}).encode(),
            headers={"Content-Type": "application/json"}), timeout=30)
        anon_text = r.read().decode("utf-8", "replace")
        anon_status = r.status
    except urllib.error.HTTPError as e:
        anon_text = e.read().decode("utf-8", "replace")
        anon_status = e.code
    check_true("anonymous admin POST rejected (no JSON envelope)",
               '"success":true' not in anon_text, f"status={anon_status} body={anon_text[:120]!r}")

    # ---- 宠物中心版主准入回归：板块版主可进后台，普通用户仍被拒 ----
    sql("INSERT INTO pre_forum_forum (fid, name, type, status, level) VALUES (990, '宠物中心', 'forum', 1, 0) "
        "ON DUPLICATE KEY UPDATE name='宠物中心'")
    _salt = "e2esalt"
    _pw_hash = _hashlib.md5((_hashlib.md5("e2emod123".encode()).hexdigest() + _salt).encode()).hexdigest()
    sql("INSERT INTO pre_common_member (username, loginname, password, email, groupid, regdate, salt) "
        f"VALUES ('e2e_center_mod', 'e2e_center_mod', '{_pw_hash}', 'mod@test.local', 10, UNIX_TIMESTAMP(), '{_salt}') "
        "ON DUPLICATE KEY UPDATE username=username")
    mod_uid = sql_scalar("SELECT uid FROM pre_common_member WHERE username='e2e_center_mod'")
    if mod_uid:
        sql(f"DELETE FROM pre_forum_moderator WHERE uid={mod_uid}")
        sql(f"INSERT INTO pre_forum_moderator (uid, fid) VALUES ({mod_uid}, 990)")

        mod_cj = http.cookiejar.CookieJar()
        mod_hash = _discuz_login("e2e_center_mod", "e2emod123", mod_cj, "center moderator")
        mod_opener = urllib.request.build_opener(
            urllib.request.HTTPCookieProcessor(mod_cj),
            urllib.request.HTTPSHandler(context=ctx))

        def mod_post(payload):
            try:
                r = mod_opener.open(urllib.request.Request(
                    f"{base}/plugin.php?id=pokemon:pokemon&index=admin",
                    data=json.dumps(payload).encode(),
                    headers={"Content-Type": "application/json", "X-Pm-Formhash": mod_hash}), timeout=30)
                text = r.read().decode("utf-8", "replace")
            except urllib.error.HTTPError as e:
                text = e.read().decode("utf-8", "replace")
            try:
                return json.loads(text)
            except json.JSONDecodeError:
                return {"_raw": text[:300]}

        try:
            r = mod_opener.open(urllib.request.Request(
                f"{base}/plugin.php?id=pokemon:pokemon&index=admin",
                data=json.dumps({"action": "count::pokemon_type"}).encode(),
                headers={"Content-Type": "application/json", "X-Pm-Formhash": mod_hash}), timeout=30)
            mod_text = r.read().decode("utf-8", "replace")
            mod_status = r.status
        except urllib.error.HTTPError as e:
            mod_text = e.read().decode("utf-8", "replace")
            mod_status = e.code
        check_true("center-board moderator can dispatch admin API (JSON envelope)",
                   '"success":true' in mod_text,
                   f"status={mod_status} body={mod_text[:120]!r}")
        try:
            mod_json = json.loads(mod_text)
            check_true("center-board moderator count returns data", isinstance(mod_json.get("data"), list),
                       f"body={mod_text[:120]!r}")
        except json.JSONDecodeError:
            pass

        # SQL 控制台已移除，版主与管理员都不能再执行 SQL
        before = sql_scalar("SELECT COUNT(*) FROM pm_config")
        test("moderator sql_console is gone",
             mod_post({"action": "run::sql_console", "sql": "DELETE FROM pm_config"}), expect_success=False)
        check_true("pm_config intact after moderator sql_console attempt",
                   sql_scalar("SELECT COUNT(*) FROM pm_config") == before, f"before={before}")

    # 普通用户（非版主、非管理员）必须仍被拒
    _pw2 = _hashlib.md5((_hashlib.md5("e2euser123".encode()).hexdigest() + _salt).encode()).hexdigest()
    sql("INSERT INTO pre_common_member (username, loginname, password, email, groupid, regdate, salt) "
        f"VALUES ('e2e_plain_user', 'e2e_plain_user', '{_pw2}', 'plain@test.local', 10, UNIX_TIMESTAMP(), '{_salt}') "
        "ON DUPLICATE KEY UPDATE username=username")
    plain_uid = sql_scalar("SELECT uid FROM pre_common_member WHERE username='e2e_plain_user'")
    if plain_uid:
        sql(f"DELETE FROM pre_forum_moderator WHERE uid={plain_uid}")
        plain_cj = http.cookiejar.CookieJar()
        _discuz_login("e2e_plain_user", "e2euser123", plain_cj, "plain user")
        plain_opener = urllib.request.build_opener(
            urllib.request.HTTPCookieProcessor(plain_cj),
            urllib.request.HTTPSHandler(context=ctx))
        try:
            r = plain_opener.open(urllib.request.Request(
                f"{base}/plugin.php?id=pokemon:pokemon&index=admin",
                data=json.dumps({"action": "count::pokemon_type"}).encode(),
                headers={"Content-Type": "application/json"}), timeout=30)
            plain_text = r.read().decode("utf-8", "replace")
            plain_status = r.status
        except urllib.error.HTTPError as e:
            plain_text = e.read().decode("utf-8", "replace")
            plain_status = e.code
        check_true("plain member admin POST rejected (no JSON envelope)",
                   '"success":true' not in plain_text, f"status={plain_status} body={plain_text[:120]!r}")

    # CSRF：管理接口与游戏 API 缺 formhash 或 formhash 错误都必须拒绝
    status, out = admin_post("count::pokemon_type", formhash="")
    test("admin POST without formhash rejected", out if isinstance(out, dict) else {"_raw": out}, expect_success=False)
    status, out = admin_post("count::pokemon_type", formhash="00000000")
    test("admin POST with wrong formhash rejected", out if isinstance(out, dict) else {"_raw": out}, expect_success=False)
    status, text = req(f"{base}/plugin.php?id=pokemon:pokemon&endpoint=boss&action=get_config&map_id=1")
    check_true("game API without formhash rejected", '"success":true' not in text, f"status={status} body={text[:120]!r}")

    status, out = admin_post("run::sql_console", {"sql": "SELECT 1"})
    test("admin sql_console is gone", out if isinstance(out, dict) else {"_raw": out}, expect_success=False)

    # representative dispatch actions over the wire
    status, out = admin_post("count::pokemon_type")
    test("HTTP count::pokemon_type", out if isinstance(out, dict) else {"_raw": out})
    status, out = admin_post("list::user_info", {"from": "0", "count": "2"})
    test("HTTP list::user_info", out if isinstance(out, dict) else {"_raw": out})

    # grant pet over the wire
    wire_payload = json.loads(json.dumps(grant_payload))
    wire_payload["name"] = "E2E_HTTP"
    wire_payload["skills"] = []
    status, out = admin_post("insert::pokemon_info", {"data": json.dumps(wire_payload, ensure_ascii=False)})
    test("HTTP insert::pokemon_info", out if isinstance(out, dict) else {"_raw": out})
    http_pet_id = int(out["data"][0]["id"]) if out.get("success") and out.get("data") else None

    # grant money over the wire
    status, out = admin_post("set::user_info", {"data": '{"id":1,"money":23456}'})
    test("HTTP set::user_info", out if isinstance(out, dict) else {"_raw": out})

    # map direct actions over the wire
    status, out = admin_post("get_wild_pokemons_for_map", {"map_id": "1"})
    test("HTTP get_wild_pokemons_for_map", out if isinstance(out, dict) else {"_raw": out})

    # boss endpoint via pokemon.inc.php routing
    status, text = req(f"{base}/plugin.php?id=pokemon:pokemon&endpoint=boss&action=get_config&map_id=1",
                       headers={"X-Pm-Formhash": admin_hash})
    try:
        boss = json.loads(text)
        ok = isinstance(boss, dict) and boss.get("success") is True and "boss_config" in boss
    except json.JSONDecodeError:
        ok = False
    check_true("HTTP boss get_config", ok, f"status={status} body={text[:120]!r}")

    if http_pet_id:
        status, out = admin_post("delete::pokemon_info", {"id": str(http_pet_id)})
        test("HTTP delete::pokemon_info (cleanup)", out if isinstance(out, dict) else {"_raw": out})

if "--http" in sys.argv:
    http_admin_tests()
else:
    print("\n(skipping HTTP-level tests; pass --http to enable)")

# Summary
total = passed + failed + warned
print(f"\n=== {passed}/{total} passed ({warned} warnings, {failed} failed) ===")
for d in details:
    print(f"  {d}")

sys.exit(0 if failed == 0 else 1)
