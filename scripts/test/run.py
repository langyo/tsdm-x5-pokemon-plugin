#!/usr/bin/env python3
"""Static test suite for TSDM Pokemon Plugin — schema consistency and stale column detection.

Runs without Docker. Validates that:
1. 02-pokemon-schema.sql and install.php define identical column sets
2. No PHP file references old (renamed) column names in SQL contexts
3. Seed data INSERT statements use columns that exist in the schema
4. The migration script covers all renamed columns

Usage: python scripts/test/run.py
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCHEMA_SQL = ROOT / "docker" / "init.d" / "02-pokemon-schema.sql"
INSTALL_PHP = ROOT / "plugin" / "install.php"
MIGRATION_SQL = ROOT / "migrations" / "from-x3" / "001_x3_to_x5_migration.sql"
PLUGIN_DIR = ROOT / "plugin"
SEED_DIR = ROOT / "docker" / "init.d"

PASS = 0
FAIL = 0
WARN = 0

def ok(name, detail=""):
    global PASS
    PASS += 1
    print(f"  PASS  {name}" + (f" — {detail}" if detail else ""))

def fail(name, detail=""):
    global FAIL
    FAIL += 1
    print(f"  FAIL  {name}" + (f" — {detail}" if detail else ""))

def warn(name, detail=""):
    global WARN
    WARN += 1
    print(f"  WARN  {name}" + (f" — {detail}" if detail else ""))


def parse_create_tables(sql_text):
    tables = {}
    for m in re.finditer(r'CREATE\s+TABLE\s+IF\s+NOT\s+EXISTS\s+`(\w+)`\s*\((.*?)\)\s*ENGINE', sql_text, re.DOTALL | re.IGNORECASE):
        tname = m.group(1)
        body = m.group(2)
        cols = set()
        for cm in re.finditer(r'`(\w+)`\s+', body):
            col = cm.group(1)
            if col not in ('PRIMARY', 'KEY', 'UNIQUE', 'INDEX', 'ENGINE', 'DEFAULT', 'CHARSET', 'COLLATE'):
                cols.add(col)
        tables[tname] = cols
    return tables


def parse_install_php_schema(php_text):
    sql_match = re.search(r'\$sql\s*=\s*<<<EOF\n(.*?)\nEOF;', php_text, re.DOTALL)
    if not sql_match:
        return {}
    return parse_create_tables(sql_match.group(1))


def test_schema_consistency():
    print("\n=== Schema consistency (02-pokemon-schema.sql vs install.php) ===")
    schema = parse_create_tables(SCHEMA_SQL.read_text(encoding="utf-8", errors="replace"))
    install = parse_install_php_schema(INSTALL_PHP.read_text(encoding="utf-8", errors="replace"))

    if not schema:
        fail("parse 02-pokemon-schema.sql", "no CREATE TABLE found")
        return
    if not install:
        fail("parse install.php", "no CREATE TABLE found")
        return

    ok("parsed schemas", f"02: {len(schema)} tables, install.php: {len(install)} tables")

    all_tables = set(schema.keys()) | set(install.keys())
    for t in sorted(all_tables):
        s_cols = schema.get(t, set())
        i_cols = install.get(t, set())
        only_in_02 = s_cols - i_cols
        only_in_install = i_cols - s_cols
        if only_in_02:
            fail(f"{t}: columns only in 02", ", ".join(sorted(only_in_02)))
        elif only_in_install:
            fail(f"{t}: columns only in install.php", ", ".join(sorted(only_in_install)))
        else:
            ok(f"{t}", f"{len(s_cols)} cols match")

    return schema


OLD_COLUMN_NAMES = {
    "txt": "pm_data/pm_skill/pm_itemdata → description",
    "sd": "pm_data → speed",
    "god": "pm_data → is_legendary",
    "pmno": "pm_mypm → species_id",
    "nowname": "pm_mypm → nickname",
    "sg": "pm_mypm → is_shiny",
    "powr": "pm_skill → power",
    "tn": "pm_skill → element",
    "lv": "pm_skill → level_required",
    "kg": "pm_map → is_enabled",
    "minlevel": "pm_map → min_level",
    "maxlevel": "pm_map → max_level",
    "expn": "pm_map → boss_config",
    "minmoney": "pm_data → drop_money",
    "maxmoney": "pm_data → drop_money",
    "minmoeny": "pm_data → drop_money (typo)",
    "mixmoeny": "pm_data → drop_money (typo)",
    "npcsg": "pm_usersdata → dropped (use allure)",
    "ppkname": "pm_usersdata → dropped",
    "ppktime": "pm_usersdata → dropped",
    "ppkround": "pm_usersdata → dropped",
    "ppkfight": "pm_usersdata → dropped",
    "ppkdodge": "pm_usersdata → dropped",
    "ppkot": "pm_usersdata → dropped",
    "ppkpriority": "pm_usersdata → dropped",
    "exchanguid": "pm_usersdata → dropped",
    "exchangepmid": "pm_usersdata → dropped",
    "birthodds": "pm_data → dropped",
    "pnclevel": "pm_data → dropped",
    "captmin": "pm_itemdata → dropped",
    "ppkallow": "pm_itemdata → dropped",
}

SAFE_CONTEXTS = [
    r"^\s*//",
    r"^\s*\*",
    r"^\s*#",
    r"level_required",
    r"available_pokemons",
    r"equipment_sd",
    r"addhp.*=>.*full_stats",
    r"equipment_hp.*=>.*full_stats",
    r"equipment_atk.*=>.*full_stats",
    r"equipment_def.*=>.*full_stats",
    r"equipment_spatk.*=>.*full_stats",
    r"equipment_spdef.*=>.*full_stats",
]


def test_stale_columns(schema):
    print("\n=== Stale column reference scan ===")
    php_files = list(PLUGIN_DIR.rglob("*.php"))
    stale_found = []

    for fpath in php_files:
        rel = fpath.relative_to(ROOT)
        lines = fpath.read_text(encoding="utf-8", errors="replace").splitlines()
        for i, line in enumerate(lines, 1):
            stripped = line.strip()
            if any(re.match(p, stripped) for p in SAFE_CONTEXTS):
                continue

            for old_name, reason in OLD_COLUMN_NAMES.items():
                patterns = [
                    rf"set\s+{old_name}\s*=",
                    rf"\b{old_name}\s*=\s*%[ds]",
                    rf"SELECT\s+{old_name}\b",
                    rf"INSERT\s+INTO.*\b{old_name}\b",
                    rf"WHERE\s+{old_name}\s*[=!<>]",
                    rf"ORDER\s+BY\s+{old_name}\b",
                ]
                for pat in patterns:
                    if re.search(pat, stripped, re.IGNORECASE):
                        if re.search(rf'\${old_name}\b', stripped) and not re.search(rf"(?<!\$)\b{old_name}\b\s*[=,!<>]", stripped):
                            continue
                        stale_found.append((str(rel), i, old_name, reason, stripped[:100]))
                        break

    if not stale_found:
        ok("no stale SQL column references", f"scanned {len(php_files)} PHP files")
    else:
        for rel, lineno, col, reason, snippet in stale_found:
            fail(f"stale column '{col}'", f"{rel}:{lineno} ({reason}) — {snippet}")

    php_files_for_array = list(PLUGIN_DIR.rglob("*.php"))
    array_stale = []
    for fpath in php_files_for_array:
        rel = fpath.relative_to(ROOT)
        lines = fpath.read_text(encoding="utf-8", errors="replace").splitlines()
        for i, line in enumerate(lines, 1):
            stripped = line.strip()
            if any(re.match(p, stripped) for p in SAFE_CONTEXTS):
                continue
            if "isset" in stripped and "sitemid" in stripped:
                continue
            for old_name in ("npcsg", "ppkname", "exchanguid", "exchangepmid", "birthodds", "pnclevel", "captmin", "ppkallow", "minmoeny", "mixmoeny"):
                if re.search(rf"\['{old_name}'\]", stripped):
                    array_stale.append((str(rel), i, old_name, stripped[:100]))

    if not array_stale:
        ok("no stale array-key references for dropped columns")
    else:
        for rel, lineno, col, snippet in array_stale:
            fail(f"stale array key '{col}'", f"{rel}:{lineno} — {snippet}")


def test_seed_data_columns(schema):
    print("\n=== Seed data column validation ===")
    seed_files = sorted(SEED_DIR.glob("0[4-9]-*.sql"))
    if not seed_files:
        warn("no seed data files found")
        return

    for sf in seed_files:
        content = sf.read_text(encoding="utf-8", errors="replace")
        for m in re.finditer(r'INSERT\s+INTO\s+`(\w+)`\s*\(([^)]+)\)\s*VALUES', content, re.IGNORECASE):
            tname = m.group(1)
            cols_in_insert = set(re.findall(r'`(\w+)`', m.group(2)))
            schema_cols = schema.get(tname, set())
            if not schema_cols:
                warn(f"{sf.name}: table {tname} not in schema")
                continue
            unknown = cols_in_insert - schema_cols
            if unknown:
                fail(f"{sf.name}: {tname} unknown columns", ", ".join(sorted(unknown)))
            else:
                ok(f"{sf.name}: {tname} columns valid", f"{len(cols_in_insert)} cols")


def test_migration_coverage():
    print("\n=== Migration script coverage ===")
    if not MIGRATION_SQL.exists():
        fail("migration script not found", str(MIGRATION_SQL))
        return

    mig = MIGRATION_SQL.read_text(encoding="utf-8", errors="replace")

    required_renames = {
        "description": "txt→description",
        "speed": "sd→speed",
        "is_legendary": "god→is_legendary",
        "effort_values": "EV columns→effort_values",
        "drop_money": "minmoney/maxmoney→drop_money",
        "nickname": "nowname→nickname",
        "species_id": "pmno→species_id",
        "is_shiny": "sg→is_shiny",
        "available_pokemons": "pmid→available_pokemons",
        "level_required": "lv→level_required",
        "power": "powr→power",
        "max_uses": "num→max_uses",
        "element": "tn→element",
        "is_enabled": "kg→is_enabled",
        "min_level": "minlevel→min_level",
        "max_level": "maxlevel→max_level",
        "experience": "exp→experience",
        "boss_config": "expn→boss_config",
        "module": "sitemid→module",
        "effects": "addhp/etc→effects",
        "equipment": "equipment_*/→equipment",
        "priority": "add priority to pm_evolution",
    }

    for new_col, desc in required_renames.items():
        if new_col in mig:
            ok(f"migration covers {new_col}", desc)
        else:
            fail(f"migration missing {new_col}", desc)

    if "DROP COLUMN IF EXISTS" in mig:
        ok("migration drops old columns")
    else:
        fail("migration does not drop old columns")

    if "pm_migration_log" in mig:
        ok("migration logs to pm_migration_log")
    else:
        warn("migration does not log")


def test_php_syntax():
    print("\n=== PHP syntax check ===")
    import shutil
    php = shutil.which("php")
    if not php:
        warn("php CLI not installed — skipping syntax check")
        return

    import subprocess
    php_files = list(PLUGIN_DIR.rglob("*.php"))
    errors = 0
    for fpath in php_files:
        r = subprocess.run([php, "-l", str(fpath)], capture_output=True, text=True)
        if "No syntax errors" not in r.stdout:
            fail(f"php -l {fpath.relative_to(ROOT)}", r.stdout.strip()[:100])
            errors += 1
    if errors == 0:
        ok(f"php -l passed", f"{len(php_files)} files")


def main():
    print("TSDM Pokemon Plugin — Static Test Suite")
    print("=" * 60)

    schema = test_schema_consistency()
    if schema:
        test_stale_columns(schema)
        test_seed_data_columns(schema)
    test_migration_coverage()
    test_php_syntax()

    print("\n" + "=" * 60)
    print(f"Results: {PASS} passed, {FAIL} failed, {WARN} warnings")
    if FAIL > 0:
        sys.exit(1)
    print("All tests passed!")


if __name__ == "__main__":
    main()
