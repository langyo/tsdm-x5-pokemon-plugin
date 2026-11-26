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
            Err(_) => on_done(Err(NETWORK_ERROR.to_string())),
        }),
    );
}

/// 响应不是 JSON 信封时的提示。
///
/// tairitsu 的 fetch 只把响应正文交给 wasm，拿不到 HTTP 状态码与响应头
/// （上游 issue celestia-island/tairitsu#169），所以这里分不清是 500、403
/// 还是登录过期后被送回的登录页——只能把这几种都说出来，并给出下一步。
const NOT_ENVELOPE: &str =
    "服务器返回了无法解析的内容（登录过期、权限不足或服务器出错时会出现），请刷新页面重试";

/// 请求没发出去 / 没等到响应。宿主给的原因是 `TypeError: Failed to fetch`
/// 一类，对使用者没有信息量，所以只留结论与下一步。
const NETWORK_ERROR: &str = "无法连接服务器，请检查网络后重试";

thread_local! {
    /// 收到过 Discuz 的提示页（未登录 / 无权限）：外壳据此弹「重新登录」。
    static SESSION_LOST: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// 是否已判定登录失效（见 [`not_envelope`]）。
pub fn session_lost() -> bool {
    SESSION_LOST.with(|flag| flag.get())
}

/// 解析失败时的提示。
///
/// 正文以 `<` 开头就是一张 HTML 页：Discuz 在未登录 / 无权限时用
/// `showmessage` 回提示页，状态码仍是 200（tairitsu 的 fetch 也拿不到状态码，
/// 见 celestia-island/tairitsu#169），所以只能看正文。这时记下登录失效，
/// 外壳据此弹「重新登录」；其余坏响应给通用提示。
fn not_envelope(body: &str) -> String {
    if body.trim_start().starts_with('<') {
        SESSION_LOST.with(|flag| flag.set(true));
        crate::request_remount();
        "登录已过期或没有权限".to_string()
    } else {
        NOT_ENVELOPE.to_string()
    }
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
    let env: Envelope = serde_json::from_str(body).map_err(|_| not_envelope(body))?;
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
    let env: Envelope = serde_json::from_str(body).map_err(|_| not_envelope(body))?;
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

/// 地图野怪管理三连（dispatch 的 direct_actions 分支，动作名即原始字符串）。
/// 野怪关系存在 pm_data.mapid（CSV）里：add 追加并排序去重，remove 差集后重排。

pub fn wild_list(map_id: u64, on_done: Box<dyn FnOnce(Result<Vec<Value>, String>)>) {
    let body = serde_json::json!({
        "action": "get_wild_pokemons_for_map",
        "map_id": map_id,
    })
    .to_string();
    request(
        body,
        Box::new(|result: Result<Value, String>| {
            on_done(result.and_then(|data| extract_items(&data)))
        }),
    );
}

pub fn wild_add(
    map_id: u64,
    pokemon_type_id: u64,
    on_done: Box<dyn FnOnce(Result<(), String>)>,
) {
    let body = serde_json::json!({
        "action": "add_pokemon_to_map",
        "map_id": map_id,
        "pokemon_type_id": pokemon_type_id,
    })
    .to_string();
    tairitsu_web::fetch_text_with_options(
        "/plugin.php?id=pokemon:pokemon&index=admin",
        &serde_json::json!({ "method": "POST", "body": body }).to_string(),
        Box::new(move |resp| match resp {
            Ok(text) => on_done(parse_envelope_ok(&text)),
            Err(_) => on_done(Err(NETWORK_ERROR.to_string())),
        }),
    );
}

pub fn wild_remove(
    map_id: u64,
    pokemon_type_id: u64,
    on_done: Box<dyn FnOnce(Result<(), String>)>,
) {
    let body = serde_json::json!({
        "action": "remove_pokemon_from_map",
        "map_id": map_id,
        "pokemon_type_id": pokemon_type_id,
    })
    .to_string();
    tairitsu_web::fetch_text_with_options(
        "/plugin.php?id=pokemon:pokemon&index=admin",
        &serde_json::json!({ "method": "POST", "body": body }).to_string(),
        Box::new(move |resp| match resp {
            Ok(text) => on_done(parse_envelope_ok(&text)),
            Err(_) => on_done(Err(NETWORK_ERROR.to_string())),
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
            Err(_) => on_done(Err(NETWORK_ERROR.to_string())),
        }),
    );
}
