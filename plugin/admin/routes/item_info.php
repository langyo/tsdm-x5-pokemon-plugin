<?php
function list_item_info($uid, $from, $count)
{
  $ret = [];
  $rows = DB::fetch_all("SELECT * from pm_myitem where `uid`='$uid' limit $from,$count");
  foreach ($rows as $query) {
      array_push($ret, new_item_info(
        intval($query['id']),
        $uid,
        intval($query['itemid']),
        intval($query['nums'])
      ));
    }
  }

  return $ret;
}

function get_item_info($id)
{
  $id = intval($id);
  $ret = [];

  if ($query = DB::fetch_first("SELECT * from pm_myitem where `id`='$id'")) {
    array_push($ret, new_item_info(
      intval($query['id']),
      intval($query['uid']),
      intval($query['itemid']),
      intval($query['nums'])
    ));

    return $ret;
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法查询物品信息 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function set_item_info($info)
{
  $id = intval($info["id"]);
  if ($query = DB::fetch_first("SELECT * from pm_myitem where `id`='$id'")) {
    // 提前检查，禁止修改持有用户
    if (intval($query['uid']) != intval($info["owner"])) {
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "无法修改物品信息，禁止修改持有用户信息 #$id";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }

    if (intval($query['itemid']) != intval($info["type_id"])) {
      DB::query("UPDATE pm_myitem set `itemid`='" . intval($info["type_id"]) . "' where `id`='$id'");
    }
    if (intval($query['num']) != intval($info["count"])) {
      DB::query("UPDATE pm_myitem set `num`='" . intval($info["count"]) . "' where `id`='$id'");
    }
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法查询物品信息 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function insert_item_info($info)
{
  $uid = intval($info["owner"]);
  $itemid = intval($info["type_id"]);
  $nums = intval($info["count"]);

  // 提前检查，对应物品类型必须存在
  if (!DB::fetch_first("SELECT id from pm_itemdata where id='$itemid'")) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "未找到物品类型 #$itemid";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 提前检查，对应用户必须存在
  if (!DB::fetch_first("SELECT uid from pm_usersdata where uid='$uid'")) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "未找到用户 #$uid";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 检查用户是否已拥有该类型物品
  if ($existing = DB::fetch_first("SELECT id, num from pm_myitem where `uid`='$uid' and `itemid`='$itemid'")) {
    // 已存在，累加数量
    $existing_id = intval($existing['id']);
    $new_nums = intval($existing['num']) + $nums;
    DB::query("UPDATE pm_myitem set `num`='$new_nums' where `id`='$existing_id'");
    return $existing_id;
  }

  // 不存在，创建新条目
  DB::query("INSERT INTO pm_myitem (
    `uid`,`itemid`,`num`
  ) VALUES (
    '$uid','$itemid','$nums'
  )");

  $new_id = DB::insert_id();
  return $new_id;
}

function delete_item_info($id)
{
  $id = intval($id);
  DB::query("DELETE FROM pm_myitem where `id`='$id'");
}
