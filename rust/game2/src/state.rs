//! 页面共享状态（tairitsu 版）。
//!
//! tairitsu 的状态模式：`use_signal` 每次渲染创建新信号（受控组件设计），
//! 持久状态必须挂在渲染闭包外——由 `run_app` 在 `with_component` 上下文里
//! 构造一次，经 props 传入组件；事件回调捕获克隆共享同一组信号。

use tairitsu_hooks::{use_signal, ReactiveSignal};

/// 远程 CDN 的精灵图库——X3 时代的动画 GIF 素材（正面 pm / 异色 pmsg，背面
/// pmb / 异色 pmsgb），覆盖含自定宠在内的全部物种（旧版 utils/sprite.rs 的
/// `IMG_PATH_REMOTE`）。
pub const IMG_PATH_REMOTE: &str = "https://img.tsdm39.com/Pokemon";

/// 正面精灵图地址（旧版 `front_sprite_url`）。CDN 缺图或不可达时，由页面模板
/// （game_v2.inc.php）的 onerror 回退到本地静态 PNG——个人中心的展示图、仓库格子、
/// 选宠列表都用它；迷你格、进化路径这些小图标位仍用本地 `spm/` 小图。
pub fn front_sprite_url(species_id: u64, is_shiny: bool) -> String {
    let dir = if is_shiny { "pmsg" } else { "pm" };
    format!("{IMG_PATH_REMOTE}/{dir}/{species_id}.gif")
}

/// 背面精灵图地址（旧版 `back_sprite_url`，普通 `pmb` / 异色 `pmsgb`）：
/// 战斗里我方出场位用的是背面图，目录名与正面图不同。
pub fn back_sprite_url(species_id: u64, is_shiny: bool) -> String {
    let dir = if is_shiny { "pmsgb" } else { "pmb" };
    format!("{IMG_PATH_REMOTE}/{dir}/{species_id}.gif")
}

/// 商店商品（与 rust/utils/src/types/api_shop.rs 字段一致）。
#[derive(Debug, Clone, Default)]
pub struct ShopItem {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub type_name: String,
    /// 物品图标文件名（tpname，不带扩展名）。
    pub image: String,
    pub price: i64,
    pub effect: ItemEffect,
    pub can_buy: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ItemEffect {
    pub addhp: i64,
    pub addexp: i64,
    pub addlv: i64,
}

/// 商店宠物（六维种族值）。
#[derive(Debug, Clone, Default)]
pub struct ShopPet {
    pub id: u64,
    pub name: String,
    pub type_1: String,
    pub type_2: Option<String>,
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub spatk: u32,
    pub spdef: u32,
    pub speed: u32,
    pub price: i64,
    pub can_buy: bool,
}

/// 反馈提示的类别（展示见 `toast.rs`；时长与配色沿用 Dioxus 版）。
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ToastKind {
    Success,
    Error,
    Info,
    Warning,
}

/// 当前用户资料（侧栏 trainer 卡 + 启动时的欢迎页判定）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UserProfile {
    pub uid: u64,
    pub username: String,
    pub is_admin: bool,
    /// 新玩家（is_new_player 或 total_pokemons==0）：先进欢迎页。
    pub needs_welcome: bool,
    pub money: i64,
    pub strength_level: u32,
    pub wins: u32,
    pub losses: u32,
}

/// 背包物品（与 rust/utils/src/types/api_user.rs InventoryItem 字段一致）。
#[derive(Debug, Clone, Default)]
pub struct InventoryItem {
    /// 背包行 ID（pm_myitem.id，仅标识用）。
    pub id: u64,
    /// 物品类型 ID（pm_itemdata.id）——使用/查询可用目标都传它，
    /// Dioxus 版同样以 type_id 作为使用键。
    pub type_id: u64,
    /// 物品大类（1=回复药 … 5=装备道具）。
    pub item_type: u64,
    /// 服务端给的使用目标：pokemon / battle / global。
    pub use_target: Option<String>,
    pub name: String,
    pub description: String,
    pub image: String,
    pub quantity: i64,
    pub type_name: String,
}

/// 弹窗里可选的目标宠物（get_usable_pokemon）。品质重洗药/怠惰药片的
/// 选择器需要精灵图与品质/努力值摘要，字段与 Dioxus 版 UsablePokemon 对齐。
#[derive(Debug, Clone, Default)]
pub struct UsablePokemon {
    pub id: u64,
    pub type_id: u64,
    pub nickname: String,
    pub level: u64,
    pub hp: i64,
    pub max_hp: i64,
    pub is_shiny: bool,
    /// 总个体值（旧载荷无此字段时为 None）。
    pub iv_total: Option<u64>,
    /// 品质档位（1=E .. 8=SS+，9=UnKnow）。
    pub quality_rank: Option<u8>,
    /// 品质名（E/D/C/B/A/S/SS/SS+）。
    pub quality_grade: Option<String>,
    /// 努力值总和（怠惰药片的目标选择器展示用）。
    pub ev_total: Option<u64>,
}

/// get_usable_pokemon 的完整载荷：物品元信息 + 可选列表 + 不可用计数。
#[derive(Debug, Clone, Default)]
pub struct UsablePokemonList {
    pub item_name: String,
    pub item_module: String,
    pub usable_pokemon: Vec<UsablePokemon>,
    pub unusable_count: usize,
}

/// 背包页持久状态。
#[derive(Clone)]
pub struct InventoryState {
    pub component_id: std::cell::Cell<u32>,
    /// 侧栏背包小格留下的分类，下次进入背包页时取用并清空（旧版的
    /// `INITIAL_INVENTORY_CATEGORY`）。None = 没有待取的。
    pub entry_category: std::cell::Cell<Option<u32>>,
    pub category: ReactiveSignal<Option<u32>>,
    pub page: ReactiveSignal<u32>,
    pub total_pages: ReactiveSignal<usize>,
    pub total: ReactiveSignal<usize>,
    pub search_input: ReactiveSignal<String>,
    pub search_query: ReactiveSignal<String>,

    pub items: ReactiveSignal<Option<Result<Vec<InventoryItem>, String>>>,
    /// 正在使用中的物品类型 ID（互斥守卫）。
    pub using: ReactiveSignal<Option<u64>>,
    /// 选中的物品（弹窗打开时的使用目标判定）。
    pub selected_item: ReactiveSignal<Option<u64>>,
    pub show_target_modal: ReactiveSignal<bool>,
    pub usable_pokemon: ReactiveSignal<Option<Result<UsablePokemonList, String>>>,
    /// 品质重洗药使用成功后的前后对比（旧版 `QualityRerollResult`）：有值时
    /// 弹窗原位换成对比面板，代替目标列表。
    pub quality_result: ReactiveSignal<Option<QualityRerollResult>>,
    /// 目标列表加载失败的提示只弹一次（旧版 `pokemon_list_error_shown`）。
    pub list_error_shown: ReactiveSignal<bool>,
}

/// use_item 带回的目标宠物更新（旧版 `PokemonUpdate`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PokemonUpdate {
    pub id: u64,
    pub hp: Option<i64>,
    pub max_hp: Option<i64>,
    pub state: Option<u64>,
    /// 品质重洗前的品质名（仅品质重洗药返回）。
    pub old_quality_grade: Option<String>,
    /// 品质重洗后的品质名（仅品质重洗药返回）。
    pub new_quality_grade: Option<String>,
    pub old_iv_total: Option<u64>,
    pub new_iv_total: Option<u64>,
}

/// 品质重洗药使用成功后的前后对比面板数据（旧版 `QualityRerollResult`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct QualityRerollResult {
    pub pokemon_id: u64,
    pub name: String,
    pub type_id: u64,
    pub level: u64,
    pub is_shiny: bool,
    pub old_grade: String,
    pub new_grade: String,
    pub old_iv_total: u64,
    pub new_iv_total: u64,
    pub item_remaining: i64,
}

impl InventoryState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            entry_category: std::cell::Cell::new(None),
            // 旧版挂载时的默认分类是「回复药」（见 inventory::on_enter）。
            category: use_signal(|| Some(1u32)),
            page: use_signal(|| 1u32),
            total_pages: use_signal(|| 1usize),
            total: use_signal(|| 0usize),
            search_input: use_signal(String::new),
            search_query: use_signal(String::new),
            items: use_signal(|| None),
            using: use_signal(|| None),
            selected_item: use_signal(|| None),
            show_target_modal: use_signal(|| false),
            usable_pokemon: use_signal(|| None),
            quality_result: use_signal(|| None),
            list_error_shown: use_signal(|| false),
        }
    }
}

impl std::fmt::Debug for InventoryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InventoryState").finish()
    }
}

/// 商店页持久状态。`PocState` 的继任者：POC 骨架已退役，
/// 这里承载第一个真实游戏页面。
#[derive(Clone)]
pub struct ShopState {
    /// 组件 id：promise 线程回调用 mark_dirty_deferred 入队时需要
    /// （不能走 mark_dirty——那会经调度回调发起异步导入并被 jco 吞掉）。
    pub component_id: std::cell::Cell<u32>,
    /// 当前分类：Some(1..=5) 为物品分类，None 为宠物页。
    pub category: ReactiveSignal<Option<u32>>,
    pub page: ReactiveSignal<u32>,
    pub keyword: ReactiveSignal<String>,
    pub filter_open: ReactiveSignal<bool>,

    /// 列表数据。None = 尚在首次加载；Some(Err) = 请求失败。
    pub items: ReactiveSignal<Option<Result<Vec<ShopItem>, String>>>,
    pub pets: ReactiveSignal<Option<Result<Vec<ShopPet>, String>>>,
    pub total_pages: ReactiveSignal<usize>,

    pub buying: ReactiveSignal<bool>,
}

impl ShopState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            category: use_signal(|| Some(1u32)),
            page: use_signal(|| 1u32),
            keyword: use_signal(String::new),
            filter_open: use_signal(|| false),
            items: use_signal(|| None),
            pets: use_signal(|| None),
            total_pages: use_signal(|| 1usize),
            buying: use_signal(|| false),
        }
    }
}

impl std::fmt::Debug for ShopState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 组件宏会给 props 派生 Debug；信号句柄无意义，报个骨架即可。
        f.debug_struct("ShopState").finish()
    }
}

/// 宠物（对齐 rust/utils/src/types/api_pokemon.rs PokemonBasic 的显示子集）。
#[derive(Debug, Clone, Default)]
pub struct PokemonBasic {
    pub id: u64,
    /// 物种名（原始名）。
    pub name: String,
    pub nickname: Option<String>,
    pub type_id: u64,
    pub level: u64,
    pub hp: i64,
    pub max_hp: i64,
    pub is_shiny: bool,
    /// 位置：1=首位/战斗中，2=队伍，其余=箱子。
    pub site: u8,
    pub state: u8,
    pub state_text: String,
    pub state_class: String,
    /// 六项个体值总和（0-186，历史宠物可能溢出；旧版载荷无此字段）。
    pub iv_total: Option<u64>,
    /// 品质档位（1=E .. 8=SS+，9=UnKnow；缺失表示旧版载荷）。
    pub quality_rank: Option<u8>,
    /// 品质名（E/D/C/B/A/S/SS/SS+，溢出历史数据为 UnKnow）。
    pub quality_grade: Option<String>,
    /// 技能槽（list 载荷恒定 4 槽，空槽 type_id=0）。
    pub skills: Vec<PokemonSkill>,
    /// 属性 1（list 载荷 base_info；仓库页属性筛选用）。
    pub type_1: String,
    /// 属性 2。
    pub type_2: Option<String>,
    pub is_legendary: bool,
    /// 经验与当前等级区间（侧栏迷你格 tooltip 的 EXP 条用）。
    pub exp: u64,
    pub exp_for_current_level: u64,
    pub exp_for_next_level: u64,
}

impl PokemonBasic {
    /// 未满 PP 的技能列表（"pp/max"），用于治疗页提示。
    pub fn depleted_pp(&self) -> Vec<String> {
        self.skills
            .iter()
            .filter(|s| s.max_pp > 0 && s.pp < s.max_pp && !s.name.is_empty())
            .map(|s| format!("{}/{}", s.pp, s.max_pp))
            .collect()
    }
}

impl PokemonBasic {
    /// 显示名：优先昵称。
    pub fn display_name(&self) -> String {
        match self.nickname.as_deref() {
            Some(n) if !n.is_empty() => n.to_string(),
            _ => self.name.clone(),
        }
    }

    /// 掉了血或处于负面状态。战斗前后的自动治疗只看这两样，不看 PP——不然连续
    /// 野战时 PP 永远是满的，PP 道具就没有意义了（旧版 `get_injured_pokemons`）。
    pub fn is_injured(&self) -> bool {
        self.hp < self.max_hp || is_negative_state(self.state)
    }

    /// 治疗中心的判定：除了掉血与异常状态，任一技能 PP 未满同样需要治疗——后端
    /// 治疗会一并补满全部技能 PP（旧版 `needs_healing`）。
    pub fn needs_healing(&self) -> bool {
        self.is_injured() || self.skills.iter().any(|s| s.max_pp > 0 && s.pp < s.max_pp)
    }
}

/// 负面状态码表（对齐服务端；20–22 是虚弱状态，同样要治疗）。
pub fn is_negative_state(state: u8) -> bool {
    matches!(state, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 11 | 15 | 20 | 21 | 22)
}

#[cfg(test)]
mod sprite_tests {
    use super::{back_sprite_url, front_sprite_url};

    #[test]
    fn front_and_back_sprites_use_their_own_directories() {
        assert_eq!(
            front_sprite_url(384, false),
            "https://img.tsdm39.com/Pokemon/pm/384.gif"
        );
        assert_eq!(
            front_sprite_url(384, true),
            "https://img.tsdm39.com/Pokemon/pmsg/384.gif"
        );
        // 背面图是 `pmb` / `pmsgb`——之前写成 `back/`，战斗里我方精灵图是裂的。
        assert_eq!(
            back_sprite_url(384, false),
            "https://img.tsdm39.com/Pokemon/pmb/384.gif"
        );
        assert_eq!(
            back_sprite_url(384, true),
            "https://img.tsdm39.com/Pokemon/pmsgb/384.gif"
        );
    }
}

#[cfg(test)]
mod healing_tests {
    use super::{is_negative_state, PokemonBasic, PokemonSkill};

    fn pokemon(hp: i64, max_hp: i64, state: u8, pp: Option<(u64, u64)>) -> PokemonBasic {
        PokemonBasic {
            hp,
            max_hp,
            state,
            skills: pp
                .map(|(pp, max_pp)| {
                    vec![PokemonSkill {
                        pp,
                        max_pp,
                        name: "撞击".to_string(),
                        ..Default::default()
                    }]
                })
                .unwrap_or_default(),
            ..Default::default()
        }
    }

    #[test]
    fn a_pokemon_below_full_hp_needs_healing_in_the_center_and_before_battle() {
        // 11/19：旧版宠物中心给治疗卡片，之前这里只认晕倒。
        let p = pokemon(11, 19, 1, None);
        assert!(p.needs_healing());
        assert!(p.is_injured());
    }

    #[test]
    fn a_fainted_pokemon_needs_healing() {
        assert!(pokemon(0, 19, 1, None).needs_healing());
    }

    #[test]
    fn a_pokemon_with_unfilled_pp_needs_the_center_but_not_the_battle_heal() {
        let p = pokemon(19, 19, 1, Some((3, 15)));
        assert!(p.needs_healing());
        assert!(!p.is_injured());
    }

    #[test]
    fn an_empty_skill_slot_does_not_count() {
        // 空槽 type_id=0、max_pp=0。
        assert!(!pokemon(19, 19, 1, Some((0, 0))).needs_healing());
    }

    #[test]
    fn a_negative_state_needs_healing_even_at_full_hp() {
        for state in [0u8, 2, 3, 4, 5, 6, 7, 11, 15, 20, 21, 22] {
            assert!(is_negative_state(state), "state {state}");
            assert!(
                pokemon(19, 19, state, None).needs_healing(),
                "state {state}"
            );
        }
        for state in [1u8, 8, 10, 19, 23, 255] {
            assert!(!is_negative_state(state), "state {state}");
        }
    }

    #[test]
    fn a_healthy_pokemon_does_not() {
        assert!(!pokemon(19, 19, 1, Some((15, 15))).needs_healing());
    }
}

/// 治疗中心持久状态。
#[derive(Clone)]
pub struct CenterState {
    pub component_id: std::cell::Cell<u32>,
    /// None = 首次加载中；Some(Err) = 失败。
    pub pokemons: ReactiveSignal<Option<Result<Vec<PokemonBasic>, String>>>,
    /// 是否处于战斗（npcid > 0）：战斗中首位宠物只能「脱战并治疗」。
    pub in_battle: ReactiveSignal<bool>,
    /// 治疗请求进行中（互斥守卫）。
    pub healing: ReactiveSignal<bool>,
}

impl CenterState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            pokemons: use_signal(|| None),
            in_battle: use_signal(|| false),
            healing: use_signal(|| false),
        }
    }
}

impl std::fmt::Debug for CenterState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CenterState").finish()
    }
}

/// 我的宝可梦页签名。
#[derive(Clone, Copy, PartialEq)]
pub enum MyPokemonTab {
    Stats,
    Equipment,
    Skills,
}

/// 宠物技能槽（list 载荷 skills 数组的显示子集）。
#[derive(Debug, Clone, Default)]
pub struct PokemonSkill {
    pub type_id: u64,
    pub pp: u64,
    pub max_pp: u64,
    pub name: String,
    pub skill_type: String,
    pub category: String,
    pub level: u32,
    pub power: u32,
    /// 是否满足遗忘条件（遗忘要求 PP 为满；max_pp=0 恒可遗忘）。
    pub can_forget: bool,
}

/// 可学习技能（learnable_skills 载荷）。
#[derive(Debug, Clone, Default)]
pub struct LearnableSkill {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub skill_type: String,
    pub category: String,
    pub power: u32,
    pub max_pp: u32,
    pub required_level: u32,
    pub is_available: bool,
}

/// 装备物品（equipment 载荷的显示子集，槽位与背包共用同一形态）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EquipmentItem {
    /// 背包行 ID（pm_myitem.id）——装备/卸下都以它为键。
    pub myitem_id: u64,
    pub name: String,
    pub description: String,
    /// 图标文件名（不带扩展名时补 .gif）。
    pub image: String,
    pub equipment_hp: i32,
    pub equipment_atk: i32,
    pub equipment_def: i32,
    pub equipment_spatk: i32,
    pub equipment_spdef: i32,
    pub equipment_sd: i32,
    /// 仅背包列表：该记录是否被本宠物装备。
    pub is_equipped: bool,
    /// 仅背包列表：被任意宠物装备的数量。
    pub equipped_count: u32,
    /// 仅背包列表：可用数量（被占用时为 0）。
    pub available_count: u32,
}

impl EquipmentItem {
    /// 六维加成 (HP, 攻击, 防御, 特攻, 特防, 速度)。
    pub fn bonuses(&self) -> [i64; 6] {
        [
            self.equipment_hp as i64,
            self.equipment_atk as i64,
            self.equipment_def as i64,
            self.equipment_spatk as i64,
            self.equipment_spdef as i64,
            self.equipment_sd as i64,
        ]
    }
}

/// 悬浮中的提示（旧版各页的悬浮提示：装备、商店瓦片、战斗道具卡）。指针总是先
/// 离开旧卡片再进新卡片，同一时刻至多一份，所以整个应用只留一席
/// （`AppState::hover_tip`）；承载与定位规则见 `crate::hover_tip`。
#[derive(Clone)]
pub struct HoverTip {
    /// 旧版提示根节点的类：game.css 里那条规则照常给观感。
    pub class: &'static str,
    /// 提示左上角，由各页自己的定位函数算好。挂到应用根部时是视口坐标；就地渲染
    /// （`in_place_owner`）时是所在卡片的内边距盒坐标——见那里的说明。
    pub x: f64,
    pub y: f64,
    /// `Some(卡片下标)`：由那张卡片自己就地渲染（`hover_tip::surface_for`），不挂到
    /// 应用根部。旧版把提示画在卡片里面，而卡片带 `transform`（商店瓦片悬停时上移）：
    /// 固定定位的提示于是相对卡片的内边距盒落位，而不是相对视口——旧版的实际效果
    /// 就是这样，端口照搬。
    pub in_place_owner: Option<usize>,
    /// 正文：每次渲染重新生成，页面数据变了内容跟着变（旧版读的是当前信号）。
    pub content: std::rc::Rc<dyn Fn() -> tairitsu_vdom::VNode>,
}

/// 单个装备槽。
#[derive(Debug, Clone, Default)]
pub struct EquipmentSlotData {
    pub slot_index: u32,
    pub item: Option<EquipmentItem>,
}

/// equipment 载荷（槽位 + 背包装备清单）。
#[derive(Debug, Clone, Default)]
pub struct EquipmentData {
    /// 恒定补齐到 4 槽。
    pub slots: Vec<EquipmentSlotData>,
    pub owned: Vec<EquipmentItem>,
}

impl EquipmentData {
    /// 已装备物品的六维加成合计（HP, 攻击, 防御, 特攻, 特防, 速度）。
    pub fn total_bonuses(&self) -> [i64; 6] {
        let mut total = [0i64; 6];
        for slot in &self.slots {
            if let Some(item) = &slot.item {
                for (i, v) in item.bonuses().into_iter().enumerate() {
                    total[i] += v;
                }
            }
        }
        total
    }
}

/// 装备页当前选中项：来自背包或某个槽位。
#[derive(Debug, Clone, PartialEq)]
pub struct SelectedEquip {
    /// true = 背包物品（点击槽位装载）；false = 已装备（点击槽位移动/卸下）。
    pub from_inventory: bool,
    pub myitem_id: u64,
    pub slot_index: Option<u32>,
}

/// 宠物详情（我的宝可梦「属性」页所需字段）。
///
/// 定义在 state 而非 api：`api` 模块只在 wasm 下编译，而持久状态要能在
/// 测试目标（host）下编译。`crate::api::pokemon_detail` 负责填充它。
#[derive(Debug, Clone, Default)]
pub struct PokemonDetail {
    pub id: u64,
    pub name: String,
    pub nickname: Option<String>,
    pub type_id: u64,
    pub level: u64,
    pub exp: u64,
    pub exp_for_current_level: u64,
    pub exp_for_next_level: u64,
    pub hp: i64,
    pub max_hp: i64,
    pub gender: u8,
    pub is_shiny: bool,
    pub state: u8,
    pub state_text: String,
    pub state_class: String,
    pub affection: u32,
    /// 六项个体值（旧端点可能无此字段）。
    pub ivs: Option<[u64; 6]>,
    pub quality_rank: Option<u8>,
    pub quality_grade: Option<String>,
    /// (攻击, 防御, 特攻, 特防, 速度)。
    pub stats: Option<[i64; 5]>,
    pub type_1: String,
    pub type_2: Option<String>,
    pub is_legendary: bool,
}

/// 我的宝可梦「属性」页持久状态。
#[derive(Clone)]
pub struct MyPokemonState {
    pub component_id: std::cell::Cell<u32>,
    pub tab: ReactiveSignal<MyPokemonTab>,
    /// 当前选中的宠物 ID；None = 列表第一只。
    pub selected_id: ReactiveSignal<Option<u64>>,
    /// 详情（属性面板数据）。
    pub detail: ReactiveSignal<Option<Result<PokemonDetail, String>>>,
    pub editing: ReactiveSignal<bool>,
    pub edit_value: ReactiveSignal<String>,
    /// 保存中（互斥守卫）。
    pub saving: ReactiveSignal<bool>,
    pub save_error: ReactiveSignal<Option<String>>,
    pub save_success: ReactiveSignal<bool>,

    // ---- 装备标签页 ----
    /// None = 首次加载中；Some(Err) = 失败。
    pub equipment: ReactiveSignal<Option<Result<EquipmentData, String>>>,
    /// 装备/卸下动作进行中（互斥守卫）。
    pub equip_loading: ReactiveSignal<bool>,
    pub equip_selected: ReactiveSignal<Option<SelectedEquip>>,

    // ---- 技能标签页 ----
    pub learn_modal_open: ReactiveSignal<bool>,
    /// None = 加载中；Some(Err) = 失败。(可学习, 等级不足)
    pub learnable:
        ReactiveSignal<Option<Result<(Vec<LearnableSkill>, Vec<LearnableSkill>), String>>>,
    /// 学习/遗忘进行中（互斥守卫）。
    pub skill_loading: ReactiveSignal<bool>,

    // ---- 属性页弹窗：努力值计算器 / 进化路径 / 进化 ----
    pub show_ev_modal: ReactiveSignal<bool>,
    /// 努力值计算器：当前值与计划追加（各六维，上限 255/项、总计 510）。
    pub ev_current: ReactiveSignal<[u32; 6]>,
    pub ev_added: ReactiveSignal<[u32; 6]>,
    pub show_evo_path_modal: ReactiveSignal<bool>,
    /// 进化路径 tab：true = 进化路线，false = 退化路线。
    pub evo_path_forward: ReactiveSignal<bool>,
    pub evo_path: ReactiveSignal<Option<Result<EvolutionPathData, String>>>,
    pub show_evolution_modal: ReactiveSignal<bool>,
    /// 进化条件检查（可进化路线列表）。
    pub evo_check: ReactiveSignal<Option<Result<Vec<AvailableEvolution>, String>>>,
    /// 进化请求进行中（互斥守卫）。
    pub evolving: ReactiveSignal<bool>,
    /// 上次进化失败的原因，显示在进化弹窗里（旧版 `evolve_error`；不用提示条）。
    pub evo_error: ReactiveSignal<Option<String>>,
}

/// 进化路径节点（evolution_path 载荷）。
#[derive(Debug, Clone, Default)]
pub struct EvolutionPathNode {
    pub id: u64,
    pub name: String,
    pub type_1: String,
    pub type_2: Option<String>,
    pub condition_display: String,
}

/// evolution_path 载荷（当前宝可梦 + 前向/后向两段）。
#[derive(Debug, Clone, Default)]
pub struct EvolutionPathData {
    pub pokemon_id: u64,
    pub pokemon_name: String,
    pub pokemon_type1: String,
    pub pokemon_type2: Option<String>,
    pub forward: Vec<EvolutionPathNode>,
    pub backward: Vec<EvolutionPathNode>,
}

/// 可进化路线（evolution/check 载荷的显示子集）。
#[derive(Debug, Clone, Default)]
pub struct AvailableEvolution {
    pub to_id: u64,
    pub to_name: String,
    pub can_evolve: bool,
    /// 未满足时的原因（服务端中文文案）。
    pub reason: String,
    /// 后端拼好的条件文案（例「等级达到 Lv.16」）。
    pub condition_display: Option<String>,
}

impl MyPokemonState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            tab: use_signal(|| MyPokemonTab::Stats),
            selected_id: use_signal(|| None),
            detail: use_signal(|| None),
            editing: use_signal(|| false),
            edit_value: use_signal(String::new),
            saving: use_signal(|| false),
            save_error: use_signal(|| None),
            save_success: use_signal(|| false),
            equipment: use_signal(|| None),
            equip_loading: use_signal(|| false),
            equip_selected: use_signal(|| None),
            learn_modal_open: use_signal(|| false),
            learnable: use_signal(|| None),
            skill_loading: use_signal(|| false),
            show_ev_modal: use_signal(|| false),
            ev_current: use_signal(|| [0u32; 6]),
            ev_added: use_signal(|| [0u32; 6]),
            show_evo_path_modal: use_signal(|| false),
            evo_path_forward: use_signal(|| true),
            evo_path: use_signal(|| None),
            show_evolution_modal: use_signal(|| false),
            evo_check: use_signal(|| None),
            evolving: use_signal(|| false),
            evo_error: use_signal(|| None),
        }
    }
}

impl std::fmt::Debug for MyPokemonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyPokemonState").finish()
    }
}

/// 仓库页持久状态（筛选值即时生效，渲染时套用——与 Dioxus 版一致）。
#[derive(Clone)]
pub struct StorageState {
    pub component_id: std::cell::Cell<u32>,
    pub select_mode: ReactiveSignal<bool>,
    pub selected_ids: ReactiveSignal<Vec<u64>>,
    pub search_input: ReactiveSignal<String>,
    pub type_filter: ReactiveSignal<String>,
    pub min_level: ReactiveSignal<String>,
    pub max_level: ReactiveSignal<String>,
    /// "" / "shiny" / "normal"。
    pub shiny_filter: ReactiveSignal<String>,
    /// "oldest" / "newest" / "level_asc" / "level_desc"。
    pub sort_value: ReactiveSignal<String>,
    /// 已渲染数量（每次 +60，筛选变化时重置）。
    pub visible_count: ReactiveSignal<usize>,
    /// 写操作进行中（互斥守卫）。
    pub action_loading: ReactiveSignal<bool>,
}

/// 仓库页每页条数（对齐 Dioxus 版 STORAGE_PAGE_SIZE）。
pub const STORAGE_PAGE_SIZE: usize = 60;

impl StorageState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            select_mode: use_signal(|| false),
            selected_ids: use_signal(Vec::new),
            search_input: use_signal(String::new),
            type_filter: use_signal(String::new),
            min_level: use_signal(String::new),
            max_level: use_signal(String::new),
            shiny_filter: use_signal(String::new),
            sort_value: use_signal(|| "oldest".to_string()),
            visible_count: use_signal(|| STORAGE_PAGE_SIZE),
            action_loading: use_signal(|| false),
        }
    }

    /// 筛选值变化：清选择、重置分页（对齐 Dioxus 版各 oninput 分支）。
    pub fn reset_view(&self) {
        self.selected_ids.inner().set(Vec::new());
        self.visible_count.inner().set(STORAGE_PAGE_SIZE);
    }
}

impl std::fmt::Debug for StorageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageState").finish()
    }
}

/// 待确认的战斗动作（重试用；与 Dioxus 版 PendingBattleAction 对应，
/// 幂等键在提交时生成）。
#[derive(Clone)]
pub struct PendingBattleAction {
    pub action: String,
    pub engine_battle_id: u64,
    pub expected_revision: u64,
    pub fields: serde_json::Value,
}

/// 战斗/冒险页持久状态。
///
/// 场景态全部从 scene 派生（整树重挂架构不做边沿检测）：
/// - needs_replacement() → 换宠弹窗；
/// - effective_battle_over() → 结算弹窗（on_end 清 scene 退出）。
#[derive(Clone)]
pub struct BattleState {
    pub component_id: std::cell::Cell<u32>,
    /// None = 恢复中/未加载；Some(None) = 无战斗；Some(Some(scene)) = 战斗中。
    pub scene: ReactiveSignal<Option<Option<crate::battle_types::BattleScene>>>,
    /// 动作进行中（互斥守卫，阻断一切入口）。
    pub loading: ReactiveSignal<bool>,
    /// 上次操作结果未确认（网络不确定失败）→ 连接错误弹窗。
    pub conn_error: ReactiveSignal<Option<String>>,
    pub pending: ReactiveSignal<Option<PendingBattleAction>>,
    /// 地图列表。
    pub maps: ReactiveSignal<Option<Result<Vec<crate::battle_types::MapInfo>, String>>>,
    pub selected_region: ReactiveSignal<Option<String>>,
    /// 详情弹窗中的地图 ID。
    pub selected_map: ReactiveSignal<Option<u64>>,
    /// 战斗快捷栏。
    pub battle_items: ReactiveSignal<Vec<crate::battle_types::BattleItem>>,
    pub battle_balls: ReactiveSignal<Vec<InventoryItem>>,
    /// PP 恢复道具的技能选择弹窗。
    pub selection: ReactiveSignal<Option<crate::battle_types::SkillSelection>>,
    /// 战斗页当前标签：0=技能 1=道具 2=捕捉 3=逃跑。
    pub tab: ReactiveSignal<u8>,
    /// 手动打开的换宠确认弹窗。
    pub show_switch_modal: ReactiveSignal<bool>,
    /// 战报弹窗（完整响应含逐回合分组与 bbcode）。
    pub show_log: ReactiveSignal<bool>,
    pub log_data: ReactiveSignal<Option<Result<crate::battle_types::BattleLogResponse, String>>>,
    /// 复制战报的结果提示（空 = 尚未操作）。
    pub log_copy_status: ReactiveSignal<String>,
    /// 上次开战的地图（结算「继续战斗」用）。
    pub last_map_id: ReactiveSignal<u64>,
    /// 点开的道具 / 精灵球卡详情（旧版 BattleItemCard 的组件内状态；整树重挂不
    /// 保留组件状态，所以放在这里；悬浮提示走应用级的 `AppState::hover_tip`）。
    /// `(是否精灵球, 列表下标)`。
    pub item_details: ReactiveSignal<Option<ItemCardKey>>,
    /// 我方出场位的悬浮详情（旧版 MyPokemonSection 的 show_tooltip）。
    pub player_hover: ReactiveSignal<bool>,
}

/// 道具 / 精灵球卡的标识：`(是否精灵球, 列表下标)`。
pub type ItemCardKey = (bool, usize);

impl BattleState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            scene: use_signal(|| None),
            loading: use_signal(|| false),
            conn_error: use_signal(|| None),
            pending: use_signal(|| None),
            maps: use_signal(|| None),
            selected_region: use_signal(|| None),
            selected_map: use_signal(|| None),
            battle_items: use_signal(Vec::new),
            battle_balls: use_signal(Vec::new),
            selection: use_signal(|| None),
            tab: use_signal(|| 0u8),
            show_switch_modal: use_signal(|| false),
            show_log: use_signal(|| false),
            log_data: use_signal(|| None),
            log_copy_status: use_signal(String::new),
            last_map_id: use_signal(|| 0u64),
            item_details: use_signal(|| None),
            player_hover: use_signal(|| false),
        }
    }
}

impl std::fmt::Debug for BattleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BattleState").finish()
    }
}
