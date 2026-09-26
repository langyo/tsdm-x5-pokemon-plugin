<?php

function run_sql_console($sql)
{
  $query = DB::query($sql, 'SILENT');
  if ($query === false) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "SQL 执行失败，请检查语法和表名";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // DB::query 已执行过语句；不能用 fetch_all($sql) 无条件取行——那会把语句
  // 再执行一遍，任何 INSERT/UPDATE 都会被双写。仅对幂等的只读语句
  // （与前端 is_read_only_sql 同一白名单）回读行集，写语句不取行。
  $result = [];
  if (preg_match('/^\s*(select|show|describe|desc|explain)\b/i', $sql)) {
    $rows = DB::fetch_all($sql);
    foreach ($rows as $data) {
      $result[] = $data;
    }
  }

  $encoded = json_encode($result, JSON_UNESCAPED_UNICODE);
  if ($encoded === false) {
    // Fallback: try with unicode escaping for non-UTF8 data
    $encoded = json_encode($result, 0);
  }
  if ($encoded === false) {
    $encoded = '[]';
  }

  $ret = [];
  $ret["_TYPE"] = "::raw_string";
  $ret["raw"] = $encoded;
  return [$ret];
}
