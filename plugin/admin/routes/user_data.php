<?php

function count_user_info()
{
  $count = DB::result_first("SELECT count(*) from pm_usersdata");

  $item = [];
  $item["count"] = intval($count);

  $item["_TYPE"] = "::count";

  return [$item];
}

function list_user_info($from, $count)
{
  $from = intval($from);
  $count = intval($count);

  $ret = [];
  if ($query_all_user = DB::query("SELECT * from pm_usersdata order by `uid` asc limit $from,$count")) {
    while ($query_user = DB::fetch($query_all_user)) {
      $uid = intval($query_user['uid']);

      $pokemon_list = list_pokemon_info($uid, 0, 100);
      $item_list = list_item_info($uid, 0, 100);

      $pokemon_id_list = [];
      foreach ($pokemon_list as $item) {
        array_push($pokemon_id_list, intval($item["id"]));
      }
      $item_id_list = [];
      foreach ($item_list as $item) {
        array_push($item_id_list, intval($item["id"]));
      }

      $extra = [
        'dataall' => intval($query_user['dataall']),
        'strength' => intval($query_user['strength']),
        'str' => intval($query_user['str']),
        'boxnum' => intval($query_user['boxnum']),
        'allure' => intval($query_user['allure']),
        'capture' => intval($query_user['capture']),
      ];

      $item = new_user_info(
        $uid,
        DB::fetch_first("SELECT * from " . DB::table('common_member') . " where `uid`='$uid'")['username'],
        intval($query_user['datawin']),
        intval($query_user['datalost']),
        intval($query_user['money']),
        intval($query_user['fullexp']),
        $pokemon_id_list,
        $item_id_list,
        $extra
      );

      $item["_TYPE"] = "user_info";
      array_push($ret, $item);

      $ret = $ret;
    }
    return $ret;
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "数据库无法访问";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function get_user_info($id)
{
  $id = intval($id);

  $ret = [];
  if ($query_user = DB::fetch_first("SELECT * from pm_usersdata where `uid`={$id}")) {
    $uid = intval($query_user['uid']);

    $pokemon_list = list_pokemon_info($uid, 0, 100);
    $item_list = list_item_info($uid, 0, 100);

    $pokemon_id_list = [];
    foreach ($pokemon_list as $item) {
      array_push($pokemon_id_list, intval($item["id"]));
    }
    $item_id_list = [];
    foreach ($item_list as $item) {
      array_push($item_id_list, intval($item["id"]));
    }

    $extra = [
      'dataall' => intval($query_user['dataall']),
      'strength' => intval($query_user['strength']),
      'str' => intval($query_user['str']),
      'boxnum' => intval($query_user['boxnum']),
      'allure' => intval($query_user['allure']),
      'capture' => intval($query_user['capture']),
    ];

    $item = new_user_info(
      $uid,
      DB::fetch_first("SELECT * from " . DB::table('common_member') . " where `uid`='$uid'")['username'],
      intval($query_user['datawin']),
      intval($query_user['datalost']),
      intval($query_user['money']),
      intval($query_user['fullexp']),
      $pokemon_id_list,
      $item_id_list,
      $extra
    );

    $item["_TYPE"] = "user_info";
    array_push($ret, $item);

    return $ret;
  } else {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "无法查询用户信息 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function set_user_info($info)
{
  $id = intval($info["id"]);

  if (!($query = DB::fetch_first("SELECT * from pm_usersdata where `uid`={$id}"))) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "用户信息更新失败，未找到用户 #$id";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  $updates = [];

  $fields = [
    'money' => 'money',
    'datawin' => 'win_count',
    'datalost' => 'lose_count',
    'fullexp' => 'experience',
    'dataall' => 'total_battles',
    'strength' => 'strength',
    'str' => 'str',
    'boxnum' => 'boxnum',
    'allure' => 'allure',
    'capture' => 'capture',
  ];

  foreach ($fields as $db_col => $json_key) {
    if (isset($info[$json_key])) {
      $val = intval($info[$json_key]);
      if ($db_col === 'money' && $val < 0) {
        $json_ret = [];
        $json_ret["success"] = false;
        $json_ret["reason"] = "金钱不得为负数";
        exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
      }
      $old_val = intval($query[$db_col]);
      if ($old_val !== $val) {
        $updates[] = "`$db_col`=$val";
      }
    }
  }

  if (empty($updates)) {
    return $id;
  }

  $sql = "UPDATE pm_usersdata SET " . implode(", ", $updates) . " WHERE `uid`=$id";
  DB::query($sql);
}

function insert_user_info($info)
{
  $id = intval($info["id"]);

  // 提前检查，如果已经有对应 ID 的用户，阻止继续创建的行为
  if (DB::fetch_first("SELECT * from pm_usersdata where `uid`={$id}")) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "用户信息创建失败，已经存在 ID 为 #$id 的用户";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  // 这个插入函数比较特殊，由于是新建用户档案，因此除了用户 ID 外，其他信息都是默认值
  DB::query("INSERT INTO pm_usersdata (`uid`, `money`) VALUES ($id, 300)");

  return $id;
}

function filter_user_info($list)
{
  $ret = [];

  foreach ($list as $item) {
    $query_sql = "SELECT * from pm_usersdata where ";
    $query_sql_list = [];
    $operator = $item["operator"];
    $value = addslashes($item["value"]);

    switch ($item["tag"]) {
      case "UID":
        array_push($query_sql_list, generate_filter_sql('uid', $operator, $value, 'id'));
        break;
      case '昵称':
        // 智能判断：如果是纯数字，优先按 UID 精确查询
        if (ctype_digit($value) && $value !== '') {
          // 先尝试 UID 精确查询
          $id_query_sql = "SELECT * from pm_usersdata where uid = " . intval($value);
          if ($query_all = DB::query($id_query_sql)) {
            $found_id_results = [];
            while ($query = DB::fetch($query_all)) {
              $found_id_results[] = $query;
            }
            // 如果找到 UID 匹配的结果，直接返回
            if (count($found_id_results) > 0) {
              foreach ($found_id_results as $query_user) {
                $uid = intval($query_user['uid']);

                $pokemon_list = list_pokemon_info($uid, 0, 100);
                $item_list = list_item_info($uid, 0, 100);

                $pokemon_id_list = [];
                foreach ($pokemon_list as $item) {
                  array_push($pokemon_id_list, intval($item["id"]));
                }
                $item_id_list = [];
                foreach ($item_list as $item) {
                  array_push($item_id_list, intval($item["id"]));
                }

                $extra = [
                  'dataall' => intval($query_user['dataall']),
                  'strength' => intval($query_user['strength']),
                  'str' => intval($query_user['str']),
                  'boxnum' => intval($query_user['boxnum']),
                  'allure' => intval($query_user['allure']),
                  'capture' => intval($query_user['capture']),
                ];

                $item = new_user_info(
                  $uid,
                  $query_user['username'],
                  intval($query_user['datawin']),
                  intval($query_user['datalost']),
                  intval($query_user['money']),
                  intval($query_user['fullexp']),
                  $pokemon_id_list,
                  $item_id_list,
                  $extra
                );

                $item["_TYPE"] = "user_info";
                array_push($ret, $item);
              }
              return $ret;
            }
          }
        }
        // UID 查询无结果或非数字输入，使用昵称模糊搜索
        if ($query_all = DB::query("SELECT * FROM %t WHERE username LIKE %s", ['common_member', "%$value%"])) {
          while ($query = DB::fetch($query_all)) {
            $uid = intval($query['uid']);
            array_push($query_sql_list, generate_filter_sql('uid', $operator, $uid, 'id'));
          }
        }
        break;
      case '胜场':
        array_push($query_sql_list, generate_filter_sql('datawin', $operator, $value, 'number'));
        break;
      case '输场':
        array_push($query_sql_list, generate_filter_sql('datalost', $operator, $value, 'number'));
        break;
      case '金钱':
        array_push($query_sql_list, generate_filter_sql('money', $operator, $value, 'number'));
        break;
      case '经验值':
        array_push($query_sql_list, generate_filter_sql('fullexp', $operator, $value, 'number'));
        break;
      default:
    }
  }

  if (count($query_sql_list) <= 0) {
    return $ret;
  }
  if (trim(implode(" and ", $query_sql_list)) == "") {
    return $ret;
  }
  $query_sql .= implode(" and ", $query_sql_list);
  $query_sql .= " limit 20";

  if ($query_all = DB::query($query_sql)) {
    while ($query_user = DB::fetch($query_all)) {
      $uid = intval($query_user['uid']);

      $pokemon_list = list_pokemon_info($uid, 0, 100);
      $item_list = list_item_info($uid, 0, 100);

      $pokemon_id_list = [];
      foreach ($pokemon_list as $item) {
        array_push($pokemon_id_list, intval($item["id"]));
      }
      $item_id_list = [];
      foreach ($item_list as $item) {
        array_push($item_id_list, intval($item["id"]));
      }

      $extra = [
        'dataall' => intval($query_user['dataall']),
        'strength' => intval($query_user['strength']),
        'str' => intval($query_user['str']),
        'boxnum' => intval($query_user['boxnum']),
        'allure' => intval($query_user['allure']),
        'capture' => intval($query_user['capture']),
      ];

      $item = new_user_info(
        $uid,
        DB::fetch_first("SELECT * from " . DB::table('common_member') . " where `uid`='$uid'")['username'],
        intval($query_user['datawin']),
        intval($query_user['datalost']),
        intval($query_user['money']),
        intval($query_user['fullexp']),
        $pokemon_id_list,
        $item_id_list,
        $extra
      );

      $item["_TYPE"] = "user_info";
      array_push($ret, $item);
    }
  }

  return $ret;
}
