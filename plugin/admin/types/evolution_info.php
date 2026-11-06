<?php

function new_evolution_limit_type(
  $type,
  $val,
  $item_name = null
) {
  $ret = [];

  switch ($type) {
    case 'level':
      $ret["min_level"] = intval($val);
      break;
    case 'item':
      $ret["use_item"] = intval($val);
      // 添加道具名称（始终包含该字段，即使是空字符串）
      $ret["item_name"] = $item_name !== null ? $item_name : '';
      break;
    case 'good':
      $ret["min_intimacy"] = intval($val);
      break;
    case 'comp_atk_def':
      switch ($val) {
        case '<':
          $ret["compare_attack_and_defense"] = 'less';
          break;
        case '>':
          $ret["compare_attack_and_defense"] = 'greater';
          break;
        case '=':
          $ret["compare_attack_and_defense"] = 'equal';
          break;
        default:
          $json_ret = [];
          $json_ret["success"] = false;
          $json_ret["reason"] = "未知的进化规则 $type $val";
          exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
      }
      break;
    case 'random':
      $ret["random"] = floatval($val);
      break;
    case 'sex':
      switch ($val) {
        case 'male':
          $ret["sex"] = 'male';
          break;
        case 'female':
          $ret["sex"] = 'female';
          break;
        default:
          $ret["sex"] = 'unknown';
          break;
      }
      break;
    case 'is_have_chairs':
      $ret["is_bag_have_chairs"] = $val == 'true' ? true : false;
      break;
    case 'is_exchange':
      // 旧格式：是否进行过交换，转换为背包有空位条件作为替代
      $ret["is_bag_have_chairs"] = true;  // 使用默认值
      break;
    default:
      $json_ret = [];
      $json_ret["success"] = false;
      $json_ret["reason"] = "未知的进化规则 $type $val";
      exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  return $ret;
}

function translate_evolution_info_label_to_db_cond($obj)
{
  if (isset($obj["min_level"])) {
    return ['level', $obj["min_level"]];
  } else if (isset($obj["use_item"])) {
    return ['item', $obj["use_item"]];
  } else if (isset($obj["min_intimacy"])) {
    return ['good', $obj["min_intimacy"]];
  } else if (isset($obj["compare_attack_and_defense"])) {
    switch ($obj["compare_attack_and_defense"]) {
      case 'greater':
        $val = '>';
        break;
      case 'less':
        $val = '<';
        break;
      case 'equal':
        $val = '=';
        break;
      default:
        $json_ret = [];
        $json_ret["success"] = false;
        $json_ret["reason"] = "未知的进化规则 " . print_r($obj, true);
        exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
    }
    return ['comp_atk_def', $val];
  } else if (isset($obj["random"])) {
    return ['random', $obj["random"]];
  } else if (isset($obj["sex"])) {
    return ['sex',  $obj["sex"]];
  } else if (isset($obj["is_bag_have_chairs"])) {
    return ['is_have_chairs', $obj["is_bag_have_chairs"] ? 'true' : 'false'];
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "未知的进化规则 " . print_r($obj, true);
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function new_evolution_info(
  $id,
  $source_id,
  $target_id,
  $source_name,
  $target_name,

  $condition,
  $priority,
  $item_name = null
) {
  $ret = [];
  $ret["_TYPE"] = "evolution_info";

  $ret["id"] = intval($id);
  $ret["source_id"] = intval($source_id);
  $ret["target_id"] = intval($target_id);
  $ret["source_name"] = $source_name;
  $ret["target_name"] = $target_name;

  $ret["condition"] = $condition;
  $ret["priority"] = intval($priority);
  // 添加道具名称字段（始终包含，即使是空字符串）
  $ret["item_name"] = $item_name !== null ? $item_name : '';

  return $ret;
}
