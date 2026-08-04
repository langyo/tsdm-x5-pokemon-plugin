#!/usr/bin/env python3
"""Convert the archived X2 database dump (pm.sql) into X5 canonical-schema INSERT SQL.

Reads the full X2 backup dump (old column names: txt/sd/god/hpn/minmoney...),
transforms every pm_* table into the canonical X5 schema (new column names +
JSON-consolidated effort_values / drop_money / effects / equipment), and emits
a single importable .sql file.

Usage:
    python x2_to_x5_export.py <input.sql> <output.sql>
"""

import csv
import io
import json
import re
import sys


def parse_inserts(text: str):
    """Yield (table, rows) for every `INSERT INTO `pm_x` VALUES ...;` block."""
    pattern = re.compile(
        r"INSERT INTO `(pm_\w+)` VALUES\s*", re.IGNORECASE
    )
    pos = 0
    while True:
        m = pattern.search(text, pos)
        if not m:
            return
        table = m.group(1)
        start = m.end()
        # find the terminating ';' not inside quotes
        i = start
        in_str = False
        esc = False
        while i < len(text):
            c = text[i]
            if in_str:
                if esc:
                    esc = False
                elif c == "\\":
                    esc = True
                elif c == "'":
                    in_str = False
            elif c == "'":
                in_str = True
            elif c == ";":
                break
            i += 1
        body = text[start:i].strip()
        pos = i + 1
        if not body:
            continue
        rows = parse_values_tuples(body)
        if rows is not None:
            yield table, rows


def parse_values_tuples(body: str):
    """Parse `(v1, v2), (v3, v4);` into a list of value lists."""
    rows = []
    i = 0
    n = len(body)
    while i < n:
        if body[i] != "(":
            i += 1
            continue
        i += 1
        vals = []
        in_str = False
        esc = False
        cur = ""
        while i < n:
            c = body[i]
            if in_str:
                if esc:
                    cur += c
                    esc = False
                elif c == "\\":
                    esc = True
                elif c == "'":
                    in_str = False
                else:
                    cur += c
            else:
                if c == "'":
                    in_str = True
                    cur = ""
                elif c == ",":
                    vals.append(cur.strip())
                    cur = ""
                elif c == ")":
                    vals.append(cur.strip())
                    cur = ""
                    rows.append(vals)
                    break
                else:
                    cur += c
            i += 1
    return rows


def esc(v):
    """Render a value for the output SQL."""
    if v is None:
        return "NULL"
    if isinstance(v, (int, float)):
        return str(v)
    s = str(v)
    return "'" + s.replace("\\", "\\\\").replace("'", "\\'") + "'"


def num(v):
    if v is None or v == "":
        return 0
    try:
        return int(v)
    except ValueError:
        return 0


def norm_uid(v):
    return num(v)


# ---------------------------------------------------------------- mappings

def row_pm_data(v):
    """old: id,name,money,txt,sex,xs,xs2,hp,atk,def,spatk,spdef,sd,mapid,
    capture,met,shop,hpn,atkn,defn,spatkn,spdefn,sdn,birth,birthodds,
    pnclevel,god,minmoney,maxmoney,strength"""
    (i, name, money, txt, sex, xs, xs2, hp, atk, d, spatk, spdef, sd,
     mapid, capture, met, shop, hpn, atkn, defn, spatkn, spdefn, sdn,
     birth, birthodds, pnclevel, god, minmoney, maxmoney, strength) = v[:30]
    ev = json.dumps(
        {
            "hp": num(hpn), "atk": num(atkn), "def": num(defn),
            "spatk": num(spatkn), "spdef": num(spdefn), "spd": num(sdn),
        },
        ensure_ascii=False,
    )
    dm = "[%s,%s]" % (num(minmoney), num(maxmoney))
    return [
        num(i), name, num(money), txt or "", num(sex), xs or "", xs2 or "",
        num(hp), num(atk), num(d), num(spatk), num(spdef), num(sd),
        mapid or "", num(capture), num(met), num(shop), ev,
        num(birth), num(god), dm, num(strength),
    ]


def row_pm_itemdata(v):
    """old: id,name,tpname,shop,money,txt,type,lvask,xsask,addhp,addexp,
    addlv,addgood,ballid,upitem,captmax,captmin,sitemname,hot,zbtype,
    equipment_hp,equipment_atk,equipment_def,equipment_spatk,
    equipment_spdef,equipment_sd"""
    (i, name, tpname, shop, money, txt, typ, lvask, xsask, addhp, addexp,
     addlv, addgood, ballid, upitem, captmax, captmin, sitemname, hot,
     zbtype, eq_hp, eq_atk, eq_def, eq_spatk, eq_spdef, eq_sd) = v[:26]
    effects = json.dumps(
        {
            "hp": num(addhp), "exp": num(addexp), "level": num(addlv),
            "intimacy": num(addgood),
        },
        ensure_ascii=False,
    )
    equip = json.dumps(
        {
            "hp": num(eq_hp), "atk": num(eq_atk), "def": num(eq_def),
            "spatk": num(eq_spatk), "spdef": num(eq_spdef), "spd": num(eq_sd),
        },
        ensure_ascii=False,
    )
    return [
        num(i), name, tpname or "", txt or "", num(shop), num(money),
        num(typ), "", num(lvask), xsask or "", effects, num(ballid),
        num(upitem), num(captmax), sitemname or "", num(zbtype), equip,
    ]


def row_pm_map(v):
    """old: id,name,kg,minlevel,maxlevel,expn,site,exp"""
    (i, name, kg, minlevel, maxlevel, expn, site, exp_) = v[:8]
    return [
        num(i), name, num(kg), num(minlevel), num(maxlevel), num(exp_),
        site or "", expn or "", "", 50, 50,
    ]


def row_pm_skill(v):
    """old: id,pmid,name,txt,lv,powr,num,type,tn,category"""
    (i, pmid, name, txt, lv, powr, num_u, typ, tn, category) = v[:10]
    return [
        num(i), pmid or "", name, txt or "", num(lv), num(powr), num(num_u),
        num(typ), tn or "", category or "",
    ]


def row_pm_mypm(v):
    """old: id,pctime,uid,pmname,nowname,pmno,level,exp,sex,sx,hp,hpg,
    atkg,defg,spatkg,spdefg,sdg,good,itemevolve,ballid,site,state,
    statetime,gduptime,hpn,atkn,defn,spatkn,spdefn,sdn,initialuid,swap,
    wakenum,sg,equipmentid1,equipmentid2,equipmentid3,equipmentid4,sx2,txkg,tx"""
    (i, pctime, uid, pmname, nowname, pmno, level, exp, sex, sx, hp, hpg,
     atkg, defg, spatkg, spdefg, sdg, good, itemevolve, ballid, site,
     state, statetime, gduptime, hpn, atkn, defn, spatkn, spdefn, sdn,
     initialuid, swap, wakenum, sg, eq1, eq2, eq3, eq4, sx2, txkg, tx) = v[:41]
    # X2 stores arbitrary 0-100 values in sg; X3/X5 semantics: only ==1 is shiny
    is_shiny = 1 if num(sg) == 1 else 0
    return [
        num(i), num(pctime), norm_uid(uid), pmname or "", nowname or "",
        num(pmno), num(level), num(exp), num(sex), sx or "", num(hp),
        num(hpg), num(atkg), num(defg), num(spatkg), num(spdefg), num(sdg),
        num(good), num(itemevolve), num(ballid), num(site), num(state),
        num(statetime), num(gduptime), num(hpn), num(atkn), num(defn),
        num(spatkn), num(spdefn), num(sdn), norm_uid(initialuid), num(swap),
        is_shiny, num(eq1), num(eq2), num(eq3), num(eq4), 0,
    ]


def row_pm_usersdata(v):
    """old: uid,username,npcid,hpg,hp,atkg,spatkg,defg,spdefg,sdg,allure,
    capture,level,ppkname,ppktime,ppkround,ppk,ppkfight,ppkdodge,ppkot,
    ppkpriority,exchanguid,exchangepmid,dataall,datawin,datalost,fullexp,
    npcsg,boxnum,strength,pkid,hpn,atkn,defn,spatkn,spdefn,sdn,money"""
    (uid, username, npcid, hpg, hp, atkg, spatkg, defg, spdefg, sdg, allure,
     capture, level, ppkname, ppktime, ppkround, ppk, ppkfight, ppkdodge,
     ppkot, ppkpriority, exchanguid, exchangepmid, dataall, datawin,
     datalost, fullexp, npcsg, boxnum, strength, pkid, hpn, atkn, defn,
     spatkn, spdefn, sdn, money) = v[:38]
    return [
        norm_uid(uid), num(npcid), num(hpg), num(hp), num(atkg), num(spatkg),
        num(defg), num(spdefg), num(sdg), num(allure), num(capture),
        num(level), num(dataall), num(datawin), num(datalost), num(fullexp),
        num(boxnum), num(strength), 100, num(money),
    ]


def row_pm_myskill(v):
    """old: uid,petid,skillid,skillnum (no id column)"""
    (uid, petid, skillid, skillnum) = v[:4]
    return [norm_uid(uid), num(petid), num(skillid), num(skillnum)]


def row_pm_myitem(v):
    """old: id,itemid,num,ball,pmid,uid"""
    (i, itemid, num_u, ball, pmid, uid) = v[:6]
    return [num(i), norm_uid(uid), str(itemid), num(num_u)]


def row_pm_up(v):
    """old evolution table -> pm_evolution: id,pmid,cond,val,targetpmid,priority"""
    (i, pmid, cond, val, targetpmid, priority) = v[:6]
    return [num(i), num(pmid), num(targetpmid), cond or "level", val or "", num(priority)]


# table -> (output columns, row transformer)
MAPPINGS = {
    "pm_config": (["key", "value", "data_type"], lambda v: v[:3]),
    "pm_data": (
        ["id", "name", "money", "description", "sex", "xs", "xs2", "hp",
         "atk", "def", "spatk", "spdef", "speed", "mapid", "capture",
         "met", "shop", "effort_values", "birth", "is_legendary",
         "drop_money", "strength"],
        row_pm_data,
    ),
    "pm_itemdata": (
        ["id", "name", "tpname", "description", "shop", "money", "type",
         "module", "lvask", "xsask", "effects", "ballid", "upitem",
         "captmax", "sitemname", "zbtype", "equipment"],
        row_pm_itemdata,
    ),
    "pm_map": (
        ["id", "name", "is_enabled", "min_level", "max_level", "experience",
         "site", "boss_config", "region", "pos_x", "pos_y"],
        row_pm_map,
    ),
    "pm_skill": (
        ["id", "available_pokemons", "name", "description", "level_required",
         "power", "max_uses", "type", "element", "category"],
        row_pm_skill,
    ),
    "pm_mypm": (
        ["id", "pctime", "uid", "pmname", "nickname", "species_id", "level",
         "exp", "sex", "sx", "hp", "hpg", "atkg", "defg", "spatkg",
         "spdefg", "sdg", "good", "itemevolve", "ballid", "site", "state",
         "statetime", "gduptime", "hpn", "atkn", "defn", "spatkn", "spdefn",
         "sdn", "initialuid", "swap", "is_shiny", "equipmentid1",
         "equipmentid2", "equipmentid3", "equipmentid4", "created_at"],
        row_pm_mypm,
    ),
    "pm_usersdata": (
        ["uid", "npcid", "hpg", "hp", "atkg", "spatkg", "defg", "spdefg",
         "sdg", "allure", "capture", "level", "dataall", "datawin",
         "datalost", "fullexp", "boxnum", "strength", "str", "money"],
        row_pm_usersdata,
    ),
    "pm_myskill": (
        ["uid", "petid", "skillid", "skillnum"],
        row_pm_myskill,
    ),
    "pm_myitem": (
        ["id", "uid", "itemid", "nums"],
        row_pm_myitem,
    ),
    "pm_up": (
        ["id", "from_id", "to_id", "method", "condition_value", "priority"],
        row_pm_up,
    ),
}

# output table name for pm_up
RENAME = {"pm_up": "pm_evolution"}

# not migrated (dropped in X5)
SKIP = {"pm_sitemm"}


def main():
    if len(sys.argv) != 3:
        print("usage: x2_to_x5_export.py <input.sql> <output.sql>")
        return 1
    src, dst = sys.argv[1], sys.argv[2]

    with open(src, "r", encoding="utf-8-sig", errors="replace") as f:
        text = f.read()

    out = io.StringIO()
    out.write("-- ============================================================\n")
    out.write("-- TSDM Pokemon Plugin — X2 full data import (X5 canonical schema)\n")
    out.write("-- Generated by scripts/backup/x2_to_x5_export.py\n")
    out.write("-- Source: X2 database backup (pm.sql)\n")
    out.write("-- Usage: import after install.php has created the tables\n")
    out.write("-- ============================================================\n\n")
    out.write("SET sql_mode = '';\n")
    out.write("SET NAMES utf8mb4;\n\n")

    stats = {}
    pending = {t: 0 for t in MAPPINGS}
    buffers = {t: [] for t in MAPPINGS}
    BATCH = 100

    for table, rows in parse_inserts(text):
        if table in SKIP:
            continue
        if table not in MAPPINGS:
            print(f"[warn] unhandled table {table} ({len(rows)} rows) — skipped")
            continue
        cols, fn = MAPPINGS[table]
        out_name = RENAME.get(table, table)
        for v in rows:
            try:
                new_row = fn(v)
            except Exception as e:
                print(f"[warn] {table} row {v[:4]} transform failed: {e}")
                continue
            buffers[table].append(new_row)
            if len(buffers[table]) >= BATCH:
                flush(out, out_name, cols, buffers[table])
                stats[table] = stats.get(table, 0) + len(buffers[table])
                buffers[table] = []

    for table, rows in buffers.items():
        if rows:
            out_name = RENAME.get(table, table)
            flush(out, out_name, MAPPINGS[table][0], rows)
            stats[table] = stats.get(table, 0) + len(rows)

    with open(dst, "w", encoding="utf-8") as f:
        f.write(out.getvalue())

    print("=== conversion complete ===")
    for t in sorted(stats):
        print(f"  {RENAME.get(t, t):16s} {stats[t]:>7d} rows")
    total = sum(stats.values())
    print(f"  {'TOTAL':16s} {total:>7d} rows")
    print(f"output: {dst}")


def flush(out, table, cols, rows):
    col_sql = ", ".join("`%s`" % c for c in cols)
    out.write(f"INSERT INTO `{table}` ({col_sql}) VALUES\n")
    parts = []
    for r in rows:
        parts.append("(" + ", ".join(esc(x) for x in r) + ")")
    out.write(",\n".join(parts))
    out.write(";\n")


if __name__ == "__main__":
    sys.exit(main())
