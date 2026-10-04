<?php

function generate_filter_sql($key, $operator, $value, $type = 'text')
{
  switch ($type) {
    case 'id':
      switch ($operator) {
        case 'equal':
          return "`$key`='$value'";
        case "not_equal":
          return "`$key`!='$value'";
        default:
          return "";
      }
    case 'text':
      switch ($operator) {
        case 'equal':
          return "`$key` like '%$value%'";
        case "not_equal":
          return "`$key` not like '%$value%'";
        default:
          return "";
      }
    case 'number':
      switch ($operator) {
        case 'equal':
          return "`$key`='$value'";
        case 'not_equal':
          return "`$key`!='$value'";
        case 'greater':
          return "`$key`>'$value'";
        case 'greater_or_equal':
          return "`$key`>='$value'";
        case 'less':
          return "`$key`<'$value'";
        case 'less_or_equal':
          return "`$key`<='$value'";
        default:
          return "";
      }
    default:
  }
  return "";
}

include_once __DIR__ . "/types.php";

// 后台路由复用 API 层的共享工具（api_calculate_pokemon_max_hp 等）。
// pm_sql/pm_table 定义在 api/index.php，但该文件在文件末尾对非 endpoint
// 路由直接输出 404 退出，无法在后台上下文引入，这里提供等价实现；
// endpoint 路由（pokemon.inc.php）与后台路由（index=admin）互斥，同一
// 请求内不会出现重复定义。
if (!function_exists('pm_table')) {
  function pm_table($table)
  {
    // Pokemon 系统表名固定为 pm_*，不带 Discuz 前缀，与 api/index.php 保持一致
    return $table;
  }
}

if (!function_exists('pm_sql_v')) {
  function pm_sql_v($sql, $args)
  {
    if (empty($args)) {
      return $sql;
    }
    $i = 0;
    return preg_replace_callback('/%([sd])/', function ($m) use ($args, &$i) {
      $v = isset($args[$i]) ? $args[$i] : '';
      $i++;
      if ($m[1] === 'd') {
        return intval($v);
      }
      return "'" . addslashes($v) . "'";
    }, $sql);
  }
}

if (!function_exists('pm_sql')) {
  function pm_sql($sql)
  {
    $args = func_get_args();
    array_shift($args);
    return pm_sql_v($sql, $args);
  }
}

require_once __DIR__ . "/../api/utils.php";

include_once __DIR__ . "/routes/global_config.php";
include_once __DIR__ . "/routes/item_data.php";
include_once __DIR__ . "/routes/map_data.php";
include_once __DIR__ . "/routes/pokemon_data.php";
include_once __DIR__ . "/routes/user_data.php";
include_once __DIR__ . "/routes/pokemon_info.php";
include_once __DIR__ . "/routes/item_info.php";
include_once __DIR__ . "/routes/evolution_data.php";
include_once __DIR__ . "/routes/skill_type.php";
require_once __DIR__ . "/../api/battle_core.php";
include_once __DIR__ . "/routes/effect_data.php";
include_once __DIR__ . "/routes/file_explorer.php";
