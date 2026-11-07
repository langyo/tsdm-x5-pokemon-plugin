-- Repair skill #19 (剑舞) imported from the original X5 Docker seed.
-- Only for installations that used docker/init.d/09-pokemon-skills.sql.
-- The original seed declared the skill category as 变化 (status move) but kept
-- power = 40. The battle engine 2 rules bind on_after_move main effects to
-- power-0 moves and on_hit secondary effects to power>0 moves
-- (effect_hooks_match_skill_power), and the skill's effect #1 (sword_dance,
-- stages_boost atk +2 self) fires on_after_move — so power 40 makes the pair
-- incompatible: set_effect_data / effect re-binding rejects any save touching
-- effect #1 while skill #19 carries power 40, and validate_skill_effect_id
-- blocks re-binding the skill to the effect as well.
-- The engine's rules_version-1 endpoints still coerce power 0 to 40 for
-- legacy battle numerics, so this change is safe for old battles.
-- Exact original-row guards leave customized skills unchanged. Safe to rerun.
-- Back up the database before applying; use the target site's configured database.

UPDATE `pm_skill` SET `power` = '0' WHERE `id` = 19 AND `name` = '剑舞' AND `power` = '40' AND `category` = '变化' AND `effect_id` = 1;
