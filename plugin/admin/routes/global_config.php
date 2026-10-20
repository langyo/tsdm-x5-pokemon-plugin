<?php

function get_global_config()
{
  require_once __DIR__ . '/../../announcements.php';
  $news_announcements = pm_get_news_announcements();
  // 兜底：确保必要的配置项存在
  $required_configs = [
    // 基础设定
    'is_open' => '1',  // boolean: 1 = true, 0 = false
    'version' => 'Unknown',
    'ann_title' => '',
    'ann_url' => '',
    // 亲密度设定
    'intimacy_increase_per_hour' => '1',
    'intimacy_increase_per_earn_xp' => '1',
    'intimacy_increase_multiple' => '1',
    // 治疗设定
    'revive_time' => '10',
    'medical_price' => '10',
    'posts_count_for_wakeup' => '10',
    // 捕捉设定
    'is_enable_catch' => '1',
    'catch_price_in_catch_area' => '10',
    'pve_catch_xp_multiple' => '1',
    'pve_catch_level' => '1',
    // PVE 设定
    'is_enable_earn_xp_on_pve' => '1',
    'drop_money_on_pve' => '1',
    'drop_money_config_by_global' => '0',
    'drop_money_percent_min_on_pve' => '0',
    'drop_money_percent_max_on_pve' => '0',
    'drop_money_level_gap' => '10',  // 0 = 不限制
    'drop_money_daily_limit' => '0',  // 0 = 不限制
    // 宠物蛋设定
    'is_enable_egg' => '1',
    'is_enable_buy_egg' => '1',
    'is_enable_buy_pokemon' => '1',
    'is_enable_multiple_eggs' => '1',
    'egg_price' => '10',
    'hatch_egg_intimacy' => '10',
    // 其他
    'is_enable_pvp' => '1',
  ];
  foreach ($required_configs as $key => $default_value) {
    $exists = DB::fetch_first("SELECT * FROM pm_config WHERE `key`='$key'");
    if (!$exists) {
      $data_type = is_bool($default_value) ? 'boolean' : (is_int($default_value) ? 'integer' : 'string');
      if (is_bool($default_value)) {
        $value = $default_value ? '1' : '0';
      } else {
        $value = strval($default_value);
      }
      pm_config_upsert($key, $value, $data_type);
    }
  }

  $rows = DB::fetch_all("SELECT * from pm_config");
  if (!empty($rows)) {
    $item = [];
    $item['_TYPE'] = "global_config";

    foreach ($rows as $query) {
      $val = null;

      switch ($query['data_type']) {
        case "string":
          $val = strval($query['value']);
          // 对 news_announcements 进行 JSON 解码
          if ($query['key'] === 'news_announcements') {
            $decoded = json_decode($val, true);
            if (is_array($decoded)) {
              $val = $decoded;
            }
          }
          break;
        case "integer":
          $val = intval($query['value']);
          break;
        case "boolean":
          $val = boolval($query['value']);
          break;
        default:
          $json_ret = [];
          $json_ret["success"] = false;
          $json_ret["reason"] = "未知的配置项类型 " . $query['data_type'] . " " . $query['key'];
          exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
      }
      $item[$query['key']] = $val;
    }
    $item['news_announcements'] = $news_announcements;
    return [$item];
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "查询不到任何有效配置数据";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function set_global_config($data)
{
  require_once __DIR__ . '/../../config_schema.php';
  // DDL must precede all setting writes because ALTER TABLE commits implicitly.
  foreach ($data as $key => $value) {
    if ($key === 'news_announcements' && is_array($value)) {
      $value = json_encode($value, JSON_UNESCAPED_UNICODE);
    }
    if (is_string($value)) pm_ensure_config_value_capacity($value);
  }
  foreach ($data as $key => $value) {
    $value = $data[$key];
    if ($query = DB::fetch_first(pm_sql('SELECT * from pm_config where `key`=%s', $key))) {
      if ($query['data_type'] == "string") {
        // 特殊处理 news_announcements：如果是数组，转换为 JSON 字符串
        if ($key === 'news_announcements' && is_array($value)) {
          $value = json_encode($value, JSON_UNESCAPED_UNICODE);
        }
        DB::query(pm_sql('UPDATE pm_config SET `value`=%s WHERE `key`=%s', $value, $key));
      } else if ($query['data_type'] == "integer") {
        $value = intval($value);
        DB::query(pm_sql('UPDATE pm_config SET `value`=%s WHERE `key`=%s', $value, $key));
      } else if ($query['data_type'] == "boolean") {
        $value = boolval($value) ? '1' : '0';
        DB::query(pm_sql('UPDATE pm_config SET `value`=%s WHERE `key`=%s', $value, $key));
      } else {
        $json_ret = [];
        $json_ret["success"] = false;
        $json_ret["reason"] = "未知的配置项类型 " . $query['data_type'] . " " . $key;
        exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
      }
    } else {
      // 键不存在，检查是否是已知的配置项，如果是则自动创建
      $known_configs = [
        // 基础设定
        'news_announcements' => 'string',
        'is_open' => 'boolean',
        'version' => 'string',
        'ann_title' => 'string',
        'ann_url' => 'string',
        // 亲密度设定
        'intimacy_increase_per_hour' => 'integer',
        'intimacy_increase_per_earn_xp' => 'integer',
        'intimacy_increase_multiple' => 'integer',
        // 治疗设定
        'revive_time' => 'integer',
        'medical_price' => 'integer',
        'posts_count_for_wakeup' => 'integer',
        // 捕捉设定
        'is_enable_catch' => 'boolean',
        'catch_price_in_catch_area' => 'integer',
        'pve_catch_xp_multiple' => 'integer',
        'pve_catch_level' => 'integer',
        // PVE 设定
        'is_enable_earn_xp_on_pve' => 'boolean',
        'drop_money_on_pve' => 'boolean',
        'drop_money_config_by_global' => 'boolean',
        'drop_money_percent_min_on_pve' => 'integer',
        'drop_money_percent_max_on_pve' => 'integer',
        'drop_money_level_gap' => 'integer',
        'drop_money_daily_limit' => 'integer',
        // 宠物蛋设定
        'is_enable_egg' => 'boolean',
        'is_enable_buy_egg' => 'boolean',
        'is_enable_buy_pokemon' => 'boolean',
        'is_enable_multiple_eggs' => 'boolean',
        'egg_price' => 'integer',
        'hatch_egg_intimacy' => 'integer',
        // 其他
        'is_enable_pvp' => 'boolean',
      ];

      if (isset($known_configs[$key])) {
        $data_type = $known_configs[$key];
        if ($data_type == "string") {
          // 特殊处理 news_announcements：如果是数组，转换为 JSON 字符串
          if ($key === 'news_announcements' && is_array($value)) {
            $value = json_encode($value, JSON_UNESCAPED_UNICODE);
          }
        } else if ($data_type == "integer") {
          $value = intval($value);
        } else if ($data_type == "boolean") {
          $value = boolval($value) ? '1' : '0';
        }
        pm_config_upsert($key, strval($value), $data_type, true);
      } else {
        // 对于未知键，忽略而不是报错
        continue;
      }
    }
  }
  return get_global_config();
}
