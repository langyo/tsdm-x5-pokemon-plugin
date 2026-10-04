-- Battle engine 2.0: effect templates and skill wiring (issue #75 phase 4).
-- pm_effect.params_json embeds the core effect declaration:
--   {"code":"<core code>", ...params} — validated by battle_core_validate_effect
--   at load time; anything invalid is dropped, never enters a battle.
-- pm_skill.effect_id links a move to its (primary) effect.

LOCK TABLES `pm_effect` WRITE;
/*!40000 ALTER TABLE `pm_effect` DISABLE KEYS */;
INSERT INTO `pm_effect` (`id`, `code`, `kind`, `hooks_json`, `params_json`, `description`, `version`) VALUES
(1, 'sword_dance', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"atk","stages":2,"target":"self"}', '剑舞：大幅提高自己的攻击。', 1),
(2, 'sand_attack', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"accuracy","stages":-1,"target":"opponent"}', '泼沙：降低对手的命中。', 1),
(3, 'tail_whip', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"def","stages":-1,"target":"opponent"}', '摇尾巴：降低对手的防御。', 1),
(4, 'leer', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"def","stages":-1,"target":"opponent"}', '瞪眼：用犀利的眼神威吓，降低对手的防御。', 1),
(5, 'growl', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"atk","stages":-1,"target":"opponent"}', '叫声：可爱的叫声让对手疏忽，降低对手的攻击。', 1),
(6, 'harden', 'move', '["on_after_move"]', '{"code":"stages_boost","stat":"def","stages":1,"target":"self"}', '变硬：提高自己的防御。', 1),
(7, 'poison_point', 'move', '["on_hit"]', '{"code":"status_inflict","status":"poison","chance":30}', '毒针附加：命中后有概率使对手中毒。', 1),
(8, 'flame_body', 'move', '["on_hit"]', '{"code":"status_inflict","status":"burn","chance":30}', '火焰之躯附加：命中后有概率使对手灼烧。', 1),
(9, 'static_abil', 'move', '["on_hit"]', '{"code":"status_inflict","status":"paralysis","chance":30}', '静电附加：命中后有概率使对手麻痹。', 1),
(10, 'sleep_powder', 'move', '["on_after_move"]', '{"code":"status_inflict","status":"sleep","chance":75}', '催眠粉：高概率使对手入睡。', 1);
/*!40000 ALTER TABLE `pm_effect` ENABLE KEYS */;
UNLOCK TABLES;

-- Wire the existing power-0 status moves to their effects (id references from
-- docker/init.d/09-pokemon-skills.sql). Attack moves keep effect_id=0 for now;
-- admins can attach poison/burn/paralysis riders from the admin editor later.
UPDATE `pm_skill` SET `effect_id` = 1 WHERE `id` = 19;  -- 剑舞（变化）
UPDATE `pm_skill` SET `effect_id` = 2 WHERE `id` = 28;  -- 泼沙
UPDATE `pm_skill` SET `effect_id` = 3 WHERE `id` = 39;  -- 摇尾巴
UPDATE `pm_skill` SET `effect_id` = 4 WHERE `id` = 43;  -- 瞪眼
UPDATE `pm_skill` SET `effect_id` = 5 WHERE `id` = 45;  -- 叫声
