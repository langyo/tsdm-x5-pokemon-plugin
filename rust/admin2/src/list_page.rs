//! 七张数据列表页共用的页面壳：hikari `AdminTablePage`。
//!
//! 七张列表页（宠物 / 物品 / 地图 / 用户 / 进化 / 技能 / 效果）只有「表格列」
//! 与「每行能做什么」不同，其余全在这里：
//!
//! ```text
//! AdminTablePage
//!   ├─ 页头     标题 + 已加载条数，右侧「刷新」「新增」
//!   ├─ 筛选栏   hikari Input × 2~4 + 筛选 / 重置
//!   ├─ 表格     hikari Table（文本单元格 + 「管理」列）
//!   └─ 加载态 / 失败态 / 空态   hikari Spinner / Alert / EmptyState
//! ```
//!
//! 行操作走表格的「管理」列：hikari 的 Rust 表格只承载文本单元格，每行
//! 末尾的 ⋯ 触发钮发出「哪一行要管理」（`on_manage`），菜单本身由宿主渲染
//! ——见 [`row_menu_overlay`]，内容是各表在 `pages.rs` 里给出的 [`HkAction`]。

use std::rc::Rc;

use hikari_components::{
    AdminTablePage, AdminTablePageProps, ButtonSize, ButtonVariant, Input, InputAlign, InputProps,
    InputSize, TableColumn, TableManageMode, TableRow,
};
use tairitsu_macros::rsx;
use tairitsu_vdom::{EventHandler, VNode};

use crate::app_state::AppState;
use crate::hk::{self, HkAction};
use crate::pages::{apply_list_filters_pub, ensure_loaded, load_more_list_pub, reset_list_filters_pub};
use crate::state::{EntitySpec, ListState, RowMenu};

/// 清空并重新拉取一张列表（「刷新」与错误态的重试共用）。
fn reload(st: &AppState, ls: &ListState) {
    ls.loaded.inner().set(None);
    ls.rows.inner().set(Vec::new());
    crate::request_remount();
    ensure_loaded(st);
}

/// 一张列表页的可变部分。
pub struct TableShape<'a> {
    /// 表头文案（不含操作列——操作走表格的「管理」列）。
    pub columns: &'static [&'static str],
    /// 行主键：表格的行身份，也是 `on_manage` 回传的键。
    pub key: Box<dyn Fn(&serde_json::Value) -> String + 'a>,
    /// 一行的文本单元格。
    pub cells: Box<dyn Fn(&serde_json::Value) -> Vec<String> + 'a>,
    /// 一行可做的操作，按行主键给出。
    pub actions: Box<dyn Fn(&AppState, &str) -> Vec<HkAction> + 'a>,
}

/// 筛选输入框。输入只写状态、不请求重挂（重挂会打断输入）；回车或点
/// 「筛选」时才由那边的动作触发重挂。
fn filter_input(
    ls: &ListState,
    id: String,
    placeholder: String,
    value: String,
    set: fn(&ListState, String),
    on_enter: Rc<dyn Fn()>,
) -> VNode {
    let ls_input = ls.clone();
    let enter = on_enter;
    Input(InputProps {
        id: Some(id),
        model_value: value,
        placeholder: Some(placeholder),
        size: InputSize::Small,
        align: InputAlign::Start,
        oninput: Some(EventHandler::new(move |text: String| {
            set(&ls_input, text);
        })),
        submit_on_enter: Some(EventHandler::new(move |_event: ()| enter())),
        ..Default::default()
    })
}

/// 「管理」列的 ⋯ 被按下：量一下触发钮的位置，记下是哪一行，重挂后由
/// [`row_menu_overlay`] 把菜单贴着它画出来。
fn open_row_menu(st: &AppState, keys: Vec<String>, row_key: String) {
    let index = keys.iter().position(|key| *key == row_key);
    let anchor = index.and_then(|index| {
        hk::measure(&format!(
            ".adm-list-page table tbody tr:nth-child({}) .hk-table-manage-cell button",
            index + 1
        ))
    });
    match anchor {
        Some(anchor) => st
            .row_menu
            .inner()
            .set(Some(RowMenu { key: row_key, anchor })),
        None => st.row_menu.inner().set(None),
    }
    crate::request_remount();
}

/// 行操作菜单 + 它的透明遮罩（点到别处即收起）。`actions` 是当前页为该行
/// 给出的操作；点其中一项时先收起菜单再执行。
pub fn row_menu_overlay(st: &AppState, actions: Vec<HkAction>) -> VNode {
    let Some(menu) = st.row_menu.get() else {
        return VNode::empty();
    };
    let close_state = st.clone();
    let actions: Vec<HkAction> = actions
        .into_iter()
        .map(|action| {
            let run = action.run.clone();
            let pick_state = st.clone();
            HkAction {
                label: action.label,
                variant: action.variant,
                disabled: action.disabled,
                icon: action.icon,
                run: Rc::new(move || {
                    pick_state.row_menu.inner().set(None);
                    crate::request_remount();
                    run();
                }),
            }
        })
        .collect();
    let menu = hk::anchored_menu(menu.anchor, actions, "admin-row-menu");
    rsx! {
        div {
            class: "admin-row-menu-layer",
            div {
                class: "admin-menu-backdrop admin-row-menu-backdrop",
                "data-test-adm-row-menu-backdrop": "1",
                onclick: move |_| {
                    close_state.row_menu.inner().set(None);
                    crate::request_remount();
                },
            }
            ..vec![menu]
        }
    }
}

/// 渲染一张数据列表页。
///
/// `key` 是页面键（"pk" / "it" / …），只用于筛选项的元素 id。
pub fn entity_list_page(
    st: &AppState,
    spec: EntitySpec,
    ls: &ListState,
    key: &str,
    shape: &TableShape<'_>,
    on_new: Option<Rc<dyn Fn()>>,
) -> VNode {
    let load_state = ls.loaded.get();
    let busy = ls.busy.get();
    let filtered = ls.filtered.get();
    let err = ls.err.get();
    let rows = ls.rows.get();
    let total = ls.total.get();
    let loaded_count = ls.loaded_count.get();
    let has_more = !filtered && loaded_count < total;
    let count_text = if filtered {
        format!("筛选结果 {loaded_count} 条")
    } else {
        format!("已加载 {loaded_count} / {total} 条")
    };

    let (loading, error) = match &load_state {
        None => (true, None),
        Some(Err(e)) => (false, Some(format!("加载失败：{e}"))),
        Some(Ok(())) => (false, None),
    };
    // 筛选失败时行还在屏上：一条横幅，不撤走用户已经拿到的数据。
    let stale_error = match (&load_state, &err) {
        (Some(Ok(())), Some(e)) => Some(format!("筛选失败：{e}")),
        _ => None,
    };

    // ---- 表格 ----
    let keys: Vec<String> = rows.iter().map(|item| (shape.key)(item)).collect();
    let table_rows: Vec<TableRow> = rows
        .iter()
        .map(|item| TableRow {
            key: Some((shape.key)(item)),
            cells: (shape.cells)(item),
            manage: TableManageMode::On,
        })
        .collect();
    let columns: Vec<TableColumn> = shape
        .columns
        .iter()
        .enumerate()
        .map(|(index, title)| TableColumn {
            key: format!("c{index}"),
            title: title.to_string(),
            width: None,
            sortable: false,
            align: None,
            copyable: false,
        })
        .collect();

    // ---- 页头右侧：刷新 + 新增 ----
    let refresh_state = st.clone();
    let refresh_ls = ls.clone();
    let mut header_actions: Vec<VNode> = vec![hk::button(
        "刷新",
        ButtonVariant::Secondary,
        ButtonSize::Sm,
        busy,
        move || reload(&refresh_state, &refresh_ls),
    )];
    if let Some(on_new) = on_new {
        header_actions.push(hk::button(
            "新增",
            ButtonVariant::Primary,
            ButtonSize::Sm,
            busy,
            move || on_new(),
        ));
    }

    // ---- 筛选栏 ----
    let apply: Rc<dyn Fn()> = {
        let apply_spec = spec;
        let apply_state = st.clone();
        let apply_ls = ls.clone();
        Rc::new(move || apply_list_filters_pub(apply_spec, &apply_state, &apply_ls))
    };
    let filter_a = match spec.a {
        Some(f) => filter_input(
            ls,
            format!("adm-f-a-{key}"),
            f.placeholder.to_string(),
            ls.f_a.get(),
            |ls, v| ls.f_a.inner().set(v),
            apply.clone(),
        ),
        None => VNode::empty(),
    };
    let filter_b = match spec.b {
        Some(f) => filter_input(
            ls,
            format!("adm-f-b-{key}"),
            f.placeholder.to_string(),
            ls.f_b.get(),
            |ls, v| ls.f_b.inner().set(v),
            apply.clone(),
        ),
        None => VNode::empty(),
    };
    let mut inputs: Vec<VNode> = vec![
        filter_input(
            ls,
            format!("adm-f-id-{key}"),
            format!("{}（精确）", spec.id_tag),
            ls.f_id.get(),
            |ls, v| ls.f_id.inner().set(v),
            apply.clone(),
        ),
        filter_input(
            ls,
            format!("adm-f-name-{key}"),
            spec.name_placeholder.to_string(),
            ls.f_name.get(),
            |ls, v| ls.f_name.inner().set(v),
            apply.clone(),
        ),
    ];
    inputs.extend([filter_a, filter_b]);
    let reset_spec = spec;
    let reset_ls = ls.clone();
    let apply_button = hk::button(
        "筛选",
        ButtonVariant::Primary,
        ButtonSize::Sm,
        busy,
        move || apply(),
    );
    let reset_button = hk::button(
        "重置",
        ButtonVariant::Secondary,
        ButtonSize::Sm,
        busy,
        move || reset_list_filters_pub(reset_spec, &reset_ls),
    );
    inputs.extend([apply_button, reset_button]);
    let filter = rsx! {
        div { class: "adm-filter", ..inputs }
    };

    // ---- 行操作菜单的入口 ----
    let menu_state = st.clone();
    let menu_keys = keys.clone();
    let on_manage = EventHandler::new(move |row_key: String| {
        open_row_menu(&menu_state, menu_keys.clone(), row_key);
    });

    let page = AdminTablePage(AdminTablePageProps {
        title: spec.title.to_string(),
        subtitle: Some(count_text),
        loading,
        error,
        stale_error,
        rows: table_rows,
        columns,
        manageable: true,
        actions: Some(rsx! {
            div { class: "hk-button-row", ..header_actions }
        }),
        filter: Some(filter),
        empty_title: format!("暂无{}数据", spec.title),
        on_manage: Some(on_manage),
        ..Default::default()
    });

    let more = if has_more {
        let label = format!("加载更多（已加载 {loaded_count}/{total}）");
        let more_spec = spec;
        let more_ls = ls.clone();
        rsx! {
            div { class: "adm-load-more",
                {hk::button(
                    label.as_str(),
                    ButtonVariant::Secondary,
                    ButtonSize::Sm,
                    busy,
                    move || load_more_list_pub(more_spec, &more_ls),
                )}
            }
        }
    } else {
        VNode::empty()
    };

    rsx! {
        div { class: "adm-list-page", ..vec![page, more] }
    }
}
