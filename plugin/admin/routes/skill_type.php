<?php

function count_skill_type()
{
  $count = DB::result_first("SELECT count(*) from pm_skill");

  $item = [];
  $item["count"] = intval($count);

  $item["_TYPE"] = "::count";

  return [$item];
}

function list_skill_type($from, $count)
{
  $from = intval($from);
  $count = intval($count);

  $ret = [];
  $rows = DB::fetch_all("SELECT * from pm_skill order by `id` asc limit $from,$count");
  if (!empty($rows)) {
    foreach ($rows as $query) {
      $pokemon_list = explode(',', $query['available_pokemons']);
      array_shift($pokemon_list);
      array_pop($pokemon_list);
      foreach ($pokemon_list as &$pokemon) {
        $pokemon = intval($pokemon);
      }
      $pokemon_list = array_filter($pokemon_list, function ($pokemon) {
        return $pokemon != 0;
      });

      $item = new_skill_type(
        intval($query['id']),
        $query['name'],
        $pokemon_list,
        $query['description'],
        $query['level_required'],
        $query['max_uses'],
        new_skill_effect(
          translate_skill_type_raw_to_id($query['category']),
          translate_chinese_kind_to_kind_id($query['element']),
          $query['power']
        )
      );

      $item["_TYPE"] = "skill_type";
      array_push($ret, $item);
    }
    return $ret;
  } else {
    return [];
  }
}

function get_skill_type($id)
{
  $id = intval($id);

  $query = DB::fetch_first("SELECT * from pm_skill where id=$id");
  if ($query) {
    $pokemon_list = explode(',', $query['available_pokemons']);
    array_shift($pokemon_list);
    array_pop($pokemon_list);
    foreach ($pokemon_list as &$pokemon) {
      $pokemon = intval($pokemon);
    }
    $pokemon_list = array_filter($pokemon_list, function ($pokemon) {
      return $pokemon != 0;
    });
    // 按 ID 排序
    sort($pokemon_list);
    $pokemon_list = array_values($pokemon_list);

    $item = new_skill_type(
      intval($query['id']),
      $query['name'],
      $pokemon_list,
      $query['description'],
      $query['level_required'],
      $query['max_uses'],
      new_skill_effect(
        translate_skill_type_raw_to_id($query['category']),
        translate_chinese_kind_to_kind_id($query['element']),
        $query['power']
      )
    );

    $item["_TYPE"] = "skill_type";
    return [$item];
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法查询技能类型 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function set_skill_type($info)
{
  $id = intval($info["id"]);
  if ($query = DB::fetch_first("SELECT * from pm_skill where id=$id")) {
    // 提前检查，available_pokemons 必须是一个数字数组
    if (!is_array($info["available_pokemons"])) {
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "available_pokemons 必须是一个数字数组";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }

    if ($query['name'] != $info["name"]) {
      DB::query("UPDATE pm_skill set name='{$info["name"]}' where id=$id");
    }

    // 排序并去重
    $available_pokemons = $info["available_pokemons"];
    sort($available_pokemons);
    $available_pokemons = array_values(array_unique($available_pokemons));

    $pokemon_list_str = "k," . implode(',', $available_pokemons) . ",k";
    DB::query("UPDATE pm_skill set available_pokemons='$pokemon_list_str' where id=$id");

    if ($query['description'] != $info["description"]) {
      DB::query("UPDATE pm_skill set description='{$info["description"]}' where id=$id");
    }

    if (intval($query['level_required']) != intval($info["min_level_limit"])) {
      DB::query("UPDATE pm_skill set level_required='{$info["min_level_limit"]}' where id=$id");
    }

    if (intval($query['max_uses']) != intval($info["use_times_limit"])) {
      DB::query("UPDATE pm_skill set max_uses='{$info["use_times_limit"]}' where id=$id");
    }

    $effect = translate_skill_type_obj_to_raw($info["effect"]);
    $category = $effect[0];
    $pokemon_type = $effect[1];
    $damage = intval($effect[2]);
    if ($query['category'] != $category) {
      DB::query("UPDATE pm_skill set category='$category' where id=$id");
    }
    if ($query['element'] != $pokemon_type) {
      DB::query("UPDATE pm_skill set element='$pokemon_type' where id=$id");
    }
    if (intval($query['power']) != $damage) {
      DB::query("UPDATE pm_skill set power='$damage' where id=$id");
    }
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "技能类型更新失败，未找到 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function insert_skill_type($info)
{
  // 提前检查，available_pokemons 必须是一个数字数组
  if (!is_array($info["available_pokemons"])) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "available_pokemons 必须是一个数字数组";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $name = strval($info["name"]);

  // 排序并去重
  $available_pokemons = $info["available_pokemons"];
  sort($available_pokemons);
  $available_pokemons = array_values(array_unique($available_pokemons));

  // array_merge 需将技能种族列表作为独立参数展开，否则嵌套数组会被
  // implode 当作 "Array" 字符串写入
  $pmid = implode(',', array_merge(['k'], $available_pokemons, ['k']));
  $txt = strval($info["description"]);
  $lv = intval($info["min_level_limit"]);
  $num = intval($info["use_times_limit"]);

  $effect = translate_skill_type_obj_to_raw($info["effect"]);
  $category = $effect[0];
  $tn = $effect[1];
  $powr = intval($effect[2]);

  $last_id = DB::fetch_first("SELECT id from pm_skill order by id desc limit 1");
  $last_id = intval($last_id['id']);
  $new_id = $last_id + 1;

  DB::query("INSERT INTO pm_skill (
    id, name, available_pokemons, description, level_required, max_uses, category, element, power
  ) VALUES (
    $new_id, '$name', '$pmid', '$txt', $lv, $num, '$category', '$tn', $powr
  )");

  return $new_id;
}

function delete_skill_type($id)
{
  $id = intval($id);
  DB::query("DELETE FROM pm_skill where id=$id");
}

function filter_skill_type($list)
{
  $ret = [];

  foreach ($list as $item) {
    $query_sql = "SELECT * from pm_skill where ";
    $query_sql_list = [];
    $operator = $item["operator"];
    $value = addslashes($item["value"]);

    switch ($item["tag"]) {
      case "ID":
        array_push($query_sql_list, generate_filter_sql('id', $operator, $value, 'id'));
        break;
      case '名称':
        // 智能判断：如果是纯数字，优先按 ID 精确查询
        if (ctype_digit($value) && $value !== '') {
          // 先尝试 ID 精确查询
          $id_query_sql = "SELECT * from pm_skill where id = " . intval($value);
          $found_id_results = DB::fetch_all($id_query_sql);
          if (!empty($found_id_results)) {
              foreach ($found_id_results as $query) {
                $pokemon_list = explode(',', $query['available_pokemons']);
                array_shift($pokemon_list);
                array_pop($pokemon_list);
                foreach ($pokemon_list as &$pokemon) {
                  $pokemon = intval($pokemon);
                }
                $pokemon_list = array_filter($pokemon_list, function ($pokemon) {
                  return $pokemon != 0;
                });
                // 按 ID 排序
                sort($pokemon_list);
                $pokemon_list = array_values($pokemon_list);

                $item = new_skill_type(
                  intval($query['id']),
                  $query['name'],
                  $pokemon_list,
                  $query['description'],
                  $query['level_required'],
                  $query['max_uses'],
                  new_skill_effect(
                    translate_skill_type_raw_to_id($query['category']),
                    translate_chinese_kind_to_kind_id($query['element']),
                    $query['power']
                  )
                );

                $item["_TYPE"] = "skill_type";
                array_push($ret, $item);
              }
              return $ret;
            }
          }
        // ID 查询无结果或非数字输入，使用名称模糊搜索
        array_push($query_sql_list, generate_filter_sql('name', $operator, $value, 'text'));
        break;
      case '描述':
        array_push($query_sql_list, generate_filter_sql('description', $operator, $value, 'text'));
        break;
      case '可学习此技能的宠物种族':
      case '可用此的种族':
        // available_pokemons 格式是 "k,1,2,3,k"，需要搜索 ",xxx," 格式
        if ($operator === 'equal' || $operator === 'contains') {
          $pokemon_id = intval($value);
          array_push($query_sql_list, "available_pokemons like '%,$pokemon_id,%'");
        }
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
    foreach ($rows as $query) {
      $pokemon_list = explode(',', $query['available_pokemons']);
      array_shift($pokemon_list);
      array_pop($pokemon_list);
      foreach ($pokemon_list as &$pokemon) {
        $pokemon = intval($pokemon);
      }
      $pokemon_list = array_filter($pokemon_list, function ($pokemon) {
        return $pokemon != 0;
      });

      $item = new_skill_type(
        intval($query['id']),
        $query['name'],
        $pokemon_list,
        $query['description'],
        $query['level_required'],
        $query['max_uses'],
        new_skill_effect(
          translate_skill_type_raw_to_id($query['category']),
          translate_chinese_kind_to_kind_id($query['element']),
          $query['power']
        )
      );

      $item["_TYPE"] = "skill_type";
      array_push($ret, $item);
    }
  }

  return $ret;
}
