<?php

function count_pokemon_type()
{
  $count = DB::result_first("SELECT count(*) from pm_data");

  $item = [];
  $item["count"] = intval($count);

  $item["_TYPE"] = "::count";

  return [$item];
}

function list_pokemon_type($from, $count)
{
  $from = intval($from);
  $count = intval($count);

  $ret = [];
  $rows = DB::fetch_all("SELECT * from pm_data order by `id` asc limit $from,$count");
  if (!empty($rows)) {
    foreach ($rows as $query) {
      $map_ids = explode(',', $query['mapid']);
      $map_ids = array_filter($map_ids, function ($map_id) {
        return trim($map_id) != '' ? intval($map_id) > 0 : false;
      });
      $map_ids = array_map(function ($map_id) {
        return intval($map_id);
      }, $map_ids);

      $pm_id = intval($query['id']);
      $evolution_info_ids = [];
      $evolution_rows = DB::fetch_all("SELECT * from pm_evolution where `from_id`='$pm_id'");
      foreach ($evolution_rows as $query_evolution_info) {
        array_push($evolution_info_ids, intval($query_evolution_info['id']));
      }

      $effort = json_decode($query['effort_values'], true);
      $item = new_pokemon_type(
        intval($query['id']),
        $query['name'],
        $query['description'],

        intval($query['money']),
        intval($query['shop']) != 0,

        intval(
          $query['sex']
        ) >= 0 ? floatval($query['sex']) / 1000 : null,
        new_pokemon_attributes(
          intval($query['hp']),
          intval($query['atk']),
          intval($query['def']),
          intval($query['spatk']),
          intval($query['spdef']),
          intval($query['speed'])
        ),
        new_pokemon_attributes(
          intval($effort['hp'] ?? 0),
          intval($effort['atk'] ?? 0),
          intval($effort['def'] ?? 0),
          intval($effort['spatk'] ?? 0),
          intval($effort['spdef'] ?? 0),
          intval($effort['spd'] ?? 0)
        ),
        [translate_chinese_kind_to_kind_id($query['xs']), translate_chinese_kind_to_kind_id($query['xs2'])],
        intval($query['is_legendary']) != 0,


        $map_ids,
        $evolution_info_ids,

        intval($query['capture']),
        intval($query['met']),
        intval($query['birth']),
        intval($query['strength']),
        json_decode($query['drop_money'], true)
      );

      $item["_TYPE"] = "pokemon_type";
      array_push($ret, $item);
    }
    return $ret;
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "宠物不存在";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function get_pokemon_type($id, $depth = 0)
{
  $id = intval($id);
  if ($depth > 3) {
    return [];
  }

  $ret = [];
  if ($query = DB::fetch_first("SELECT * from pm_data where id={$id}")) {
    $map_ids = explode(',', $query['mapid']);
    $map_ids = array_filter($map_ids, function ($map_id) {
      return trim($map_id) != '' ? intval($map_id) > 0 : false;
    });
    $map_ids = array_map(function ($map_id) {
      return intval($map_id);
    }, $map_ids);

    $evolution_info_ids = [];
    $evolution_target_ids = [];
    $evolution_rows = DB::fetch_all("SELECT * from pm_evolution where `from_id`='$id'");
    foreach ($evolution_rows as $query_evolution_info) {
      array_push($evolution_info_ids, intval($query_evolution_info['id']));
      $target_id = intval($query_evolution_info['to_id']);
      if ($target_id > 0 && $target_id != $id) {
        array_push($evolution_target_ids, $target_id);
      }
    }
    $evolution_target_ids = array_unique($evolution_target_ids);

      $effort = json_decode($query['effort_values'], true);
      $item = new_pokemon_type(
        intval($query['id']),
        $query['name'],
      $query['description'],

      intval($query['money']),
      intval($query['shop']) != 0,

      intval(
        $query['sex']
      ) >= 0 ? floatval($query['sex']) / 1000 : null,
      new_pokemon_attributes(
        intval($query['hp']),
        intval($query['atk']),
        intval($query['def']),
        intval($query['spatk']),
        intval($query['spdef']),
        intval($query['speed'])
      ),
      new_pokemon_attributes(
        intval($effort['hp'] ?? 0),
        intval($effort['atk'] ?? 0),
        intval($effort['def'] ?? 0),
        intval($effort['spatk'] ?? 0),
        intval($effort['spdef'] ?? 0),
        intval($effort['spd'] ?? 0)
      ),
      [translate_chinese_kind_to_kind_id($query['xs']), translate_chinese_kind_to_kind_id($query['xs2'])],
      intval($query['is_legendary']) != 0,

      $map_ids,
      $evolution_info_ids,

      intval($query['capture']),
      intval($query['met']),
      intval($query['birth']),
      intval($query['strength']),
      json_decode($query['drop_money'], true)
    );

    $item["_TYPE"] = "pokemon_type";
    array_push($ret, $item);

    foreach ($map_ids as $map_id) {
      $ret = array_merge($ret, get_map_info($map_id));
    }

    foreach ($evolution_info_ids as $evolution_info_id) {
      $ret = array_merge($ret, get_evolution_info($evolution_info_id));
    }

    foreach ($evolution_target_ids as $target_pokemon_id) {
      $ret = array_merge($ret, get_pokemon_type($target_pokemon_id, $depth + 1));
    }

    return $ret;
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法查询宠物类型 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function set_pokemon_type($info)
{
  $id = intval($info["id"]);

  if ($query = DB::fetch_first("SELECT * from pm_data where id={$id}")) {
    // 提前检查，每个个体值必须在 0 到 255 之间
    if (
      intval($info["initial_statistic"]["hit_points"]) < 0 || intval($info["initial_statistic"]["hit_points"]) > 255 ||
      intval($info["initial_statistic"]["attack"]) < 0 || intval($info["initial_statistic"]["attack"]) > 255 ||
      intval($info["initial_statistic"]["defense"]) < 0 || intval($info["initial_statistic"]["defense"]) > 255 ||
      intval($info["initial_statistic"]["special_attack"]) < 0 || intval($info["initial_statistic"]["special_attack"]) > 255 ||
      intval($info["initial_statistic"]["special_defense"]) < 0 || intval($info["initial_statistic"]["special_defense"]) > 255 ||
      intval($info["initial_statistic"]["speed"]) < 0 || intval($info["initial_statistic"]["speed"]) > 255
    ) {
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "个体值必须在 0 到 255 之间";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }

    // 提前检查，每个努力值点数必须在 0 到 3 之间
    if (
      intval($info["initial_base_points"]["hit_points"]) < 0 || intval($info["initial_base_points"]["hit_points"]) > 3 ||
      intval($info["initial_base_points"]["attack"]) < 0 || intval($info["initial_base_points"]["attack"]) > 3 ||
      intval($info["initial_base_points"]["defense"]) < 0 || intval($info["initial_base_points"]["defense"]) > 3 ||
      intval($info["initial_base_points"]["special_attack"]) < 0 || intval($info["initial_base_points"]["special_attack"]) > 3 ||
      intval($info["initial_base_points"]["special_defense"]) < 0 || intval($info["initial_base_points"]["special_defense"]) > 3 ||
      intval($info["initial_base_points"]["speed"]) < 0 || intval($info["initial_base_points"]["speed"]) > 3
    ) {
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "努力值点数必须在 0 到 3 之间";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }

    if ($query['name'] != $info["name"]) {
      DB::query("UPDATE pm_data set name='" . addslashes($info['name']) . "' where id={$info["id"]}");
    }
    if ($query['description'] != $info["description"]) {
      DB::query("UPDATE pm_data set description='" . addslashes($info['description']) . "' where id={$info["id"]}");
    }

    if (intval($query['money']) != intval($info["cost"])) {
      DB::query("UPDATE pm_data set money=" . intval($info["cost"]) . " where id={$info["id"]}");
    }
    if (boolval($query['shop']) != boolval($info["is_selling"])) {
      $is_enabled = boolval($info["is_selling"]) ? 1 : 0;
      DB::query("UPDATE pm_data set shop={$is_enabled} where id={$info["id"]}");
    }

    if (((intval($query['sex']) == -1) != is_null($info["sex_weight"])) ||
      ((floatval($query['sex']) / 1000) != floatval($info["sex_weight"]))
    ) {
      $sex_weight = is_null($info["sex_weight"]) ? -1 : intval(floatval($info["sex_weight"]) * 1000);
      DB::query("UPDATE pm_data set sex=" . $sex_weight . " where id={$info["id"]}");
    }

    if (intval($query['hp']) != intval($info["initial_statistic"]["hit_points"])) {
      DB::query("UPDATE pm_data set hp=" . intval($info["initial_statistic"]["hit_points"]) . " where id={$info["id"]}");
    }
    if (intval($query['atk']) != intval($info["initial_statistic"]["attack"])) {
      DB::query("UPDATE pm_data set atk=" . intval($info["initial_statistic"]["attack"]) . " where id={$info["id"]}");
    }
    if (intval($query['def']) != intval($info["initial_statistic"]["defense"])) {
      DB::query("UPDATE pm_data set def=" . intval($info["initial_statistic"]["defense"]) . " where id={$info["id"]}");
    }
    if (intval($query['spatk']) != intval($info["initial_statistic"]["special_attack"])) {
      DB::query("UPDATE pm_data set spatk=" . intval($info["initial_statistic"]["special_attack"]) . " where id={$info["id"]}");
    }
    if (intval($query['spdef']) != intval($info["initial_statistic"]["special_defense"])) {
      DB::query("UPDATE pm_data set spdef=" . intval($info["initial_statistic"]["special_defense"]) . " where id={$info["id"]}");
    }
    if (intval($query['speed']) != intval($info["initial_statistic"]["speed"])) {
      DB::query("UPDATE pm_data set speed=" . intval($info["initial_statistic"]["speed"]) . " where id={$info["id"]}");
    }

    $effort = json_decode($query['effort_values'], true);
    $hpn = intval($info["initial_base_points"]["hit_points"]);
    $atkn = intval($info["initial_base_points"]["attack"]);
    $defn = intval($info["initial_base_points"]["defense"]);
    $spatkn = intval($info["initial_base_points"]["special_attack"]);
    $spdefn = intval($info["initial_base_points"]["special_defense"]);
    $sdn = intval($info["initial_base_points"]["speed"]);
    if (
      intval($effort['hp'] ?? 0) != $hpn ||
      intval($effort['atk'] ?? 0) != $atkn ||
      intval($effort['def'] ?? 0) != $defn ||
      intval($effort['spatk'] ?? 0) != $spatkn ||
      intval($effort['spdef'] ?? 0) != $spdefn ||
      intval($effort['spd'] ?? 0) != $sdn
    ) {
      $effort_values = '{"hp":' . intval($hpn) . ',"atk":' . intval($atkn) . ',"def":' . intval($defn) . ',"spatk":' . intval($spatkn) . ',"spdef":' . intval($spdefn) . ',"spd":' . intval($sdn) . '}';
      DB::query("UPDATE pm_data set effort_values='" . addslashes($effort_values) . "' where id={$info["id"]}");
    }

    if (translate_chinese_kind_to_kind_id($query['xs']) != $info["kind"][0]) {
      DB::query("UPDATE pm_data set xs='" . translate_kind_id_to_chinese_kind($info["kind"][0]) . "' where id={$info["id"]}");
    }
    if (($query['xs2'] == '') != is_null($info["kind"][1]) ||
      translate_chinese_kind_to_kind_id($query['xs2']) != $info["kind"][1]
    ) {
      DB::query("UPDATE pm_data set xs2='" . translate_kind_id_to_chinese_kind($info["kind"][1]) . "' where id={$info["id"]}");
    }
    if (boolval($query['is_legendary']) != boolval($info["is_legendary"])) {
      DB::query("UPDATE pm_data set is_legendary=" . (boolval($info["is_legendary"]) ? 1 : 0) . " where id={$info["id"]}");
    }

    $mapid = [];
    foreach ($info["map_ids"] as $map_id) {
      if (intval($map_id) > 0) {
        array_push($mapid, intval($map_id));
      }
    }
    $mapid = array_unique($mapid);
    sort($mapid, SORT_NUMERIC);
    $mapid = implode(",", $mapid);
    DB::query("UPDATE pm_data set mapid='$mapid' where id={$info["id"]}");

    if (intval($query['capture']) != intval($info["capture_weight"])) {
      DB::query("UPDATE pm_data set capture=" . intval($info["capture_weight"]) . " where id={$info["id"]}");
    }
    if (intval($query['met']) != intval($info["meet_weight"])) {
      DB::query("UPDATE pm_data set met=" . intval($info["meet_weight"]) . " where id={$info["id"]}");
    }
    if (intval($query['birth']) != intval($info["birth_order"])) {
      DB::query("UPDATE pm_data set birth=" . intval($info["birth_order"]) . " where id={$info["id"]}");
    }
    if (intval($query['strength']) != intval($info["strength_weight"])) {
      DB::query("UPDATE pm_data set strength=" . intval($info["strength_weight"]) . " where id={$info["id"]}");
    }
    $old_drop = json_decode($query['drop_money'], true);
    $new_drop = [intval($info["drop_money_range"][0]), intval($info["drop_money_range"][1])];
    if ($old_drop != $new_drop) {
      DB::query("UPDATE pm_data set drop_money='" . addslashes(json_encode($new_drop, JSON_UNESCAPED_UNICODE)) . "' where id={$info["id"]}");
    }
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "宠物类型更新失败，未找到 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
  return get_pokemon_type($id);
}

function insert_pokemon_type($info)
{
  // 提前检查，每个个体值必须在 0 到 255 之间
  if (
    intval($info["initial_statistic"]["hit_points"]) < 0 || intval($info["initial_statistic"]["hit_points"]) > 255 ||
    intval($info["initial_statistic"]["attack"]) < 0 || intval($info["initial_statistic"]["attack"]) > 255 ||
    intval($info["initial_statistic"]["defense"]) < 0 || intval($info["initial_statistic"]["defense"]) > 255 ||
    intval($info["initial_statistic"]["special_attack"]) < 0 || intval($info["initial_statistic"]["special_attack"]) > 255 ||
    intval($info["initial_statistic"]["special_defense"]) < 0 || intval($info["initial_statistic"]["special_defense"]) > 255 ||
    intval($info["initial_statistic"]["speed"]) < 0 || intval($info["initial_statistic"]["speed"]) > 255
  ) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "个体值必须在 0 到 255 之间";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 提前检查，每个努力值点数必须在 0 到 3 之间
  if (
    intval($info["initial_base_points"]["hit_points"]) < 0 || intval($info["initial_base_points"]["hit_points"]) > 3 ||
    intval($info["initial_base_points"]["attack"]) < 0 || intval($info["initial_base_points"]["attack"]) > 3 ||
    intval($info["initial_base_points"]["defense"]) < 0 || intval($info["initial_base_points"]["defense"]) > 3 ||
    intval($info["initial_base_points"]["special_attack"]) < 0 || intval($info["initial_base_points"]["special_attack"]) > 3 ||
    intval($info["initial_base_points"]["special_defense"]) < 0 || intval($info["initial_base_points"]["special_defense"]) > 3 ||
    intval($info["initial_base_points"]["speed"]) < 0 || intval($info["initial_base_points"]["speed"]) > 3
  ) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "努力值点数必须在 0 到 3 之间";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $name = strval($info["name"]);
  $txt = strval($info["description"]);
  $money = intval($info["cost"]);
  $shop = boolval($info["is_selling"]) ? 1 : 0;
  $sex = is_null($info["sex_weight"]) ? -1 : intval(floatval($info["sex_weight"]) * 1000);

  $hp = intval($info["initial_statistic"]["hit_points"]);
  $atk = intval($info["initial_statistic"]["attack"]);
  $def = intval($info["initial_statistic"]["defense"]);
  $spatk = intval($info["initial_statistic"]["special_attack"]);
  $spdef = intval($info["initial_statistic"]["special_defense"]);
  $sd = intval($info["initial_statistic"]["speed"]);

  $hpn = intval($info["initial_base_points"]["hit_points"]);
  $atkn = intval($info["initial_base_points"]["attack"]);
  $defn = intval($info["initial_base_points"]["defense"]);
  $spatkn = intval($info["initial_base_points"]["special_attack"]);
  $spdefn = intval($info["initial_base_points"]["special_defense"]);
  $sdn = intval($info["initial_base_points"]["speed"]);

  // 提前检查，属性必须能够通过转换校验
  translate_kind_id_to_chinese_kind($info["kind"][0]);
  if (!is_null($info["kind"][1])) {
    translate_kind_id_to_chinese_kind($info["kind"][1]);
  }

  $xs = translate_kind_id_to_chinese_kind($info["kind"][0]);
  $xs2 = !is_null($info["kind"][1]) ? translate_kind_id_to_chinese_kind($info["kind"][1]) : "";
  $god = boolval($info["is_legendary"]) ? 1 : 0;

  $mapid = [];
  foreach ($info["map_ids"] as $map_id) {
    if (intval($map_id) > 0) {
      $mapid[] = intval($map_id);
    }
  }
  $mapid = array_unique($mapid);
  sort($mapid, SORT_NUMERIC);
  $mapid = implode(",", $mapid);
  $capture = boolval($info["capture_weight"]) ? 1 : 0;
  $met = boolval($info["meet_weight"]) ? 1 : 0;
  $birth = intval($info["birth_order"]);
  $strength = intval($info["strength_weight"]);
  $minmoney = intval($info["drop_money_range"][0]);
  $maxmoney = intval($info["drop_money_range"][1]);

  $last_id = DB::fetch_first("SELECT id from pm_data order by id desc limit 1");
  $last_id = intval($last_id['id']);
  $new_id = $last_id + 1;

  $effort_values = '{"hp":' . intval($hpn) . ',"atk":' . intval($atkn) . ',"def":' . intval($defn) . ',"spatk":' . intval($spatkn) . ',"spdef":' . intval($spdefn) . ',"spd":' . intval($sdn) . '}';
  $drop_money = '[' . intval($minmoney) . ',' . intval($maxmoney) . ']';
  DB::query("INSERT INTO pm_data (
    id, name, description, money, shop, sex,
    hp, atk, def, spatk, spdef, speed,
    effort_values,
    xs, xs2, is_legendary, mapid, capture, met,
    birth, strength, drop_money
  ) VALUES (
    $new_id, '$name', '$txt', $money, $shop, $sex,
    $hp, $atk, $def, $spatk, $spdef, $sd,
    '$effort_values',
    '$xs', '$xs2', $god, '$mapid', $capture, $met,
    $birth, $strength, '$drop_money'
  )");

  return $new_id;
}

function delete_pokemon_type($id)
{
  $id = intval($id);
  DB::query("DELETE FROM pm_data WHERE id = $id");
}

function filter_pokemon_type($list)
{
  $ret = [];

  foreach ($list as $item) {
    $query_sql = "SELECT * from pm_data where ";
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
          $id_query_sql = "SELECT * from pm_data where id = " . intval($value);
          $found_id_results = DB::fetch_all($id_query_sql);
          if (!empty($found_id_results)) {
              foreach ($found_id_results as $query) {
                $map_ids = explode(',', $query['mapid']);
                $map_ids = array_filter($map_ids, function ($map_id) {
                  return trim($map_id) != '' ? intval($map_id) > 0 : false;
                });
                $map_ids = array_map(function ($map_id) {
                  return intval($map_id);
                }, $map_ids);

                $pm_id = intval($query['id']);
                $evolution_info_ids = [];
      $evolution_rows = DB::fetch_all("SELECT * from pm_evolution where `from_id`='$pm_id'");
      foreach ($evolution_rows as $query_evolution_info) {
                  array_push($evolution_info_ids, intval($query_evolution_info['id']));
                }

                $effort = json_decode($query['effort_values'], true);
      $item = new_pokemon_type(
                  intval($query['id']),
                  $query['name'],
                  $query['description'],

                  intval($query['money']),
                  intval($query['shop']) != 0,

                  intval(
                    $query['sex']
                  ) >= 0 ? floatval($query['sex']) / 1000 : null,
                  new_pokemon_attributes(
                    intval($query['hp']),
                    intval($query['atk']),
                    intval($query['def']),
                    intval($query['spatk']),
                    intval($query['spdef']),
                    intval($query['speed'])
                  ),
                  new_pokemon_attributes(
                    intval($effort['hp'] ?? 0),
                    intval($effort['atk'] ?? 0),
                    intval($effort['def'] ?? 0),
                    intval($effort['spatk'] ?? 0),
                    intval($effort['spdef'] ?? 0),
                    intval($effort['spd'] ?? 0)
                  ),
                  [translate_chinese_kind_to_kind_id($query['xs']), translate_chinese_kind_to_kind_id($query['xs2'])],
                  intval($query['is_legendary']) != 0,

                  $map_ids,
                  $evolution_info_ids,

                  intval($query['capture']),
                  intval($query['met']),
                  intval($query['birth']),
                  intval($query['strength']),
                  json_decode($query['drop_money'], true)
                );

                $item["_TYPE"] = "pokemon_type";
                array_push($ret, $item);
              }
              return $ret;
            }
          }
        // ID 查询无结果或非数字输入，使用名称模糊搜索
        array_push($query_sql_list, generate_filter_sql('name', $operator, $value, 'text'));
        break;
      case '宠物类型':
        switch ($operator) {
          case 'equal':
            array_push($query_sql_list, "(`xs` like '%$value%' or `xs2` like '%$value%')");
            break;
          case 'not_equal':
            array_push($query_sql_list, "(`xs` not like '%$value%' and `xs2` not like '%$value%')");
            break;
          default:
        }
        break;
      case '描述':
        array_push($query_sql_list, generate_filter_sql('description', $operator, $value, 'text'));
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
      $map_ids = explode(',', $query['mapid']);
      $map_ids = array_filter($map_ids, function ($map_id) {
        return trim($map_id) != '' ? intval($map_id) > 0 : false;
      });
      $map_ids = array_map(function ($map_id) {
        return intval($map_id);
      }, $map_ids);

      $pm_id = intval($query['id']);
      $evolution_info_ids = [];
      $evolution_rows = DB::fetch_all("SELECT * from pm_evolution where `from_id`='$pm_id'");
      foreach ($evolution_rows as $query_evolution_info) {
        array_push($evolution_info_ids, intval($query_evolution_info['id']));
      }

      $effort = json_decode($query['effort_values'], true);
      $item = new_pokemon_type(
        intval($query['id']),
        $query['name'],
        $query['description'],

        intval($query['money']),
        intval($query['shop']) != 0,

        intval(
          $query['sex']
        ) >= 0 ? floatval($query['sex']) / 1000 : null,
        new_pokemon_attributes(
          intval($query['hp']),
          intval($query['atk']),
          intval($query['def']),
          intval($query['spatk']),
          intval($query['spdef']),
          intval($query['speed'])
        ),
        new_pokemon_attributes(
          intval($effort['hp'] ?? 0),
          intval($effort['atk'] ?? 0),
          intval($effort['def'] ?? 0),
          intval($effort['spatk'] ?? 0),
          intval($effort['spdef'] ?? 0),
          intval($effort['spd'] ?? 0)
        ),
        [translate_chinese_kind_to_kind_id($query['xs']), translate_chinese_kind_to_kind_id($query['xs2'])],
        intval($query['is_legendary']) != 0,

        $map_ids,
        $evolution_info_ids,

        intval($query['capture']),
        intval($query['met']),
        intval($query['birth']),
        intval($query['strength']),
        json_decode($query['drop_money'], true)
      );

      $item["_TYPE"] = "pokemon_type";
      array_push($ret, $item);
    }
  }

  return $ret;
}

/**
 * 根据 ID 列表批量获取宠物类型信息（简化版，只返回 id 和 name）
 * 用于加载技能或地图的可用宠物列表
 */
function get_pokemon_types_by_ids($ids)
{
  if (!is_array($ids) || empty($ids)) {
    return [];
  }

  // 过滤并转换 ID 为整数
  $valid_ids = array_filter($ids, function($id) {
    return is_numeric($id) && intval($id) > 0;
  });

  if (empty($valid_ids)) {
    return [];
  }

  $valid_ids = array_map('intval', $valid_ids);
  $valid_ids = array_unique($valid_ids);
  sort($valid_ids, SORT_NUMERIC);

  $ret = [];
  // 使用 IN 查询一次性获取所有宠物
  $ids_str = implode(',', $valid_ids);
  $rows = DB::fetch_all("SELECT id, name FROM pm_data WHERE id IN ($ids_str) ORDER BY id ASC");
  foreach ($rows as $query) {
      $item = [
        "id" => intval($query['id']),
        "name" => $query['name'],
        "_TYPE" => "wild_pokemon_info"
      ];
      array_push($ret, $item);
    }

  return $ret;
}
