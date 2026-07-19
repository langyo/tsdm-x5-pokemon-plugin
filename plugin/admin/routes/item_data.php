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
        $query['txt'],

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
    $json_ret["reason"] = "数据库无法访问";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
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
        $query['txt'],

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
      DB::query("UPDATE pm_itemdata set name='" . $info["name"] . "' where id=$id");
    }

    if ($query['tpname'] != $info["img_name"]) {
      DB::query("UPDATE pm_itemdata set tpname='" . $info["img_name"] . "' where id=$id");
    }

    if ($query['txt'] != $info["description"]) {
      DB::query("UPDATE pm_itemdata set txt='" . $info["description"] . "' where id=$id");
    }

    if (boolval($query['shop']) != boolval($info["is_selling"])) {
      DB::query("UPDATE pm_itemdata set shop='" . (boolval($info["is_selling"]) ? 1 : 0) . "' where id=$id");
    }

    if (intval($query['money']) != intval($info["price"])) {
      DB::query("UPDATE pm_itemdata set money=" . intval($info["price"]) . " where id=$id");
    }

    $tag = translate_item_tag_id_to_db_raw($info["tag"]);
    $tag_type = $tag[0];
    $tag_value = $tag[1];
    if ($query['type'] != $tag_value) {
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
          DB::query("UPDATE pm_itemdata set sitemname='" . $tag_value . "' where id=$id");
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

    if (intval($query['addhp']) != intval($info["effects"]["add_hit_points"])) {
      DB::query("UPDATE pm_itemdata set addhp=" . intval($info["effects"]["add_hit_points"]) . " where id=$id");
    }
    if (intval($query['addexp']) != intval($info["effects"]["add_experience"])) {
      DB::query("UPDATE pm_itemdata set addexp=" . intval($info["effects"]["add_experience"]) . " where id=$id");
    }
    if (intval($query['addlv']) != intval($info["effects"]["add_level"])) {
      DB::query("UPDATE pm_itemdata set addlv=" . intval($info["effects"]["add_level"]) . " where id=$id");
    }
    if (intval($query['addgood']) != intval($info["effects"]["add_intimacy"])) {
      DB::query("UPDATE pm_itemdata set addgood=" . intval($info["effects"]["add_intimacy"]) . " where id=$id");
    }
    if (intval($query['equipment_hp']) != intval($info["effects"]["attribute_add_hit_points"])) {
      DB::query("UPDATE pm_itemdata set equipment_hp=" . intval($info["effects"]["attribute_add_hit_points"]) . " where id=$id");
    }
    if (intval($query['equipment_atk']) != intval($info["effects"]["attribute_add_attack"])) {
      DB::query("UPDATE pm_itemdata set equipment_atk=" . intval($info["effects"]["attribute_add_attack"]) . " where id=$id");
    }
    if (intval($query['equipment_def']) != intval($info["effects"]["attribute_add_defense"])) {
      DB::query("UPDATE pm_itemdata set equipment_def=" . intval($info["effects"]["attribute_add_defense"]) . " where id=$id");
    }
    if (intval($query['equipment_spatk']) != intval($info["effects"]["attribute_add_special_attack"])) {
      DB::query("UPDATE pm_itemdata set equipment_spatk=" . intval($info["effects"]["attribute_add_special_attack"]) . " where id=$id");
    }
    if (intval($query['equipment_spdef']) != intval($info["effects"]["attribute_add_special_defense"])) {
      DB::query("UPDATE pm_itemdata set equipment_spdef=" . intval($info["effects"]["attribute_add_special_defense"]) . " where id=$id");
    }
    if (intval($query['equipment_sd']) != intval($info["effects"]["attribute_add_speed"])) {
      DB::query("UPDATE pm_itemdata set equipment_sd=" . intval($info["effects"]["attribute_add_speed"]) . " where id=$id");
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
  $name = strval($info["name"]);
  $tpname = strval($info["img_name"]);
  $txt = strval($info["description"]);
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
      $sitemname = $tag[1];
      break;
    case 5:
      // 装备
      $zbtype = intval($tag[1]);
      break;
    default:
      break;
  }

  $lvask = intval($info["limits"]["min_level"]);
  $xsask = translate_chinese_kind_to_kind_id($info["limits"]["kind_require"]);
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

  $last_id = DB::fetch_first("SELECT id from pm_itemdata order by id desc limit 1");
  $last_id = intval($last_id['id']);
  $new_id = $last_id + 1;

  DB::query("INSERT INTO pm_itemdata (
    id, name, tpname, txt, shop, money, type,
    ballid, upitem, sitemname, zbtype,
    lvask, xsask,
    addhp, addexp, addlv, addgood,
    equipment_hp, equipment_atk, equipment_def,
    equipment_spatk, equipment_spdef, equipment_sd,
    captmax
  ) VALUES (
    $new_id, '$name', '$tpname', '$txt', $shop, $money, '$type',
    $ballid, $upitem, '$sitemname', $zbtype,
    $lvask, '$xsask',
    $addhp, $addexp, $addlv, $addgood,
    $equipment_hp, $equipment_atk, $equipment_def,
    $equipment_spatk, $equipment_spdef, $equipment_sd,
    $captmax
  )");

  return $new_id;
}

function delete_item_type($id)
{
  $id = intval($id);
  DB::query("DELETE FROM pm_itemdata where id=$id");
}

function filter_item_type($list)
{
  $ret = [];

  foreach ($list as $item) {
    $query_sql = "SELECT * from pm_itemdata where ";
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
          $id_query_sql = "SELECT * from pm_itemdata where id = " . intval($value);
          $found_id_results = DB::fetch_all($id_query_sql);
          if (!empty($found_id_results)) {
              foreach ($found_id_results as $query) {
                $item = new_item_type(
                  intval($query['id']),
                  $query['name'],
                  $query['tpname'],
                  $query['txt'],

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
            }
          }
        // ID 查询无结果或非数字输入，使用名称模糊搜索
        array_push($query_sql_list, generate_filter_sql('name', $operator, $value, 'text'));
        break;
      case '类型':
        array_push($query_sql_list, generate_filter_sql('type', $operator, $value, 'text'));
        break;
      case '价格':
        array_push($query_sql_list, generate_filter_sql('money', $operator, $value, 'number'));
        break;
      case '是否出售':
        array_push($query_sql_list, generate_filter_sql('shop', $operator, boolval($value), 'id'));
        break;
      case '描述':
        array_push($query_sql_list, generate_filter_sql('txt', $operator, $value, 'text'));
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
      $item = new_item_type(
        intval($query['id']),
        $query['name'],
        $query['tpname'],
        $query['txt'],

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
