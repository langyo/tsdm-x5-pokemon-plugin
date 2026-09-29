#!/usr/bin/env python3
"""Generate the exp backfill migration from the PHP default exp table.

Extracts $default_exp_table from plugin/api/utils.php and emits
migrations/2026-09-backfill-pokemon-exp.sql so the SQL thresholds can
never drift from the values the PHP API actually uses.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
UTILS = ROOT / "plugin" / "api" / "utils.php"
OUT = ROOT / "migrations" / "2026-09-backfill-pokemon-exp.sql"

source = UTILS.read_text(encoding="utf-8")
block = re.search(r"\$default_exp_table\s*=\s*\[(.*?)\];", source, re.S)
if not block:
    sys.exit("failed to locate $default_exp_table in utils.php")

pairs = re.findall(r"(\d+)\s*=>\s*(\d+)", block.group(1))
table = {int(k): int(v) for k, v in pairs}
levels = sorted(table)
assert levels == list(range(1, 101)), f"expected levels 1..100, got {levels[0]}..{levels[-1]}"

rows = ",\n".join(f"    ({lv}, {table[lv]})" for lv in levels)

sql = f"""-- ============================================================
-- TSDM Pokemon Plugin - 存量宠物经验回填
-- ============================================================
-- 背景：expdata/ 旧经验曲线删除后统一使用默认经验表。存量和捕获
-- 宠物的 exp 低于其等级在默认表中的门槛 exp_table[level-1]，前端
-- exp.saturating_sub(exp_for_current_level) 下溢，经验条恒为 0。
--
-- 修复：把每只宠物的 exp 抬到当前等级的门槛（等级不变，本级从
-- 0% 重新开始积累）。门槛数值由 scripts/migrate/gen_exp_backfill.py
-- 从 plugin/api/utils.php 的 $default_exp_table 自动生成，不会漂移。
--
-- 幂等性：GREATEST 保证重复执行结果不变，可安全重跑。
-- ============================================================

DROP TEMPORARY TABLE IF EXISTS pm_exp_level_threshold;
CREATE TEMPORARY TABLE pm_exp_level_threshold (
    lvl int(10) unsigned NOT NULL PRIMARY KEY,
    threshold int(10) unsigned NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;

INSERT INTO pm_exp_level_threshold (lvl, threshold) VALUES
{rows};

-- level=1 时 level-1=0 越界，GREATEST 归位到 lvl=1（门槛 0，不受影响）
UPDATE `pm_mypm` pm
JOIN pm_exp_level_threshold t
  ON t.lvl = GREATEST(1, pm.level - 1)
SET pm.exp = GREATEST(pm.exp, t.threshold)
WHERE pm.exp < t.threshold;

DROP TEMPORARY TABLE IF EXISTS pm_exp_level_threshold;
"""

OUT.write_text(sql, encoding="utf-8", newline="\n")
print(f"wrote {OUT} with {len(levels)} thresholds")
