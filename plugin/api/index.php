<?php

/**
 * 纯JSON API - 索引文件
 *
 * 所有API端点返回纯JSON，无HTML混排
 * 统一错误处理和响应格式
 */

// 禁止直接访问
if (!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

/**
 * 全局异常处理器
 *
 * 捕获所有未处理的异常，返回统一的JSON错误响应
 * 记录详细错误到日志，但不向用户暴露敏感信息
 */
function api_exception_handler($exception)
{
    // 记录详细错误到日志
    error_log(sprintf(
        "[API Exception] %s:%d - %s\nStack trace:\n%s",
        $exception->getFile(),
        $exception->getLine(),
        $exception->getMessage(),
        $exception->getTraceAsString()
    ));

    // 根据异常类型返回适当的错误码
    $code = 500;
    $message = sprintf("%s: %s", get_class($exception), $exception->getMessage());

    // 根据异常类型设置HTTP状态码
    if ($exception instanceof InvalidArgumentException) {
        $code = 400;
        $message = 'Invalid Request';
    } elseif ($exception instanceof RuntimeException) {
        $code = 500;
        $message = 'Server Error';
    }

    http_response_code($code);
    echo json_encode([
        'success' => false,
        'error' => $message,
        'code' => $code,
        'timestamp' => time()
    ], JSON_UNESCAPED_UNICODE);
    exit;
}

// 设置全局异常处理器
set_exception_handler('api_exception_handler');

/**
 * 全局错误处理器（处理致命错误）
 *
 * PHP 5.6 兼容：register_shutdown_function 无法检测致命错误类型
 * 只能记录错误日志
 */
function api_shutdown_handler()
{
    $error = error_get_last();
    if ($error !== null && in_array($error['type'], [E_ERROR, E_PARSE, E_CORE_ERROR, E_COMPILE_ERROR])) {
        error_log(sprintf(
            "[Fatal Error] %s:%d - %s",
            $error['file'],
            $error['line'],
            $error['message']
        ));

        // 尝试发送JSON响应（如果headers还未发送）
        if (!headers_sent()) {
            http_response_code(500);
            echo json_encode([
                'success' => false,
                'error' => 'Internal Server Error',
                'code' => 500,
                'timestamp' => time()
            ], JSON_UNESCAPED_UNICODE);
        }
    }
}

// 注册关闭处理函数
register_shutdown_function('api_shutdown_handler');

// 设置JSON响应头
header('Content-Type: application/json; charset=utf-8');
header('Cache-Control: no-store, no-cache, must-revalidate');
header('Pragma: no-cache');

// 统一响应格式
function api_success($data = null)
{
    echo json_encode([
        'success' => true,
        'data' => $data,
        'timestamp' => time()
    ], JSON_UNESCAPED_UNICODE);
    exit;
}

function api_error($message, $code = 400, $debug_info = null, $error_code = null)
{
    http_response_code($code);
    $response = [
        'success' => false,
        'error' => $message,
        'code' => $code,
        'timestamp' => time()
    ];
    // 稳定的机器可读错误码（如 skill_slots_full），供第三方客户端
    // 做本地化映射；不传则不输出该字段，保持旧响应形状
    if ($error_code !== null) {
        $response['error_code'] = $error_code;
    }
    if ($debug_info !== null) {
        $response['debug'] = $debug_info;
    }
    echo json_encode($response, JSON_UNESCAPED_UNICODE);
    exit;
}

function api_json($data)
{
    echo json_encode($data, JSON_UNESCAPED_UNICODE);
    exit;
}

// 验证用户登录
function require_login()
{
    global $_G;
    if (!$_G['uid']) {
        api_error('需要登录', 401);
    }
}

// 获取请求参数
function get_param($key, $default = null)
{
    $value = isset($_GET[$key]) ? $_GET[$key] : $default;
    if ($value === null && isset($_POST[$key])) {
        $value = $_POST[$key];
    }
    return $value;
}

// 获取JSON输入
function get_json_input()
{
    $input = file_get_contents('php://input');
    return json_decode($input, true) ?: [];
}

/**
 * 格式化 SQL 查询字符串（数组参数版本）
 *
 * Discuz X2 的 DB::query 不支持参数绑定，本函数提供
 * %d (整数) 和 %s (转义并加引号的字符串) 的格式化支持
 * PHP 5.6 兼容
 *
 * @param string $sql SQL 模板，支持 %d 和 %s 占位符
 * @param array $args 参数数组
 * @return string 格式化后的 SQL
 */
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

/**
 * 格式化 SQL 查询字符串（可变参数版本）
 *
 * @param string $sql SQL 模板
 * @return string 格式化后的 SQL
 */
function pm_sql($sql)
{
    $args = func_get_args();
    array_shift($args);
    return pm_sql_v($sql, $args);
}

/**
 * 获取 Pokemon 表名
 *
 * Pokemon 系统的表名保持为 pm_*，不受 Discuz 表前缀影响
 * 旧版代码直接使用 pm_mypm 等表名，这里保持一致
 *
 * @param string $table 表名（如 'pm_usersdata'）
 * @return string 表名
 */
function pm_table($table)
{
    // Pokemon 系统表名固定为 pm_*，不带前缀
    // 与旧版代码保持一致
    return $table;
}

/**
 * 验证ID参数（防止SQL注入）
 *
 * @param mixed $id 待验证的ID
 * @param string $name 参数名称（用于错误提示）
 * @return int 验证通过的整数ID
 */
function validate_id($id, $name = 'ID')
{
    // 检查是否为数字或数字字符串
    if (!is_numeric($id)) {
        api_error("Invalid {$name}: must be numeric", 400);
    }

    // 转换为整数
    $int_id = (int) $id;

    // 检查是否为正数
    if ($int_id <= 0) {
        api_error("Invalid {$name}: must be positive", 400);
    }

    return $int_id;
}

/**
 * 验证UID参数（额外的安全检查）
 *
 * @param int $uid 用户ID
 * @return int 验证通过的用户ID
 */
function validate_uid($uid)
{
    if (!is_numeric($uid) || $uid <= 0) {
        api_error('Invalid user ID', 400);
    }
    return (int) $uid;
}

/**
 * 验证字符串参数（防止XSS和SQL注入）
 *
 * @param string $str 待验证的字符串
 * @param string $name 参数名称
 * @param int $max_length 最大长度
 * @return string 清理后的安全字符串
 */
function validate_string($str, $name = 'string', $max_length = 255)
{
    if (!is_string($str)) {
        api_error("Invalid {$name}: must be string", 400);
    }

    // 去除前后空格
    $str = trim($str);

    // 检查长度
    if (strlen($str) > $max_length) {
        api_error("Invalid {$name}: too long (max {$max_length} chars)", 400);
    }

    // 移除潜在的危险字符（基础XSS防护）
    $str = strip_tags($str);

    return $str;
}

/**
 * 验证整数范围
 *
 * @param mixed $value 待验证的值
 * @param string $name 参数名称
 * @param int $min 最小值（包含）
 * @param int $max 最大值（包含）
 * @return int 验证通过的整数
 */
function validate_int_range($value, $name, $min, $max)
{
    if (!is_numeric($value)) {
        api_error("Invalid {$name}: must be numeric", 400);
    }

    $int_value = (int) $value;

    if ($int_value < $min || $int_value > $max) {
        api_error("Invalid {$name}: must be between {$min} and {$max}", 400);
    }

    return $int_value;
}

/**
 * 验证布尔值
 *
 * @param mixed $value 待验证的值
 * @param string $name 参数名称
 * @return bool 布尔值
 */
function validate_bool($value, $name = 'boolean')
{
    if (is_bool($value)) {
        return $value;
    }

    if (is_numeric($value)) {
        return (bool) (int) $value;
    }

    if (is_string($value)) {
        $lower = strtolower($value);
        if ($lower === 'true' || $lower === '1' || $lower === 'yes') {
            return true;
        }
        if ($lower === 'false' || $lower === '0' || $lower === 'no') {
            return false;
        }
    }

    api_error("Invalid {$name}: must be boolean", 400);
}

/**
 * 验证枚举值
 *
 * @param mixed $value 待验证的值
 * @param string $name 参数名称
 * @param array $allowed_values 允许的值列表
 * @return mixed 验证通过的值
 */
function validate_enum($value, $name, array $allowed_values)
{
    if (!in_array($value, $allowed_values, true)) {
        $allowed_list = array();
        foreach ($allowed_values as $v) {
            if (is_string($v)) {
                $allowed_list[] = "'" . $v . "'";
            } else {
                $allowed_list[] = var_export($v, true);
            }
        }
        $allowed = implode(', ', $allowed_list);
        api_error("Invalid {$name}: must be one of [" . $allowed . "]", 400);
    }

    return $value;
}

/**
 * 验证战斗ID格式
 *
 * @param string $battle_id 战斗ID
 * @return string 验证通过的战斗ID
 */
function validate_battle_id($battle_id)
{
    if (!is_string($battle_id)) {
        api_error('Invalid battle_id: must be string', 400);
    }

    $battle_id = trim($battle_id);

    // 检查格式：battle_ 前缀 + 时间戳 + 微秒
    if (!preg_match('/^battle_[0-9]+\.[0-9]+$/', $battle_id)) {
        api_error('Invalid battle_id format', 400);
    }

    return $battle_id;
}

/**
 * 从输入中安全获取必需参数
 *
 * @param array $input 输入数组
 * @param string $key 参数键名
 * @param string $type 参数类型 (id|int|string|bool)
 * @param array $options 额外选项（如 min, max, allowed, max_length）
 * @return mixed 验证后的参数值
 */
function validate_required_param(array $input, $key, $type = 'string', array $options = [])
{
    if (!isset($input[$key])) {
        api_error("Missing required parameter: {$key}", 400);
    }

    $value = $input[$key];

    switch ($type) {
        case 'id':
            return validate_id($value, $key);
        case 'int':
            $min = isset($options['min']) ? $options['min'] : 0;
            $max = isset($options['max']) ? $options['max'] : PHP_INT_MAX;
            return validate_int_range($value, $key, $min, $max);
        case 'string':
            $max_length = isset($options['max_length']) ? $options['max_length'] : 255;
            return validate_string($value, $key, $max_length);
        case 'bool':
            return validate_bool($value, $key);
        case 'enum':
            if (!isset($options['allowed'])) {
                api_error("Validation error: enum type requires 'allowed' option", 500);
            }
            return validate_enum($value, $key, $options['allowed']);
        case 'battle_id':
            return validate_battle_id($value);
        default:
            api_error("Unknown validation type: {$type}", 500);
    }
}

/**
 * 从输入中安全获取可选参数
 *
 * @param array $input 输入数组
 * @param string $key 参数键名
 * @param mixed $default 默认值
 * @param string $type 参数类型
 * @param array $options 额外选项
 * @return mixed 验证后的参数值或默认值
 */
function validate_optional_param(array $input, $key, $default = null, $type = 'string', array $options = [])
{
    if (!isset($input[$key]) || $input[$key] === '') {
        return $default;
    }

    return validate_required_param($input, $key, $type, $options);
}

// 游戏与管理接口会改写玩家数据（购买、放生、改金钱等），必须来自本站页面（防 CSRF）。
// 前端页面的 fetch 包装会自动带 X-Pm-Formhash；图片类端点（badge/avatar/badges）不经过本文件。
require_once __DIR__ . '/../security.php';
if (!pm_formhash_ok()) {
    api_error('formhash 校验失败，请刷新页面后重试', 403);
}

// 如果是通过路由加载的API文件，不返回404
if (!defined('API_ROUTED')) {
    // 404响应（仅在直接访问 index.php 时）
    echo json_encode([
        'success' => false,
        'error' => 'API endpoint not found',
        'code' => 404,
        'timestamp' => time()
    ], JSON_UNESCAPED_UNICODE);
    exit;
}
