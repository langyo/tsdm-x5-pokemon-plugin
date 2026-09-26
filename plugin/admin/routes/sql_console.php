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

  // 从首次执行的结果集中取行：此前 DB::fetch_all($sql) 会把语句再执行一遍，
  // 任何 INSERT/UPDATE 都会被双写，这里改为直接消费 $query 结果。
  $result = [];
  while ($data = DB::fetch($query)) {
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
