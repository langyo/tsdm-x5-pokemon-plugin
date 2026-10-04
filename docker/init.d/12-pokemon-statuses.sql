-- Battle engine 2.0: status condition definitions (issue #75 phase 3).
-- Behavior JSON keys (consumed by battle_core rules_version 2):
--   turn_damage      per-turn end-of-turn damage ratio of max HP ('1/8', '1/16')
--   skip_turn_chance chance the unit cannot act before moving (0-100)
--   speed_divisor    speed divisor while inflicted (paralysis: 2)
--   wake_chance      per-turn natural cure chance (sleep/freeze)
--   min_turns        minimum enforced turns before waking (sleep)
-- Rows here mirror battle_core_status_catalog(); the built-in catalog is the
-- fallback when this table is empty (fresh engine installs before seeding).

LOCK TABLES `pm_status` WRITE;
/*!40000 ALTER TABLE `pm_status` DISABLE KEYS */;
INSERT INTO `pm_status` (`code`, `name`, `behavior_json`, `overlap`, `version`) VALUES
('poison', '中毒', '{"turn_damage":"1/8","skip_turn_chance":0,"wake_chance":0}', 'replace', 1),
('burn', '灼烧', '{"turn_damage":"1/16","skip_turn_chance":0,"wake_chance":0}', 'replace', 1),
('paralysis', '麻痹', '{"turn_damage":null,"skip_turn_chance":25,"speed_divisor":2,"wake_chance":0}', 'replace', 1),
('sleep', '睡眠', '{"turn_damage":null,"skip_turn_chance":100,"wake_chance":33,"min_turns":1}', 'replace', 1),
('freeze', '冰冻', '{"turn_damage":null,"skip_turn_chance":100,"wake_chance":20}', 'replace', 1),
('confusion', '混乱', '{"turn_damage":null,"skip_turn_chance":0,"wake_chance":0}', 'replace', 1);
/*!40000 ALTER TABLE `pm_status` ENABLE KEYS */;
UNLOCK TABLES;
