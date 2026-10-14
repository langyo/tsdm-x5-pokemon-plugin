<?php

function count_item_type()
{
  $count = DB::result_first("SELECT count(*) from pm_itemdata");

  $item = [];
  $item["count"] = intval($count);

  $item["_TYPE"] = "::count";

  return [$item];
}

function list_item_type($from, $count)
{
  $from = intval($from);
  $count = intval($count);

  $ret = [];
  $rows = DB::fetch_all("SELECT * from pm_itemdata order by `id` asc limit $from,$count");
  if (!empty($rows)) {
    foreach ($rows as $query) {
      $item = new_item_type(
        intval($query['id']),
        $query['name'],
        $query['tpname'],
        $query['description'],

        intval($query['shop']) != 0,
        intval($query['money']),
        new_item_tag(intval($query['type']), $query),
        new_item_limits($query),
        new_item_effects($query)
      );

      $item["_TYPE"] = "item_type";
      array_push($ret, $item);
    }
    return $ret;
  } else {
    return [];
  }
}

function get_item_type($id)
{
  $id = intval($id);

  $ret = [];
  $rows = DB::fetch_all("SELECT * from pm_itemdata where id=$id");
  if (!empty($rows)) {
    foreach ($rows as $query) {
      $item = new_item_type(
        intval($query['id']),
        $query['name'],
        $query['tpname'],
        $query['description'],

        intval($query['shop']) != 0,
        intval($query['money']),
        new_item_tag(intval($query['type']), $query),
        new_item_limits($query),
        new_item_effects($query)
      );

      $item["_TYPE"] = "item_type";
      array_push($ret, $item);
    }
    return $ret;
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法查询物品类型 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function set_item_type($info)
{
  $id = intval($info["id"]);

  if ($query = DB::fetch_first("SELECT * from pm_itemdata where id=$id")) {
    if ($query['name'] != $info["name"]) {
      DB::query("UPDATE pm_itemdata set name='" . addslashes($info["name"]) . "' where id=$id");
    }

    if ($query['tpname'] != $info["img_name"]) {
      DB::query("UPDATE pm_itemdata set tpname='" . addslashes(strval($info["img_name"])) . "' where id=$id");
    }

    if ($query['description'] != $info["description"]) {
      DB::query("UPDATE pm_itemdata set description='" . addslashes($info["description"]) . "' where id=$id");
    }

    if (boolval($query['shop']) != boolval($info["is_selling"])) {
      DB::query("UPDATE pm_itemdata set shop='" . (boolval($info["is_selling"]) ? 1 : 0) . "' where id=$id");
    }

    if (intval($query['money']) != intval($info["price"])) {
      DB::query("UPDATE pm_itemdata set money=" . intval($info["price"]) . " where id=$id");
    }

    $tag = translate_item_tag_id_to_db_raw($info["tag"]);
    $tag_type = $tag[0];
    // 药品标签没有附加值（translator 只返回单元素数组）
    $tag_value = $tag[1] ?? 0;
    if ($query['type'] != $tag_type) {
      DB::query("UPDATE pm_itemdata set type=" . $tag_type . " where id=$id");
    }
    switch ($tag_type) {
      case 1:
        // 药品
        break;
      case 2:
        // 宠物球
        if (intval($query['ballid']) != intval($tag_value)) {
          DB::query("UPDATE pm_itemdata set ballid=" . intval($tag_value) . " where id=$id");
        }
        break;
      case 3:
        // 升级素材
        if (intval($query['upitem']) != intval($tag_value)) {
          DB::query("UPDATE pm_itemdata set upitem=" . intval($tag_value) . " where id=$id");
        }
        break;
      case 4:
        // 特殊物品
        if ($query['sitemname'] != $tag_value) {
          DB::query("UPDATE pm_itemdata set sitemname='" . addslashes(strval($tag_value)) . "' where id=$id");
        }
        break;
      case 5:
        // 装备
        if (intval($query['zbtype']) != intval($tag_value)) {
          DB::query("UPDATE pm_itemdata set zbtype=" . intval($tag_value) . " where id=$id");
        }
        break;
      default:
        break;
    }

    if (intval($query['lvask']) != intval($info["limits"]["min_level"])) {
      DB::query("UPDATE pm_itemdata set lvask=" . intval($info["limits"]["min_level"]) . " where id=$id");
    }
    if (translate_chinese_kind_to_kind_id($query['xsask']) != $info["limits"]["kind_require"]) {
      DB::query("UPDATE pm_itemdata set xsask='" . translate_kind_id_to_chinese_kind($info["limits"]["kind_require"]) . "' where id=$id");
    }

    $effects = json_decode($query['effects'], true);
    $addhp = intval($info["effects"]["add_hit_points"]);
    $addexp = intval($info["effects"]["add_experience"]);
    $addlv = intval($info["effects"]["add_level"]);
    $addgood = intval($info["effects"]["add_intimacy"]);
    if (
      intval($effects['hp'] ?? 0) != $addhp ||
      intval($effects['exp'] ?? 0) != $addexp ||
      intval($effects['level'] ?? 0) != $addlv ||
      intval($effects['intimacy'] ?? 0) != $addgood
    ) {
      $effects_json = '{"hp":' . intval($addhp) . ',"exp":' . intval($addexp) . ',"level":' . intval($addlv) . ',"intimacy":' . intval($addgood) . '}';
      DB::query("UPDATE pm_itemdata set effects='" . addslashes($effects_json) . "' where id=$id");
    }
    $equipment = json_decode($query['equipment'], true);
    $equipment_hp = intval($info["effects"]["attribute_add_hit_points"]);
    $equipment_atk = intval($info["effects"]["attribute_add_attack"]);
    $equipment_def = intval($info["effects"]["attribute_add_defense"]);
    $equipment_spatk = intval($info["effects"]["attribute_add_special_attack"]);
    $equipment_spdef = intval($info["effects"]["attribute_add_special_defense"]);
    $equipment_sd = intval($info["effects"]["attribute_add_speed"]);
    if (
      intval($equipment['hp'] ?? 0) != $equipment_hp ||
      intval($equipment['atk'] ?? 0) != $equipment_atk ||
      intval($equipment['def'] ?? 0) != $equipment_def ||
      intval($equipment['spatk'] ?? 0) != $equipment_spatk ||
      intval($equipment['spdef'] ?? 0) != $equipment_spdef ||
      intval($equipment['spd'] ?? 0) != $equipment_sd
    ) {
      $equipment_json = '{"hp":' . intval($equipment_hp) . ',"atk":' . intval($equipment_atk) . ',"def":' . intval($equipment_def) . ',"spatk":' . intval($equipment_spatk) . ',"spdef":' . intval($equipment_spdef) . ',"spd":' . intval($equipment_sd) . '}';
      DB::query("UPDATE pm_itemdata set equipment='" . addslashes($equipment_json) . "' where id=$id");
    }
    if (intval($query['captmax']) != intval($info["effects"]["capture"])) {
      DB::query("UPDATE pm_itemdata set captmax=" . intval($info["effects"]["capture"]) . " where id=$id");
    }
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "物品类型更新失败，未找到 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function insert_item_type($info)
{
  $name = addslashes(strval($info["name"]));
  $tpname = addslashes(strval($info["img_name"]));
  $txt = addslashes(strval($info["description"]));
  $shop = boolval($info["is_selling"]) ? 1 : 0;
  $money = intval($info["price"]);
  $tag = translate_item_tag_id_to_db_raw($info["tag"]);
  $type = $tag[0];
  $ballid = 0;
  $upitem = 0;
  $sitemname = '';
  $zbtype = 0;
  switch ($type) {
    case 1:
      // 药品
      break;
    case 2:
      // 宠物球
      $ballid = intval($tag[1]);
      break;
    case 3:
      // 升级素材
      $upitem = intval($tag[1]);
      break;
    case 4:
      // 特殊物品
      $sitemname = addslashes(strval($tag[1]));
      break;
    case 5:
      // 装备
      $zbtype = intval($tag[1]);
      break;
    default:
      break;
  }

  $lvask = intval($info["limits"]["min_level"]);
  $xsask = translate_kind_id_to_chinese_kind($info["limits"]["kind_require"]) ?? '';
  $addhp = intval($info["effects"]["add_hit_points"]);
  $addexp = intval($info["effects"]["add_experience"]);
  $addlv = intval($info["effects"]["add_level"]);
  $addgood = intval($info["effects"]["add_intimacy"]);
  $equipment_hp = intval($info["effects"]["attribute_add_hit_points"]);
  $equipment_atk = intval($info["effects"]["attribute_add_attack"]);
  $equipment_def = intval($info["effects"]["attribute_add_defense"]);
  $equipment_spatk = intval($info["effects"]["attribute_add_special_attack"]);
  $equipment_spdef = intval($info["effects"]["attribute_add_special_defense"]);
  $equipment_sd = intval($info["effects"]["attribute_add_speed"]);
  $captmax = intval($info["effects"]["capture"]);

  $effects_json = '{"hp":' . intval($addhp) . ',"exp":' . intval($addexp) . ',"level":' . intval($addlv) . ',"intimacy":' . intval($addgood) . '}';
  $equipment_json = '{"hp":' . intval($equipment_hp) . ',"atk":' . intval($equipment_atk) . ',"def":' . intval($equipment_def) . ',"spatk":' . intval($equipment_spatk) . ',"spdef":' . intval($equipment_spdef) . ',"spd":' . intval($equipment_sd) . '}';

  DB::query("INSERT INTO pm_itemdata (
    name, tpname, description, shop, money, type,
    ballid, upitem, sitemname, zbtype,
    lvask, xsask,
    effects,
    equipment,
    captmax
  ) VALUES (
    '$name', '$tpname', '$txt', $shop, $money, '$type',
    $ballid, $upitem, '$sitemname', $zbtype,
    $lvask, '$xsask',
    '$effects_json',
    '$equipment_json',
    $captmax
  )");

  return intval(DB::insert_id());
}

function delete_item_type($id)
{
  $id = intval($id);
  DB::query("DELETE FROM pm_itemdata where id=$id");
}

function filter_item_type($list)
{
  $ret = [];

  $query_sql = "SELECT * from pm_itemdata where ";
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
            && DB::fetch_first("SELECT id FROM pm_itemdata WHERE id=" . intval($value))) {
          $query_sql_list[] = generate_filter_sql('id', $operator, intval($value), 'id');
        } else {
          $query_sql_list[] = generate_filter_sql('name', $operator, $value, 'text');
        }
        break;
      case '类型':
        array_push($query_sql_list, generate_filter_sql('type', $operator, $value, 'text'));
        break;
      case '价格':
        array_push($query_sql_list, generate_filter_sql('money', $operator, $value, 'number'));
        break;
      case '是否出售':
        if (in_array($value, ['是', '1', 'true'], true)) $for_sale = 1;
        elseif (in_array($value, ['否', '0', 'false'], true)) $for_sale = 0;
        else return [];
        array_push($query_sql_list, generate_filter_sql('shop', $operator, $for_sale, 'id'));
        break;
      case '描述':
        array_push($query_sql_list, generate_filter_sql('description', $operator, $value, 'text'));
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
      $item = new_item_type(
        intval($query['id']),
        $query['name'],
        $query['tpname'],
        $query['description'],

        intval($query['shop']) != 0,
        intval($query['money']),
        new_item_tag(intval($query['type']), $query),
        new_item_limits($query),
        new_item_effects($query)
      );

      $item["_TYPE"] = "item_type";
      array_push($ret, $item);
    }
  }

  return $ret;
}
