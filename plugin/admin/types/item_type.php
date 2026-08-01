<?php

function new_item_tag($type, $query)
{
  $ret = [];

  switch ($type) {
    case 1:
      // 药品
      $ret["drug"] = null;
      break;
    case 2:
      // 宠物球
      $ret["ball"] = intval($query['ballid']);
      break;
    case 3:
      // 升级素材
      $ret["evolution"] = intval($query['upitem']);
      break;
    case 4:
      // 特殊物品
      $ret["special"] = $query['sitemname'];
      break;
    case 5:
      // 装备
      switch (intval($query['zbtype'])) {
        case 1:
          $ret["armor"] = "head";
          break;
        case 2:
          $ret["armor"] = "necklace";
          break;
        case 3:
          $ret["armor"] = "weapon";
          break;
        case 4:
          $ret["armor"] = "armor";
          break;
        default:
          $json_ret = [];
          $json_ret["success"] = false;
          $json_ret["reason"] = "未知的物品装备类型 $type {$query['zbtype']}";
          exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
      }
      break;
    default:
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "未知的物品装备类型 $type";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  return $ret;
}

function new_item_limits($query)
{
  $ret = [];

  $ret["min_level"] = intval($query['lvask']);
  $ret["kind_require"] = translate_chinese_kind_to_kind_id($query['xsask']);

  return $ret;
}

function new_item_effects($query)
{
  $ret = [];

  $effects = json_decode($query['effects'], true);
  $equipment = json_decode($query['equipment'], true);
  $ret["add_hit_points"] = intval($effects['hp'] ?? 0);
  $ret["add_experience"] = intval($effects['exp'] ?? 0);
  $ret["add_level"] = intval($effects['level'] ?? 0);
  $ret["add_intimacy"] = intval($effects['intimacy'] ?? 0);
  $ret["attribute_add_hit_points"] = intval($equipment['hp'] ?? 0);
  $ret["attribute_add_attack"] = intval($equipment['atk'] ?? 0);
  $ret["attribute_add_defense"] = intval($equipment['def'] ?? 0);
  $ret["attribute_add_special_attack"] = intval($equipment['spatk'] ?? 0);
  $ret["attribute_add_special_defense"] = intval($equipment['spdef'] ?? 0);
  $ret["attribute_add_speed"] = intval($equipment['spd'] ?? 0);
  $ret["capture"] = intval($query['captmax']);

  return $ret;
}

function translate_item_tag_id_to_db_raw($obj)
{
  if ($obj == 'drug') {
    return [1];
  } else if (isset($obj["ball"])) {
    return [2, intval($obj["ball"])];
  } else if (isset($obj["evolution"])) {
    return [3, intval($obj["evolution"])];
  } else if (isset($obj["special"])) {
    return [4, $obj["special"]];
  } else if (isset($obj["armor"])) {
    switch ($obj["armor"]) {
      case "head":
        return [5, 1];
      case "necklace":
        return [5, 2];
      case "weapon":
        return [5, 3];
      case "armor":
        return [5, 4];
      default:
        $json_ret = [];
        $json_ret["success"] = false;
        $json_ret["reason"] = "未知的物品装备类型 {$obj["armor"]}";
        exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "未知的物品类型";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function new_item_type(
  $id,
  $name,
  $img_name,
  $description,

  $is_selling,
  $price,
  $tag,

  $limits,
  $effects
) {
  $ret = [];
  $ret["_TYPE"] = "item_type";

  $ret["id"] = intval($id);
  $ret["name"] = $name;
  $ret["img_name"] = $img_name;
  $ret["description"] = $description;

  $ret["is_selling"] = boolval($is_selling);
  $ret["price"] = intval($price);
  $ret["tag"] = $tag;

  $ret["limits"] = $limits;
  $ret["effects"] = $effects;

  return $ret;
}
