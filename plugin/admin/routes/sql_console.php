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

  $rows = DB::fetch_all($sql);
  $result = [];
  foreach ($rows as $data) {
    $result[] = $data;
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
