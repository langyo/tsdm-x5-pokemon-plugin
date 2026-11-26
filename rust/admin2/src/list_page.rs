//! 数据列表页的公共壳（后台七张表共用）。
//!
//! 七张列表页（宠物 / 物品 / 地图 / 用户 / 进化 / 技能 / 效果）原本各有一份
//! ~230 行的实现，只有「表格列」和「新增按钮去哪儿」不同。那个复制法有两
//! 个代价：改一处交互要改七遍；每份都自带一套 `data-pm-*` 委托属性。
//!
//! 这里把不变的部分收进来：标题栏、筛选栏、加载/错误态、加载更多、空态；
//! 变的两个点作为参数：
//! - `columns`：表头文案；
//! - `row`：把一个 JSON 行渲染成 `<tr>` 的子节点（各实体的列不同）。
//!
//! 筛选与工具栏全部用 tairitsu 原生事件绑定，`EntitySpec` 决定服务端的
//! 筛选 tag。

use tairitsu_macros::rsx;
use tairitsu_vdom::VNode;

use crate::app_state::AppState;
use crate::pages::{apply_list_filters_pub, ensure_loaded, load_more_list_pub, reset_list_filters_pub};
use crate::state::{EntitySpec, ListState};

/// 清空并重新拉取一张列表（「刷新」与错误态的重试共用）。
fn reload(st: &AppState, ls: &ListState) {
    ls.loaded.inner().set(None);
    ls.rows.inner().set(Vec::new());
    crate::request_remount();
    ensure_loaded(st);
}

/// 列表页的一块表格：表头 + 每行的渲染函数。
///
/// `row` 拿到页面状态和该行 JSON，返回该 `<tr>` 的子节点 —— 各实体的列不同，
/// 行内的操作按钮也要靠状态绑定事件。
pub struct TableShape<'a> {
    pub columns: &'static [&'static str],
    pub row: Box<dyn Fn(&AppState, &serde_json::Value) -> Vec<VNode> + 'a>,
}

/// 渲染一张数据列表页。
///
/// `key` 是页面键（"pk" / "it" / …），只用于 `data-test-*` 钩子；
/// `on_new` 是「新增」按钮的动作，`None` 时不渲染该按钮。
pub fn entity_list_page(
    st: &AppState,
    spec: EntitySpec,
    ls: &ListState,
    key: &str,
    shape: &TableShape<'_>,
    on_new: Option<std::rc::Rc<dyn Fn()>>,
) -> VNode {
    let load_state = ls.loaded.get();
    let busy = ls.busy.get();
    let filtered = ls.filtered.get();
    let err = ls.err.get();
    let rows = ls.rows.get();
    let total = ls.total.get();
    let loaded_count = ls.loaded_count.get();
    let has_more = !filtered && loaded_count < total;
    let f_id = ls.f_id.get();
    let f_name = ls.f_name.get();
    let f_a = ls.f_a.get();
    let f_b = ls.f_b.get();
    let count_text = if filtered {
        format!("筛选结果 {loaded_count} 条")
    } else {
        format!("已加载 {loaded_count} / {total} 条")
    };

    // ---- 数据体：加载中 / 失败 / 表格 ----
    let body = match &load_state {
        None => {
            let text = format!("正在加载{}...", spec.title);
            rsx! { div { class: "admin-loading", "{text}" } }
        }
        Some(Err(e)) => {
            let msg = format!("加载失败：{e}");
            let retry_state = st.clone();
            let retry_ls = ls.clone();
            let reload_key = key.to_string();
            rsx! {
                div { class: "admin-card admin-error",
                    p { "{msg}" }
                    button {
                        class: "admin-btn",
                        disabled: busy,
                        "data-test-adm-l-reload": reload_key,
                        onclick: move |_| reload(&retry_state, &retry_ls),
                        "重新加载"
                    }
                }
            }
        }
        Some(Ok(())) => {
            let err_bar = match &err {
                Some(e) => {
                    let msg = format!("筛选失败：{e}");
                    rsx! { div { class: "admin-card admin-error", p { "{msg}" } } }
                }
                None => VNode::empty(),
            };
            let table = if rows.is_empty() {
                let text = format!("暂无{}数据。", spec.title);
                rsx! { p { class: "admin-empty-hint", "{text}" } }
            } else {
                let trs: Vec<VNode> = rows
                    .iter()
                    .map(|item| {
                        let cells = (shape.row)(st, item);
                        rsx! { tr { ..cells } }
                    })
                    .collect();
                let heads: Vec<VNode> = shape
                    .columns
                    .iter()
                    .map(|c| {
                        let c: &str = c;
                        rsx! { th { "{c}" } }
                    })
                    .collect();
                rsx! {
                    table { class: "adm-table",
                        thead { tr { ..heads } }
                        tbody { ..trs }
                    }
                }
            };
            let more_btn = if has_more {
                let label = format!("加载更多（已加载 {loaded_count}/{total}）");
                let more_spec = spec;
                let more_ls = ls.clone();
                rsx! {
                    div { class: "adm-load-more",
                        button {
                            class: "admin-btn",
                            disabled: busy,
                            "data-test-adm-l-more": key.to_string(),
                            onclick: move |_| load_more_list_pub(more_spec, &more_ls),
                            "{label}"
                        }
                    }
                }
            } else {
                VNode::empty()
            };
            let parts = vec![err_bar, table, more_btn];
            rsx! {
                div { ..parts }
            }
        }
    };

    // ---- 筛选栏 ----
    let id_state = st.clone();
    let id_ls = ls.clone();
    let name_state = st.clone();
    let name_ls = ls.clone();
    let a_state = st.clone();
    let a_ls = ls.clone();
    let b_state = st.clone();
    let b_ls = ls.clone();
    let apply_spec = spec;
    let apply_state = st.clone();
    let apply_ls = ls.clone();
    let reset_spec = spec;
    let reset_ls = ls.clone();
    let refresh_state = st.clone();
    let refresh_ls = ls.clone();
    let reload_key = key.to_string();

    // 两个实体特定筛选框：只有 spec 配了才渲染。
    let filter_a = match spec.a {
        Some(f) => {
            let placeholder = f.placeholder.to_string();
            rsx! {
                input {
                    class: "adm-filter-input adm-filter-kind",
                    r#type: "text",
                    placeholder: placeholder,
                    value: f_a,
                    "data-test-adm-f-a": key.to_string(),
                    onchange: move |e: tairitsu_vdom::ChangeEvent| {
                        a_ls.f_a.inner().set(e.value.clone());
                        crate::request_remount();
                    }
                }
            }
        }
        None => VNode::empty(),
    };
    let filter_b = match spec.b {
        Some(f) => {
            let placeholder = f.placeholder.to_string();
            rsx! {
                input {
                    class: "adm-filter-input adm-filter-kind",
                    r#type: "text",
                    placeholder: placeholder,
                    value: f_b,
                    "data-test-adm-f-b": key.to_string(),
                    onchange: move |e: tairitsu_vdom::ChangeEvent| {
                        b_ls.f_b.inner().set(e.value.clone());
                        crate::request_remount();
                    }
                }
            }
        }
        None => VNode::empty(),
    };

    // 「新增」按钮：各实体文案不同，动作由调用方给。
    let new_btn = match on_new {
        Some(f) => rsx! {
            button {
                class: "admin-btn admin-btn--primary",
                disabled: busy,
                "data-test-adm-list-new": key.to_string(),
                onclick: move |_| f(),
                "新增"
            }
        },
        None => VNode::empty(),
    };

    let id_placeholder = format!("{}（精确）", spec.id_tag);
    let name_placeholder = spec.name_placeholder.to_string();
    let title = spec.title;

    rsx! {
        section { class: "admin-page",
            div { class: "admin-page-header",
                h2 { "{title}" }
                span { class: "adm-count", "{count_text}" }
            }
            div { class: "admin-card adm-filter-bar",
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: id_placeholder,
                    value: f_id,
                    "data-test-adm-f-id": key.to_string(),
                    onchange: move |e: tairitsu_vdom::ChangeEvent| {
                        id_ls.f_id.inner().set(e.value.clone());
                        crate::request_remount();
                    }
                }
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: name_placeholder,
                    value: f_name,
                    "data-test-adm-f-name": key.to_string(),
                    onchange: move |e: tairitsu_vdom::ChangeEvent| {
                        name_ls.f_name.inner().set(e.value.clone());
                        crate::request_remount();
                    }
                }
                ..vec![filter_a, filter_b]
                button {
                    class: "admin-btn admin-btn--primary",
                    disabled: busy,
                    "data-test-adm-l-apply": key.to_string(),
                    onclick: move |_| apply_list_filters_pub(apply_spec, &apply_state, &apply_ls),
                    "筛选"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-test-adm-l-reset": key.to_string(),
                    onclick: move |_| reset_list_filters_pub(reset_spec, &reset_ls),
                    "重置"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-test-adm-l-reload-btn": reload_key,
                    onclick: move |_| reload(&refresh_state, &refresh_ls),
                    "刷新"
                }
                ..vec![new_btn]
            }
            ..vec![body]
        }
    }
}
