-- Sample seed data for development
SET sql_mode = '';

INSERT IGNORE INTO pm_itemdata (id, name, tpname, txt, shop, money, type) VALUES 
(1, '精灵球', 'normal_ball', '用于捕捉野生精灵', 1, 200, 2),
(2, '超级球', 'great_ball', '比普通精灵球更高级', 1, 600, 2),
(3, '伤药', 'potion', '回复20点HP', 1, 100, 1);

INSERT IGNORE INTO pm_map (id, name, kg, minlevel, maxlevel, exp, site, expn, region) VALUES 
(1, '101号道路', 1, 2, 5, 30, 'grassland', '', '初代'),
(2, '常磐森林', 1, 3, 8, 50, 'forest', '', '初代');

INSERT IGNORE INTO pm_evolution (from_id, to_id, method, condition_value) VALUES 
(1, 2, 'level', '16'),
(2, 3, 'level', '32');

INSERT IGNORE INTO pm_skill (id, name, pmid, txt, lv, num, type, tn, category) VALUES 
(1, '撞击', 'k,1,2,4,5,7,10,13,14,16,19,k', '用身体撞击对方', 1, 35, 1, '一般', '物理'),
(2, '飞叶快刀', 'k,1,2,3,k', '用锋利的叶片切砍', 10, 25, 1, '草', '特殊');
