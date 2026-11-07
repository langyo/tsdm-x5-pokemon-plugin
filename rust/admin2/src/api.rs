//! 后台 API：与 Dioxus 版同协议——POST JSON `{action: "op::entity", ...}`
//! 到 `plugin.php?id=pokemon:pokemon&index=admin`，返回
//! `{"success":bool,"data":[...],"reason":...}`（data 项带 `_TYPE` 标记）。

use serde_json::Value;

fn request(body: String, on_done: Box<dyn FnOnce(Result<Value, String>)>) {
    let url = "/plugin.php?id=pokemon:pokemon&index=admin";
    let options = serde_json::json!({ "method": "POST", "body": body }).to_string();
    tairitsu_web::fetch_text_with_options(
        url,
        &options,
        Box::new(move |resp| match resp {
            Ok(text) => match parse_envelope(&text) {
                Ok(v) => on_done(Ok(v)),
                Err(e) => on_done(Err(e)),
            },
            Err(e) => on_done(Err(format!("网络错误: {e}"))),
        }),
    );
}

fn parse_envelope(body: &str) -> Result<Value, String> {
    #[derive(serde::Deserialize)]
    struct Envelope {
        #[serde(default)]
        success: bool,
        #[serde(default)]
        data: Option<Value>,
        #[serde(default)]
        reason: Option<String>,
    }
    let env: Envelope = serde_json::from_str(body).map_err(|e| format!("响应解析失败: {e}"))?;
    if !env.success {
        return Err(env.reason.unwrap_or_else(|| "未知错误".to_string()));
    }
    env.data.ok_or_else(|| "响应缺少 data".to_string())
}

/// 只看 `success` 的信封解析：delete 端点不返回 data（PHP 函数无返回值时
/// `data` 键缺失或为 null），按 data 解析会把它误判成失败。
fn parse_envelope_ok(body: &str) -> Result<(), String> {
    #[derive(serde::Deserialize)]
    struct Envelope {
        #[serde(default)]
        success: bool,
        #[serde(default)]
        reason: Option<String>,
    }
    let env: Envelope = serde_json::from_str(body).map_err(|e| format!("响应解析失败: {e}"))?;
    if !env.success {
        return Err(env.reason.unwrap_or_else(|| "未知错误".to_string()));
    }
    Ok(())
}

/// `list::global_config` → 配置对象。
///
/// 服务端的 data[0] 就是配置本身（扁平，带 `_TYPE` 标记），不是
/// `{config: {...}}` 包装——剥掉 `_TYPE` 后原样返回。
pub fn get_global_config(on_done: Box<dyn FnOnce(Result<Value, String>)>) {
    let body = serde_json::json!({ "action": "list::global_config" }).to_string();
    request(
        body,
        Box::new(|result: Result<Value, String>| {
            on_done(result.and_then(|data| extract_config(&data, "返回的数据包为空")))
        }),
    );
}

/// `set::global_config`（data 为配置 JSON 字符串）→ 保存后的配置对象。
pub fn set_global_config(config_json: String, on_done: Box<dyn FnOnce(Result<Value, String>)>) {
    let body =
        serde_json::json!({ "action": "set::global_config", "data": config_json }).to_string();
    request(
        body,
        Box::new(|result: Result<Value, String>| {
            on_done(result.and_then(|data| extract_config(&data, "保存响应为空")))
        }),
    );
}

/// data[0] → 配置对象（剥 `_TYPE` 标记）。
fn extract_config(data: &Value, empty_err: &str) -> Result<Value, String> {
    let mut item = data
        .as_array()
        .and_then(|items| items.first())
        .cloned()
        .ok_or_else(|| empty_err.to_string())?;
    if let Some(obj) = item.as_object_mut() {
        obj.remove("_TYPE");
    }
    Ok(item)
}

/// data 数组 → 逐项剥 `_TYPE` 后原样返回（列表端点通用）。
fn extract_items(data: &Value) -> Result<Vec<Value>, String> {
    let items = data
        .as_array()
        .cloned()
        .ok_or_else(|| "返回的数据包不是数组".to_string())?;
    Ok(items
        .into_iter()
        .map(|mut item| {
            if let Some(obj) = item.as_object_mut() {
                obj.remove("_TYPE");
            }
            item
        })
        .collect())
}

/// `count::<entity>` → 总数（data[0].count）。
pub fn count_entity(entity: &str, on_done: Box<dyn FnOnce(Result<u64, String>)>) {
    let body = serde_json::json!({ "action": format!("count::{entity}") }).to_string();
    request(
        body,
        Box::new(|result: Result<Value, String>| {
            on_done(result.map(|data| {
                data.as_array()
                    .and_then(|items| items.first())
                    .and_then(|item| item.get("count"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0)
            }))
        }),
    );
}

/// `list::<entity>`（from/count 分页，每页 ≤100）。
pub fn list_entity(
    entity: &str,
    from: u64,
    count: u64,
    on_done: Box<dyn FnOnce(Result<Vec<Value>, String>)>,
) {
    let body = serde_json::json!({
        "action": format!("list::{entity}"),
        "from": from.to_string(),
        "count": count.to_string(),
    })
    .to_string();
    request(
        body,
        Box::new(|result: Result<Value, String>| {
            on_done(result.and_then(|data| extract_items(&data)))
        }),
    );
}

/// 筛选包（与 Dioxus 版 FilterPackage 同形：tag/operator/value）。
pub fn filter_package(tag: &str, operator: &str, value: &str) -> Value {
    serde_json::json!({ "tag": tag, "operator": operator, "value": value })
}

/// `filter::<entity>`（filters 为包数组的 JSON 字符串）。
pub fn filter_entity(
    entity: &str,
    filters_json: String,
    on_done: Box<dyn FnOnce(Result<Vec<Value>, String>)>,
) {
    let body =
        serde_json::json!({ "action": format!("filter::{entity}"), "filters": filters_json })
            .to_string();
    request(
        body,
        Box::new(|result: Result<Value, String>| {
            on_done(result.and_then(|data| extract_items(&data)))
        }),
    );
}

/// `set::<entity>`（data 为完整记录 JSON 字符串）→ 保存后的记录。
pub fn set_entity(entity: &str, data_json: String, on_done: Box<dyn FnOnce(Result<Value, String>)>) {
    let body = serde_json::json!({
        "action": format!("set::{entity}"),
        "data": data_json,
    })
    .to_string();
    request(
        body,
        Box::new(|result: Result<Value, String>| {
            on_done(result.and_then(|data| extract_config(&data, "保存响应为空")))
        }),
    );
}

/// `insert::<entity>`（data 为完整记录 JSON 字符串）→ 新建后的记录。
pub fn insert_entity(
    entity: &str,
    data_json: String,
    on_done: Box<dyn FnOnce(Result<Value, String>)>,
) {
    let body = serde_json::json!({
        "action": format!("insert::{entity}"),
        "data": data_json,
    })
    .to_string();
    request(
        body,
        Box::new(|result: Result<Value, String>| {
            on_done(result.and_then(|data| extract_config(&data, "新建响应为空")))
        }),
    );
}

/// `list::pokemon_info` / `list::item_info`：按用户取实例列表。
///
/// dispatch 对这两个实体走特殊分支——`list_<entity>(uid, from, count)`，
/// `uid` 是必带参数（不同于普通实体的 from/count 分页）。
pub fn list_user_instances(
    entity: &str,
    uid: u64,
    on_done: Box<dyn FnOnce(Result<Vec<Value>, String>)>,
) {
    let body = serde_json::json!({
        "action": format!("list::{entity}"),
        "uid": uid.to_string(),
        "from": "0",
        "count": "100",
    })
    .to_string();
    request(
        body,
        Box::new(|result: Result<Value, String>| {
            on_done(result.and_then(|data| extract_items(&data)))
        }),
    );
}

/// `delete::<entity>`（id 为路径参数；dispatch 端按 `params["id"]` 取整数）。
pub fn delete_entity(entity: &str, id: u64, on_done: Box<dyn FnOnce(Result<(), String>)>) {
    let body = serde_json::json!({
        "action": format!("delete::{entity}"),
        "id": id,
    })
    .to_string();
    tairitsu_web::fetch_text_with_options(
        "/plugin.php?id=pokemon:pokemon&index=admin",
        &serde_json::json!({ "method": "POST", "body": body }).to_string(),
        Box::new(move |resp| match resp {
            Ok(text) => on_done(parse_envelope_ok(&text)),
            Err(e) => on_done(Err(format!("网络错误: {e}"))),
        }),
    );
}
