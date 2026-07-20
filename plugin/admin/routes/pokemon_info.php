<?php

function normalize_pokemon_status($status, $pokemon_id)
{
  $status = intval($status);
  if ($status < 1 || $status > 20) {
    DB::query("UPDATE pm_mypm set `state`='1' where `id`='$pokemon_id'");
    return 1;
  }
  return $status;
}

function list_pokemon_info($uid, $from, $count)
{
  $ret = [];
  $rows = DB::fetch_all("SELECT * from pm_mypm where `uid`='$uid' limit $from,$count");
  foreach ($rows as $query) {
      $pokemon_id = intval($query['id']);
      $skills = [];
      $skill_rows = DB::fetch_all("SELECT * from pm_myskill where `uid`='$uid' and `petid`='$pokemon_id'");
      foreach ($skill_rows as $query_skill) {
          array_push($skills, new_pokemon_skill_info($query_skill['skillid'], $query_skill['skillnum']));
        }

      $item = new_pokemon_info(
        $pokemon_id,
        intval($query['pmno']),
        $uid,
        $query['nowname'],
        translate_pokemon_site_id_to_label(intval($query['site'])),
        intval($query['level']),
        intval($query['exp']),
        intval($query['good']),
        intval($query['ballid']),
        intval($query['sg']) == 1,
        translate_pokemon_status_id_to_label(normalize_pokemon_status($query['state'], $pokemon_id)),
        translate_pokemon_sex_id_to_label(intval($query['sex'])),
        new_pokemon_attributes(
          intval($query['hpg']),
          intval($query['atkg']),
          intval($query['defg']),
          intval($query['spatkg']),
          intval($query['spdefg']),
          intval($query['sdg'])
        ),
        new_pokemon_attributes(
          intval($query['hpn']),
          intval($query['atkn']),
          intval($query['defn']),
          intval($query['spatkn']),
          intval($query['spdefn']),
          intval($query['sdn'])
        ),
        $skills,
        [
          intval($query['equipmentid1']) > 0 ? $query['equipmentid1'] : null,
          intval($query['equipmentid2']) > 0 ? $query['equipmentid2'] : null,
          intval($query['equipmentid3']) > 0 ? $query['equipmentid3'] : null,
          intval($query['equipmentid4']) > 0 ? $query['equipmentid4'] : null,
        ]
      );
      array_push($ret, $item);
    }

  return $ret;
}

function get_pokemon_info($id)
{
  $id = intval($id);
  $ret = [];

  if ($query = DB::fetch_first("SELECT * from pm_mypm where `id`='$id'")) {
    $uid = intval($query['uid']);
    $pokemon_id = intval($query['id']);
    $skills = [];
    $skill_rows = DB::fetch_all("SELECT * from pm_myskill where `uid`='$uid' and `petid`='$pokemon_id'");
    foreach ($skill_rows as $query_skill) {
        array_push($skills, new_pokemon_skill_info($query_skill['skillid'], $query_skill['skillnum']));
      }

    array_push($ret, new_pokemon_info(
      $pokemon_id,
      intval($query['pmno']),
      $uid,
      $query['nowname'],
      translate_pokemon_site_id_to_label(intval($query['site'])),
      intval($query['level']),
      intval($query['exp']),
      intval($query['good']),
      intval($query['ballid']),
      intval($query['sg']) == 1,
      translate_pokemon_status_id_to_label(normalize_pokemon_status($query['state'], $pokemon_id)),
      translate_pokemon_sex_id_to_label(intval($query['sex'])),

      new_pokemon_attributes(
        intval($query['hpg']),
        intval($query['atkg']),
        intval($query['defg']),
        intval($query['spatkg']),
        intval($query['spdefg']),
        intval($query['sdg'])
      ),
      new_pokemon_attributes(
        intval($query['hpn']),
        intval($query['atkn']),
        intval($query['defn']),
        intval($query['spatkn']),
        intval($query['spdefn']),
        intval($query['sdn'])
      ),
      $skills,
      [
        intval($query['equipmentid1']) > 0 ? $query['equipmentid1'] : null,
        intval($query['equipmentid2']) > 0 ? $query['equipmentid2'] : null,
        intval($query['equipmentid3']) > 0 ? $query['equipmentid3'] : null,
        intval($query['equipmentid4']) > 0 ? $query['equipmentid4'] : null,
      ]
    ));

    return $ret;
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法查询宠物信息 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function set_pokemon_info($info)
{
  $id = intval($info["id"]);
  if ($query = DB::fetch_first("SELECT * from pm_mypm where `id`='$id'")) {
    // 提前检查，禁止修改持有用户
    if (intval($query['uid']) != intval($info["owner"])) {
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "无法修改宠物信息，禁止修改持有用户信息 #$id";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }

    if ($query['nowname'] != $info["name"]) {
      DB::query("UPDATE pm_mypm set `nowname`='{$info["name"]}' where `id`='$id'");
    }
    if (intval($query['site']) != translate_pokemon_site_label_to_id($info["site"])) {
      DB::query("UPDATE pm_mypm set `site`='" . translate_pokemon_site_label_to_id($info["site"]) . "' where `id`='$id'");
    }

    if (intval($query['level']) != intval($info["level"])) {
      DB::query("UPDATE pm_mypm set `level`='" . intval($info["level"]) . "' where `id`='$id'");
    }
    if (intval($query['exp']) != intval($info["experience"])) {
      DB::query("UPDATE pm_mypm set `exp`='" . intval($info["exp"]) . "' where `id`='$id'");
    }
    if (intval($query['good']) != intval($info["intimacy"])) {
      DB::query("UPDATE pm_mypm set `good`='" . intval($info["good"]) . "' where `id`='$id'");
    }
    if (intval($query['ballid']) != intval($info["using_ball_id"])) {
      DB::query("UPDATE pm_mypm set `ballid`='" . intval($info["ballid"]) . "' where `id`='$id'");
    }
    if (boolval($query['sg']) != boolval($info["is_shiny"])) {
      DB::query("UPDATE pm_mypm set `sg`='" . boolval($info["sg"]) ? 1 : 0 . "' where `id`='$id'");
    }
    if ($query['state'] != translate_pokemon_status_label_to_id($info["status"])) {
      DB::query("UPDATE pm_mypm set `state`='" . translate_pokemon_status_label_to_id($info["state"]) . "' where `id`='$id'");
    }
    if ($query['sex'] != translate_pokemon_sex_label_to_id($info["sex"])) {
      DB::query("UPDATE pm_mypm set `sex` ='" . translate_pokemon_sex_label_to_id($info["sex"]) . "' where `id`='$id'");
    }

    if (intval($query['hpg']) != intval($info["statistic"]["hit_points"])) {
      DB::query("UPDATE pm_mypm set `hpg`='" . intval($info["statistic"]["hit_points"]) . "' where `id`='$id'");
    }
    if (intval($query['atkg']) != intval($info["statistic"]["attack"])) {
      DB::query("UPDATE pm_mypm set `atkg`='" . intval($info["statistic"]["attack"]) . "' where `id`='$id'");
    }
    if (intval($query['defg']) != intval($info["statistic"]["defense"])) {
      DB::query("UPDATE pm_mypm set `defg`='" . intval($info["statistic"]["defense"]) . "' where `id`='$id'");
    }
    if (intval($query['spatkg']) != intval($info["statistic"]["special_attack"])) {
      DB::query("UPDATE pm_mypm set `spatkg`='" . intval($info["statistic"]["special_attack"]) . "' where `id`='$id'");
    }
    if (intval($query['spdefg']) != intval($info["statistic"]["special_defense"])) {
      DB::query("UPDATE pm_mypm set `spdefg`='" . intval($info["statistic"]["special_defense"]) . "' where `id`='$id'");
    }
    if (intval($query['sdg']) != intval($info["statistic"]["speed"])) {
      DB::query("UPDATE pm_mypm set `sdg`='" . intval($info["statistic"]["speed"]) . "' where `id`='$id'");
    }

    if (intval($query['hpn']) != intval($info["base_points"]["hit_points"])) {
      DB::query("UPDATE pm_mypm set `hpn`='" . intval($info["base_points"]["hit_points"]) . "' where `id`='$id'");
    }
    if (intval($query['atkn']) != intval($info["base_points"]["attack"])) {
      DB::query("UPDATE pm_mypm set `atkn`='" . intval($info["base_points"]["attack"]) . "' where `id`='$id'");
    }
    if (intval($query['defn']) != intval($info["base_points"]["defense"])) {
      DB::query("UPDATE pm_mypm set `defn`='" . intval($info["base_points"]["defense"]) . "' where `id`='$id'");
    }
    if (intval($query['spatkn']) != intval($info["base_points"]["special_attack"])) {
      DB::query("UPDATE pm_mypm set `spatkn`='" . intval($info["base_points"]["special_attack"]) . "' where `id`='$id'");
    }
    if (intval($query['spdefn']) != intval($info["base_points"]["special_defense"])) {
      DB::query("UPDATE pm_mypm set `spdefn`='" . intval($info["base_points"]["special_defense"]) . "' where `id`='$id'");
    }
    if (intval($query['sdn']) != intval($info["base_points"]["speed"])) {
      DB::query("UPDATE pm_mypm set `sdn`='" . intval($info["base_points"]["speed"]) . "' where `id`='$id'");
    }

    if (intval($query['equipmentid1']) != intval($info["armor_slots_id"][0])) {
      DB::query("UPDATE pm_mypm set `equipmentid1`='" . intval($info["armor_slots_id"][0]) . "' where `id`='$id'");
    }
    if (intval($query['equipmentid2']) != intval($info["armor_slots_id"][1])) {
      DB::query("UPDATE pm_mypm set `equipmentid2`='" . intval($info["armor_slots_id"][1]) . "' where `id`='$id'");
    }
    if (intval($query['equipmentid3']) != intval($info["armor_slots_id"][2])) {
      DB::query("UPDATE pm_mypm set `equipmentid3`='" . intval($info["armor_slots_id"][2]) . "' where `id`='$id'");
    }
    if (intval($query['equipmentid4']) != intval($info["armor_slots_id"][3])) {
      DB::query("UPDATE pm_mypm set `equipmentid4`='" . intval($info["armor_slots_id"][3]) . "' where `id`='$id'");
    }

    // 额外更新技能数据表
    foreach ($info["skills"] as $skill) {
      $uid = intval($info["uid"]);
      $petid = intval($id);
      $skillid = intval($skill["type_id"]);
      $skillnum = intval($skill["count"]);

      if ($query = DB::fetch_first("SELECT * FROM pm_myskill WHERE `uid`='$uid' AND `petid`='$petid' AND `skillid`='$skillid'")) {
        if (intval($query['skillnum']) != $skillnum) {
          DB::query("UPDATE pm_myskill set `skillnum`='$skillnum' where `uid`='$uid' AND `petid`='$petid' AND `skillid`='$skillid'");
        }
      } else {
        $json_ret = [];
        $json_ret["success"] = false;
        $json_ret["reason"] = "宠物信息更新失败，未找到技能信息 #$id";
        exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
      }
    }
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "宠物信息更新失败，未找到 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function insert_pokemon_info($info)
{
  $pmno = intval($info["type_id"]);
  $uid = intval($info["owner"]);
  $nowname = strval($info["name"]);
  $site = translate_pokemon_site_label_to_id($info["site"]);

  $level = intval($info["level"]);
  $exp = intval($info["experience"]);
  $good = intval($info["intimacy"]);
  $ballid = intval($info["using_ball_id"]);
  $sg = boolval($info["is_shiny"]) ? 1 : 0;
  $state = translate_pokemon_status_label_to_id($info["status"]);
  $sex = translate_pokemon_sex_label_to_id($info["sex"]);

  $hpg = intval($info["statistic"]["hit_points"]);
  $atkg = intval($info["statistic"]["attack"]);
  $defg = intval($info["statistic"]["defense"]);
  $spatkg = intval($info["statistic"]["special_attack"]);
  $spdefg = intval($info["statistic"]["special_defense"]);
  $sdg = intval($info["statistic"]["speed"]);

  $hpn = intval($info["base_points"]["hit_points"]);
  $atkn = intval($info["base_points"]["attack"]);
  $defn = intval($info["base_points"]["defense"]);
  $spatkn = intval($info["base_points"]["special_attack"]);
  $spdefn = intval($info["base_points"]["special_defense"]);
  $sdn = intval($info["base_points"]["speed"]);

  $equipmentid1 = is_null($info["armor_slots_id"][0]) ? 0 : intval($info["armor_slots_id"][0]);
  $equipmentid2 = is_null($info["armor_slots_id"][1]) ? 0 : intval($info["armor_slots_id"][1]);
  $equipmentid3 = is_null($info["armor_slots_id"][2]) ? 0 : intval($info["armor_slots_id"][2]);
  $equipmentid4 = is_null($info["armor_slots_id"][3]) ? 0 : intval($info["armor_slots_id"][3]);

  // 提前检查，对应宠物类型必须存在
  if (!DB::fetch_first("SELECT id from pm_data where id='$pmno'")) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "未找到宠物类型 #$pmno";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 提前检查，对应用户必须存在
  if (!DB::fetch_first("SELECT uid from pm_usersdata where uid='$uid'")) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "未找到用户 #$uid";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $last_id = DB::fetch_first("SELECT id from pm_mypm order by id desc limit 1");
  $last_id = intval($last_id['id']);
  $new_id = $last_id + 1;

  DB::query("INSERT into pm_mypm (
      `id`, `pmno`, `uid`, `nowname`, `site`,
      `level`, `exp`, `good`,
      `ballid`, `sg`, `state`, `sex`,
      `hpg`, `atkg`, `defg`, `spatkg`, `spdefg`, `sdg`,
      `hpn`, `atkn`, `defn`, `spatkn`, `spdefn`, `sdn`,
      `equipmentid1`, `equipmentid2`, `equipmentid3`, `equipmentid4`
    ) values (
      '$new_id', '$pmno', '$uid', '$nowname', '$site',
      '$level', '$exp', '$good',
      '$ballid', '$sg', '$state', '$sex',
      '$hpg', '$atkg', '$defg', '$spatkg', '$spdefg', '$sdg',
      '$hpn', '$atkn', '$defn', '$spatkn', '$spdefn', '$sdn',
      '$equipmentid1', '$equipmentid2', '$equipmentid3', '$equipmentid4'
    )");

  // 额外更新血量

  $ajax_pokemon = my_pokemon_data($new_id);
  $pmno = $ajax_pokemon['pmno'];
  $pmsdata = pm_data($pmno);
  list($petmaxhp, $petatk, $petdef, $petspatk, $petspdef, $petsd) = get_pet_stats($pmsdata, $ajax_pokemon);
  parse_pet_wear_items($ajax_pokemon, false, $petmaxhp, $petatk, $petdef, $petspatk, $petspdef, $petsd);
  DB::query("UPDATE pm_mypm set `hp`='$petmaxhp' WHERE `id`='$new_id'");

  // 额外更新技能数据表
  foreach ($info["skills"] as $skill) {
    $uid = intval($info["uid"]);
    $petid = intval($info["type_id"]);
    $skillid = intval($skill["type_id"]);
    $skillnum = intval($skill["count"]);

    DB::query("INSERT into pm_myskill (
        `uid`, `petid`, `skillid`, `skillnum`
      ) values (
        $uid, $petid, $skillid, $skillnum
      )");
  }

  return $new_id;
}

function delete_pokemon_info($id)
{
  $id = intval($id);

  // 获取要删除的宠物信息
  $pokemon = DB::fetch_first("SELECT `uid`, `site` FROM pm_mypm WHERE `id`='$id'");
  if (!$pokemon) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "未找到宠物 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $uid = intval($pokemon['uid']);
  $site = intval($pokemon['site']);

  // 检查玩家的宠物总数
  $total_count = intval(DB::result_first("SELECT COUNT(*) FROM pm_mypm WHERE `uid`='$uid'"));
  if ($total_count <= 1) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法放生最后一只宠物";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 如果是首位宠物（site=1），需要选择替补
  if ($site === 1) {
    // 优先从背包（site=2）选择第一个宠物作为替补
    $replacement = DB::fetch_first("SELECT `id` FROM pm_mypm WHERE `uid`='$uid' AND `site`='2' ORDER BY `id` ASC LIMIT 1");

    // 如果背包没有，从仓库（site=3）选择
    if (!$replacement) {
      $replacement = DB::fetch_first("SELECT `id` FROM pm_mypm WHERE `uid`='$uid' AND `site`='3' ORDER BY `id` ASC LIMIT 1");
    }

    // 如果仓库也没有，从医院（site=4）选择
    if (!$replacement) {
      $replacement = DB::fetch_first("SELECT `id` FROM pm_mypm WHERE `uid`='$uid' AND `site`='4' ORDER BY `id` ASC LIMIT 1");
    }

    // 更新替补宠物为首位
    if ($replacement) {
      $replacement_id = intval($replacement['id']);
      DB::query("UPDATE pm_mypm SET `site`='1' WHERE `id`='$replacement_id'");
    }
  }

  // 删除宠物
  DB::query("DELETE FROM pm_mypm WHERE `id`='$id'");
  DB::query("DELETE FROM pm_myskill WHERE `petid`='$id'");
}
