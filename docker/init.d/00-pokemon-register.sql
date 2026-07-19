-- Pokemon plugin registration
INSERT IGNORE INTO pre_common_plugin (available, adminid, name, identifier, description, directory, copyright, version, modules) 
VALUES (1, 1, 'TSDM 宠物小精灵', 'pokemon', '天使动漫论坛宠物小精灵插件系统', 'pokemon/', 'TSDM.net', 'X5.0', 
'a:2:{i:0;a:5:{s:4:\"name\";s:7:\"pokemon\";s:4:\"menu\";s:12:\"宠物中心\";s:3:\"url\";s:0:\"\";s:4:\"type\";s:2:\"11\";s:12:\"displayorder\";s:1:\"0\";}i:1;a:5:{s:4:\"name\";s:7:\"admincp\";s:4:\"menu\";s:12:\"宠物管理\";s:3:\"url\";s:0:\"\";s:4:\"type\";s:1:\"3\";s:12:\"displayorder\";s:1:\"0\";}}'
);

SET @pid = (SELECT pluginid FROM pre_common_plugin WHERE identifier='pokemon');
INSERT IGNORE INTO pre_common_pluginvar (pluginid, displayorder, title, description, variable, type, value, extra) VALUES
(@pid, 1, '是否开启宠物系统', '控制宠物系统整体的开关状态', 'is_open', 'radio', '1', ''),
(@pid, 2, '是否启用捕捉功能', '控制是否允许用户捕捉野生宠物', 'is_enable_catch', 'radio', '1', ''),
(@pid, 3, '宠物医疗价格', '在PC中心恢复宠物的价格', 'medical_price', 'number', '0', ''),
(@pid, 4, '宠物蛋价格', '购买宠物蛋的价格', 'egg_price', 'number', '0', ''),
(@pid, 5, '是否启用PVP对战', '控制是否允许玩家之间进行对战', 'is_enable_pvp', 'radio', '1', '');

-- Pokemon navigation item
INSERT IGNORE INTO pre_common_nav (parentid, name, title, url, identifier, target, type, available, displayorder, highlight, level, subtype, subcols, icon, subname, suburl, navtype, logo)
VALUES (0, '宠物中心', 'Pokemon', 'plugin.php?id=pokemon:game', 'pokemon', 0, 0, 1, 6, 0, 0, 0, 0, '', '', '', 0, '');

