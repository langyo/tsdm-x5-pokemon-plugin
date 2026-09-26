<?php

function translate_win_path_to_linux_path($path)
{
  // 如果为 win 平台，将路径转换为 linux 平台形式，例如 D:\abc\123 转换为 /d/abc/123
  return "/" . strtolower(
    str_replace(
      ":",
      "",
      str_replace(
        "\\",
        "/",
        substr(
          $path,
          0,
          strpos($path, ":") + 1
        )
      )
    )
  ) .
    str_replace("\\", "/", substr($path, strpos($path, ":") + 1));
}

function translate_linux_path_to_win_path($path)
{
  // 如果为 linux 平台，将路径转换为 win 平台形式，例如 /d/abc/123 转换为 D:\abc\123
  return substr($path, 0, strpos($path, "/", 1) + 1) .
    ":\\" .
    str_replace("/", "\\", substr($path, strpos($path, "/", 1) + 1));
}

function validate_explorer_path_segment($value)
{
  // 拒绝空段、相对路径穿越、分隔符与 NUL：正常目录/文件名不会包含这些内容。
  // 该校验是 list_file/get_file 的统一入口防线，防止拼出根目录以外的意外路径形态。
  return is_string($value)
    && $value !== ''
    && $value !== '.'
    && $value !== '..'
    && strpos($value, '/') === false
    && strpos($value, "\\") === false
    && strpos($value, "\0") === false;
}

function translate_path_array_to_path_raw($list)
{
  // 在 linux 下与在 windows 下的路径生成是不太一样的
  // linux 的路径相对很好拼接，直接中间塞 /，开头再加个 / 就完事了
  // windows 的路径就不太好拼接了，开头的第一个元素不是别的，而是盘符

  $path = "/";
  foreach ($list as $value) {
    if (!validate_explorer_path_segment($value)) {
      throw new Exception("非法的路径段");
    }
    $path .= $value . "/";
  }
  $path = substr($path, 0, -1);

  if (strtoupper(substr(PHP_OS, 0, 3)) === 'WIN') {
    return translate_linux_path_to_win_path($path);
  } else {
    return $path;
  }
}

function translate_path_raw_to_path_array($raw)
{
  // 将原始路径分割为一连串的字符串数组
  if (strtoupper(substr(PHP_OS, 0, 3)) === 'WIN') {
    $raw = translate_win_path_to_linux_path($raw);
  }

  $arr = explode("/", $raw);
  array_shift($arr);
  return $arr;
}

function get_cwd()
{
  $ret = [];
  $ret["_TYPE"] = "file_explorer::path";

  try {
    $result = getcwd();
    if (boolval($result) && !$result) {
      throw new Exception("获取当前目录失败");
    }
    $result = translate_path_raw_to_path_array($result);

    $ret["list"] = $result;
  } catch (Exception $e) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "$e";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }

  return $ret;
}

function list_file($path)
{
  $ret = [];
  $ret["_TYPE"] = "file_explorer::directory";

  try {
    // 提前特殊判定，如果数组为空，意味着要获取根目录
    // 在 linux 下的根目录为 /，而在 windows 下其实没有所谓的根目录，而是应当返回一个盘符列表
    if (count($path) == 0) {
      if (strtoupper(substr(PHP_OS, 0, 3)) === 'WIN') {
        // 动态获取都有哪些盘能用
        // 由于权限原因，不得调用主机的 shell，所以改为通过枚举盘符路径直接读取，通过判断是否抛出异常来确定盘是否能用
        $list = [];
        for ($i = intval('A'); $i <= intval('Z'); ++$i) {
          try {
            $result = scandir(chr($i) . ":");
            if (boolval($result) && !$result) {
              $list[] = chr($i);
            }
          } catch (Exception $e) {
            // 忽略错误
          }
        }
        $list = array_values(array_filter($list, function ($value) {
          return $value != "." && $value != "..";
        }));
        foreach ($result as &$value) {
          $value = [$value, "directory"];
        }

        $ret["list"] = $list;
        return $ret;
      } else {
        $list = scandir("/");
        if (boolval($list) && !$list) {
          throw new Exception("目录读取失败");
        }
        $list = array_values(array_filter($list, function ($value) {
          return $value != "." && $value != "..";
        }));
        foreach ($list as &$value) {
          $value = [$value,  "directory"];
        }

        $ret["list"] = $list;
        return $ret;
      }
    }

    $path = translate_path_array_to_path_raw($path);
    $result = scandir($path);

    if (is_null($result) || boolval($result) && !$result) {
      throw new Exception("目录读取失败");
    }

    $result = array_values(array_filter(
      $result,
      function ($value) {
        return $value != "." && $value != "..";
      }
    ));
    if (strtoupper(substr(PHP_OS, 0, 3)) === 'WIN') {
      foreach ($result as &$value) {
        $value = [translate_win_path_to_linux_path($value), is_dir($path . "/" . $value) ? "directory" : "file"];
      }
    } else {
      foreach ($result as &$value) {
        $value = [$value, is_dir($path . "/" . $value) ? "directory" : "file"];
      }
    }

    $ret["list"] = $result;

    return $ret;
  } catch (Exception $e) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "$e";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}

function get_file($path)
{
  $ret = [];
  $ret["_TYPE"] = "::raw_string";

  try {
    $path = translate_path_array_to_path_raw($path);
    $result = file_get_contents($path);

    if (boolval($result) && !$result) {
      throw new Exception("文件读取失败");
    }

    $ret["raw"] = $result;

    return $ret;
  } catch (Exception $e) {
    $json_ret = [];
    $json_ret["success"] = false;
    $json_ret["reason"] = "$e";
    exit(json_encode($json_ret, JSON_UNESCAPED_UNICODE));
  }
}
