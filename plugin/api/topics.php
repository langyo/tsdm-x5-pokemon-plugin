<?php

/**
 * 话题数据API
 *
 * 端点:
 * - GET  ?action=list    获取话题列表
 */

// 如果通过路由访问，加载API辅助函数
if (defined('API_ROUTED')) {
  require_once __DIR__ . '/index.php';
} elseif (!defined('IN_DISCUZ')) {
  require_once __DIR__ . '/bootstrap.php';
  require_once __DIR__ . '/index.php';
}

global $_G;

// IDE 类型提示
if (false) {
  function validate_required_param($input, $key, $type = 'string', $options = []) {}
  function validate_optional_param($input, $key, $default = null, $type = 'string', $options = []) {}
  function validate_int_range($value, $name, $min, $max) {}
}

$action = get_param('action', '');

switch ($action) {
  case 'list':
    api_get_topics();
    break;

  default:
    api_error('Invalid action', 400);
}

/**
 * 获取话题列表
 */
function api_get_topics()
{
  global $_G;

  // 从插件配置中获取论坛版块ID
  $settings = isset($_G['cache']['plugin']['pokemon']) ? $_G['cache']['plugin']['pokemon'] : array();
  $fid = isset($settings['fid']) ? intval($settings['fid']) : 0;

  // 如果 fid 没有配置，使用默认值 2
  if (!$fid) {
    $fid = 2;
  }

  // 获取 limit 参数（简化处理，避免验证失败）
  $limit_param = get_param('limit', 6);
  $limit = intval($limit_param);
  if ($limit < 1 || $limit > 20) {
    $limit = 6;
  }

  // 获取新闻公告配置（最多 6 条）
  $news_announcements = array();
  try {
    $news_config = DB::fetch_first("SELECT * FROM pm_config WHERE `key` = 'news_announcements'");
    if ($news_config) {
      $decoded = json_decode(stripslashes($news_config['value']), true);
      if (is_array($decoded)) {
        // 限制最多 6 条
        $news_announcements = array_slice($decoded, 0, 6);
      }
    } else {
      // 兜底：如果 news_announcements 键不存在，自动创建空数组
      // 同时尝试从旧的 ann_title/ann_url 迁移数据
      $old_title = DB::fetch_first("SELECT * FROM pm_config WHERE `key` = 'ann_title'");
      $old_url = DB::fetch_first("SELECT * FROM pm_config WHERE `key` = 'ann_url'");
      if ($old_title && $old_url) {
        $title = stripslashes($old_title['value']);
        $url = stripslashes($old_url['value']);
        if (!empty($title)) {
          $news_announcements = array(array('title' => $title, 'url' => $url));
        }
      }

      // 同步到数据库（使用 INSERT IGNORE 避免重复插入）
      $json_value = json_encode($news_announcements);
      @DB::query(pm_sql(
          "INSERT IGNORE INTO pm_config (`key`, `value`, `data_type`) VALUES ('news_announcements', %s, 'string')",
          $json_value
      ));
    }
  } catch (Exception $e) {
    // 忽略错误，使用空数组
  }

  // 查询最新话题
  // displayorder>=0: 只显示正常帖子和置顶帖子
  // displayorder>0: 置顶帖子
  $topics = array();
  try {
    $fid_escaped = intval($fid);
    $limit_escaped = intval($limit);
    $rows = DB::fetch_all(pm_sql(
      "SELECT tid, subject, dateline, displayorder, author, authorid, views, replies
           FROM " . DB::table('forum_thread') . "
           WHERE fid = %d AND displayorder >= 0
           ORDER BY displayorder DESC, lastpost DESC
           LIMIT 0, %d",
      $fid_escaped, $limit_escaped
    ));

    foreach ($rows as $topic) {
      $topics[] = [
        'id' => (int) $topic['tid'],
        'title' => $topic['subject'],
        'author' => $topic['author'],
        'author_id' => (int) $topic['authorid'],
        'date' => date('m-d', $topic['dateline']),
        'timestamp' => (int) $topic['dateline'],
        'views' => (int) $topic['views'],
        'replies' => (int) $topic['replies'],
        'is_pinned' => (int) $topic['displayorder'] > 0,
      ];
    }
  } catch (Exception $e) {
    // 查询失败，返回空数组
  }

  api_success([
    'news_announcements' => $news_announcements,
    'topics' => $topics,
    'total' => count($topics),
  ]);
}
