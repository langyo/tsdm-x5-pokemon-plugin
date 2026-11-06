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
    endpoint: &str,
    action_and_query: &str,
    body: Option<String>,
    on_done: Box<dyn FnOnce(Result<T, String>)>,
) {
    let url =
        format!("/plugin.php?id=pokemon:pokemon&endpoint={endpoint}&action={action_and_query}");
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
    let env: Envelope<T> = serde_json::from_str(body).map_err(|e| format!("响应解析失败: {e}"))?;
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
        "shop",
        &format!("list&page={page}{type_param}{keyword_param}"),
        None,
        Box::new(move |result: Result<ShopListDto, String>| {
            on_done(result.map(|dto| {
                ShopList {
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
                }
            }))
        }),
    );
}

pub fn shop_pets(page: u32, keyword: &str, on_done: Box<dyn FnOnce(Result<ShopPetList, String>)>) {
    let keyword_param = if keyword.is_empty() {
        String::new()
    } else {
        format!("&keyword={}", encode_query_component(keyword))
    };
    request(
        "shop",
        &format!("pets&page={page}{keyword_param}"),
        None,
        Box::new(move |result: Result<ShopPetListDto, String>| {
            on_done(result.map(|dto| {
                ShopPetList {
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
                }
            }))
        }),
    );
}

pub fn buy_item(item_id: u64, on_done: Box<dyn FnOnce(Result<BuyOutcome, String>)>) {
    let body = serde_json::to_string(&BuyItemRequest {
        item_id,
        quantity: 1,
    })
    .unwrap_or_default();
    request(
        "shop",
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
    let body = serde_json::to_string(&BuyPetRequest { pokemon_type_id }).unwrap_or_default();
    request(
        "shop",
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

// ===========================================================================
// 用户与话题端点（welcome/home 页）
// ===========================================================================

// ---- wire 类型 ----

#[derive(Deserialize)]
struct TopicDto {
    id: u64,
    title: String,
    #[serde(default)]
    is_pinned: bool,
}

#[derive(Deserialize)]
struct TopicsDto {
    #[serde(default)]
    news_announcements: Vec<NewsDto>,
    #[serde(default)]
    topics: Vec<TopicDto>,
}

#[derive(Deserialize)]
struct NewsDto {
    #[serde(default)]
    title: String,
    #[serde(default)]
    url: String,
}

#[derive(Deserialize)]
struct OnlinePlayerDto {
    uid: u64,
    username: String,
}

#[derive(Deserialize)]
struct OnlineDto {
    #[serde(default)]
    total: usize,
    #[serde(default)]
    players: Vec<OnlinePlayerDto>,
}

#[derive(Deserialize)]
struct InitializeDto {
    #[serde(default)]
    success: bool,
    #[serde(default)]
    message: String,
}

// ---- 公开载荷 ----

use crate::app_state::{OnlinePlayer, TopicItem};

pub struct TopicsList {
    pub items: Vec<TopicItem>,
}

pub struct OnlineList {
    pub total: usize,
    pub players: Vec<OnlinePlayer>,
}

pub struct InitializeOutcome {
    pub success: bool,
    pub message: String,
}

// ---- 端点方法 ----

/// 话题+新闻合并列表（对齐 Dioxus 版 Home 的 use_memo 合并规则：
/// 新闻在前，话题补足到 6 条）。
pub fn topics(on_done: Box<dyn FnOnce(Result<TopicsList, String>)>) {
    request(
        "topics",
        "list&limit=6",
        None,
        Box::new(|result: Result<TopicsDto, String>| {
            on_done(result.map(|dto| {
                let mut items = Vec::new();
                for news in &dto.news_announcements {
                    if !news.title.is_empty() {
                        items.push(TopicItem::News {
                            title: news.title.clone(),
                            url: news.url.clone(),
                        });
                    }
                }
                let remaining = 6usize.saturating_sub(items.len());
                for topic in dto.topics.iter().take(remaining) {
                    items.push(TopicItem::Topic {
                        id: topic.id,
                        title: topic.title.clone(),
                        is_pinned: topic.is_pinned,
                    });
                }
                TopicsList { items }
            }))
        }),
    );
}

pub fn online_players(on_done: Box<dyn FnOnce(Result<OnlineList, String>)>) {
    request(
        "user",
        "online_players",
        None,
        Box::new(|result: Result<OnlineDto, String>| {
            on_done(result.map(|dto| {
                OnlineList {
                    total: dto.total,
                    players: dto
                        .players
                        .into_iter()
                        .map(|p| OnlinePlayer {
                            uid: p.uid,
                            username: p.username,
                        })
                        .collect(),
                }
            }))
        }),
    );
}

pub fn initialize_player(on_done: Box<dyn FnOnce(Result<InitializeOutcome, String>)>) {
    request(
        "user",
        "initialize",
        Some(String::new()),
        Box::new(|result: Result<InitializeDto, String>| {
            on_done(result.map(|dto| InitializeOutcome {
                success: dto.success,
                message: dto.message,
            }))
        }),
    );
}

// ---- 启动路由判定（user/profile） ----

#[derive(Deserialize)]
struct ProfileDto {
    #[serde(default)]
    is_new_player: bool,
    #[serde(default)]
    total_pokemons: Option<u64>,
}

/// 拉取用户资料并判定欢迎页条件（对齐 Dioxus 版 Layout：
/// is_new_player 或 total_pokemons==0 → Welcome，否则 Home）。
pub fn profile(on_done: Box<dyn FnOnce(Result<bool, String>)>) {
    request(
        "user",
        "profile",
        None,
        Box::new(|result: Result<ProfileDto, String>| {
            on_done(result.map(|dto| dto.is_new_player || dto.total_pokemons.unwrap_or(0) == 0))
        }),
    );
}

// ===========================================================================
// 背包端点（inventory 页）
// ===========================================================================

#[derive(Deserialize)]
struct InventoryItemDto {
    id: u64,
    #[serde(default)]
    type_id: u64,
    #[serde(default)]
    item_type: u64,
    #[serde(default)]
    use_target: Option<String>,
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    image: String,
    #[serde(default)]
    quantity: i64,
    #[serde(default)]
    type_name: String,
}

#[derive(Deserialize)]
struct InventoryDto {
    #[serde(default)]
    items: Vec<InventoryItemDto>,
    #[serde(default)]
    total: usize,
    #[serde(default)]
    total_pages: usize,
}

#[derive(Deserialize)]
struct UsablePokemonDto {
    id: u64,
    #[serde(default)]
    nickname: String,
    #[serde(default)]
    level: u64,
    #[serde(default)]
    hp: i64,
    #[serde(default)]
    max_hp: i64,
}

#[derive(Deserialize)]
struct UsableListDto {
    #[serde(default)]
    usable_pokemon: Vec<UsablePokemonDto>,
}

#[derive(Deserialize)]
struct UseItemDto {
    #[serde(default)]
    message: String,
    #[serde(default)]
    item_remaining: Option<i64>,
}

/// use_item 的公开载荷。
pub struct UseItemOutcome {
    pub message: String,
    pub item_remaining: Option<i64>,
}

pub struct InventoryList {
    pub items: Vec<crate::state::InventoryItem>,
    pub total: usize,
    pub total_pages: usize,
}

pub fn inventory(
    item_type: Option<u32>,
    page: u32,
    search: &str,
    on_done: Box<dyn FnOnce(Result<InventoryList, String>)>,
) {
    let category = item_type.map(|t| format!("&type={t}")).unwrap_or_default();
    let search_param = if search.is_empty() {
        String::new()
    } else {
        format!("&search={}", encode_query_component(search))
    };
    request(
        "user",
        &format!("inventory&page={page}{category}{search_param}"),
        None,
        Box::new(|result: Result<InventoryDto, String>| {
            on_done(result.map(|dto| {
                InventoryList {
                    items: dto
                        .items
                        .into_iter()
                        .map(|i| crate::state::InventoryItem {
                            id: i.id,
                            type_id: i.type_id,
                            item_type: i.item_type,
                            use_target: i.use_target,
                            name: i.name,
                            description: i.description,
                            image: i.image,
                            quantity: i.quantity,
                            type_name: i.type_name,
                        })
                        .collect(),
                    total: dto.total,
                    total_pages: dto.total_pages,
                }
            }))
        }),
    );
}

pub fn usable_pokemon(
    item_id: u64,
    on_done: Box<dyn FnOnce(Result<Vec<crate::state::UsablePokemon>, String>)>,
) {
    request(
        "user",
        &format!("get_usable_pokemon&item_id={item_id}"),
        None,
        Box::new(|result: Result<UsableListDto, String>| {
            on_done(result.map(|dto| {
                dto.usable_pokemon
                    .into_iter()
                    .map(|p| crate::state::UsablePokemon {
                        id: p.id,
                        nickname: p.nickname,
                        level: p.level,
                        hp: p.hp,
                        max_hp: p.max_hp,
                    })
                    .collect()
            }))
        }),
    );
}

pub fn use_item(
    item_id: u64,
    pokemon_id: Option<u64>,
    on_done: Box<dyn FnOnce(Result<UseItemOutcome, String>)>,
) {
    let body = serde_json::json!({ "item_id": item_id, "pokemon_id": pokemon_id }).to_string();
    request(
        "user",
        "use_item",
        Some(body),
        Box::new(|result: Result<UseItemDto, String>| {
            on_done(result.map(|dto| UseItemOutcome {
                message: dto.message,
                item_remaining: dto.item_remaining,
            }))
        }),
    );
}
