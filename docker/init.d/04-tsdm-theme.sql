-- Register tsdm_newWing (天使新翼) style for Discuz X5.
-- Defensive script: no-ops while Discuz core tables are absent (e.g. during
-- MariaDB first init, before Discuz install.php creates pre_common_* tables).
-- docker/setup.sh re-applies this file after Discuz is ready, so the style
-- is registered on fresh installs.

SET @tsdm_theme_ok = (
    SELECT COUNT(*) FROM information_schema.TABLES
    WHERE TABLE_SCHEMA = DATABASE()
      AND TABLE_NAME IN ('pre_common_template', 'pre_common_style', 'pre_common_stylevar')
);

-- Register template directory
INSERT IGNORE INTO pre_common_template (name, directory)
SELECT 're_tsdm_newWing', './template/re_tsdm_newWing'
FROM DUAL
WHERE @tsdm_theme_ok = 3
  AND NOT EXISTS (SELECT 1 FROM pre_common_template WHERE directory = './template/re_tsdm_newWing');

-- Register style (天使新翼) bound to the template directory
SET @tsdm_theme_templateid = (
    SELECT templateid FROM pre_common_template
    WHERE directory = './template/re_tsdm_newWing' LIMIT 1
);
INSERT IGNORE INTO pre_common_style (name, available, templateid, version)
SELECT '天使新翼', 1, @tsdm_theme_templateid, 'X5.0'
FROM DUAL
WHERE @tsdm_theme_ok = 3
  AND @tsdm_theme_templateid IS NOT NULL
  AND NOT EXISTS (SELECT 1 FROM pre_common_style WHERE name = '天使新翼');

-- Apply theme color and layout variables
SET @tsdm_theme_styleid = (
    SELECT styleid FROM pre_common_style WHERE name = '天使新翼' LIMIT 1
);
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'menuhoverbgcolor', '#d33774' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'menubgcolor', '#d33774' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'link', '#333' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'highlightlink', '#e7628d' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'lightlink', '#f2f2f2' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'noticetext', '#e7628d' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'bgcolor', '#fcfcfc' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'wrapbg', '#fefefe' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'wrapbordercolor', '#f7d9e5' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'contentseparate', '#fae0e8' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'commonbg', '#fafafa' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'commonborder', '#fae0e8' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'inputbg', '#fcfcfc' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'titlebgcolor', '#f6d7e3' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'menuhovertext', '#60cff0' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'menutext', '#fafafa' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'floatbgcolor', '#fcfcfc' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'floatmaskbgcolor', '#d33774' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'dropmenubgcolor', '#FEFEFE' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'specialbg', '#fcf5f8' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'specialborder', '#ea9fbc' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'btnbg', '#fcf5f8' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'btntxt', '#d33774' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'fontsize', '12px/1.5' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'font', 'Tahoma,Helvetica,"Microsoft Yahei","Simsun"' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
INSERT IGNORE INTO pre_common_stylevar (styleid, variable, substitute)
SELECT @tsdm_theme_styleid, 'contentwidth', '630px' FROM DUAL
WHERE @tsdm_theme_ok = 3 AND @tsdm_theme_styleid IS NOT NULL;
