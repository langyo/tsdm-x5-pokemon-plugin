use crate::dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

/// 搜索结果项
#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult {
    pub id: u64,
    pub name: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct SearchBoxProps {
    /// 占位符文本
    pub placeholder: String,
    /// 是否禁用
    #[props(default)]
    pub disabled: bool,
    /// 防抖延迟（毫秒）
    #[props(default = 300)]
    pub debounce_ms: u64,
    /// 搜索结果（由外部提供）
    pub results: Vec<SearchResult>,
    /// 是否正在加载
    #[props(default)]
    pub loading: bool,
    /// 防抖后的输入回调 - 外部执行搜索并更新 results
    pub on_search: EventHandler<String>,
    /// 选择结果时的回调
    pub on_select: EventHandler<SearchResult>,
    /// 当前已选中项（显示蓝底卡片）
    #[props(default)]
    pub selected: Option<SearchResult>,
    /// 清除已选中项的回调（None 则不显示清除按钮）
    #[props(default)]
    pub on_clear: Option<EventHandler<()>>,
}

/// 搜索框组件 - 带防抖和下拉菜单
/// 下拉菜单使用 fixed 定位渲染，避免被父容器 overflow 裁剪
#[component]
pub fn SearchBox(props: SearchBoxProps) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut search_value = use_signal(String::new);
    let mut debounce_counter = use_signal(|| 0u32);
    let mut dropdown_position = use_signal(|| (0i32, 0i32, 0u32));

    let show_dropdown = *is_focused.read() && !props.results.is_empty();

    let mut update_position = move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(input_el) = document
                        .query_selector(".admin-search-box__input--active")
                        .ok()
                        .flatten()
                    {
                        let rect = input_el.get_bounding_client_rect();
                        dropdown_position.set((
                            rect.x() as i32,
                            (rect.y() + rect.height()) as i32,
                            rect.width() as u32,
                        ));
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "admin-search-box",
            div { class: "admin-search-box__input-wrapper",
                input {
                    class: if *is_focused.read() { "admin-input admin-search-box__input admin-search-box__input--active" } else { "admin-input admin-search-box__input" },
                    r#type: "text",
                    placeholder: props.placeholder,
                    disabled: props.disabled,
                    value: "{search_value}",
                    onfocus: move |_| {
                        is_focused.set(true);
                        update_position();
                    },
                    onblur: move |_| {
                        let mut is_focused_clone = is_focused.clone();
                        spawn(async move {
                            TimeoutFuture::new(200).await;
                            is_focused_clone.set(false);
                        });
                    },
                    oninput: move |evt| {
                        let value = evt.value();
                        search_value.set(value.clone());

                        let current_counter = *debounce_counter.read() + 1;
                        debounce_counter.set(current_counter);

                        if value.trim().is_empty() {
                            return;
                        }

                        update_position();

                        let debounce_ms = props.debounce_ms;
                        let on_search = props.on_search.clone();
                        let value_for_search = value.clone();

                        spawn(async move {
                            TimeoutFuture::new(debounce_ms as u32).await;

                            if *debounce_counter.read() == current_counter {
                                on_search.call(value_for_search);
                            }
                        });
                    },
                }
                if props.loading {
                    div { class: "admin-search-box__loading",
                        div { class: "admin-search-box__spinner" }
                    }
                }
            }
            if let Some(sel) = props.selected.clone() {
                div { class: "admin-grant-selected",
                    span { class: "admin-grant-selected__label", "已选择:" }
                    span { class: "admin-grant-selected__name", "#{sel.id} {sel.name}" }
                    if let Some(on_clear) = props.on_clear.clone() {
                        button {
                            class: "admin-grant-selected__clear",
                            r#type: "button",
                            onclick: move |_| on_clear.call(()),
                            "×"
                        }
                    }
                }
            }
        }

        if show_dropdown {
            SearchDropdownPortal {
                results: props.results.clone(),
                position: *dropdown_position.read(),
                on_select: {
                    let on_select = props.on_select.clone();
                    move |result: SearchResult| {
                        on_select.call(result);
                    }
                },
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SearchDropdownPortalProps {
    results: Vec<SearchResult>,
    position: (i32, i32, u32),
    on_select: EventHandler<SearchResult>,
}

#[component]
fn SearchDropdownPortal(props: SearchDropdownPortalProps) -> Element {
    let (x, y, width) = props.position;

    rsx! {
        div {
            class: "admin-search-dropdown-portal",
            style: "left: {x}px; top: {y}px; min-width: {width}px;",

            for result in props.results.iter().take(8) {
                button {
                    class: "admin-search-dropdown-portal__item",
                    r#type: "button",
                    onclick: {
                        let result = result.clone();
                        let on_select = props.on_select.clone();
                        move |_| {
                            on_select.call(result.clone());
                        }
                    },
                    span { class: "admin-search-dropdown-portal__id", "#{result.id}" }
                    span { class: "admin-search-dropdown-portal__name", "{result.name}" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct IdSearchBoxProps {
    /// 占位符文本
    pub placeholder: String,
    /// 是否禁用
    #[props(default)]
    pub disabled: bool,
    /// 防抖延迟（毫秒）
    #[props(default = 300)]
    pub debounce_ms: u64,
    /// 搜索结果（由外部提供）
    pub results: Vec<SearchResult>,
    /// 是否正在加载
    #[props(default)]
    pub loading: bool,
    /// ID 搜索回调 - 接收 u64 ID
    pub on_search: EventHandler<u64>,
    /// 选择结果时的回调
    pub on_select: EventHandler<SearchResult>,
    /// 当前已选中项
    #[props(default)]
    pub selected: Option<SearchResult>,
    /// 清除已选中项的回调
    #[props(default)]
    pub on_clear: Option<EventHandler<()>>,
}

/// ID 搜索框组件 - 只接受数字输入
/// 输入数字后直接按 ID 查询，不需要"包含/不包含"等操作符
#[component]
pub fn IdSearchBox(props: IdSearchBoxProps) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut search_value = use_signal(String::new);
    let mut debounce_counter = use_signal(|| 0u32);
    let mut dropdown_position = use_signal(|| (0i32, 0i32, 0u32));

    let show_dropdown = *is_focused.read() && !props.results.is_empty();

    let mut update_position = move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(input_el) = document
                        .query_selector(".admin-id-search-box__input--active")
                        .ok()
                        .flatten()
                    {
                        let rect = input_el.get_bounding_client_rect();
                        dropdown_position.set((
                            rect.x() as i32,
                            (rect.y() + rect.height()) as i32,
                            rect.width() as u32,
                        ));
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "admin-search-box",
            div { class: "admin-search-box__input-wrapper",
                input {
                    class: if *is_focused.read() { "admin-input admin-search-box__input admin-id-search-box__input admin-id-search-box__input--active" } else { "admin-input admin-search-box__input admin-id-search-box__input" },
                    r#type: "text",
                    inputmode: "numeric",
                    pattern: "[0-9]*",
                    placeholder: props.placeholder,
                    disabled: props.disabled,
                    value: "{search_value}",
                    onfocus: move |_| {
                        is_focused.set(true);
                        update_position();
                    },
                    onblur: move |_| {
                        let mut is_focused_clone = is_focused.clone();
                        spawn(async move {
                            TimeoutFuture::new(200).await;
                            is_focused_clone.set(false);
                        });
                    },
                    oninput: move |evt| {
                        let value = evt.value();
                        // 只保留数字字符
                        let filtered: String = value.chars().filter(|c| c.is_numeric()).collect();
                        search_value.set(filtered.clone());

                        let current_counter = *debounce_counter.read() + 1;
                        debounce_counter.set(current_counter);

                        if filtered.trim().is_empty() {
                            return;
                        }

                        // 尝试解析为 u64
                        if let Ok(id) = filtered.trim().parse::<u64>() {
                            update_position();

                            let debounce_ms = props.debounce_ms;
                            let on_search = props.on_search.clone();

                            spawn(async move {
                                TimeoutFuture::new(debounce_ms as u32).await;

                                if *debounce_counter.read() == current_counter {
                                    on_search.call(id);
                                }
                            });
                        }
                    },
                }
                if props.loading {
                    div { class: "admin-search-box__loading",
                        div { class: "admin-search-box__spinner" }
                    }
                }
            }
            if let Some(sel) = props.selected.clone() {
                div { class: "admin-grant-selected",
                    span { class: "admin-grant-selected__label", "已选择:" }
                    span { class: "admin-grant-selected__name", "#{sel.id} {sel.name}" }
                    if let Some(on_clear) = props.on_clear.clone() {
                        button {
                            class: "admin-grant-selected__clear",
                            r#type: "button",
                            onclick: move |_| on_clear.call(()),
                            "×"
                        }
                    }
                }
            }
        }

        if show_dropdown {
            SearchDropdownPortal {
                results: props.results.clone(),
                position: *dropdown_position.read(),
                on_select: {
                    let on_select = props.on_select.clone();
                    move |result: SearchResult| {
                        on_select.call(result);
                    }
                },
            }
        }
    }
}
