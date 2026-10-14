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
      $item["effect_id"] = isset($query['effect_id']) ? intval($query['effect_id']) : 0;
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
    $item["effect_id"] = isset($query['effect_id']) ? intval($query['effect_id']) : 0;
    return [$item];
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法查询技能类型 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function validate_skill_effect_id($info, $power, $current_id = 0, $current_power = null)
{
  // Older clients do not know this field: only an explicit zero unbinds it.
  if (!array_key_exists('effect_id', $info)) {
    if ($current_power === null || intval($power) === intval($current_power)) return intval($current_id);
    $value = intval($current_id);
  } else {
    $value = $info['effect_id'];
  }
  if ((!is_int($value) && !(is_string($value) && ctype_digit($value))) || $value < 0 || $value > 4294967295) {
    exit(json_encode(['success' => false, 'reason' => 'effect_id 必须是非负整数'], JSON_UNESCAPED_UNICODE));
  }
  $effect_id = intval($value);
  if ($effect_id > 0) {
    $row = DB::fetch_first("SELECT * from pm_effect where id=$effect_id");
    if (!$row) {
      exit(json_encode(['success' => false, 'reason' => "effect_id #$effect_id 不存在（pm_effect）"], JSON_UNESCAPED_UNICODE));
    }
    // Preserve legacy bindings for unrelated edits, but validate a new binding
    // or changed power before writing any other skill field.
    if ($effect_id !== intval($current_id) || $current_power === null || intval($power) !== intval($current_power)) {
      require_once __DIR__ . '/effect_data.php';
      require_once __DIR__ . '/../../api/battle_core.php';
      if (!effect_row_matches_skill_power($row, $power)) {
        exit(json_encode(['success' => false, 'reason' => '技能威力与效果类型或触发时机不相容'], JSON_UNESCAPED_UNICODE));
      }
    }
  }
  return $effect_id;
}

function set_skill_type($info)
{
  if (!is_array($info)) {
    exit(json_encode(['success' => false, 'reason' => '技能资料必须是对象'], JSON_UNESCAPED_UNICODE));
  }
  // 在任何字段写入前验证效果，防止失败时留下部分修改。
  $effect = translate_skill_type_obj_to_raw($info["effect"] ?? null);
  $id = intval($info["id"]);
  if ($query = DB::fetch_first("SELECT * from pm_skill where id=$id")) {
    $effect_id = validate_skill_effect_id($info, $effect[2], $query['effect_id'] ?? 0, $query['power']);
    // 提前检查，available_pokemons 必须是一个数字数组
    if (!is_array($info["available_pokemons"])) {
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "available_pokemons 必须是一个数字数组";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }

    if ($query['name'] != $info["name"]) {
      DB::query("UPDATE pm_skill set name='" . addslashes($info["name"]) . "' where id=$id");
    }

    // 排序并去重
    $available_pokemons = array_map('intval', (array)$info["available_pokemons"]);
    sort($available_pokemons);
    $available_pokemons = array_values(array_unique($available_pokemons));

    $pokemon_list_str = "k," . implode(',', $available_pokemons) . ",k";
    DB::query("UPDATE pm_skill set available_pokemons='$pokemon_list_str' where id=$id");

    if ($query['description'] != $info["description"]) {
      DB::query("UPDATE pm_skill set description='" . addslashes($info["description"]) . "' where id=$id");
    }

    if (intval($query['level_required']) != intval($info["min_level_limit"])) {
      DB::query("UPDATE pm_skill set level_required=" . intval($info["min_level_limit"]) . " where id=$id");
    }

    if (intval($query['max_uses']) != intval($info["use_times_limit"])) {
      DB::query("UPDATE pm_skill set max_uses=" . intval($info["use_times_limit"]) . " where id=$id");
    }

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

    // 战斗引擎 2.0：技能效果模板关联（0 = 无效果；非零时会校验该 pm_effect 行存在）
    if ((isset($query['effect_id']) ? intval($query['effect_id']) : 0) != $effect_id) {
      DB::query("UPDATE pm_skill set effect_id='$effect_id' where id=$id");
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
  if (!is_array($info)) {
    exit(json_encode(['success' => false, 'reason' => '技能资料必须是对象'], JSON_UNESCAPED_UNICODE));
  }
  $effect = translate_skill_type_obj_to_raw($info["effect"] ?? null);
  $effect_id = validate_skill_effect_id($info, $effect[2]);
  // 提前检查，available_pokemons 必须是一个数字数组
  if (!is_array($info["available_pokemons"])) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "available_pokemons 必须是一个数字数组";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $name = addslashes(strval($info["name"]));

  // 排序并去重
  $available_pokemons = array_map('intval', (array)$info["available_pokemons"]);
  sort($available_pokemons);
  $available_pokemons = array_values(array_unique($available_pokemons));

  // array_merge 需将技能种族列表作为独立参数展开，否则嵌套数组会被
  // implode 当作 "Array" 字符串写入
  $pmid = implode(',', array_merge(['k'], $available_pokemons, ['k']));
  $txt = addslashes(strval($info["description"]));
  $lv = intval($info["min_level_limit"]);
  $num = intval($info["use_times_limit"]);

  $category = $effect[0];
  $tn = $effect[1];
  $powr = intval($effect[2]);

  DB::query("INSERT INTO pm_skill (
    name, available_pokemons, description, level_required, max_uses, category, element, power, effect_id
  ) VALUES (
    '$name', '$pmid', '$txt', $lv, $num, '$category', '$tn', $powr, $effect_id
  )");

  return intval(DB::insert_id());
}

function delete_skill_type($id)
{
  $id = intval($id);
  DB::query("DELETE FROM pm_skill where id=$id");
}

function filter_skill_type($list)
{
  $ret = [];

  $query_sql = "SELECT * from pm_skill where ";
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
            && DB::fetch_first("SELECT id FROM pm_skill WHERE id=" . intval($value))) {
          $query_sql_list[] = generate_filter_sql('id', $operator, intval($value), 'id');
        } else {
          $query_sql_list[] = generate_filter_sql('name', $operator, $value, 'text');
        }
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
        } elseif ($operator === 'not_equal') {
          $pokemon_id = intval($value);
          array_push($query_sql_list, "available_pokemons not like '%,$pokemon_id,%'");
        }
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
      $item["effect_id"] = isset($query['effect_id']) ? intval($query['effect_id']) : 0;
      array_push($ret, $item);
    }
  }

  return $ret;
}
