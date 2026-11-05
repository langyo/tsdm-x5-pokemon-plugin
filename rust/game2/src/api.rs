//! 商店 API 客户端（tairitsu 版）。
//!
//! 与 Dioxus 版 `NewApiClient` 同一套端点约定：
//! `plugin.php?id=pokemon:pokemon&endpoint=shop&action=<action>&...`
//! 响应是 `{success, data, error, code}` 信封，这里只解出 data 并映射为
//! state 里的展示类型（wire 类型带 serde derive，state 类型保持纯净）。
//!
//! WASI 侧没有 async/await（fetch 是回调桥），调用方在回调里 set 信号。

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::state::{ItemEffect, ShopItem, ShopPet};

// ---- wire 类型（与 rust/utils/src/types/api_shop.rs 字段一致） ----

#[derive(Deserialize)]
struct ShopItemDto {
    id: u64,
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    type_name: String,
    /// 物品图标文件名（tpname，不带扩展名）。
    #[serde(default)]
    image: String,
    price: i64,
    #[serde(default)]
    effect: ItemEffectDto,
    can_buy: bool,
}

#[derive(Default, Deserialize)]
struct ItemEffectDto {
    #[serde(default)]
    addhp: i64,
    #[serde(default)]
    addexp: i64,
    #[serde(default)]
    addlv: i64,
}

#[derive(Deserialize)]
struct ShopListDto {
    #[serde(default)]
    items: Vec<ShopItemDto>,
    #[serde(default)]
    total_pages: usize,
}

#[derive(Deserialize)]
struct ShopPetDto {
    id: u64,
    name: String,
    type_1: String,
    #[serde(default)]
    type_2: Option<String>,
    #[serde(default)]
    hp: u32,
    #[serde(default)]
    atk: u32,
    #[serde(default)]
    def: u32,
    #[serde(default)]
    spatk: u32,
    #[serde(default)]
    spdef: u32,
    #[serde(default)]
    speed: u32,
    price: i64,
    #[serde(default)]
    can_buy: bool,
}

#[derive(Deserialize)]
struct ShopPetListDto {
    #[serde(default)]
    pets: Vec<ShopPetDto>,
    #[serde(default)]
    total_pages: usize,
}

#[derive(Deserialize)]
struct BuyItemDto {
    #[serde(default)]
    remaining_money: i64,
}

#[derive(Deserialize)]
struct BuyPetDto {
    #[serde(default)]
    remaining_money: i64,
    #[serde(default)]
    site: u64,
}

#[derive(Serialize)]
struct BuyItemRequest {
    item_id: u64,
    quantity: u64,
}

#[derive(Serialize)]
struct BuyPetRequest {
    pokemon_type_id: u64,
}

/// 解析后的公开载荷。
pub struct ShopList {
    pub items: Vec<ShopItem>,
    pub total_pages: usize,
}

pub struct ShopPetList {
    pub pets: Vec<ShopPet>,
    pub total_pages: usize,
}

pub struct BuyOutcome {
    pub remaining_money: i64,
    pub site: u64,
}

// ---- 请求核心 ----

/// API 统一信封。`data` 用 `default = "none_option"`：泛型字段上的
/// `#[serde(default)]` 会生成 `Default::default()` 并要求 `T: Default`。
fn none_option<T>() -> Option<T> {
    None
}

#[derive(Deserialize)]
struct Envelope<T> {
    success: bool,
    #[serde(default = "none_option")]
    data: Option<T>,
    #[serde(default = "none_option")]
    error: Option<String>,
}

fn request<T: DeserializeOwned + 'static>(
    action_and_query: &str,
    body: Option<String>,
    on_done: Box<dyn FnOnce(Result<T, String>)>,
) {
    let url = format!(
        "/plugin.php?id=pokemon:pokemon&endpoint=shop&action={action_and_query}"
    );
    let options = match &body {
        Some(payload) => serde_json::json!({ "method": "POST", "body": payload }).to_string(),
        None => r#"{"method":"GET"}"#.to_string(),
    };
    tairitsu_web::fetch_text_with_options(
        &url,
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

fn parse_envelope<T: DeserializeOwned>(body: &str) -> Result<T, String> {
    let env: Envelope<T> =
        serde_json::from_str(body).map_err(|e| format!("响应解析失败: {e}"))?;
    if !env.success {
        return Err(env.error.unwrap_or_else(|| "未知错误".to_string()));
    }
    env.data.ok_or_else(|| "响应缺少 data".to_string())
}

/// RFC 3986 百分号编码（与 Dioxus 版一致）：中文与保留字符
/// 出现在查询串时原样送达，服务端按字面子串匹配。
pub fn encode_query_component(value: &str) -> String {
    value
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

// ---- 端点方法 ----

pub fn shop_items(
    item_type: Option<u32>,
    page: u32,
    keyword: &str,
    on_done: Box<dyn FnOnce(Result<ShopList, String>)>,
) {
    let type_param = item_type.map(|t| format!("&type={t}")).unwrap_or_default();
    let keyword_param = if keyword.is_empty() {
        String::new()
    } else {
        format!("&keyword={}", encode_query_component(keyword))
    };
    request(
        &format!("list&page={page}{type_param}{keyword_param}"),
        None,
        Box::new(move |result: Result<ShopListDto, String>| {
            on_done(result.map(|dto| ShopList {
                items: dto
                    .items
                    .into_iter()
                    .map(|i| ShopItem {
                        id: i.id,
                        name: i.name,
                        description: i.description,
                        type_name: i.type_name,
                        image: i.image,
                        price: i.price,
                        effect: ItemEffect {
                            addhp: i.effect.addhp,
                            addexp: i.effect.addexp,
                            addlv: i.effect.addlv,
                        },
                        can_buy: i.can_buy,
                    })
                    .collect(),
                total_pages: dto.total_pages,
            }))
        }),
    );
}

pub fn shop_pets(
    page: u32,
    keyword: &str,
    on_done: Box<dyn FnOnce(Result<ShopPetList, String>)>,
) {
    let keyword_param = if keyword.is_empty() {
        String::new()
    } else {
        format!("&keyword={}", encode_query_component(keyword))
    };
    request(
        &format!("pets&page={page}{keyword_param}"),
        None,
        Box::new(move |result: Result<ShopPetListDto, String>| {
            on_done(result.map(|dto| ShopPetList {
                pets: dto
                    .pets
                    .into_iter()
                    .map(|p| ShopPet {
                        id: p.id,
                        name: p.name,
                        type_1: p.type_1,
                        type_2: p.type_2,
                        hp: p.hp,
                        atk: p.atk,
                        def: p.def,
                        spatk: p.spatk,
                        spdef: p.spdef,
                        speed: p.speed,
                        price: p.price,
                        can_buy: p.can_buy,
                    })
                    .collect(),
                total_pages: dto.total_pages,
            }))
        }),
    );
}

pub fn buy_item(item_id: u64, on_done: Box<dyn FnOnce(Result<BuyOutcome, String>)>) {
    let body = serde_json::to_string(&BuyItemRequest { item_id, quantity: 1 })
        .unwrap_or_default();
    request(
        "buy",
        Some(body),
        Box::new(|result: Result<BuyItemDto, String>| {
            on_done(result.map(|dto| BuyOutcome {
                remaining_money: dto.remaining_money,
                site: 0,
            }))
        }),
    );
}

pub fn buy_pet(pokemon_type_id: u64, on_done: Box<dyn FnOnce(Result<BuyOutcome, String>)>) {
    let body =
        serde_json::to_string(&BuyPetRequest { pokemon_type_id }).unwrap_or_default();
    request(
        "buy_pet",
        Some(body),
        Box::new(|result: Result<BuyPetDto, String>| {
            on_done(result.map(|dto| BuyOutcome {
                remaining_money: dto.remaining_money,
                site: dto.site,
            }))
        }),
    );
}
