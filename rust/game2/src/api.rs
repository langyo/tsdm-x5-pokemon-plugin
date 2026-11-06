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

// ===========================================================================
// 宠物列表 / 治疗（宠物中心页）
// ===========================================================================

#[derive(Deserialize)]
struct PokemonSkillDto {
    #[serde(default)]
    type_id: u64,
    #[serde(default)]
    pp: i64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    skill_type: String,
    #[serde(default)]
    category: String,
    #[serde(default)]
    level: u32,
    #[serde(default)]
    power: u32,
    #[serde(default)]
    max_pp: i64,
    #[serde(default)]
    can_forget: bool,
}

#[derive(Deserialize)]
struct PokemonBaseInfoDto {
    #[serde(default)]
    is_legendary: bool,
}

#[derive(Deserialize)]
struct PokemonBasicDto {
    id: u64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    nickname: Option<String>,
    #[serde(default)]
    type_id: u64,
    #[serde(default)]
    level: u64,
    #[serde(default)]
    hp: i64,
    #[serde(default)]
    max_hp: i64,
    #[serde(default)]
    is_shiny: bool,
    #[serde(default)]
    site: u8,
    #[serde(default)]
    state: u8,
    #[serde(default)]
    state_text: String,
    #[serde(default)]
    state_class: String,
    #[serde(default)]
    skills: Vec<PokemonSkillDto>,
    #[serde(default)]
    base_info: Option<PokemonBaseInfoDto>,
}

#[derive(Deserialize)]
struct PokemonListDto {
    #[serde(default)]
    pokemons: Vec<PokemonBasicDto>,
    #[serde(default)]
    #[allow(dead_code)]
    total: usize,
}

#[derive(Deserialize)]
struct HealDto {
    #[serde(default)]
    cost: i64,
    #[serde(default)]
    message: String,
}

pub struct HealOutcome {
    pub cost: i64,
    pub message: String,
}

pub fn pokemon_list(on_done: Box<dyn FnOnce(Result<Vec<crate::state::PokemonBasic>, String>)>) {
    request(
        "pokemon",
        "list",
        None,
        Box::new(|result: Result<PokemonListDto, String>| {
            on_done(result.map(|dto| {
                dto.pokemons
                    .into_iter()
                    .map(|p| {
                        let skills = p
                            .skills
                            .into_iter()
                            .map(|s| crate::state::PokemonSkill {
                                type_id: s.type_id,
                                pp: s.pp.max(0) as u64,
                                max_pp: s.max_pp.max(0) as u64,
                                name: s.name,
                                skill_type: s.skill_type,
                                category: s.category,
                                level: s.level,
                                power: s.power,
                                can_forget: s.can_forget,
                            })
                            .collect();
                        crate::state::PokemonBasic {
                            id: p.id,
                            name: p.name,
                            nickname: p.nickname,
                            type_id: p.type_id,
                            level: p.level,
                            hp: p.hp,
                            max_hp: p.max_hp,
                            is_shiny: p.is_shiny,
                            site: p.site,
                            state: p.state,
                            state_text: p.state_text,
                            state_class: p.state_class,
                            skills,
                            is_legendary: p.base_info.map(|b| b.is_legendary).unwrap_or(false),
                        }
                    })
                    .collect()
            }))
        }),
    );
}

/// 治疗一只宠物（服务端当前免费，cost 仍透传）。`flee=true` 走脱战治疗。
pub fn heal_pokemon(
    pokemon_id: u64,
    flee: bool,
    on_done: Box<dyn FnOnce(Result<HealOutcome, String>)>,
) {
    let action = if flee { "heal_and_flee" } else { "heal" };
    request(
        "user",
        &format!("{action}&pokemon_id={pokemon_id}"),
        None,
        Box::new(|result: Result<HealDto, String>| {
            on_done(result.map(|dto| HealOutcome {
                cost: dto.cost,
                message: dto.message,
            }))
        }),
    );
}

// ===========================================================================
// 宠物详情 / 改名（我的宝可梦页）
// ===========================================================================

#[derive(Deserialize)]
struct PokemonIvsDto {
    #[serde(default, alias = "hit_points")]
    hp: u64,
    #[serde(default)]
    attack: u64,
    #[serde(default)]
    defense: u64,
    #[serde(default, rename = "sp_attack", alias = "special_attack")]
    sp_attack: u64,
    #[serde(default, rename = "sp_defense", alias = "special_defense")]
    sp_defense: u64,
    #[serde(default)]
    speed: u64,
}

#[derive(Deserialize)]
struct PokemonStatsDto {
    #[serde(default)]
    attack: i64,
    #[serde(default)]
    defense: i64,
    #[serde(default, rename = "sp_attack", alias = "special_attack")]
    sp_attack: i64,
    #[serde(default, rename = "sp_defense", alias = "special_defense")]
    sp_defense: i64,
    #[serde(default)]
    speed: i64,
}

#[derive(Deserialize)]
struct PokemonDetailBaseDto {
    #[serde(default)]
    type_1: String,
    #[serde(default)]
    type_2: Option<String>,
    #[serde(default)]
    is_legendary: bool,
}

#[derive(Deserialize)]
struct PokemonDetailDto {
    id: u64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    nickname: Option<String>,
    #[serde(default)]
    type_id: u64,
    #[serde(default)]
    level: u64,
    #[serde(default)]
    exp: u64,
    #[serde(default)]
    exp_for_current_level: u64,
    #[serde(default)]
    exp_for_next_level: u64,
    #[serde(default)]
    hp: i64,
    #[serde(default)]
    max_hp: i64,
    #[serde(default)]
    gender: u8,
    #[serde(default)]
    is_shiny: bool,
    #[serde(default)]
    state: u8,
    #[serde(default)]
    state_text: String,
    #[serde(default)]
    state_class: String,
    #[serde(default)]
    affection: u32,
    #[serde(default)]
    ivs: Option<PokemonIvsDto>,
    #[serde(default)]
    quality_rank: Option<u8>,
    #[serde(default)]
    quality_grade: Option<String>,
    #[serde(default)]
    stats: Option<PokemonStatsDto>,
    #[serde(default)]
    base_info: Option<PokemonDetailBaseDto>,
}

pub fn pokemon_detail(
    pokemon_id: u64,
    on_done: Box<dyn FnOnce(Result<crate::state::PokemonDetail, String>)>,
) {
    request(
        "pokemon",
        &format!("detail&pokemon_id={pokemon_id}"),
        None,
        Box::new(|result: Result<PokemonDetailDto, String>| {
            on_done(result.map(|d| {
                crate::state::PokemonDetail {
                    id: d.id,
                    name: d.name,
                    nickname: d.nickname,
                    type_id: d.type_id,
                    level: d.level,
                    exp: d.exp,
                    exp_for_current_level: d.exp_for_current_level,
                    exp_for_next_level: d.exp_for_next_level,
                    hp: d.hp,
                    max_hp: d.max_hp,
                    gender: d.gender,
                    is_shiny: d.is_shiny,
                    state: d.state,
                    state_text: d.state_text,
                    state_class: d.state_class,
                    affection: d.affection,
                    ivs: d.ivs.map(|i| {
                        [
                            i.hp,
                            i.attack,
                            i.defense,
                            i.sp_attack,
                            i.sp_defense,
                            i.speed,
                        ]
                    }),
                    quality_rank: d.quality_rank,
                    quality_grade: d.quality_grade,
                    stats: d
                        .stats
                        .map(|s| [s.attack, s.defense, s.sp_attack, s.sp_defense, s.speed]),
                    type_1: d
                        .base_info
                        .as_ref()
                        .map(|b| b.type_1.clone())
                        .unwrap_or_default(),
                    type_2: d.base_info.as_ref().and_then(|b| b.type_2.clone()),
                    is_legendary: d.base_info.map(|b| b.is_legendary).unwrap_or(false),
                }
            }))
        }),
    );
}

/// 改名：POST `{id, name}`（服务端限 20 字节，前端限 14 字）。
pub fn rename_pokemon(pokemon_id: u64, name: &str, on_done: Box<dyn FnOnce(Result<(), String>)>) {
    let body = serde_json::json!({ "id": pokemon_id, "name": name }).to_string();
    request(
        "pokemon",
        "rename",
        Some(body),
        Box::new(move |result: Result<serde_json::Value, String>| on_done(result.map(|_| ()))),
    );
}

// ===========================================================================
// 装备（我的宝可梦「装备」标签页）
// ===========================================================================

#[derive(Deserialize)]
struct EquipmentItemDto {
    #[serde(default)]
    myitem_id: u64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    image: String,
    #[serde(default)]
    equipment_hp: i32,
    #[serde(default)]
    equipment_atk: i32,
    #[serde(default)]
    equipment_def: i32,
    #[serde(default)]
    equipment_spatk: i32,
    #[serde(default)]
    equipment_spdef: i32,
    #[serde(default)]
    equipment_sd: i32,
    #[serde(default)]
    is_equipped: bool,
    #[serde(default)]
    equipped_count: u32,
    #[serde(default)]
    available_count: u32,
}

#[derive(Deserialize)]
struct EquipmentSlotDto {
    #[serde(default)]
    slot_index: u32,
    #[serde(default)]
    item: Option<EquipmentItemDto>,
}

#[derive(Deserialize)]
struct EquipmentResponseDto {
    #[serde(default)]
    equipment_slots: Vec<EquipmentSlotDto>,
    #[serde(default)]
    owned_items: Vec<EquipmentItemDto>,
}

fn map_equipment_item(d: EquipmentItemDto) -> crate::state::EquipmentItem {
    crate::state::EquipmentItem {
        myitem_id: d.myitem_id,
        name: d.name,
        description: d.description,
        image: d.image,
        equipment_hp: d.equipment_hp,
        equipment_atk: d.equipment_atk,
        equipment_def: d.equipment_def,
        equipment_spatk: d.equipment_spatk,
        equipment_spdef: d.equipment_spdef,
        equipment_sd: d.equipment_sd,
        is_equipped: d.is_equipped,
        equipped_count: d.equipped_count,
        available_count: d.available_count,
    }
}

pub fn equipment(
    pokemon_id: u64,
    on_done: Box<dyn FnOnce(Result<crate::state::EquipmentData, String>)>,
) {
    request(
        "pokemon",
        &format!("equipment&pokemon_id={pokemon_id}"),
        None,
        Box::new(|result: Result<EquipmentResponseDto, String>| {
            on_done(result.map(|dto| {
                let mut slots: Vec<crate::state::EquipmentSlotData> = dto
                    .equipment_slots
                    .into_iter()
                    .map(|s| crate::state::EquipmentSlotData {
                        slot_index: s.slot_index,
                        item: s.item.map(map_equipment_item),
                    })
                    .collect();
                // 恒定补齐到 4 槽（对齐 Dioxus 版）。
                for i in slots.len()..4 {
                    slots.push(crate::state::EquipmentSlotData {
                        slot_index: i as u32,
                        item: None,
                    });
                }
                crate::state::EquipmentData {
                    slots,
                    owned: dto
                        .owned_items
                        .into_iter()
                        .map(map_equipment_item)
                        .collect(),
                }
            }))
        }),
    );
}

pub fn equip_item(
    pokemon_id: u64,
    myitem_id: u64,
    slot_index: u32,
    on_done: Box<dyn FnOnce(Result<(), String>)>,
) {
    let body = serde_json::json!({
        "pokemon_id": pokemon_id,
        "myitem_id": myitem_id,
        "slot_index": slot_index,
    })
    .to_string();
    request(
        "pokemon",
        "equip_item",
        Some(body),
        Box::new(move |result: Result<serde_json::Value, String>| on_done(result.map(|_| ()))),
    );
}

pub fn unequip_item(
    pokemon_id: u64,
    slot_index: u32,
    on_done: Box<dyn FnOnce(Result<(), String>)>,
) {
    let body =
        serde_json::json!({ "pokemon_id": pokemon_id, "slot_index": slot_index }).to_string();
    request(
        "pokemon",
        "unequip_item",
        Some(body),
        Box::new(move |result: Result<serde_json::Value, String>| on_done(result.map(|_| ()))),
    );
}

// ===========================================================================
// 技能（我的宝可梦「技能」标签页）
// ===========================================================================

#[derive(Deserialize)]
struct LearnableSkillDto {
    #[serde(default)]
    id: u64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(rename = "type", default)]
    skill_type: String,
    #[serde(default)]
    category: String,
    #[serde(default)]
    power: u32,
    #[serde(default)]
    max_pp: u32,
    #[serde(default)]
    required_level: u32,
    #[serde(default)]
    is_available: bool,
}

#[derive(Deserialize)]
struct LearnableResponseDto {
    #[serde(default)]
    available_skills: Vec<LearnableSkillDto>,
    #[serde(default)]
    unlocked_skills: Vec<LearnableSkillDto>,
}

fn map_learnable(d: LearnableSkillDto) -> crate::state::LearnableSkill {
    crate::state::LearnableSkill {
        id: d.id,
        name: d.name,
        description: d.description,
        skill_type: d.skill_type,
        category: d.category,
        power: d.power,
        max_pp: d.max_pp,
        required_level: d.required_level,
        is_available: d.is_available,
    }
}

/// 可学习技能列表：返回 (可学习, 等级不足)。
pub fn learnable_skills(
    pokemon_id: u64,
    on_done: Box<
        dyn FnOnce(
            Result<
                (
                    Vec<crate::state::LearnableSkill>,
                    Vec<crate::state::LearnableSkill>,
                ),
                String,
            >,
        ),
    >,
) {
    request(
        "pokemon",
        &format!("learnable_skills&pokemon_id={pokemon_id}"),
        None,
        Box::new(|result: Result<LearnableResponseDto, String>| {
            on_done(result.map(|dto| {
                (
                    dto.available_skills
                        .into_iter()
                        .map(map_learnable)
                        .collect(),
                    dto.unlocked_skills.into_iter().map(map_learnable).collect(),
                )
            }))
        }),
    );
}

pub fn learn_skill(pokemon_id: u64, skill_id: u64, on_done: Box<dyn FnOnce(Result<(), String>)>) {
    let body = serde_json::json!({ "pokemon_id": pokemon_id, "skill_id": skill_id }).to_string();
    request(
        "pokemon",
        "learn_skill",
        Some(body),
        Box::new(move |result: Result<serde_json::Value, String>| on_done(result.map(|_| ()))),
    );
}

pub fn forget_skill(pokemon_id: u64, skill_id: u64, on_done: Box<dyn FnOnce(Result<(), String>)>) {
    let body = serde_json::json!({ "pokemon_id": pokemon_id, "skill_id": skill_id }).to_string();
    request(
        "pokemon",
        "forget_skill",
        Some(body),
        Box::new(move |result: Result<serde_json::Value, String>| on_done(result.map(|_| ()))),
    );
}

// ===========================================================================
// 进化（我的宝可梦「属性」页弹窗）
// ===========================================================================

#[derive(Deserialize, Default)]
struct EvoConditionDetailsDto {
    #[serde(default, rename = "level_requirement")]
    level_requirement: Option<u64>,
    #[serde(default, rename = "intimacy_requirement")]
    intimacy_requirement: Option<u64>,
}

#[derive(Deserialize)]
struct EvoConditionsDto {
    #[serde(default)]
    reason: String,
    #[serde(default)]
    details: EvoConditionDetailsDto,
}

#[derive(Deserialize)]
struct AvailableEvolutionDto {
    #[serde(default)]
    to_id: u64,
    #[serde(default)]
    to_name: String,
    #[serde(default)]
    evolution_type: String,
    #[serde(default)]
    can_evolve: bool,
    #[serde(default)]
    conditions: Option<EvoConditionsDto>,
    #[serde(default)]
    condition_display: Option<String>,
}

#[derive(Deserialize)]
struct EvolutionCheckDto {
    #[serde(default)]
    available_evolutions: Vec<AvailableEvolutionDto>,
}

/// 进化条件检查（可进化路线列表，条件文案服务端拼好或按类型回退）。
pub fn evolution_check(
    pet_id: u64,
    on_done: Box<dyn FnOnce(Result<Vec<crate::state::AvailableEvolution>, String>)>,
) {
    request(
        "evolution",
        &format!("check&pet_id={pet_id}"),
        None,
        Box::new(|result: Result<EvolutionCheckDto, String>| {
            on_done(result.map(|dto| {
                dto.available_evolutions
                    .into_iter()
                    .map(|e| {
                        let (reason, condition_display) = match e.conditions {
                            Some(c) => {
                                let fallback =
                                    e.condition_display.clone().unwrap_or_else(|| {
                                        match e.evolution_type.as_str() {
                                            "level" => format!(
                                                "等级达到 Lv.{}",
                                                c.details.level_requirement.unwrap_or(0)
                                            ),
                                            "item" => "需要使用进化道具".to_string(),
                                            "good" => format!(
                                                "亲密度达到 {}",
                                                c.details.intimacy_requirement.unwrap_or(0)
                                            ),
                                            _ => c.reason.clone(),
                                        }
                                    });
                                (c.reason, Some(fallback))
                            }
                            None => (String::new(), e.condition_display.clone()),
                        };
                        crate::state::AvailableEvolution {
                            to_id: e.to_id,
                            to_name: e.to_name,
                            can_evolve: e.can_evolve,
                            reason,
                            condition_display,
                        }
                    })
                    .collect()
            }))
        }),
    );
}

#[derive(Deserialize)]
struct EvoPathNodeDto {
    #[serde(default)]
    id: u64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    type_1: String,
    #[serde(default)]
    type_2: Option<String>,
    #[serde(default)]
    condition_display: String,
}

#[derive(Deserialize)]
struct EvoPathResponseDto {
    #[serde(default)]
    pokemon_name: String,
    #[serde(default)]
    forward: Vec<EvoPathNodeDto>,
    #[serde(default)]
    backward: Vec<EvoPathNodeDto>,
}

/// 进化路径（pmid = 物种 ID，非宠物实例 ID）。
pub fn evolution_path(
    pmid: u64,
    on_done: Box<dyn FnOnce(Result<crate::state::EvolutionPathData, String>)>,
) {
    request(
        "evolution",
        &format!("evolution_path&pmid={pmid}"),
        None,
        Box::new(|result: Result<EvoPathResponseDto, String>| {
            on_done(result.map(|dto| {
                crate::state::EvolutionPathData {
                    pokemon_name: dto.pokemon_name,
                    forward: dto
                        .forward
                        .into_iter()
                        .map(|n| crate::state::EvolutionPathNode {
                            id: n.id,
                            name: n.name,
                            type_1: n.type_1,
                            type_2: n.type_2,
                            condition_display: n.condition_display,
                        })
                        .collect(),
                    backward: dto
                        .backward
                        .into_iter()
                        .map(|n| crate::state::EvolutionPathNode {
                            id: n.id,
                            name: n.name,
                            type_1: n.type_1,
                            type_2: n.type_2,
                            condition_display: n.condition_display,
                        })
                        .collect(),
                }
            }))
        }),
    );
}

/// 执行进化（to_id 指定目标形态；返回新形态名）。
pub fn evolve(pet_id: u64, to_id: u64, on_done: Box<dyn FnOnce(Result<String, String>)>) {
    let body = serde_json::json!({ "pet_id": pet_id, "to_id": to_id }).to_string();
    #[derive(Deserialize)]
    struct EvolveDto {
        #[serde(default)]
        new_name: String,
    }
    request(
        "evolution",
        "evolve",
        Some(body),
        Box::new(|result: Result<EvolveDto, String>| on_done(result.map(|d| d.new_name))),
    );
}
