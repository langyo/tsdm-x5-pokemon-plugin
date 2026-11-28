//! 弹出菜单与装备提示的数据与定位（纯逻辑，host 可测）。两者都由 hikari
//! `Popover` 的锚定分支承接（`crate::popup` / `crate::equip_tooltip`）。
//!
//! 与 Dioxus 版 `state/ui.rs` 的 `open_popup_at_mouse`、`pages/my_pokemon.rs`
//! 的技能卡菜单、`pages/pokemon_storage.rs` 的 `PokemonSlot` 菜单逐项对齐。
//! 菜单项在打开的那一刻定格（文案、配色、禁用态和点了要做的事），之后页面
//! 数据怎么变都不影响已经打开的菜单——Dioxus 版 `PopupMenuItem` 的闭包捕获的
//! 就是点击那一刻的值。

/// 菜单项点了之后要做的事。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PopupAction {
    /// 遗忘当前宠物的某个技能（`skill_type_id`）。
    ForgetSkill { skill_type_id: u64 },
    /// 把宠物设为首位。
    SetFirst { id: u64 },
    /// 把宠物移到某个位置（2 = 背包，3 = 仓库）；失败提示以 `err_prefix` 开头。
    MoveToSite {
        id: u64,
        site: u8,
        err_prefix: &'static str,
    },
    /// 放生。
    Release { id: u64 },
}

/// 一条菜单项（Dioxus `PopupMenuItem` 的四个 props + 它的 `onclick`）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PopupItem {
    pub label: String,
    pub danger: bool,
    pub primary: bool,
    pub disabled: bool,
    pub action: PopupAction,
}

impl PopupItem {
    /// Dioxus `PopupMenuItem` 的类：danger 优先于 primary。
    pub fn class(&self) -> &'static str {
        match (self.danger, self.primary) {
            (true, _) => "popup-menu-item danger",
            (_, true) => "popup-menu-item primary",
            _ => "popup-menu-item",
        }
    }
}

/// 打开中的弹出菜单：菜单项 + 面板左上角（视口坐标）。
#[derive(Clone, Debug, PartialEq)]
pub struct OpenPopup {
    pub items: Vec<PopupItem>,
    pub x: f64,
    pub y: f64,
}

/// Dioxus 版 `open_popup_at_mouse` 的定位：按 120 × 80 的估计尺寸避让视口的
/// 右缘和下缘。调用处传进来的 `client_y` 已经加了 10（两处调用都是
/// `client_coordinates().y + 10.0`）。
pub fn position_at_mouse(
    client_x: f64,
    client_y: f64,
    win_width: f64,
    win_height: f64,
) -> (f64, f64) {
    const MENU_WIDTH: f64 = 120.0;
    const MENU_HEIGHT: f64 = 80.0;

    let x = if client_x + MENU_WIDTH > win_width {
        (client_x - MENU_WIDTH - 10.0).max(10.0)
    } else {
        (client_x - 40.0).max(10.0)
    };
    let y = if client_y + MENU_HEIGHT > win_height {
        (client_y - MENU_HEIGHT - 10.0).max(10.0)
    } else {
        (client_y + 10.0).max(10.0)
    };
    (x, y)
}

/// 旧版 `EquipmentTooltipInline` 的定位：提示的最小宽度是 180，横向把指针所在点
/// 当作一张 48 宽卡片的中心来对中，顶边落在指针下方 8px。`(x, y)` 是指针进入
/// 卡片那一刻的视口坐标。
pub fn equipment_tooltip_position(x: f64, y: f64) -> (f64, f64) {
    const TOOLTIP_WIDTH: f64 = 180.0;
    const CARD_WIDTH: f64 = 48.0;
    (x + (CARD_WIDTH - TOOLTIP_WIDTH) / 2.0, y + 8.0)
}

/// 技能卡的菜单：只有「遗忘此技能」，技能动作进行中禁用。
pub fn skill_menu(skill_type_id: u64, skill_loading: bool) -> Vec<PopupItem> {
    vec![PopupItem {
        label: "遗忘此技能".to_string(),
        danger: true,
        primary: false,
        disabled: skill_loading,
        action: PopupAction::ForgetSkill { skill_type_id },
    }]
}

/// 仓库页卡片菜单的依据（都是打开那一刻的值）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotMenuFacts {
    pub site: u8,
    pub state: u8,
    pub hp: i64,
    /// 用户此刻在战斗里。
    pub user_in_battle: bool,
    /// 背包里的宠物数（site 1 / 2）。
    pub bag_count: usize,
}

/// 仓库页卡片（`PokemonSlot`）的菜单：携带区 → [设为首位] + 放入仓库；
/// 仓库区 → 放入背包 + 放生。方括号里的「设为首位」只有非首位才有。
pub fn slot_menu(facts: &SlotMenuFacts, id: u64) -> Vec<PopupItem> {
    let is_first = facts.site == 1;
    let is_bag = facts.site == 1 || facts.site == 2;
    // 只有首位宠物会出场：其余宠物即使在战斗中也照常操作。
    let in_battle = facts.site == 1 && facts.user_in_battle;

    if is_bag {
        let mut items = Vec::new();
        if !is_first {
            items.push(PopupItem {
                label: "设为首位".to_string(),
                danger: false,
                primary: true,
                disabled: in_battle,
                action: PopupAction::SetFirst { id },
            });
        }
        items.push(PopupItem {
            label: "放入仓库".to_string(),
            danger: false,
            primary: false,
            disabled: in_battle,
            action: PopupAction::MoveToSite {
                id,
                site: 3,
                err_prefix: "放入仓库",
            },
        });
        items
    } else {
        let bag_full = facts.bag_count >= 6;
        let cannot_release = facts.state == 0 || facts.hp <= 0 || in_battle;
        vec![
            PopupItem {
                label: if bag_full {
                    "背包已满（6/6）"
                } else {
                    "放入背包"
                }
                .to_string(),
                danger: false,
                primary: false,
                disabled: bag_full,
                action: PopupAction::MoveToSite {
                    id,
                    site: 2,
                    err_prefix: "放入背包",
                },
            },
            PopupItem {
                label: "放生".to_string(),
                danger: true,
                primary: false,
                disabled: cannot_release,
                action: PopupAction::Release { id },
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 旧版 open_popup_at_mouse 的四个分支（视口 1280 × 900）。
    #[test]
    fn popup_position_follows_the_dioxus_quadrants() {
        // 视口中间：左移 40、下移 10（入参已经是 client_y + 10）。
        assert_eq!(
            position_at_mouse(300.0, 210.0, 1280.0, 900.0),
            (260.0, 220.0)
        );
        // 贴右缘：面板右沿收到点击点左边 10px。
        assert_eq!(
            position_at_mouse(1200.0, 210.0, 1280.0, 900.0),
            (1070.0, 220.0)
        );
        // 贴下缘：面板底沿收到点击点上方。
        assert_eq!(
            position_at_mouse(300.0, 830.0, 1280.0, 900.0),
            (260.0, 740.0)
        );
        // 贴左缘 / 上缘：都夹到 10。
        assert_eq!(position_at_mouse(0.0, 0.0, 1280.0, 900.0), (10.0, 10.0));
    }

    /// 旧版 EquipmentTooltipInline：`left = x + (48 - 180) / 2`、`top = y + 8`。
    #[test]
    fn equipment_tooltip_sits_centered_under_the_pointer() {
        assert_eq!(equipment_tooltip_position(400.0, 300.0), (334.0, 308.0));
        assert_eq!(equipment_tooltip_position(100.5, 50.0), (34.5, 58.0));
    }

    #[test]
    fn skill_menu_is_one_danger_item() {
        let items = skill_menu(15, false);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].class(), "popup-menu-item danger");
        assert_eq!(items[0].label, "遗忘此技能");
        assert!(!items[0].disabled);
        assert_eq!(
            items[0].action,
            PopupAction::ForgetSkill { skill_type_id: 15 }
        );
        assert!(skill_menu(15, true)[0].disabled);
    }

    #[test]
    fn slot_menu_in_bag_offers_first_and_storage() {
        // 首位：没有「设为首位」。
        let first = slot_menu(
            &SlotMenuFacts {
                site: 1,
                state: 1,
                hp: 50,
                user_in_battle: false,
                bag_count: 2,
            },
            7,
        );
        assert_eq!(first.len(), 1);
        assert_eq!(first[0].label, "放入仓库");
        assert_eq!(first[0].class(), "popup-menu-item");

        // 队伍位：primary 的「设为首位」在前。
        let party = slot_menu(
            &SlotMenuFacts {
                site: 2,
                state: 1,
                hp: 50,
                user_in_battle: false,
                bag_count: 2,
            },
            8,
        );
        assert_eq!(party.len(), 2);
        assert_eq!(party[0].label, "设为首位");
        assert_eq!(party[0].class(), "popup-menu-item primary");
        assert_eq!(
            party[1].action,
            PopupAction::MoveToSite {
                id: 8,
                site: 3,
                err_prefix: "放入仓库"
            }
        );
    }

    #[test]
    fn slot_menu_disables_the_first_pokemon_while_in_battle() {
        // 战斗中只有首位宠物被锁：队伍位照常。
        let facts = |site| SlotMenuFacts {
            site,
            state: 1,
            hp: 50,
            user_in_battle: true,
            bag_count: 2,
        };
        assert!(slot_menu(&facts(1), 1)[0].disabled);
        assert!(!slot_menu(&facts(2), 2)[0].disabled);
    }

    #[test]
    fn slot_menu_in_storage_gates_to_bag_and_release() {
        let facts = |bag_count, state, hp| SlotMenuFacts {
            site: 3,
            state,
            hp,
            user_in_battle: false,
            bag_count,
        };
        let open = slot_menu(&facts(2, 1, 50), 9);
        assert_eq!(open.len(), 2);
        assert_eq!(open[0].label, "放入背包");
        assert!(!open[0].disabled);
        assert_eq!(open[1].label, "放生");
        assert_eq!(open[1].class(), "popup-menu-item danger");
        assert!(!open[1].disabled);

        // 背包满：文案换成「背包已满（6/6）」并禁用。
        let full = slot_menu(&facts(6, 1, 50), 9);
        assert_eq!(full[0].label, "背包已满（6/6）");
        assert!(full[0].disabled);

        // 濒危（state 0）或 HP 归零：不能放生。
        assert!(slot_menu(&facts(2, 0, 50), 9)[1].disabled);
        assert!(slot_menu(&facts(2, 1, 0), 9)[1].disabled);
    }

    #[test]
    fn danger_wins_over_primary() {
        let item = PopupItem {
            label: String::new(),
            danger: true,
            primary: true,
            disabled: false,
            action: PopupAction::Release { id: 0 },
        };
        assert_eq!(item.class(), "popup-menu-item danger");
    }
}
