<?php
if (!defined("IN_DISCUZ")) exit;

include_once __DIR__ . "/routes.php";

$entity_map = [
    "global_config" => "global_config",
    "pokemon_type" => "pokemon_data",
    "pokemon_info" => "pokemon_info",
    "item_type" => "item_data",
    "item_info" => "item_info",
    "map_info" => "map_data",
    "user_info" => "user_data",
    "evolution_info" => "evolution_data",
    "skill_type" => "skill_type",
    "effect_data" => "effect_data",
];

function admin_dispatch($action, $params) {
    global $entity_map;

    // 无 op::entity 前缀的直连动作（地图野生宠物管理、按 ID 批量取宠物种族，前端按动作名直接调用）
    $direct_actions = [
        "get_wild_pokemons_for_map",
        "add_pokemon_to_map",
        "remove_pokemon_from_map",
        "get_pokemon_types_by_ids",
    ];
    if (in_array($action, $direct_actions, true)) {
        // 路由文件已由顶部 routes.php 统一加载，这里直接分发
        switch ($action) {
            case "get_wild_pokemons_for_map":
                $result = get_wild_pokemons_for_map(intval($params["map_id"] ?? 0));
                break;
            case "add_pokemon_to_map":
                add_pokemon_to_map(intval($params["map_id"] ?? 0), intval($params["pokemon_type_id"] ?? 0));
                $result = [];
                break;
            case "remove_pokemon_from_map":
                remove_pokemon_from_map(intval($params["map_id"] ?? 0), intval($params["pokemon_type_id"] ?? 0));
                $result = [];
                break;
            case "get_pokemon_types_by_ids":
                // 前端 fetch() 把所有参数序列化成字符串，ids 是 JSON 数组文本
                $ids = $params["ids"] ?? "[]";
                if (is_string($ids)) {
                    $ids = json_decode($ids, true);
                }
                $result = get_pokemon_types_by_ids(is_array($ids) ? $ids : []);
                break;
        }
        echo json_encode(["success"=>true,"data"=>$result], JSON_UNESCAPED_UNICODE);
        exit;
    }

    $parts = explode("::", $action);
    $op = $parts[0] ?? "";
    $entity = $parts[1] ?? "";
    
    $fileEntity = $entity_map[$entity] ?? "";
    if (!$fileEntity) {
        echo json_encode(["success"=>false,"reason"=>"Unknown entity: $entity"]);
        exit;
    }
    
    $routeFile = __DIR__ . "/routes/" . $fileEntity . ".php";
    if (!file_exists($routeFile)) {
        echo json_encode(["success"=>false,"reason"=>"File not found: $fileEntity"]);
        exit;
    }
    require_once $routeFile;
    
    $result = [];
    
    switch ($op) {
        case "count":
            $fn = "count_" . $entity;
            if (!function_exists($fn)) { echo json_encode(["success"=>true,"data"=>["count"=>0]]); exit; }
            $result = call_user_func($fn);
            break;
        case "list":
            $fn = "list_" . $entity;
            if (!function_exists($fn)) $fn = "get_" . $entity;
            if (!function_exists($fn)) { echo json_encode(["success"=>true,"data"=>[]]); exit; }
            $from = intval($params["from"] ?? 0);
            $count = intval($params["count"] ?? 100);
            if (in_array($entity, ["pokemon_info", "item_info"])) {
                $uid = intval($params["uid"] ?? 0);
                $result = function_exists("list_" . $entity) ? call_user_func($fn, $uid, $from, $count) : call_user_func($fn, $uid);
            } else {
                $result = function_exists("list_" . $entity) ? call_user_func($fn, $from, $count) : call_user_func($fn);
            }
            break;
        case "get":
            $fn = "get_" . $entity;
            if (!function_exists($fn)) break;
            $id = intval($params["id"] ?? 0);
            $result = call_user_func($fn, $id);
            break;
        case "set":
            $fn = "set_" . $entity;
            if (!function_exists($fn)) break;
            $data = isset($params["data"]) ? json_decode($params["data"], true) : $params;
            $result = call_user_func($fn, $data) ?? [];
            $id = intval($data["id"] ?? 0);
            $getFn = "get_" . $entity;
            if ($id > 0 && function_exists($getFn)) {
                $result = call_user_func($getFn, $id);
            }
            break;
        case "insert":
            $fn = "insert_" . $entity;
            if (!function_exists($fn)) break;
            $data = isset($params["data"]) ? json_decode($params["data"], true) : $params;
            $id = call_user_func($fn, $data);
            $getFn = "get_" . $entity;
            if ($id > 0 && function_exists($getFn)) {
                $result = call_user_func($getFn, $id);
            }
            break;
        case "delete":
            $fn = "delete_" . $entity;
            if (!function_exists($fn)) break;
            $id = intval($params["id"] ?? 0);
            $result = call_user_func($fn, $id);
            break;
        case "filter":
            $fn = "filter_" . $entity;
            if (!function_exists($fn)) break;
            $filters = isset($params["filters"]) ? json_decode($params["filters"], true) : [];
            $result = call_user_func($fn, $filters);
            break;
        case "run":
            $fn = "run_" . $entity;
            if (!function_exists($fn)) break;
            $result = call_user_func($fn, strval($params["sql"] ?? $params["data"] ?? ""));
            break;
        default:
            echo json_encode(["success"=>false,"reason"=>"Unknown operation: $op"]);
            exit;
    }
    
    echo json_encode(["success"=>true,"data"=>$result], JSON_UNESCAPED_UNICODE);
    exit;
}

// Merge $_POST with JSON body for JSON POST support
$params = $_POST;
if (empty($params) || (count($params) === 1 && isset($params['action']))) {
    $rawJson = json_decode(file_get_contents('php://input'), true);
    if ($rawJson) {
        $params = $rawJson;
    }
}
admin_dispatch($params["action"] ?? $_POST["action"] ?? "", $params);
