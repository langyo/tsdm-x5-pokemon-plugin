<?php

/**
 * pm_effect 管理（战斗引擎 2.0 效果模板）。
 *
 * params_json 必须内嵌合法的核心效果声明（{"code":..., ...}），
 * hooks_json 必须是钩子数组；写入前经 battle_core_validate_effect 严格
 * 校验，拒绝未知效果码/钩子/参数，避免"存了不支持的效果把数据搞坏"。
 */

function count_effect_data()
{
  $count = DB::result_first("SELECT count(*) from pm_effect");
  $item = [];
  $item["count"] = intval($count);
  $item["_TYPE"] = "::count";
  return [$item];
}

function list_effect_data($from, $count)
{
  $from = intval($from);
  $count = intval($count);
  $ret = [];
  $rows = DB::fetch_all("SELECT * from pm_effect order by `id` asc limit $from,$count");
  foreach ((array)$rows as $row) {
    $ret[] = effect_row_to_item($row);
  }
  return $ret;
}

function get_effect_data($id)
{
  $id = intval($id);
  $row = DB::fetch_first("SELECT * from pm_effect where id=$id");
  if ($row) {
    return [effect_row_to_item($row)];
  }
  $json_ret = [];
  $json_ret["success"] = false;
  $json_ret["reason"] = "无法查询效果 #$id";
  exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
}

function effect_row_to_item($row)
{
  $item = [
    "id" => intval($row["id"]),
    "code" => $row["code"],
    "kind" => $row["kind"],
    "hooks" => json_decode($row["hooks_json"], true) ?: [],
    "params" => json_decode($row["params_json"], true) ?: [],
    "description" => $row["description"],
    "version" => intval($row["version"]),
  ];
  $item["_TYPE"] = "effect_data";
  return $item;
}

/**
 * 组装并校验一条核心效果声明（来自 hooks/params 字段）。
 * 返回 null 表示数据非法（拒绝写入）。
 */
function build_effect_declaration($hooks, $params, $kind, $version)
{
  if (!is_array($hooks) || !is_array($params) || !isset($params["code"])) {
    return null;
  }
  $effect = [
    "code" => strval($params["code"]),
    "kind" => strval($kind ?: "move"),
    "hooks" => array_values($hooks),
    "params" => $params,
    "version" => intval($version) > 0 ? intval($version) : 1,
  ];
  if (battle_core_validate_effect($effect) !== true) {
    return null;
  }
  return $effect;
}

function set_effect_data($info)
{
  $id = intval($info["id"]);
  $row = DB::fetch_first("SELECT * from pm_effect where id=$id");
  if (!$row) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "效果更新失败，未找到 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $hooks = isset($info["hooks"]) ? $info["hooks"] : json_decode($row["hooks_json"], true);
  $params = isset($info["params"]) ? $info["params"] : json_decode($row["params_json"], true);
  $kind = isset($info["kind"]) ? $info["kind"] : $row["kind"];
  $version = isset($info["version"]) ? $info["version"] : $row["version"];
  $effect = build_effect_declaration($hooks, $params, $kind, $version);
  if ($effect === null) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "效果声明校验失败（未知效果码/钩子或参数越界）";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $code = isset($info["code"]) ? $info["code"] : $row["code"];
  $description = isset($info["description"]) ? $info["description"] : $row["description"];
  $hooks_json = json_encode($effect["hooks"], JSON_UNESCAPED_UNICODE);
  $params_json = json_encode($effect["params"], JSON_UNESCAPED_UNICODE);
  DB::query("UPDATE pm_effect set code='" . addslashes($code) . "', kind='" . addslashes($effect["kind"]) . "', hooks_json='" . addslashes($hooks_json) . "', params_json='" . addslashes($params_json) . "', description='" . addslashes($description) . "', version=" . $effect["version"] . " where id=$id");
}

function insert_effect_data($info)
{
  $hooks = isset($info["hooks"]) ? $info["hooks"] : [];
  $params = isset($info["params"]) ? $info["params"] : [];
  $kind = isset($info["kind"]) ? $info["kind"] : "move";
  $version = isset($info["version"]) ? $info["version"] : 1;
  $effect = build_effect_declaration($hooks, $params, $kind, $version);
  if ($effect === null) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "效果声明校验失败（未知效果码/钩子或参数越界）";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $code = isset($info["code"]) ? $info["code"] : "";
  if ($code === "") {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "code 不能为空";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
  $exists = DB::fetch_first("SELECT id from pm_effect where code='" . addslashes($code) . "'");
  if ($exists) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "code 已存在 #" . $exists["id"];
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $description = isset($info["description"]) ? $info["description"] : "";
  $last_id = intval(DB::result_first("SELECT max(id) from pm_effect"));
  $new_id = $last_id + 1;
  $hooks_json = json_encode($effect["hooks"], JSON_UNESCAPED_UNICODE);
  $params_json = json_encode($effect["params"], JSON_UNESCAPED_UNICODE);
  DB::query("INSERT INTO pm_effect (id, code, kind, hooks_json, params_json, description, version) VALUES (
    $new_id, '" . addslashes($code) . "', '" . addslashes($effect["kind"]) . "', '" . addslashes($hooks_json) . "', '" . addslashes($params_json) . "', '" . addslashes($description) . "', " . $effect["version"] . "
  )");
  return $new_id;
}

function delete_effect_data($id)
{
  $id = intval($id);
  $in_use = DB::result_first("SELECT count(*) from pm_skill where effect_id=$id");
  if (intval($in_use) > 0) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "效果 #$id 正被 {$in_use} 个技能引用，先解除关联";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
  DB::query("DELETE from pm_effect where id=$id");
}
