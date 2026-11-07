//! 后台持久状态。

use tairitsu_hooks::{use_signal, ReactiveSignal};

/// 后台路由（对齐 Dioxus 版 AdminRoute；未移植页面在导航中占位）。
#[derive(Clone, Copy, PartialEq)]
pub enum AdminRoute {
    GlobalConfig,
    PokemonData,
    ItemData,
    MapData,
    UserData,
    EvolutionData,
    SkillType,
    EffectData,
}

impl AdminRoute {
    pub fn key(self) -> &'static str {
        match self {
            AdminRoute::GlobalConfig => "global-config",
            AdminRoute::PokemonData => "pokemon-data",
            AdminRoute::ItemData => "item-data",
            AdminRoute::MapData => "map-data",
            AdminRoute::UserData => "user-data",
            AdminRoute::EvolutionData => "evolution-data",
            AdminRoute::SkillType => "skill-type",
            AdminRoute::EffectData => "effect-data",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            AdminRoute::GlobalConfig => "全局配置",
            AdminRoute::PokemonData => "宝可梦数据",
            AdminRoute::ItemData => "物品数据",
            AdminRoute::MapData => "地图数据",
            AdminRoute::UserData => "用户数据",
            AdminRoute::EvolutionData => "进化链数据",
            AdminRoute::SkillType => "技能数据",
            AdminRoute::EffectData => "效果数据",
        }
    }

    /// 该路由是否已移植到 tairitsu。
    pub fn ported(self) -> bool {
        // 八个数据页都已移植（切片 1-7）；保留该函数是为了后续新增路由
        // 时导航徽章仍能自动生效。
        true
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ToastKind {
    Success,
    Error,
}

/// 全局配置页状态：JSON 文本直接编辑（切片 1 形态；表单化随后续切片）。
#[derive(Clone)]
pub struct GlobalConfigState {
    pub component_id: std::cell::Cell<u32>,
    /// None = 加载中；Some(Err) = 加载失败。
    pub loaded: ReactiveSignal<Option<Result<(), String>>>,
    /// 服务端返回的原文（美化后）。
    pub saved_json: ReactiveSignal<String>,
    /// 编辑框当前内容。
    pub draft: ReactiveSignal<String>,
    pub saving: ReactiveSignal<bool>,
    pub toast_visible: ReactiveSignal<bool>,
    pub toast_message: ReactiveSignal<String>,
    pub toast_kind: ReactiveSignal<ToastKind>,
}

impl GlobalConfigState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            loaded: use_signal(|| None),
            saved_json: use_signal(String::new),
            draft: use_signal(String::new),
            saving: use_signal(|| false),
            toast_visible: use_signal(|| false),
            toast_message: use_signal(String::new),
            toast_kind: use_signal(|| ToastKind::Success),
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.draft.get() != self.saved_json.get()
    }
}

impl std::fmt::Debug for GlobalConfigState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalConfigState").finish()
    }
}

pub fn show_toast_silent(state: &GlobalConfigState, kind: ToastKind, message: String) {
    state.toast_kind.inner().set(kind);
    state.toast_message.inner().set(message);
    state.toast_visible.inner().set(true);
}

/// 新闻公告条目（config.news_announcements 的元素）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NewsAnnouncement {
    pub title: String,
    pub url: String,
}

/// 全局配置的分节表单状态（切片 14）。
///
/// Dioxus 版的配置页是分节表单 + 一个新闻公告列表编辑器；admin2 先前只有
/// 裸 JSON 文本框。这里补上表单化：字段用字符串信号承载（输入框直绑），
/// 提交时按原始配置对象的键写回，未在表单里建模的键原样保留。
///
/// 关于 JSON 树：Dioxus 的 `JsonPanel` / `JsonEditorField` 从初始 commit
/// 起就没有任何渲染调用点（只在文件里定义），属于死代码，因此不移植。
#[derive(Clone)]
pub struct ConfigFormState {
    /// 公告列表编辑弹窗是否打开。
    pub ann_open: ReactiveSignal<bool>,
    /// 正在编辑的条目下标；None = 新增。
    pub ann_editing: ReactiveSignal<Option<usize>>,
    pub ann_title: ReactiveSignal<String>,
    pub ann_url: ReactiveSignal<String>,
    /// 表单模式 / 原始 JSON 模式。
    pub raw_mode: ReactiveSignal<bool>,
}

impl ConfigFormState {
    pub fn new() -> Self {
        Self {
            ann_open: use_signal(|| false),
            ann_editing: use_signal(|| None),
            ann_title: use_signal(String::new),
            ann_url: use_signal(String::new),
            raw_mode: use_signal(|| false),
        }
    }
}

impl std::fmt::Debug for ConfigFormState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigFormState").finish()
    }
}

/// 通用数据列表页状态（切片 2 定型、切片 3 起复用）。
///
/// 与 Dioxus 版 ListPageState 对应：rows 保留原始 JSON（显示字段渲染时
/// 取，缺字段回退），装载守卫是 busy + loaded 组合（防重挂活锁）。
#[derive(Clone)]
pub struct ListState {
    pub component_id: std::cell::Cell<u32>,
    /// None = 首次加载中；Some(Err) = 加载失败。
    pub loaded: ReactiveSignal<Option<Result<(), String>>>,
    pub rows: ReactiveSignal<Vec<serde_json::Value>>,
    pub total: ReactiveSignal<u64>,
    pub loaded_count: ReactiveSignal<u64>,
    pub busy: ReactiveSignal<bool>,
    /// 筛选中（filter 结果不分页，隐藏加载更多）。
    pub filtered: ReactiveSignal<bool>,
    pub err: ReactiveSignal<Option<String>>,
    // 筛选输入：ID（equal）+ 名称（contains）+ 两个实体特定筛选。
    pub f_id: ReactiveSignal<String>,
    pub f_name: ReactiveSignal<String>,
    pub f_a: ReactiveSignal<String>,
    pub f_b: ReactiveSignal<String>,
}

impl ListState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            loaded: use_signal(|| None),
            rows: use_signal(Vec::new),
            total: use_signal(|| 0u64),
            loaded_count: use_signal(|| 0u64),
            busy: use_signal(|| false),
            filtered: use_signal(|| false),
            err: use_signal(|| None),
            f_id: use_signal(String::new),
            f_name: use_signal(String::new),
            f_a: use_signal(String::new),
            f_b: use_signal(String::new),
        }
    }
}

impl std::fmt::Debug for ListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListState").finish()
    }
}

/// 实体列表配置：动作名 + 实体特定筛选（f_a/f_b）的服务端 tag 与
/// operator + 输入框占位文案。切片 3 起每个数据页一份。
#[derive(Clone, Copy)]
pub struct EntitySpec {
    /// 如 "pokemon_type"（拼进 list::/count::/filter::）。
    pub entity: &'static str,
    pub title: &'static str,
    /// f_a 筛选；None 则该输入框不渲染。
    pub a: Option<FilterSpec>,
    pub b: Option<FilterSpec>,
    /// ID 筛选的服务端 tag（绝大多数实体是「ID」，用户页是「UID」）。
    pub id_tag: &'static str,
    /// 名称筛选的服务端 tag（绝大多数实体是「名称」，用户页是「昵称」）。
    pub name_tag: &'static str,
    /// 名称筛选的 operator（默认 contains；用户页昵称也用 contains）。
    pub name_operator: &'static str,
    /// 名称输入框占位（如「名称（包含）」；进化页是「进化来源」）。
    pub name_placeholder: &'static str,
}

#[derive(Clone, Copy)]
pub struct FilterSpec {
    /// 服务端 filter 的 tag（如「宠物类型」「类型」）。
    pub tag: &'static str,
    pub operator: &'static str,
    pub placeholder: &'static str,
}

/// 宝可梦数据（切片 2 实例化）。
pub const POKEMON_SPEC: EntitySpec = EntitySpec {
    entity: "pokemon_type",
    title: "宠物数据",
    a: Some(FilterSpec {
        tag: "宠物类型",
        operator: "contains",
        placeholder: "属性（如火）",
    }),
    b: None,
    id_tag: "ID",
    name_tag: "名称",
    name_operator: "contains",
    name_placeholder: "名称（包含）",
};

/// 物品数据（切片 3 实例化）。类型筛选对数字列（1药品/2球/3素材/4强化/5装备）。
pub const ITEM_SPEC: EntitySpec = EntitySpec {
    entity: "item_type",
    title: "物品数据",
    a: Some(FilterSpec {
        tag: "类型",
        operator: "contains",
        placeholder: "类型（1药品/2球/3素材/4强化/5装备）",
    }),
    b: Some(FilterSpec {
        tag: "价格",
        operator: "greater_or_equal",
        placeholder: "价格 ≥",
    }),
    id_tag: "ID",
    name_tag: "名称",
    name_operator: "contains",
    name_placeholder: "名称（包含）",
};

/// 地图数据（切片 4 实例化）。地形筛选传英文 snake_case 名
/// （服务端 translate_map_full_name_to_alpha 转单字符）；等级筛选走数字列。
pub const MAP_SPEC: EntitySpec = EntitySpec {
    entity: "map_info",
    title: "地图数据",
    a: Some(FilterSpec {
        tag: "地形类型",
        operator: "equal",
        placeholder: "地形（如 plain/grass/sea/cave）",
    }),
    b: Some(FilterSpec {
        tag: "野怪最低等级",
        operator: "greater_or_equal",
        placeholder: "最低等级 ≥",
    }),
    id_tag: "ID",
    name_tag: "名称",
    name_operator: "contains",
    name_placeholder: "名称（包含）",
};

/// 用户数据（切片 5 实例化）。ID 筛选的 tag 是「UID」，名称是「昵称」
/// （服务端按 common_member.username 反查 uid）；胜场/金钱走数字列。
pub const USER_SPEC: EntitySpec = EntitySpec {
    entity: "user_info",
    title: "用户数据",
    a: Some(FilterSpec {
        tag: "胜场",
        operator: "greater_or_equal",
        placeholder: "胜场 ≥",
    }),
    b: Some(FilterSpec {
        tag: "金钱",
        operator: "greater_or_equal",
        placeholder: "金钱 ≥",
    }),
    id_tag: "UID",
    name_tag: "昵称",
    name_operator: "contains",
    name_placeholder: "用户名（包含）",
};

/// 进化链数据（切片 6 实例化）。无「名称」概念：name 槽位承载「进化来源」，
/// a 槽位承载「进化目标」（两者服务端都按 pm_data.name 反查 id）。
pub const EVOLUTION_SPEC: EntitySpec = EntitySpec {
    entity: "evolution_info",
    title: "进化链数据",
    a: Some(FilterSpec {
        tag: "进化目标",
        operator: "contains",
        placeholder: "进化目标（宠物名）",
    }),
    b: None,
    id_tag: "ID",
    name_tag: "进化来源",
    name_operator: "contains",
    name_placeholder: "进化来源（宠物名）",
};

/// 技能数据（切片 7a 实例化）。「可用此的种族」是数字 ID（服务端按
/// available_pokemons 里的 ",id," 模式匹配）。
pub const SKILL_SPEC: EntitySpec = EntitySpec {
    entity: "skill_type",
    title: "技能数据",
    a: Some(FilterSpec {
        tag: "可用此的种族",
        operator: "contains",
        placeholder: "种族 ID（如 25 皮卡丘）",
    }),
    b: None,
    id_tag: "ID",
    name_tag: "名称",
    name_operator: "contains",
    name_placeholder: "名称（包含）",
};

/// 效果数据（切片 7b 实例化）。服务端没有 filter_effect_data，
/// 前端也不提供筛选（与 Dioxus 版一致，只 list/count + 分页）。
pub const EFFECT_SPEC: EntitySpec = EntitySpec {
    entity: "effect_data",
    title: "效果数据",
    a: None,
    b: None,
    id_tag: "ID",
    name_tag: "名称",
    name_operator: "contains",
    name_placeholder: "名称（包含）",
};

/// 宝可梦编辑弹窗状态（切片 8）。
///
/// 表单值以字符串/布尔的扁平结构保存（输入框直接绑定），提交时组装成
/// 服务端要求的完整 PokemonType JSON；`extra_json` 兜底承载表单未覆盖的
/// 字段（map_ids/evolution_info_ids/drop_money_range 等），保持载荷完整。
#[derive(Clone)]
pub struct PokemonEditorState {
    pub open: ReactiveSignal<bool>,
    /// None = 新增；Some(id) = 编辑现有。
    pub editing_id: ReactiveSignal<Option<u64>>,
    pub name: ReactiveSignal<String>,
    pub description: ReactiveSignal<String>,
    pub cost: ReactiveSignal<String>,
    pub is_selling: ReactiveSignal<bool>,
    /// 六项初始个体值（0-255）。
    pub stat: ReactiveSignal<[String; 6]>,
    /// 六项初始努力值（0-3）。
    pub base: ReactiveSignal<[String; 6]>,
    /// 双属性（中文名，空串 = 无）。
    pub kind1: ReactiveSignal<String>,
    pub kind2: ReactiveSignal<String>,
    pub is_legendary: ReactiveSignal<bool>,
    /// 表单未覆盖字段的 JSON 原文（新建时给出默认骨架）。
    pub extra_json: ReactiveSignal<String>,
    pub saving: ReactiveSignal<bool>,
    pub err: ReactiveSignal<Option<String>>,
}

impl PokemonEditorState {
    pub fn new() -> Self {
        Self {
            open: use_signal(|| false),
            editing_id: use_signal(|| None),
            name: use_signal(String::new),
            description: use_signal(String::new),
            cost: use_signal(String::new),
            is_selling: use_signal(|| false),
            stat: use_signal(|| std::array::from_fn(|_| String::new())),
            base: use_signal(|| std::array::from_fn(|_| String::new())),
            kind1: use_signal(String::new),
            kind2: use_signal(String::new),
            is_legendary: use_signal(|| false),
            extra_json: use_signal(|| "{}".to_string()),
            saving: use_signal(|| false),
            err: use_signal(|| None),
        }
    }
}

impl std::fmt::Debug for PokemonEditorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PokemonEditorState").finish()
    }
}

/// 物品编辑弹窗状态（切片 9）。
///
/// 服务端 `item_type` 的载荷分四块：基本信息、tag（带附加值的枚举）、
/// limits、effects。effects 是 11 个数值，用定长数组承载，键序见
/// `ITEM_EFFECT_KEYS`。
///
/// 注意：Dioxus 版的「恢复技能 PP / 恢复 PP 数量」两个输入框在服务端
/// 没有对应列（`new_item_effects` 不读、`set_item_type` 不写），属于
/// 从不落库的死字段，这里不移植。
#[derive(Clone)]
pub struct ItemEditorState {
    pub open: ReactiveSignal<bool>,
    pub editing_id: ReactiveSignal<Option<u64>>,
    pub name: ReactiveSignal<String>,
    pub img_name: ReactiveSignal<String>,
    pub description: ReactiveSignal<String>,
    pub price: ReactiveSignal<String>,
    pub is_selling: ReactiveSignal<bool>,
    /// 类型：drug/ball/evolution/armor/special（服务端只认这五种；
    /// Dioxus 选项里的 enhance 没有 translator 分支，提交必报错）。
    pub tag_kind: ReactiveSignal<String>,
    /// 类型附加值：球/素材 ID、装备部位（head/necklace/weapon/armor）、
    /// 特殊物品标识文本；药品忽略。
    pub tag_value: ReactiveSignal<String>,
    pub min_level: ReactiveSignal<String>,
    /// 种族限制（中文名，空串 = 无）。
    pub kind_require: ReactiveSignal<String>,
    pub effects: ReactiveSignal<[String; 11]>,
    pub saving: ReactiveSignal<bool>,
    pub err: ReactiveSignal<Option<String>>,
}

impl ItemEditorState {
    pub fn new() -> Self {
        Self {
            open: use_signal(|| false),
            editing_id: use_signal(|| None),
            name: use_signal(String::new),
            img_name: use_signal(String::new),
            description: use_signal(String::new),
            price: use_signal(String::new),
            is_selling: use_signal(|| false),
            tag_kind: use_signal(|| "drug".to_string()),
            tag_value: use_signal(String::new),
            min_level: use_signal(String::new),
            kind_require: use_signal(String::new),
            effects: use_signal(|| std::array::from_fn(|_| "0".to_string())),
            saving: use_signal(|| false),
            err: use_signal(|| None),
        }
    }
}

impl std::fmt::Debug for ItemEditorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ItemEditorState").finish()
    }
}

/// 地图编辑弹窗状态（切片 10）。
///
/// 服务端载荷是扁平的：`{name, area_type, is_enabled, min_level, max_level,
/// min_quality, mode, experience, experience_increase_times, bosses}`，
/// `translate_map_mode_to_db_fields` 按 mode 决定写 experience=-1（boss）
/// 还是写倍率（wild），bosses 只在 boss/hybrid 下被读。
///
/// Boss 配置在 Dioxus 版是独立二级弹窗（宠物搜索 + 个体值 + 倍率），这里
/// 先用 JSON 文本框承载 `bosses` 数组：字段完整可编辑，但不提供选择器 UI。
#[derive(Clone)]
pub struct MapEditorState {
    pub open: ReactiveSignal<bool>,
    pub editing_id: ReactiveSignal<Option<u64>>,
    pub name: ReactiveSignal<String>,
    /// 地形单字符（l/g/p/…，服务端 translate_map_alpha_to_full_name 认的码）。
    pub area_type: ReactiveSignal<String>,
    pub is_enabled: ReactiveSignal<bool>,
    pub min_level: ReactiveSignal<String>,
    pub max_level: ReactiveSignal<String>,
    pub min_quality: ReactiveSignal<String>,
    /// wild / boss / hybrid
    pub mode: ReactiveSignal<String>,
    pub experience: ReactiveSignal<String>,
    pub experience_increase_times: ReactiveSignal<String>,
    /// bosses 数组的 JSON 原文（boss/hybrid 模式下参与提交）。
    pub bosses_json: ReactiveSignal<String>,
    pub saving: ReactiveSignal<bool>,
    pub err: ReactiveSignal<Option<String>>,
}

impl MapEditorState {
    pub fn new() -> Self {
        Self {
            open: use_signal(|| false),
            editing_id: use_signal(|| None),
            name: use_signal(String::new),
            area_type: use_signal(|| "l".to_string()),
            is_enabled: use_signal(|| true),
            min_level: use_signal(|| "1".to_string()),
            max_level: use_signal(|| "10".to_string()),
            min_quality: use_signal(|| "0".to_string()),
            mode: use_signal(|| "wild".to_string()),
            experience: use_signal(|| "0".to_string()),
            experience_increase_times: use_signal(|| "0".to_string()),
            bosses_json: use_signal(|| "[]".to_string()),
            saving: use_signal(|| false),
            err: use_signal(|| None),
        }
    }
}

impl std::fmt::Debug for MapEditorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapEditorState").finish()
    }
}

/// 技能编辑弹窗状态（切片 11）。
///
/// 服务端 `pm_skill` 只保存 category（物攻/特攻/其他）、element（中文属性）、
/// power 三元组，外加名称/描述/等级/次数/可学种族/effect_id。效果对象形态是
/// `{"physical_damage":[属性, 威力]}`，translator 只认这三种 category——
/// 治疗/状态等加值效果在库层面没有列，提交会直接被拒。
#[derive(Clone)]
pub struct SkillEditorState {
    pub open: ReactiveSignal<bool>,
    pub editing_id: ReactiveSignal<Option<u64>>,
    pub name: ReactiveSignal<String>,
    pub description: ReactiveSignal<String>,
    pub min_level_limit: ReactiveSignal<String>,
    pub use_times_limit: ReactiveSignal<String>,
    /// physical_damage / special_damage / others
    pub category: ReactiveSignal<String>,
    /// 中文属性名（空串 = 无，对应服务端 kind 翻译的 null 分支）。
    pub element: ReactiveSignal<String>,
    pub power: ReactiveSignal<String>,
    /// 可学种族 ID 列表（逗号分隔文本：服务端载荷是数字数组，
    /// 技能动辄关联上百个种族，纯文本串比逐个 chip 更实用）。
    pub available: ReactiveSignal<String>,
    /// 战斗引擎 2.0 效果模板 ID（0 = 不绑定）。
    pub effect_id: ReactiveSignal<String>,
    pub saving: ReactiveSignal<bool>,
    pub err: ReactiveSignal<Option<String>>,
}

impl SkillEditorState {
    pub fn new() -> Self {
        Self {
            open: use_signal(|| false),
            editing_id: use_signal(|| None),
            name: use_signal(String::new),
            description: use_signal(String::new),
            min_level_limit: use_signal(|| "0".to_string()),
            use_times_limit: use_signal(|| "0".to_string()),
            category: use_signal(|| "physical_damage".to_string()),
            element: use_signal(String::new),
            power: use_signal(|| "0".to_string()),
            available: use_signal(String::new),
            effect_id: use_signal(|| "0".to_string()),
            saving: use_signal(|| false),
            err: use_signal(|| None),
        }
    }
}

impl std::fmt::Debug for SkillEditorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SkillEditorState").finish()
    }
}

/// 进化链编辑弹窗状态（切片 12）。
///
/// 条件在载荷里是单键对象，服务端 translator 按哪个键在就取哪个（顺序是
/// min_level → use_item → min_intimacy → compare_attack_and_defense →
/// random → sex → is_bag_have_chairs），所以这里用「条件类型 + 值」两个
/// 控件表达，提交时再还原成单键对象。
#[derive(Clone)]
pub struct EvolutionEditorState {
    pub open: ReactiveSignal<bool>,
    pub editing_id: ReactiveSignal<Option<u64>>,
    pub source_id: ReactiveSignal<String>,
    pub target_id: ReactiveSignal<String>,
    pub priority: ReactiveSignal<String>,
    /// level / item / good / comp_atk_def / random / sex / chairs
    pub cond_kind: ReactiveSignal<String>,
    /// 条件值。语义随 cond_kind：等级数 / 道具 ID / 亲密度 /
    /// greater|less|equal / 概率小数 / male|female|unknown / true|false。
    pub cond_value: ReactiveSignal<String>,
    pub saving: ReactiveSignal<bool>,
    pub err: ReactiveSignal<Option<String>>,
}

impl EvolutionEditorState {
    pub fn new() -> Self {
        Self {
            open: use_signal(|| false),
            editing_id: use_signal(|| None),
            source_id: use_signal(String::new),
            target_id: use_signal(String::new),
            priority: use_signal(|| "0".to_string()),
            cond_kind: use_signal(|| "level".to_string()),
            cond_value: use_signal(|| "0".to_string()),
            saving: use_signal(|| false),
            err: use_signal(|| None),
        }
    }
}

impl std::fmt::Debug for EvolutionEditorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EvolutionEditorState").finish()
    }
}

/// 用户发放弹窗状态（切片 13）。
///
/// 对应 Dioxus 版的 GrantPokemonModal / GrantItemModal 两个两步向导。这里
/// 合成一个弹窗、用 `mode` 区分：两边的第一步都是「按名称搜类型 → 选中」，
/// 第二步才分叉（宠物是属性配置，物品只是数量）。
///
/// 发放宠物走 `insert::pokemon_info`，载荷字段由服务端硬校验（type_id 必须
/// 存在、skills 数组、site/status/sex 是固定串），个体值三档模式与 Dioxus
/// 一致：随机 0-31 / 按品质档位随机 / 手动指定。
#[derive(Clone)]
pub struct GrantPokeDraft {
    pub type_id: Option<u64>,
    pub type_name: String,
    pub name: String,
    pub level: String,
    /// header / bag / store
    pub site: String,
    /// male / female / unknown
    pub sex: String,
    pub is_shiny: bool,
    pub intimacy: String,
    /// random / quality / manual
    pub iv_mode: String,
    /// 1-8（E..SS+）
    pub iv_quality: String,
    pub ivs: [String; 6],
    /// 手动模式下的努力值（0-3）。Dioxus 版固定为 0，这里保留可编辑。
    pub base_points: [String; 6],
}

impl GrantPokeDraft {
    pub fn blank() -> Self {
        Self {
            type_id: None,
            type_name: String::new(),
            name: String::new(),
            level: "1".to_string(),
            site: "store".to_string(),
            sex: "unknown".to_string(),
            is_shiny: false,
            intimacy: "50".to_string(),
            iv_mode: "random".to_string(),
            iv_quality: "5".to_string(),
            ivs: std::array::from_fn(|_| "0".to_string()),
            base_points: std::array::from_fn(|_| "0".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct GrantState {
    pub open: ReactiveSignal<bool>,
    /// pokemon / item
    pub mode: ReactiveSignal<String>,
    /// 被发放的用户 UID（从用户行按钮带上来的）
    pub owner_uid: ReactiveSignal<u64>,
    /// 1 = 选择类型，2 = 配置
    pub step: ReactiveSignal<u8>,
    // 第一步：搜索
    pub query: ReactiveSignal<String>,
    pub results: ReactiveSignal<Vec<(u64, String)>>,
    pub searching: ReactiveSignal<bool>,
    pub picked_id: ReactiveSignal<Option<u64>>,
    pub picked_name: ReactiveSignal<String>,
    // 第二步（宠物）
    pub poke: ReactiveSignal<GrantPokeDraft>,
    // 第二步（物品）
    pub item_count: ReactiveSignal<String>,
    pub saving: ReactiveSignal<bool>,
    pub err: ReactiveSignal<Option<String>>,
}

impl GrantState {
    pub fn new() -> Self {
        Self {
            open: use_signal(|| false),
            mode: use_signal(|| "pokemon".to_string()),
            owner_uid: use_signal(|| 0u64),
            step: use_signal(|| 1u8),
            query: use_signal(String::new),
            results: use_signal(Vec::new),
            searching: use_signal(|| false),
            picked_id: use_signal(|| None),
            picked_name: use_signal(String::new),
            poke: use_signal(GrantPokeDraft::blank),
            item_count: use_signal(|| "1".to_string()),
            saving: use_signal(|| false),
            err: use_signal(|| None),
        }
    }
}

impl std::fmt::Debug for GrantState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GrantState").finish()
    }
}

/// 行删除确认弹窗状态（切片 15）。
///
/// 六个数据实体（宝可梦/物品/地图/进化链/技能/效果）共用一个确认弹窗：
/// 行按钮只带 `page:id`，实体名与行名在打开时从列表状态里解析，确认后
/// 按 page 键路由到对应实体的 `delete::` 端点并整表刷新。
/// 用户数据页没有行删除（Dioxus 版同样没有 delete::user_info 调用）。
#[derive(Clone)]
pub struct DeleteState {
    pub open: ReactiveSignal<bool>,
    /// 页键：pk / it / mp / ev / sk / ef。
    pub page: ReactiveSignal<String>,
    pub id: ReactiveSignal<Option<u64>>,
    /// 确认弹窗里展示的行名（解析不到时退化为 #id）。
    pub label: ReactiveSignal<String>,
    pub busy: ReactiveSignal<bool>,
    pub err: ReactiveSignal<Option<String>>,
}

impl DeleteState {
    pub fn new() -> Self {
        Self {
            open: use_signal(|| false),
            page: use_signal(String::new),
            id: use_signal(|| None),
            label: use_signal(String::new),
            busy: use_signal(|| false),
            err: use_signal(|| None),
        }
    }
}

impl std::fmt::Debug for DeleteState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteState").finish()
    }
}

/// 效果数据编辑弹窗状态（切片 15）。
///
/// 服务端载荷：`{id, code, kind, hooks, params, description, version}`。
/// `hooks` 必须是非空数组（触发时机），`params` 必须是含 `code` 键的对象
/// （核心效果码）——两者都以 JSON 文本框承载，校验交给服务端的
/// `build_effect_declaration`，前端只拦 JSON 解析失败与空 code。
#[derive(Clone)]
pub struct EffectEditorState {
    pub open: ReactiveSignal<bool>,
    pub editing_id: ReactiveSignal<Option<u64>>,
    pub code: ReactiveSignal<String>,
    /// move / ability / item / weather / field / status
    pub kind: ReactiveSignal<String>,
    pub hooks_json: ReactiveSignal<String>,
    pub params_json: ReactiveSignal<String>,
    pub description: ReactiveSignal<String>,
    pub version: ReactiveSignal<String>,
    pub saving: ReactiveSignal<bool>,
    pub err: ReactiveSignal<Option<String>>,
}

impl EffectEditorState {
    pub fn new() -> Self {
        Self {
            open: use_signal(|| false),
            editing_id: use_signal(|| None),
            code: use_signal(String::new),
            kind: use_signal(|| "move".to_string()),
            hooks_json: use_signal(|| "[]".to_string()),
            params_json: use_signal(|| "{\n  \"code\": \"\"\n}".to_string()),
            description: use_signal(String::new),
            version: use_signal(|| "1".to_string()),
            saving: use_signal(|| false),
            err: use_signal(|| None),
        }
    }
}

impl std::fmt::Debug for EffectEditorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EffectEditorState").finish()
    }
}

/// 应用级状态。
#[derive(Clone)]
pub struct AppState {
    pub component_id: std::cell::Cell<u32>,
    pub route: ReactiveSignal<AdminRoute>,
    pub config: GlobalConfigState,
    pub pokemon: ListState,
    pub item: ListState,
    pub map: ListState,
    pub user: ListState,
    pub evolution: ListState,
    pub skill: ListState,
    pub effect: ListState,
    pub pokemon_editor: PokemonEditorState,
    pub item_editor: ItemEditorState,
    pub map_editor: MapEditorState,
    pub skill_editor: SkillEditorState,
    pub evolution_editor: EvolutionEditorState,
    pub grant: GrantState,
    pub config_form: ConfigFormState,
    pub delete: DeleteState,
    pub effect_editor: EffectEditorState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            route: use_signal(|| AdminRoute::GlobalConfig),
            config: GlobalConfigState::new(),
            pokemon: ListState::new(),
            item: ListState::new(),
            map: ListState::new(),
            user: ListState::new(),
            evolution: ListState::new(),
            skill: ListState::new(),
            effect: ListState::new(),
            pokemon_editor: PokemonEditorState::new(),
            item_editor: ItemEditorState::new(),
            map_editor: MapEditorState::new(),
            skill_editor: SkillEditorState::new(),
            evolution_editor: EvolutionEditorState::new(),
            grant: GrantState::new(),
            config_form: ConfigFormState::new(),
            delete: DeleteState::new(),
            effect_editor: EffectEditorState::new(),
        }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}
