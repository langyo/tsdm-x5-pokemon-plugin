<?php

function count_evolution_info()
{
  $count = DB::result_first("SELECT count(*) from pm_evolution");

  $item = [];
  $item["count"] = intval($count);

  $item["_TYPE"] = "::count";

  return [$item];
}

function list_evolution_info($from, $count)
{
  $from = intval($from);
  $count = intval($count);

  $ret = [];
  $rows = DB::fetch_all("SELECT * from pm_evolution order by `id` asc, `priority` asc limit $from,$count");
  if (!empty($rows)) {
    $pokemon_ids = [];
    $item_ids = [];
    foreach ($rows as $query) {
      $pokemon_ids[intval($query['from_id'])] = true;
      $pokemon_ids[intval($query['to_id'])] = true;
      if ($query['cond'] == 'item') {
        $item_ids[intval($query['val'])] = true;
      }
    }

    // 批量查询宠物名称
    $pokemon_names = [];
    if (!empty($pokemon_ids)) {
      $ids_str = implode(',', array_keys($pokemon_ids));
      $name_rows = DB::fetch_all("SELECT id, name from pm_data where id in ($ids_str)");
      foreach ($name_rows as $row) {
        $pokemon_names[intval($row['id'])] = $row['name'];
      }
    }

    // 批量查询道具名称
    $item_names = [];
    if (!empty($item_ids)) {
      $ids_str = implode(',', array_keys($item_ids));
      $item_rows = DB::fetch_all("SELECT id, name from pm_itemdata where id in ($ids_str)");
      foreach ($item_rows as $row) {
        $item_names[intval($row['id'])] = $row['name'];
      }
    }

    // 构建结果
    foreach ($rows as $query) {
      $source_id = intval($query['from_id']);
      $target_id = intval($query['to_id']);
      // 获取道具名称
      $item_name = null;
      if ($query['cond'] == 'item' && isset($item_names[intval($query['val'])])) {
        $item_name = $item_names[intval($query['val'])];
      }
      $item = new_evolution_info(
        intval($query['id']),
        $source_id,
        $target_id,
        isset($pokemon_names[$source_id]) ? $pokemon_names[$source_id] : '',
        isset($pokemon_names[$target_id]) ? $pokemon_names[$target_id] : '',
        new_evolution_limit_type($query['cond'], $query['val'], $item_name),
        intval($query['priority']),
        $item_name
      );

      $item["_TYPE"] = "evolution_info";
      array_push($ret, $item);
    }
    return $ret;
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "数据库无法访问";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function get_evolution_info($id)
{
  $id = intval($id);

  $ret = [];
  if ($query = DB::fetch_first("SELECT * from pm_evolution where `id`=$id")) {
    $source_id = intval($query['from_id']);
    $target_id = intval($query['to_id']);

    // 查询宠物名称
    $pokemon_names = [];
    $ids = [$source_id, $target_id];
    $ids_str = implode(',', $ids);
    $name_rows = DB::fetch_all("SELECT id, name from pm_data where id in ($ids_str)");
    foreach ($name_rows as $row) {
      $pokemon_names[intval($row['id'])] = $row['name'];
    }

    // 查询道具名称
    $item_name = null;
    if ($query['cond'] == 'item') {
      $item_query = DB::fetch_first("SELECT name from pm_itemdata where id=" . intval($query['val']));
      if ($item_query) {
        $item_name = $item_query['name'];
      }
    }

    $item = new_evolution_info(
      intval($query['id']),
      $source_id,
      $target_id,
      isset($pokemon_names[$source_id]) ? $pokemon_names[$source_id] : '',
      isset($pokemon_names[$target_id]) ? $pokemon_names[$target_id] : '',
      new_evolution_limit_type($query['cond'], $query['val'], $item_name),
      intval($query['priority']),
      $item_name
    );

    $item["_TYPE"] = "evolution_info";
    array_push($ret, $item);

    return $ret;
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法查询进化路线 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function set_evolution_info($info)
{
  $id = intval($info["id"]);

  if ($query = DB::fetch_first("SELECT * from pm_evolution where `id`=$id")) {
    if (intval($query['from_id']) != intval($info["source_id"])) {
      DB::query("UPDATE pm_evolution set `from_id`=" . intval($info["source_id"]) . " where `id`=$id");
    }
    if (intval($query['to_id']) != intval($info["target_id"])) {
      DB::query("UPDATE pm_evolution set `to_id`=" . intval($info["target_id"]) . " where `id`=$id");
    }
    $cond = translate_evolution_info_label_to_db_cond($info["condition"]);
    if ($query['cond'] != $cond[0]) {
      DB::query("UPDATE pm_evolution set `cond`='" . $cond[0] . "' where `id`=$id");
    }
    if ($query['val'] != $cond[1]) {
      DB::query("UPDATE pm_evolution set `val`='" . $cond[1] . "' where `id`=$id");
    }
    if (intval($query['priority']) != intval($info["priority"])) {
      DB::query("UPDATE pm_evolution set `priority`=" . intval($info["priority"]) . " where `id`=$id");
    }
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "进化路线更新失败，未找到 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function insert_evolution_info($info)
{
  $source_id = intval($info["source_id"]);
  $target_id = intval($info["target_id"]);
  $cond = translate_evolution_info_label_to_db_cond($info["condition"]);
  $priority = intval($info["priority"]);

  $last_id = DB::fetch_first("SELECT id from pm_evolution order by id desc limit 1");
  $last_id = intval($last_id['id']);
  $new_id = $last_id + 1;

  DB::query("INSERT INTO pm_evolution (
    `id`, `from_id`, `to_id`, `cond`, `val`, `priority`
  ) VALUES (
    $new_id, $source_id, $target_id, '" .
    $cond[0] . "', '" .
    $cond[1] . "', 
    $priority
  )");

  return $new_id;
}

function delete_evolution_info($id)
{
  $id = intval($id);
  DB::query("DELETE from pm_evolution where `id`=$id");
}

function filter_evolution_info($list)
{
  $ret = [];

  foreach ($list as $item) {
    $query_sql = "SELECT * from pm_evolution where ";
    $query_sql_list = [];
    $operator = $item["operator"];
    $value = addslashes($item["value"]);

    switch ($item["tag"]) {
      case "ID":
        array_push($query_sql_list, generate_filter_sql('id', $operator, $value, 'id'));
        break;
      case '进化来源':
        // 智能判断：如果是纯数字，优先按 ID 精确查询
        if (ctype_digit($value) && $value !== '') {
          // 先尝试 ID 精确查询
          $id_query_sql = "SELECT * from pm_evolution where from_id = " . intval($value);
          $found_id_results = DB::fetch_all($id_query_sql);
          if (!empty($found_id_results)) {
            // 收集所有 source_id 和 target_id
            $pokemon_ids = [];
            foreach ($found_id_results as $query) {
              $pokemon_ids[intval($query['from_id'])] = true;
              $pokemon_ids[intval($query['to_id'])] = true;
            }

            // 批量查询宠物名称
            $pokemon_names = [];
            if (!empty($pokemon_ids)) {
              $ids_str = implode(',', array_keys($pokemon_ids));
              $name_rows = DB::fetch_all("SELECT id, name from pm_data where id in ($ids_str)");
              foreach ($name_rows as $row) {
                $pokemon_names[intval($row['id'])] = $row['name'];
              }
            }

            foreach ($found_id_results as $query) {
              $source_id = intval($query['from_id']);
              $target_id = intval($query['to_id']);
              $item = new_evolution_info(
                intval($query['id']),
                $source_id,
                $target_id,
                isset($pokemon_names[$source_id]) ? $pokemon_names[$source_id] : "未知宠物 #$source_id",
                isset($pokemon_names[$target_id]) ? $pokemon_names[$target_id] : "未知宠物 #$target_id",
                new_evolution_limit_type($query['cond'], $query['val'], null),
                intval($query['priority']),
                null
              );

              $item["_TYPE"] = "evolution_info";
              array_push($ret, $item);
            }
            return $ret;
          }
        }
        // ID 查询无结果或非数字输入，使用名称模糊搜索
        array_push($query_sql_list, generate_filter_sql('from_id', $operator, $value, 'text'));
        break;
      case '进化目标':
        // 智能判断：如果是纯数字，优先按 ID 精确查询
        if (ctype_digit($value) && $value !== '') {
          // 先尝试 ID 精确查询
          $id_query_sql = "SELECT * from pm_evolution where to_id = " . intval($value);
          $found_id_results = DB::fetch_all($id_query_sql);
          if (!empty($found_id_results)) {
            // 收集所有 source_id 和 target_id
            $pokemon_ids = [];
            foreach ($found_id_results as $query) {
              $pokemon_ids[intval($query['from_id'])] = true;
              $pokemon_ids[intval($query['to_id'])] = true;
            }

            // 批量查询宠物名称
            $pokemon_names = [];
            if (!empty($pokemon_ids)) {
              $ids_str = implode(',', array_keys($pokemon_ids));
              $name_rows = DB::fetch_all("SELECT id, name from pm_data where id in ($ids_str)");
              foreach ($name_rows as $row) {
                $pokemon_names[intval($row['id'])] = $row['name'];
              }
            }

            foreach ($found_id_results as $query) {
              $source_id = intval($query['from_id']);
              $target_id = intval($query['to_id']);
              $item = new_evolution_info(
                intval($query['id']),
                $source_id,
                $target_id,
                isset($pokemon_names[$source_id]) ? $pokemon_names[$source_id] : "未知宠物 #$source_id",
                isset($pokemon_names[$target_id]) ? $pokemon_names[$target_id] : "未知宠物 #$target_id",
                new_evolution_limit_type($query['cond'], $query['val'], null),
                intval($query['priority']),
                null
              );

              $item["_TYPE"] = "evolution_info";
              array_push($ret, $item);
            }
            return $ret;
          }
        }
        // ID 查询无结果或非数字输入，使用名称模糊搜索
        array_push($query_sql_list, generate_filter_sql('to_id', $operator, $value, 'text'));
        break;
      default:
    }
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
    $pokemon_ids = [];
    foreach ($rows as $query) {
      $pokemon_ids[intval($query['from_id'])] = true;
      $pokemon_ids[intval($query['to_id'])] = true;
    }

    // 批量查询宠物名称
    $pokemon_names = [];
    if (!empty($pokemon_ids)) {
      $ids_str = implode(',', array_keys($pokemon_ids));
      $name_rows = DB::fetch_all("SELECT id, name from pm_data where id in ($ids_str)");
      foreach ($name_rows as $row) {
        $pokemon_names[intval($row['id'])] = $row['name'];
      }
    }

    // 构建结果
    foreach ($rows as $query) {
      $source_id = intval($query['from_id']);
      $target_id = intval($query['to_id']);
      $item = new_evolution_info(
        intval($query['id']),
        $source_id,
        $target_id,
        isset($pokemon_names[$source_id]) ? $pokemon_names[$source_id] : '',
        isset($pokemon_names[$target_id]) ? $pokemon_names[$target_id] : '',
        new_evolution_limit_type($query['cond'], $query['val'], null),
        intval($query['priority']),
        null
      );

      $item["_TYPE"] = "evolution_info";
      array_push($ret, $item);
    }
  }

  return $ret;
}
