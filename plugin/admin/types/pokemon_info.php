<?php

function translate_pokemon_site_id_to_label($id)
{
  switch (intval($id)) {
    case 1:
      return 'header';
    case 2:
      return 'bag';
    case 3:
      return 'store';
    case 4:
      return 'hospital';
    default:
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "未知的宠物位置 $id";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function translate_pokemon_site_label_to_id($str)
{
  switch ($str) {
    case 'header':
      return 1;
    case 'bag':
      return 2;
    case 'store':
      return 3;
    case 'hospital':
      return 4;
    default:
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "未知的宠物位置 $str";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function translate_pokemon_status_id_to_label($id)
{
  switch ($id) {
    case 1:
      return "normal";
    case 2:
      return "sick1";
    case 3:
      return "sick2";
    case 4:
      return "sick3";
    case 5:
      return "hungry1";
    case 6:
      return "hungry2";
    case 7:
      return "tired";
    case 8:
      return "excited1";
    case 9:
      return "excited2";
    case 10:
      return "excited3";
    case 11:
      return "hurt";
    case 12:
      return "happy1";
    case 13:
      return "happy2";
    case 14:
      return "happy3";
    case 15:
      return "shock";
    case 16:
      return "self_love1";
    case 17:
      return "self_love2";
    case 18:
      return "angry1";
    case 19:
      return "angry2";
    case 20:
      return "dead";
    default:
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "未知的宠物状态 $id";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function translate_pokemon_status_label_to_id($str)
{
  // 兼容直接以数字 ID（1-20）传入的情况
  if (is_int($str) || (is_string($str) && ctype_digit($str))) {
    $numeric_id = intval($str);
    if ($numeric_id >= 1 && $numeric_id <= 20) {
      return $numeric_id;
    }
  }

  switch ($str) {
    case "normal":
      return 1;
    case "sick1":
      return 2;
    case "sick2":
      return 3;
    case "sick3":
      return 4;
    case "hungry1":
      return 5;
    case "hungry2":
      return 6;
    case "tired":
      return 7;
    case "excited1":
      return 8;
    case "excited2":
      return 9;
    case "excited3":
      return 10;
    case "hurt":
      return 11;
    case "happy1":
      return 12;
    case "happy2":
      return 13;
    case "happy3":
      return 14;
    case "shock":
      return 15;
    case "self_love1":
      return 16;
    case "self_love2":
      return 17;
    case "angry1":
      return 18;
    case "angry2":
      return 19;
    case "dead":
      return 20;
    default:
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "未知的宠物状态 $str";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function translate_pokemon_sex_id_to_label($id)
{
  switch (intval($id)) {
    case 1:
      return "male";
    case 2:
      return "female";
    default:
      return "unknown";
  }
}

function translate_pokemon_sex_label_to_id($str)
{
  switch ($str) {
    case "male":
      return 1;
    case "female":
      return 2;
    default:
      return 0;
  }
}

function new_pokemon_skill_info($type_id, $count)
{
  $ret = [];

  $ret["type_id"] = intval($type_id);
  $ret["count"] = intval($count);

  return $ret;
}

function new_pokemon_info(
  $id,
  $type_id,
  $owner,
  $name,
  $site,

  $level,
  $experience,
  $intimacy,
  $using_ball_id,
  $is_shiny,
  $status,

  $sex,
  $statistic,
  $base_points,
  $skills,
  $armor_slots_id
) {
  $ret = [];
  $ret["_TYPE"] = "pokemon_info";

  $ret["id"] = intval($id);
  $ret["type_id"] = intval($type_id);
  $ret["owner"] = intval($owner);
  $ret["name"] = $name;
  $ret["site"] = $site;

  $ret["level"] = intval($level);
  $ret["experience"] = intval($experience);
  $ret["intimacy"] = intval($intimacy);
  $ret["using_ball_id"] = intval($using_ball_id);
  $ret["is_shiny"] = boolval($is_shiny);
  $ret["status"] = $status;
  $ret["sex"] = $sex;

  $ret["statistic"] = $statistic;
  $ret["base_points"] = $base_points;
  $ret["skills"] = $skills;
  $ret["armor_slots_id"] = [
    is_null($armor_slots_id[0]) ? null : intval($armor_slots_id[0]),
    is_null($armor_slots_id[1]) ? null : intval($armor_slots_id[1]),
    is_null($armor_slots_id[2]) ? null : intval($armor_slots_id[2]),
    is_null($armor_slots_id[3]) ? null : intval($armor_slots_id[3])
  ];

  return $ret;
}
