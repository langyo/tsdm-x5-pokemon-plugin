<?php

function translate_map_alpha_to_full_name($str)
{
  switch ($str) {
    case 'l':
      return 'plain';
    case 'g':
      return 'grass';
    case 'p':
      return 'water';
    case 's':
      return 'sea';
    case 'b':
      return 'sea_bottom';
    case 'm':
      return 'mountain';
    case 'c':
      return 'cave';
    case 'd':
      return 'sand';
    case 'f':
      return 'factory';
    case 't':
      return 'base';
    case 'v':
      return 'town';
    case 'n':
      return 'gym';
    case 'h':
      return 'sky';
    case 'o':
      return 'deep_sea';
    case 'k':
      return 'lava';
    default:
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "未知的地图类型 $str";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function translate_map_full_name_to_alpha($str)
{
  switch ($str) {
    case 'plain':
      return 'l';
    case 'grass':
      return 'g';
    case 'water':
      return 'p';
    case 'sea':
      return 's';
    case 'sea_bottom':
      return 'b';
    case 'mountain':
      return 'm';
    case 'cave':
      return 'c';
    case 'sand':
      return 'd';
    case 'factory':
      return 'f';
    case 'base':
      return 't';
    case 'town':
      return 'v';
    case 'gym':
      return 'n';
    case 'sky':
      return 'h';
    case 'deep_sea':
      return 'o';
    case 'lava':
      return 'k';
    default:
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "未知的地图类型 $str";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

/** Convert both current and legacy admin mode payloads to the stored map fields. */
function translate_map_mode_to_db_fields($info, $current_experience = 0)
{
  $mode = $info['mode'] ?? null;
  $data = is_array($mode) ? ($mode['data'] ?? []) : $info;
  $mode_type = is_array($mode) ? ($mode['mode'] ?? '') : $mode;
  $experience = max(0, intval($data['experience'] ?? $current_experience));

  if ($mode_type === 'boss' || $mode_type === 'hybrid') {
    $boss_config = ['bosses' => $data['bosses'] ?? []];
    return [
      $mode_type === 'boss' ? -1 : $experience,
      json_encode($boss_config, JSON_UNESCAPED_UNICODE),
      true,
    ];
  }
  if ($mode_type === 'wild') {
    return [$experience, strval(max(0, intval($data['experience_increase_times'] ?? 0))), false];
  }

  // Older clients send the Boss configuration directly instead of a mode tag.
  $has_boss_config = !empty($info['boss_config']['bosses']);
  return [
    $current_experience,
    $has_boss_config ? json_encode($info['boss_config'], JSON_UNESCAPED_UNICODE) : '',
    $has_boss_config,
  ];
}

function new_map_info(
  $id,
  $name,
  $area_type,

  $is_enabled,
  $min_level,
  $max_level,
  $experience,
  $expn_raw,
  $min_quality = 0
) {
  $ret = [];

  $ret["_TYPE"] = "map_info";
  $ret["id"] = intval($id);
  $ret["name"] = $name;
  $ret["area_type"] = $area_type;

  $ret["is_enabled"] = boolval($is_enabled);
  $ret["min_level"] = intval($min_level);
  $ret["max_level"] = intval($max_level);
  // 活动地图品质门槛：0=不限制，1-8 = E..SS+（见 pm_quality_grades()）
  $ret["min_quality"] = intval($min_quality);

  // 解析地图模式
  // Rust 的 #[serde(tag = "mode", content = "data")] 会生成扁平化格式：
  // Wild: {"mode":"wild","experience":0,"experience_increase_times":0}
  // Boss: {"mode":"boss","bosses":[...]}
  // Hybrid: {"mode":"hybrid","experience":0,"experience_increase_times":0,"bosses":[...]}
  $exp = intval($experience);
  $boss_json = json_decode($expn_raw, true);

  // 检查 boss 配置：必须是数组且有元素
  $has_boss_config = is_array($boss_json) &&
                       isset($boss_json['bosses']) &&
                       is_array($boss_json['bosses']) &&
                       count($boss_json['bosses']) > 0;

  if ($exp === -1 && $has_boss_config) {
    // Boss 模式: {"mode":"boss","bosses":[...]}
    $ret["mode"] = "boss";
    $ret["bosses"] = $boss_json['bosses'];
  } elseif ($exp !== -1 && $has_boss_config) {
    // 混合模式: {"mode":"hybrid","experience":X,"experience_increase_times":0,"bosses":[...]}
    $ret["mode"] = "hybrid";
    $ret["experience"] = $exp;
    $ret["experience_increase_times"] = 0;
    $ret["bosses"] = $boss_json['bosses'];
  } else {
    // 野生模式: {"mode":"wild","experience":X,"experience_increase_times":Y}
    $ret["mode"] = "wild";
    $ret["experience"] = $exp === -1 ? 0 : $exp;
    $ret["experience_increase_times"] = intval($expn_raw);
  }

  // 如果数据库中有旧的 boss 数据格式（空对象等），需要清理
  // 这里不包含 bosses 字段，所以不需要额外处理

  return $ret;
}
