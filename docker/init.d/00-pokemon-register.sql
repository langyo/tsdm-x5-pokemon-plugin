-- Register Pokemon plugin
INSERT IGNORE INTO pre_common_plugin (available, adminid, name, identifier, description, directory, copyright, version, modules) 
VALUES (1, 1, 'TSDM Pokemon', 'pokemon', 'TSDM Pokemon Plugin', 'pokemon/', 'TSDM.net', 'X5.0', 
  'a:2:{i:0;a:5:{s:4:"name";s:4:"game";s:4:"menu";s:12:"pet center";s:3:"url";s:0:"";s:4:"type";s:1:"7";s:12:"displayorder";s:1:"0";}i:1;a:5:{s:4:"name";s:7:"admincp";s:4:"menu";s:5:"admin";s:3:"url";s:0:"";s:4:"type";s:1:"3";s:12:"displayorder";s:1:"0";}}'
);

INSERT IGNORE INTO pre_common_pluginvar (pluginid, displayorder, title, description, variable, type, value, extra)
SELECT pluginid, 1, 'System Open', 'Enable/disable the Pokemon system', 'is_open', 'radio', '1', ''
FROM pre_common_plugin WHERE identifier='pokemon';

INSERT IGNORE INTO pre_common_pluginvar (pluginid, displayorder, title, description, variable, type, value, extra)
SELECT pluginid, 2, 'Enable Catch', 'Allow users to catch wild Pokemon', 'is_enable_catch', 'radio', '1', ''
FROM pre_common_plugin WHERE identifier='pokemon';

INSERT IGNORE INTO pre_common_pluginvar (pluginid, displayorder, title, description, variable, type, value, extra)
SELECT pluginid, 3, 'Medical Price', 'Cost to heal at PC Center', 'medical_price', 'number', '0', ''
FROM pre_common_plugin WHERE identifier='pokemon';

INSERT IGNORE INTO pre_common_pluginvar (pluginid, displayorder, title, description, variable, type, value, extra)
SELECT pluginid, 4, 'Egg Price', 'Cost to buy a Pokemon egg', 'egg_price', 'number', '0', ''
FROM pre_common_plugin WHERE identifier='pokemon';

INSERT IGNORE INTO pre_common_pluginvar (pluginid, displayorder, title, description, variable, type, value, extra)
SELECT pluginid, 5, 'Enable PVP', 'Allow player vs player battles', 'is_enable_pvp', 'radio', '1', ''
FROM pre_common_plugin WHERE identifier='pokemon';

-- Add pokemon badge column
ALTER TABLE pre_common_member_field_forum ADD COLUMN IF NOT EXISTS pokemon TEXT AFTER medals;
