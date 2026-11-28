//! 弹出菜单（旧版 `PopupProvider` / `PopupMenu` / `PopupMenuItem`）：面板与背板
//! 由 hikari `Popover` 的锚定分支承接，菜单项是 hikari `Button` 带旧版类名。
//!
//! 旧版的流程：卡片在鼠标处 `open_at_mouse(x, y + 10, content)` → 应用根部挂出
//! `div.popup-overlay`（点哪儿都关）> `div.popup-container`（点里面不关）>
//! `div.popup-menu` > `button.popup-menu-item`。端口是同一套：
//!
//! - **面板** = `Popover`（`BottomStart`、偏移 0、零尺寸锚点）：面板左上角正好
//!   落在 [`crate::popup_types::position_at_mouse`] 给出的 `(x, y)`。`class` 给
//!   `popup-container`，外观由 game.css 那条旧规则定；皮肤只把 hikari 面板表的
//!   `width: max-content` / `max-height` 退回去（`styles/main.scss` 的弹出菜单段）。
//! - **背板** = Popover 自带的 `div.hk-popover-backdrop`。hikari 的锚定分支里它
//!   是 `pointer-events: none` 的纯装饰：Vue 版靠 document 级捕获监听关面板，
//!   Rust 版没有这条缝（`feedback/popover.rs` 头注释：the outside-click shield
//!   … without a tairitsu analogue in this wave），并且写明「host's own dismissal
//!   paths are the close channels」。所以皮肤把背板改成可点、层叠 999：点下去
//!   冒泡到应用根，由根上的 `onclick` 调 [`dismiss`]——旧版遮罩的同一句话。
//! - **菜单项**点了先关菜单再做事（旧版每个 `PopupMenuItem` 的 `onclick` 都以
//!   `popup.close()` 打头），禁用项什么都不做。
//!
//! 菜单内容是打开那一刻定格的数据（[`crate::popup_types`]），不是节点：tairitsu
//! 没有 Dioxus 那种把 `Element` 存进全局再交给 Provider 渲染的写法，而定格的
//! 值正是旧版闭包捕获到的值，语义相同。

use hikari_components::{
    Button, ButtonProps, Popover, PopoverAnchorRect, PopoverPlacement, PopoverProps,
};
use tairitsu_macros::rsx;
use tairitsu_vdom::{EventHandler, MouseEvent, VNode};
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::window::{
    get_inner_height, get_inner_width,
};

use crate::app_state::AppState;
use crate::popup_types::{position_at_mouse, OpenPopup, PopupAction, PopupItem};

/// 在鼠标处打开菜单。对齐旧版 `popup.open_at_mouse(x, y + 10, content)`：
/// 加 10 的是调用处，这里拿到的就是旧版函数的入参。
pub fn open_at_mouse(state: &AppState, items: Vec<PopupItem>, client_x: i32, client_y: i32) {
    let (x, y) = position_at_mouse(
        client_x as f64,
        client_y as f64,
        viewport_width(),
        viewport_height(),
    );
    state.popup.inner().set(Some(OpenPopup { items, x, y }));
    crate::request_remount();
}

/// 关菜单；没开着就什么都不做。
pub fn close(state: &AppState) {
    if state.popup.get().is_some() {
        state.popup.inner().set(None);
        crate::request_remount();
    }
}

/// 应用根上的 `onclick`：有菜单开着就关。背板挡住了页面，所以能冒泡到根的
/// 点击来自背板；菜单本体自己拦了冒泡（皮肤给它铺满面板内边距的命中区，所以
/// 点面板的内边距也不关，同旧版）。
pub fn dismiss(state: &AppState) {
    close(state);
}

// 旧版 `window.inner_width()` 的兜底值（`unwrap_or(375.0)` / `unwrap_or(700.0)`）。
fn viewport_width() -> f64 {
    match get_inner_width() {
        w if w > 0 => w as f64,
        _ => 375.0,
    }
}

fn viewport_height() -> f64 {
    match get_inner_height() {
        h if h > 0 => h as f64,
        _ => 700.0,
    }
}

/// 菜单项的 `onclick`：先关菜单；禁用项到此为止。
fn run(state: &AppState, item: &PopupItem) {
    close(state);
    if item.disabled {
        return;
    }
    match &item.action {
        PopupAction::ForgetSkill { skill_type_id } => {
            crate::my_pokemon::forget_skill(state, *skill_type_id)
        }
        PopupAction::SetFirst { id } => crate::pokemon_storage::set_first(state, *id),
        PopupAction::MoveToSite {
            id,
            site,
            err_prefix,
        } => crate::pokemon_storage::move_to_site(state, *id, *site, err_prefix),
        PopupAction::Release { id } => crate::pokemon_storage::release_one(state, *id),
    }
}

/// 回归脚本找菜单项用的钩子（旧版行内按钮上那几个，名字和取值不变）。
fn test_hook(action: &PopupAction) -> Option<(&'static str, String)> {
    match action {
        PopupAction::ForgetSkill { skill_type_id } => {
            Some(("data-test-skill-forget", skill_type_id.to_string()))
        }
        PopupAction::MoveToSite { id, site: 2, .. } => {
            Some(("data-test-st-to-bag", id.to_string()))
        }
        PopupAction::Release { id } => Some(("data-test-st-release", id.to_string())),
        _ => None,
    }
}

/// 应用根部常驻的一席：有菜单开着才出节点（旧版 `if is_open { div.popup-overlay … }`），
/// 没开着是 `None`——根下不多挂空节点。要挂在应用根 `div.app-layout` 里面——背板上
/// 的点击靠冒泡到根才会关菜单。
pub fn provider(state: &AppState) -> Option<VNode> {
    let open = state.popup.get()?;

    let mut rows: Vec<VNode> = Vec::with_capacity(open.items.len());
    for item in &open.items {
        let row_state = state.clone();
        let row = item.clone();
        let label = item.label.clone();
        let hook = test_hook(&item.action);
        rows.push(crate::button::with_attrs(
            tairitsu_macros::rsx! { Button {
            class: (item.class()).to_string(),
            disabled: item.disabled,
            onclick: Some(EventHandler::new(move |_: MouseEvent| run(&row_state, &row))),
            "{label}"
                       } },
            move |el| {
                if let Some((name, value)) = hook {
                    el.attributes.insert(name.to_string(), value);
                }
            },
        ));
    }

    let close_state = state.clone();
    Some(Popover(PopoverProps {
        open: true,
        placement: PopoverPlacement::BottomStart,
        offset: 0.0,
        // 背板与外部点击关闭都留着（默认值）：背板是上面说的点击挡板；
        // `close_on_backdrop` 在锚定分支只是声明，hikari 这一波没有接监听。
        backdrop: true,
        close_on_backdrop: true,
        // 外观全由 game.css 的 `.popup-container` 给，不要 hikari 的玻璃面板表。
        glass: false,
        anchor: Some(PopoverAnchorRect {
            x: open.x,
            y: open.y,
            width: 0.0,
            height: 0.0,
        }),
        class: "popup-container".to_string(),
        children: rsx! {
            div { class: "popup-menu",
                // 旧版把「点里面不关」写在容器上；hikari 的面板没有事件槽，
                // 落在它的直接子节点上，效果一样。
                onclick: move |e: MouseEvent| e.stop_propagation(),
                for i in 0..rows.len() {
                    ..vec![rows[i].clone()]
                }
            }
        },
        // 面板里按 Esc（hikari 面板自带；旧版没有这条）。
        on_open_change: Some(EventHandler::new(move |open: bool| {
            if !open {
                close(&close_state);
            }
        })),
        ..Default::default()
    }))
}
