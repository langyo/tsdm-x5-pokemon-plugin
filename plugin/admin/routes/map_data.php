<?php

function count_map_info()
{
  $count = DB::result_first("SELECT count(*) from pm_map");

  $item = [];
  $item["count"] = intval($count);

  return [$item];
}

function list_map_info($from, $count)
{
  $from = intval($from);
  $count = intval($count);

  $ret = [];
  $rows = DB::fetch_all("SELECT * from pm_map order by `id` asc limit $from,$count");
  if (!empty($rows)) {
    foreach ($rows as $query) {
      $item = new_map_info(
        intval($query['id']),
        $query['name'],
        $query['site'],
        intval($query['is_enabled']) == 1,
        intval($query['min_level']),
        intval($query['max_level']),
        intval($query['experience']),
        $query['boss_config']
      );

      array_push($ret, $item);
    }
    return $ret;
  } else {
    return [];
  }
}

function get_map_info($id)
{
  $id = intval($id);

  $ret = [];
  if ($query = DB::fetch_first("SELECT * from pm_map where id={$id}")) {
    $item = new_map_info(
      intval($query['id']),
      $query['name'],
      $query['site'],
      intval($query['is_enabled']) == 1,
      intval($query['min_level']),
      intval($query['max_level']),
      intval($query['experience']),
      $query['boss_config']
    );

    array_push($ret, $item);

    return $ret;
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法查询地图类型 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function set_map_info($info)
{
  $id = intval($info["id"]);

  if ($query = DB::fetch_first("SELECT * from pm_map where id={$id}")) {
    // 提前检查，地图类型必须在已知类型中
    translate_map_alpha_to_full_name($info["area_type"]);

    // 提前检查，最低等级必须高于或等于 1 级
    if (intval($info["min_level"]) < 1) {
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "最低等级必须高于或等于 1 级";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }

    // 提前检查，最高等级必须低于或等于 255 级
    if (intval($info["max_level"]) > 255) {
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "最高等级必须低于或等于 255 级";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }

    // 提前检查，最低等级不得高于最高等级
    if (intval($info["min_level"]) > intval($info["max_level"])) {
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "最低等级不得高于最高等级";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }

    if ($query['name'] != $info["name"]) {
      $name = addslashes($info["name"]);
      DB::query("UPDATE pm_map set name='$name' where id=$id");
    }
    if ($query['site'] != $info["area_type"]) {
      DB::query("UPDATE pm_map set site='" . addslashes(strval($info["area_type"])) . "' where id=$id");
    }

    if (boolval($query['is_enabled']) != boolval($info["is_enabled"])) {
      $is_enabled = boolval($info["is_enabled"]) ? 1 : 0;
      DB::query("UPDATE pm_map set is_enabled={$is_enabled} where id=$id");
    }
    if (intval($query['min_level']) != intval($info["min_level"])) {
      DB::query("UPDATE pm_map set min_level=" . intval($info["min_level"]) . " where id=$id");
    }
    if (intval($query['max_level']) != intval($info["max_level"])) {
      DB::query("UPDATE pm_map set max_level=" . intval($info["max_level"]) . " where id=$id");
    }

    // experience=-1 identifies Boss-only maps; wild maps keep their numeric multiplier.
    list($new_experience, $new_expn, $has_boss_config) =
      translate_map_mode_to_db_fields($info, intval($query['experience']));
    if (intval($query['experience']) !== $new_experience) {
      DB::query("UPDATE pm_map set experience=$new_experience where id=$id");
    }

    // 如果是 Boss 配置，确保 expn 字段可以存储 JSON
    if ($has_boss_config) {
      $column_info = DB::fetch_first("SHOW COLUMNS FROM pm_map LIKE 'boss_config'");
      if ($column_info && strpos($column_info['Type'], 'text') === false && strpos($column_info['Type'], 'varchar') === false) {
        DB::query("ALTER TABLE pm_map MODIFY COLUMN boss_config TEXT NOT NULL DEFAULT ''");
      }
    }

    // 检查是否需要更新 expn
    $current_expn = $query['boss_config'];
    if ($current_expn != $new_expn) {
      $new_expn_escaped = addslashes($new_expn);
      DB::query("UPDATE pm_map set boss_config='{$new_expn_escaped}' where id=$id");
    }
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "地图类型更新失败，未找到宠物 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function insert_map_info($info)
{
  // 提前检查，地图类型必须在已知类型中
  translate_map_alpha_to_full_name($info["area_type"]);

  // 提前检查，最低等级必须高于或等于 1 级
  if (intval($info["min_level"]) < 1) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "最低等级必须高于或等于 1 级";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 提前检查，最高等级必须低于或等于 255 级
  if (intval($info["max_level"]) > 255) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "最高等级必须低于或等于 255 级";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 提前检查，最低等级不得高于最高等级
  if (intval($info["min_level"]) > intval($info["max_level"])) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "最低等级不得高于最高等级";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $name = addslashes(strval($info["name"]));
  $site = addslashes(strval($info["area_type"]));
  $kg = boolval($info["is_enabled"]) ? 1 : 0;
  $minlevel = intval($info["min_level"]);
  $maxlevel = intval($info["max_level"]);

  list($experience, $expn, $has_boss_config) = translate_map_mode_to_db_fields($info);

  // 如果是 Boss 配置，确保 expn 字段可以存储 JSON
  if ($has_boss_config) {
    $column_info = DB::fetch_first("SHOW COLUMNS FROM pm_map LIKE 'boss_config'");
    if ($column_info && strpos($column_info['Type'], 'text') === false && strpos($column_info['Type'], 'varchar') === false) {
      DB::query("ALTER TABLE pm_map MODIFY COLUMN boss_config TEXT NOT NULL DEFAULT ''");
    }
  }

  $expn_escaped = addslashes($expn);
  DB::query("INSERT INTO pm_map (
    name, site, is_enabled, min_level, max_level, experience, boss_config
  ) VALUES (
    '$name', '$site', $kg, $minlevel, $maxlevel, $experience, '$expn_escaped'
  )");

  return intval(DB::insert_id());
}

function delete_map_info($id)
{
  $id = intval($id);
  DB::query("DELETE FROM pm_map where id={$id}");
}

function filter_map_info($list)
{
  $ret = [];

  $query_sql = "SELECT * from pm_map where ";
  $query_sql_list = [];
  foreach ($list as $item) {
    $condition_count = count($query_sql_list);
    $operator = $item["operator"];
    $value = addslashes($item["value"]);

    switch ($item["tag"]) {
      case "ID":
        array_push($query_sql_list, generate_filter_sql('id', $operator, $value, 'id'));
        break;
      case '名称':
        // A numeric name may select an ID, but must still obey every filter.
        if (in_array($operator, ['equal', 'not_equal'], true) && ctype_digit($value) && $value !== ''
            && DB::fetch_first("SELECT id FROM pm_map WHERE id=" . intval($value))) {
          $query_sql_list[] = generate_filter_sql('id', $operator, intval($value), 'id');
        } else {
          $query_sql_list[] = generate_filter_sql('name', $operator, $value, 'text');
        }
        break;
      case '地形类型':
        array_push(
          $query_sql_list,
          generate_filter_sql(
            'site',
            $operator,
            translate_map_full_name_to_alpha($value),
            'id'
          )
        );
        break;
      case '野怪最低等级':
        array_push($query_sql_list, generate_filter_sql('min_level', $operator, $value, 'number'));
        break;
      case '野怪最高等级':
        array_push($query_sql_list, generate_filter_sql('max_level', $operator, $value, 'number'));
        break;
      default:
        return [];
    }
    if (count($query_sql_list) === $condition_count || end($query_sql_list) === '') return [];
  }

  if (count($query_sql_list) <= 0) {
    return $ret;
  }
  if (trim(implode(" and ", $query_sql_list)) == "") {
    return $ret;
  }
  $query_sql .= implode(" and ", $query_sql_list);
  $query_sql .= " limit 20";

  $rows = DB::fetch_all($query_sql);
  if (!empty($rows)) {
    foreach ($rows as $query) {
      $item = new_map_info(
        intval($query['id']),
        $query['name'],
        $query['site'],
        intval($query['is_enabled']) == 1,
        intval($query['min_level']),
        intval($query['max_level']),
        intval($query['experience']),
        $query['boss_config']
      );

      array_push($ret, $item);
    }
  }

  return $ret;
}

/**
 * 获取地图的野生宠物列表
 * 查询 pm_data 表中 mapid 包含指定地图 ID 的所有宠物
 */
function get_wild_pokemons_for_map($map_id)
{
  $map_id = intval($map_id);

  if ($map_id <= 0) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无效的地图ID";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $ret = [];
  // 使用 FIND_IN_SET 查询 mapid 字段中包含该地图 ID 的宠物
  $rows = DB::fetch_all("SELECT id, name FROM pm_data WHERE FIND_IN_SET({$map_id}, mapid) > 0 ORDER BY id ASC");
  foreach ($rows as $query) {
      $item = [
        "id" => intval($query['id']),
        "name" => $query['name'],
        "_TYPE" => "wild_pokemon_info",
      ];
      array_push($ret, $item);
    }

  return $ret;
}

/**
 * 将宠物类型添加到地图的野生宠物列表
 * 通过更新宠物类型的 mapid 字段实现
 */
function add_pokemon_to_map($map_id, $pokemon_type_id)
{
  $map_id = intval($map_id);
  $pokemon_type_id = intval($pokemon_type_id);

  // 检查参数有效性
  if ($map_id <= 0 || $pokemon_type_id <= 0) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无效的地图ID或宠物类型ID";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 检查地图是否存在
  $map_exists = DB::result_first("SELECT id FROM pm_map WHERE id = {$map_id}");
  if (!$map_exists) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "地图 #{$map_id} 不存在";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 检查宠物类型是否存在
  $pokemon_exists = DB::result_first("SELECT id FROM pm_data WHERE id = {$pokemon_type_id}");
  if (!$pokemon_exists) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "宠物类型 #{$pokemon_type_id} 不存在";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 获取当前宠物类型的 mapid
  $current_mapid = DB::result_first("SELECT mapid FROM pm_data WHERE id = {$pokemon_type_id}");

  // 解析现有的 mapid
  $map_ids = [];
  if (!empty($current_mapid)) {
    $map_ids = array_map('intval', array_filter(explode(',', $current_mapid), 'trim'));
  }

  // 检查是否已经在该地图中
  if (in_array($map_id, $map_ids)) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "宠物 #{$pokemon_type_id} 已在地图 #{$map_id} 中";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 添加新的地图ID
  $map_ids[] = $map_id;
  // 排序并去重
  $map_ids = array_unique($map_ids);
  sort($map_ids, SORT_NUMERIC);
  $new_mapid = implode(',', $map_ids);

  // 更新数据库（成功时返回数据由 dispatch 统一封装，失败保持错误信封退出）
  $result = DB::query("UPDATE pm_data SET mapid = '{$new_mapid}' WHERE id = {$pokemon_type_id}");

  if ($result) {
    return [];
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "更新宠物地图关联失败";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

/**
 * 从地图的野生宠物列表中移除宠物类型
 * 通过更新宠物类型的 mapid 字段实现
 */
function remove_pokemon_from_map($map_id, $pokemon_type_id)
{
  $map_id = intval($map_id);
  $pokemon_type_id = intval($pokemon_type_id);

  // 检查参数有效性
  if ($map_id <= 0 || $pokemon_type_id <= 0) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无效的地图ID或宠物类型ID";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 获取当前宠物类型的 mapid
  $current_mapid = DB::result_first("SELECT mapid FROM pm_data WHERE id = {$pokemon_type_id}");

  if (empty($current_mapid)) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "宠物 #{$pokemon_type_id} 没有关联任何地图";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 解析现有的 mapid
  $map_ids = array_map('intval', array_filter(explode(',', $current_mapid), 'trim'));

  // 移除指定的地图ID
  $map_ids = array_diff($map_ids, [$map_id]);
  // 重新索引数组并排序
  $map_ids = array_values($map_ids);
  sort($map_ids, SORT_NUMERIC);
  $new_mapid = implode(',', $map_ids);

  // 更新数据库（成功时返回数据由 dispatch 统一封装，失败保持错误信封退出）
  $result = DB::query("UPDATE pm_data SET mapid = '{$new_mapid}' WHERE id = {$pokemon_type_id}");

  if ($result) {
    return [];
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "移除宠物地图关联失败";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}
