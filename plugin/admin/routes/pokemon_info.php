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
        intval($query['species_id']),
        $uid,
        $query['nickname'],
        translate_pokemon_site_id_to_label(intval($query['site'])),
        intval($query['level']),
        intval($query['exp']),
        intval($query['good']),
        intval($query['ballid']),
        intval($query['is_shiny']) == 1,
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
      intval($query['species_id']),
      $uid,
      $query['nickname'],
      translate_pokemon_site_id_to_label(intval($query['site'])),
      intval($query['level']),
      intval($query['exp']),
      intval($query['good']),
      intval($query['ballid']),
      intval($query['is_shiny']) == 1,
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

    if ($query['nickname'] != ($info["name"] ?? $query['nickname'])) {
      DB::query("UPDATE pm_mypm set `nickname`='" . addslashes($info["name"] ?? $query['nickname']) . "' where `id`='$id'");
    }
    if (intval($query['site']) != translate_pokemon_site_label_to_id($info["site"] ?? "header")) {
      DB::query("UPDATE pm_mypm set `site`='" . translate_pokemon_site_label_to_id($info["site"] ?? "header") . "' where `id`='$id'");
    }

    if (intval($query['level']) != intval($info["level"] ?? $query['level'])) {
      DB::query("UPDATE pm_mypm set `level`='" . intval($info["level"] ?? $query['level']) . "' where `id`='$id'");
    }
    if (intval($query['exp']) != intval($info["experience"] ?? $query['exp'])) {
      DB::query("UPDATE pm_mypm set `exp`='" . intval($info["experience"] ?? $query['exp']) . "' where `id`='$id'");
    }
    if (intval($query['good']) != intval($info["intimacy"] ?? $query['good'])) {
      DB::query("UPDATE pm_mypm set `good`='" . intval($info["intimacy"] ?? $query['good']) . "' where `id`='$id'");
    }
    if (intval($query['ballid']) != intval($info["using_ball_id"] ?? $query['ballid'])) {
      DB::query("UPDATE pm_mypm set `ballid`='" . intval($info["using_ball_id"] ?? $query['ballid']) . "' where `id`='$id'");
    }
    if (boolval($query['is_shiny']) != boolval($info["is_shiny"] ?? false)) {
      DB::query("UPDATE pm_mypm set `is_shiny`='" . (boolval($info["is_shiny"] ?? false) ? 1 : 0) . "' where `id`='$id'");
    }
    if ($query['state'] != translate_pokemon_status_label_to_id($info["status"] ?? "normal")) {
      DB::query("UPDATE pm_mypm set `state`='" . translate_pokemon_status_label_to_id($info["status"] ?? "normal") . "' where `id`='$id'");
    }
    if ($query['sex'] != translate_pokemon_sex_label_to_id($info["sex"] ?? "male")) {
      DB::query("UPDATE pm_mypm set `sex` ='" . translate_pokemon_sex_label_to_id($info["sex"] ?? "male") . "' where `id`='$id'");
    }

    $new_hpg = intval($info["statistic"]["hit_points"]);
    $new_atkg = intval($info["statistic"]["attack"]);
    $new_defg = intval($info["statistic"]["defense"]);
    $new_spatkg = intval($info["statistic"]["special_attack"]);
    $new_spdefg = intval($info["statistic"]["special_defense"]);
    $new_sdg = intval($info["statistic"]["speed"]);
    if (
      intval($query['hpg']) != $new_hpg ||
      intval($query['atkg']) != $new_atkg ||
      intval($query['defg']) != $new_defg ||
      intval($query['spatkg']) != $new_spatkg ||
      intval($query['spdefg']) != $new_spdefg ||
      intval($query['sdg']) != $new_sdg
    ) {
      DB::query("UPDATE pm_mypm set hpg=$new_hpg, atkg=$new_atkg, defg=$new_defg, spatkg=$new_spatkg, spdefg=$new_spdefg, sdg=$new_sdg where id='$id'");
    }
    $new_hpn = intval($info["base_points"]["hit_points"]);
    $new_atkn = intval($info["base_points"]["attack"]);
    $new_defn = intval($info["base_points"]["defense"]);
    $new_spatkn = intval($info["base_points"]["special_attack"]);
    $new_spdefn = intval($info["base_points"]["special_defense"]);
    $new_sdn = intval($info["base_points"]["speed"]);
    if (
      intval($query['hpn']) != $new_hpn ||
      intval($query['atkn']) != $new_atkn ||
      intval($query['defn']) != $new_defn ||
      intval($query['spatkn']) != $new_spatkn ||
      intval($query['spdefn']) != $new_spdefn ||
      intval($query['sdn']) != $new_sdn
    ) {
      DB::query("UPDATE pm_mypm set hpn=$new_hpn, atkn=$new_atkn, defn=$new_defn, spatkn=$new_spatkn, spdefn=$new_spdefn, sdn=$new_sdn where id='$id'");
    }

    if (intval($query['equipmentid1']) != intval($info["armor_slots_id"][0] ?? 0)) {
      DB::query("UPDATE pm_mypm set `equipmentid1`='" . intval($info["armor_slots_id"][0] ?? 0) . "' where `id`='$id'");
    }
    if (intval($query['equipmentid2']) != intval($info["armor_slots_id"][1] ?? 0)) {
      DB::query("UPDATE pm_mypm set `equipmentid2`='" . intval($info["armor_slots_id"][1] ?? 0) . "' where `id`='$id'");
    }
    if (intval($query['equipmentid3']) != intval($info["armor_slots_id"][2] ?? 0)) {
      DB::query("UPDATE pm_mypm set `equipmentid3`='" . intval($info["armor_slots_id"][2] ?? 0) . "' where `id`='$id'");
    }
    if (intval($query['equipmentid4']) != intval($info["armor_slots_id"][3] ?? 0)) {
      DB::query("UPDATE pm_mypm set `equipmentid4`='" . intval($info["armor_slots_id"][3] ?? 0) . "' where `id`='$id'");
    }

    // 额外更新技能数据表
    if (!empty($info["skills"])) {
    foreach ($info["skills"] as $skill) {
      $uid = intval($info["owner"] ?? $query['uid']);
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

  // 提前检查，对应宠物类型必须存在（名称与性格沿用种族数据，与捕捉路径一致）
  $type_data = DB::fetch_first("SELECT * from pm_data where id='$pmno'");
  if (!$type_data) {
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

  $pmname = addslashes($type_data['name']);
  $sx = addslashes($type_data['xs']);
  $nickname = addslashes($nowname);
  $current_time = time();

  DB::query("INSERT into pm_mypm (
      `species_id`, `uid`, `pmname`, `nickname`, `site`,
      `level`, `exp`, `good`, `sex`, `sx`,
      `ballid`, `is_shiny`, `state`, `statetime`, `gduptime`,
      `initialuid`, `created_at`,
      `hpg`, `atkg`, `defg`, `spatkg`, `spdefg`, `sdg`,
      `hpn`, `atkn`, `defn`, `spatkn`, `spdefn`, `sdn`,
      `equipmentid1`, `equipmentid2`, `equipmentid3`, `equipmentid4`
    ) values (
      '$pmno', '$uid', '$pmname', '$nickname', '$site',
      '$level', '$exp', '$good', '$sex', '$sx',
      '$ballid', '$sg', '$state', '$current_time', '$current_time',
      '$uid', '$current_time',
      '$hpg', '$atkg', '$defg', '$spatkg', '$spdefg', '$sdg',
      '$hpn', '$atkn', '$defn', '$spatkn', '$spdefn', '$sdn',
      '$equipmentid1', '$equipmentid2', '$equipmentid3', '$equipmentid4'
    )");

  $new_id = intval(DB::insert_id());

  // 满血入库，血量口径与 api_calculate_pokemon_max_hp 保持一致
  $max_hp = api_calculate_pokemon_max_hp(
    [
      'species_id' => $pmno,
      'level' => $level,
      'hpg' => $hpg,
      'hpn' => $hpn,
      'state' => $state,
      'is_shiny' => $sg,
      'equipmentid1' => $equipmentid1,
      'equipmentid2' => $equipmentid2,
      'equipmentid3' => $equipmentid3,
      'equipmentid4' => $equipmentid4,
    ],
    $type_data
  );
  DB::query("UPDATE pm_mypm set `hp`='$max_hp' WHERE `id`='$new_id'");

  // 额外更新技能数据表
  foreach (($info["skills"] ?? []) as $skill) {
    $skillid = intval($skill["type_id"]);
    $skillnum = intval($skill["count"]);

    DB::query("INSERT into pm_myskill (
        `uid`, `petid`, `skillid`, `skillnum`
      ) values (
        '$uid', '$new_id', '$skillid', '$skillnum'
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
