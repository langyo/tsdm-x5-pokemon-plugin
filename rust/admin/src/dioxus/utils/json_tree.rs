use std::collections::HashSet;

use serde_json::Value;

use crate::dioxus::prelude::*;

use crate::dioxus::state::{set_notice, AdminNoticeLevel};
use crate::dioxus::utils::clipboard::copy_to_clipboard;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum JsonTreeMode {
    #[default]
    Readonly,
    Editable,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ExpandState {
    #[default]
    All,
    None,
    First,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JsonValueType {
    Null,
    Bool,
    Number,
    String,
}

impl JsonValueType {
    pub fn default_value(&self) -> Value {
        match self {
            JsonValueType::Null => Value::Null,
            JsonValueType::Bool => Value::Bool(false),
            JsonValueType::Number => Value::Number(0.into()),
            JsonValueType::String => Value::String(String::new()),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            JsonValueType::Null => "null",
            JsonValueType::Bool => "boolean",
            JsonValueType::Number => "number",
            JsonValueType::String => "string",
        }
    }

    pub fn from_value(value: &Value) -> Self {
        match value {
            Value::Null => JsonValueType::Null,
            Value::Bool(_) => JsonValueType::Bool,
            Value::Number(_) => JsonValueType::Number,
            Value::String(_) => JsonValueType::String,
            _ => JsonValueType::Null,
        }
    }

    pub fn convert_value(&self, value: &Value) -> Value {
        let current_type = Self::from_value(value);
        if std::mem::discriminant(self) == std::mem::discriminant(&current_type) {
            return value.clone();
        }
        let str_val = match value {
            Value::Null => String::new(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            _ => String::new(),
        };
        match self {
            JsonValueType::Null => Value::Null,
            JsonValueType::Bool => {
                let lower = str_val.to_lowercase();
                Value::Bool(lower == "true" || lower == "1" || lower == "yes")
            }
            JsonValueType::Number => {
                if let Ok(n) = str_val.parse::<i64>() {
                    Value::Number(n.into())
                } else if let Ok(n) = str_val.parse::<f64>() {
                    serde_json::Number::from_f64(n)
                        .map(Value::Number)
                        .unwrap_or(Value::Number(0.into()))
                } else {
                    Value::Number(0.into())
                }
            }
            JsonValueType::String => Value::String(str_val),
        }
    }
}

#[component]
pub fn JsonTree(
    label: String,
    value: Value,
    #[props(default)] mode: JsonTreeMode,
    onchange: Option<EventHandler<Value>>,
    #[props(default = true)] show_toolbar: bool,
    #[props(default)] max_depth: Option<usize>,
    #[props(default = ExpandState::First)] default_expand: ExpandState,
    #[props(default)] search: Option<String>,
) -> Element {
    let mut expanded_paths = use_signal(HashSet::<String>::new);
    let mut search_query = use_signal(|| search.clone().unwrap_or_default());
    let mut current_value = use_signal(|| value.clone());

    let handle_value_change = {
        let onchange = onchange.clone();
        move |new_value: Value| {
            current_value.set(new_value.clone());
            if let Some(handler) = &onchange {
                handler.call(new_value);
            }
        }
    };

    let root_path_for_expand = label.clone();
    let root_path_for_node = label.clone();

    let should_expand_all = matches!(default_expand, ExpandState::All);
    let should_expand_first = matches!(default_expand, ExpandState::First);

    rsx! {
        div { class: "json-tree",
            if show_toolbar {
                JsonTreeToolbar {
                    search_query: search_query(),
                    on_search_change: move |q| search_query.set(q),
                    on_expand_all: move |_| {
                        let mut set = expanded_paths.write();
                        set.clear();
                        collect_all_paths(&current_value.read(), &root_path_for_expand, &mut set);
                    },
                    on_collapse_all: move |_| {
                        expanded_paths.write().clear();
                    },
                }
            }

            div { class: "json-tree__content",
                JsonNode {
                    path: root_path_for_node.clone(),
                    node_key: label,
                    value: current_value(),
                    mode,
                    depth: 0,
                    max_depth,
                    expanded_paths,
                    search_query: search_query(),
                    on_change: handle_value_change,
                    is_root: true,
                    should_expand_all,
                    should_expand_first,
                }
            }
        }
    }
}

fn collect_all_paths(value: &Value, prefix: &str, paths: &mut HashSet<String>) {
    match value {
        Value::Object(map) => {
            paths.insert(prefix.to_string());
            for (k, v) in map {
                let path = format!("{}.{}", prefix, k);
                collect_all_paths(v, &path, paths);
            }
        }
        Value::Array(arr) => {
            paths.insert(prefix.to_string());
            for (i, v) in arr.iter().enumerate() {
                let path = format!("{}[{}]", prefix, i);
                collect_all_paths(v, &path, paths);
            }
        }
        _ => {}
    }
}

#[component]
fn JsonTreeToolbar(
    search_query: String,
    on_search_change: EventHandler<String>,
    on_expand_all: EventHandler<()>,
    on_collapse_all: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "json-tree__toolbar",
            input {
                class: "admin-input json-tree__search",
                r#type: "text",
                placeholder: "搜索...",
                value: "{search_query}",
                oninput: move |e| on_search_change.call(e.value()),
            }
            div { class: "json-tree__toolbar-actions",
                button {
                    class: "admin-btn admin-btn--ghost admin-btn--small",
                    onclick: move |_| on_expand_all.call(()),
                    "展开全部"
                }
                button {
                    class: "admin-btn admin-btn--ghost admin-btn--small",
                    onclick: move |_| on_collapse_all.call(()),
                    "折叠全部"
                }
            }
        }
    }
}

/// 统一下拉菜单项类型
#[derive(Clone, Debug, PartialEq)]
pub enum JsonMenuItem {
    Type {
        label: String,
        value: JsonValueType,
        is_active: bool,
    },
    Custom {
        label: String,
        value: String,
        is_active: bool,
    },
}

/// 统一的 JSON 树编辑器下拉菜单组件
#[component]
fn JsonDropdownMenu(
    #[props(default)] trigger_class: String,
    #[props(default)] trigger_title: String,
    items: Vec<JsonMenuItem>,
    on_select: EventHandler<String>,
    children: Element,
) -> Element {
    let mut show_menu = use_signal(|| false);

    // 点击外部关闭菜单
    use_effect(move || {
        if *show_menu.read() {
            let _ = dioxus_document::eval(
                r#"
                (event) => {
                    if (!event.target.closest('.json-dropdown')) {
                        window.__close_json_menu = true;
                    } else {
                        window.__close_json_menu = false;
                    }
                }
                "#,
            );
        }
    });

    rsx! {
        div { class: "json-dropdown",
            button {
                class: "{trigger_class}",
                title: "{trigger_title}",
                onclick: move |e| {
                    e.stop_propagation();
                    let current = *show_menu.read();
                    show_menu.set(!current);
                },
                {children}
            }
            if *show_menu.read() {
                div { class: "json-dropdown-menu",
                    for item in items.clone() {
                        button {
                            class: match &item {
                                JsonMenuItem::Type { is_active: true, .. } => {
                                    "json-dropdown-menu__item json-dropdown-menu__item--active"
                                }
                                JsonMenuItem::Custom { is_active: true, .. } => {
                                    "json-dropdown-menu__item json-dropdown-menu__item--active"
                                }
                                _ => "json-dropdown-menu__item",
                            },
                            onclick: {
                                let on_select = on_select.clone();
                                let label = match &item {
                                    JsonMenuItem::Type { label, .. } => label.clone(),
                                    JsonMenuItem::Custom { label, .. } => label.clone(),
                                };
                                move |e| {
                                    e.stop_propagation();
                                    on_select.call(label.clone());
                                    show_menu.set(false);
                                }
                            },
                            {
                                match &item {
                                    JsonMenuItem::Type { label, .. } => label.clone(),
                                    JsonMenuItem::Custom { label, .. } => label.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn JsonAddButton(
    path: String,
    node_type: &'static str,
    on_add: EventHandler<JsonValueType>,
) -> Element {
    let items = vec![
        JsonMenuItem::Type {
            label: "string".to_string(),
            value: JsonValueType::String,
            is_active: false,
        },
        JsonMenuItem::Type {
            label: "number".to_string(),
            value: JsonValueType::Number,
            is_active: false,
        },
        JsonMenuItem::Type {
            label: "boolean".to_string(),
            value: JsonValueType::Bool,
            is_active: false,
        },
        JsonMenuItem::Type {
            label: "null".to_string(),
            value: JsonValueType::Null,
            is_active: false,
        },
    ];

    rsx! {
        div { class: "json-add-wrapper",
            JsonDropdownMenu {
                trigger_class: "json-node__action json-node__action--add".to_string(),
                trigger_title: "添加项".to_string(),
                items,
                on_select: {
                    let on_add = on_add.clone();
                    move |label: String| {
                        let value_type = match label.as_str() {
                            "string" => JsonValueType::String,
                            "number" => JsonValueType::Number,
                            "boolean" => JsonValueType::Bool,
                            "null" => JsonValueType::Null,
                            _ => JsonValueType::String,
                        };
                        on_add.call(value_type);
                    }
                },
                "+"
            }
        }
    }
}

#[component]
fn JsonNode(
    path: String,
    node_key: String,
    value: Value,
    mode: JsonTreeMode,
    depth: usize,
    max_depth: Option<usize>,
    expanded_paths: Signal<HashSet<String>>,
    search_query: String,
    on_change: EventHandler<Value>,
    #[props(default)] is_root: bool,
    #[props(default)] should_expand_all: bool,
    #[props(default)] should_expand_first: bool,
) -> Element {
    let is_editable = mode == JsonTreeMode::Editable;

    let search_lower = search_query.to_lowercase();
    let matches_search = if search_lower.is_empty() {
        true
    } else {
        let key_matches = node_key.to_lowercase().contains(&search_lower);
        let value_matches = match &value {
            Value::String(s) => s.to_lowercase().contains(&search_lower),
            Value::Number(n) => n.to_string().contains(&search_lower),
            Value::Bool(b) => b.to_string().contains(&search_lower),
            _ => false,
        };
        key_matches || value_matches
    };

    if !matches_search && !is_root {
        return rsx! {};
    }

    let is_expanded = if should_expand_all {
        true
    } else if should_expand_first && depth <= 1 {
        true
    } else {
        expanded_paths.read().contains(&path)
    };

    let can_expand = max_depth.map_or(true, |max| depth < max);

    match value {
        Value::Null => rsx! {
            JsonPrimitiveNode {
                node_key,
                value_type: "null",
                value_display: "null".to_string(),
                is_editable,
                on_change,
            }
        },
        Value::Bool(b) => rsx! {
            JsonPrimitiveNode {
                node_key,
                value_type: "bool",
                value_display: b.to_string(),
                is_editable,
                on_change,
            }
        },
        Value::Number(n) => {
            let display = n.to_string();
            rsx! {
                JsonPrimitiveNode {
                    node_key,
                    value_type: "number",
                    value_display: display.clone(),
                    is_editable,
                    on_change,
                }
            }
        }
        Value::String(s) => rsx! {
            JsonPrimitiveNode {
                node_key,
                value_type: "string",
                value_display: s.clone(),
                is_editable,
                on_change,
            }
        },
        Value::Array(items) => {
            let count = items.len();
            let is_currently_expanded = is_expanded && can_expand;
            let indices: Vec<usize> = (0..items.len()).collect();

            rsx! {
                div { class: if is_editable { "json-node json-node--collapsible json-node--editable" } else { "json-node json-node--collapsible" },
                    div { class: "json-node__line",
                        button {
                            class: "json-node__toggle",
                            onclick: {
                                let path = path.clone();
                                move |_| {
                                    let mut set = expanded_paths.write();
                                    if set.contains(&path) {
                                        set.remove(&path);
                                    } else {
                                        set.insert(path.clone());
                                    }
                                }
                            },
                            if is_currently_expanded {
                                "▼"
                            } else {
                                "▶"
                            }
                        }
                        if is_editable {
                            JsonAddButton {
                                path: path.clone(),
                                node_type: "array",
                                on_add: {
                                    let on_change = on_change.clone();
                                    let items = items.clone();
                                    move |value_type: JsonValueType| {
                                        let mut new_items = items.clone();
                                        new_items.push(value_type.default_value());
                                        on_change.call(Value::Array(new_items));
                                    }
                                },
                            }
                        }
                        span { class: "json-node__key", "{node_key}" }
                        span { class: "json-node__meta", "[{count}]" }
                        if !is_editable {
                            button {
                                class: "json-node__copy",
                                onclick: {
                                    let items = items.clone();
                                    move |e| {
                                        e.stop_propagation();
                                        let json = serde_json::to_string_pretty(&items).unwrap_or_default();
                                        copy_to_clipboard(&json);
                                    }
                                },
                                "复制"
                            }
                        }
                    }
                    if is_currently_expanded {
                        div { class: "json-node__children",
                            for idx in indices {
                                {
                                    let child_path = format!("{}[{}]", path, idx);
                                    let item = items[idx].clone();
                                    let on_change_child = {
                                        let on_change = on_change.clone();
                                        let items = items.clone();
                                        move |new_val: Value| {
                                            let mut new_items = items.clone();
                                            if idx < new_items.len() {
                                                new_items[idx] = new_val;
                                            }
                                            on_change.call(Value::Array(new_items));
                                        }
                                    };
                                    let on_delete_child = {
                                        let on_change = on_change.clone();
                                        let items = items.clone();
                                        move |_| {
                                            let mut new_items = items.clone();
                                            if idx < new_items.len() {
                                                new_items.remove(idx);
                                            }
                                            on_change.call(Value::Array(new_items));
                                        }
                                    };
                                    rsx! {
                                        JsonArrayItem {
                                            path: child_path,
                                            index: idx,
                                            value: item,
                                            mode,
                                            depth: depth + 1,
                                            max_depth,
                                            expanded_paths,
                                            search_query: search_query.clone(),
                                            on_change: on_change_child,
                                            on_delete: on_delete_child,
                                            should_expand_all,
                                            should_expand_first,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Value::Object(map) => {
            let count = map.len();
            let is_currently_expanded = is_expanded && can_expand;
            let keys: Vec<String> = map.keys().cloned().collect();

            rsx! {
                div { class: if is_editable { "json-node json-node--collapsible json-node--editable" } else { "json-node json-node--collapsible" },
                    div { class: "json-node__line",
                        button {
                            class: "json-node__toggle",
                            onclick: {
                                let path = path.clone();
                                move |_| {
                                    let mut set = expanded_paths.write();
                                    if set.contains(&path) {
                                        set.remove(&path);
                                    } else {
                                        set.insert(path.clone());
                                    }
                                }
                            },
                            if is_currently_expanded {
                                "▼"
                            } else {
                                "▶"
                            }
                        }
                        if is_editable {
                            JsonAddButton {
                                path: path.clone(),
                                node_type: "object",
                                on_add: {
                                    let on_change = on_change.clone();
                                    let map = map.clone();
                                    move |value_type: JsonValueType| {
                                        let mut new_map = map.clone();
                                        let mut new_key = "new_key".to_string();
                                        let mut counter = 1;
                                        while new_map.contains_key(&new_key) {
                                            new_key = format!("new_key_{}", counter);
                                            counter += 1;
                                        }
                                        new_map.insert(new_key, value_type.default_value());
                                        on_change.call(Value::Object(new_map));
                                    }
                                },
                            }
                        }
                        span { class: "json-node__key", "{node_key}" }
                        span { class: "json-node__meta", "{{{count}}}" }
                        if !is_editable {
                            button {
                                class: "json-node__copy",
                                onclick: {
                                    let map = map.clone();
                                    move |e| {
                                        e.stop_propagation();
                                        let json = serde_json::to_string_pretty(&map).unwrap_or_default();
                                        copy_to_clipboard(&json);
                                    }
                                },
                                "复制"
                            }
                        }
                    }
                    if is_currently_expanded {
                        div { class: "json-node__children",
                            for k in keys {
                                {
                                    let child_path = format!("{}.{}", path, k);
                                    let v = map.get(&k).cloned().unwrap_or(Value::Null);
                                    let on_change_child = {
                                        let on_change = on_change.clone();
                                        let map = map.clone();
                                        let k = k.clone();
                                        move |new_val: Value| {
                                            let mut new_map = map.clone();
                                            new_map.insert(k.clone(), new_val);
                                            on_change.call(Value::Object(new_map));
                                        }
                                    };
                                    let on_delete_child = {
                                        let on_change = on_change.clone();
                                        let map = map.clone();
                                        let k = k.clone();
                                        move |_| {
                                            let mut new_map = map.clone();
                                            new_map.remove(&k);
                                            on_change.call(Value::Object(new_map));
                                        }
                                    };
                                    let on_rename_child = {
                                        let on_change = on_change.clone();
                                        let map = map.clone();
                                        let k = k.clone();
                                        move |new_key: String| {
                                            if new_key == k || new_key.is_empty() {
                                                return;
                                            }
                                            let mut new_map = map.clone();
                                            if let Some(val) = new_map.remove(&k) {
                                                new_map.insert(new_key, val);
                                            }
                                            on_change.call(Value::Object(new_map));
                                        }
                                    };
                                    rsx! {
                                        JsonObjectItem {
                                            path: child_path,
                                            item_key: k,
                                            value: v,
                                            mode,
                                            depth: depth + 1,
                                            max_depth,
                                            expanded_paths,
                                            search_query: search_query.clone(),
                                            on_change: on_change_child,
                                            on_delete: on_delete_child,
                                            on_rename: on_rename_child,
                                            should_expand_all,
                                            should_expand_first,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn JsonPrimitiveNode(
    node_key: String,
    value_type: &'static str,
    value_display: String,
    is_editable: bool,
    on_change: EventHandler<Value>,
) -> Element {
    let mut is_editing = use_signal(|| false);
    let mut edit_val = use_signal(|| value_display.clone());

    let value_for_onkeydown = value_display.clone();
    let value_for_onclick = value_display.clone();
    let value_for_copy = value_display.clone();

    let type_class = match value_type {
        "null" => "json-node__value--null",
        "bool" => "json-node__value--bool",
        "number" => "json-node__value--number",
        "string" => "json-node__value--string",
        _ => "json-node__value",
    };

    rsx! {
        div { class: "json-node json-node--primitive",
            div { class: "json-node__line",
                span { class: "json-node__key", "{node_key}" }
                if *is_editing.read() {
                    input {
                        class: "admin-input json-node__input",
                        r#type: "text",
                        value: "{edit_val}",
                        autofocus: true,
                        oninput: move |e| edit_val.set(e.value()),
                        onkeydown: {
                            move |e: KeyboardEvent| {
                                if e.key() == Key::Enter {
                                    let new_val = edit_val.read().clone();
                                    let parsed = parse_primitive_value(&new_val);
                                    on_change.call(parsed);
                                    is_editing.set(false);
                                } else if e.key() == Key::Escape {
                                    edit_val.set(value_for_onkeydown.clone());
                                    is_editing.set(false);
                                }
                            }
                        },
                        onblur: {
                            move |_| {
                                let new_val = edit_val.read().clone();
                                let parsed = parse_primitive_value(&new_val);
                                on_change.call(parsed);
                                is_editing.set(false);
                            }
                        },
                    }
                } else {
                    if value_type == "string" {
                        span {
                            class: "json-node__value json-node__value--quoted",
                            onclick: {
                                move |_| {
                                    if is_editable {
                                        edit_val.set(value_for_onclick.clone());
                                        is_editing.set(true);
                                    }
                                }
                            },
                            "\"{value_display}\""
                        }
                    } else {
                        span {
                            class: "json-node__value {type_class}",
                            onclick: {
                                move |_| {
                                    if is_editable {
                                        edit_val.set(value_for_onclick.clone());
                                        is_editing.set(true);
                                    }
                                }
                            },
                            "{value_display}"
                        }
                    }
                }
                if !*is_editing.read() && !is_editable {
                    button {
                        class: "json-node__copy",
                        onclick: move |_| {
                            match copy_to_clipboard(&value_for_copy) {
                                Ok(()) => {
                                    set_notice(AdminNoticeLevel::Info, "已复制到剪贴板");
                                }
                                Err(error) => {
                                    set_notice(
                                        AdminNoticeLevel::Error,
                                        format!("复制失败: {}", error),
                                    );
                                }
                            }
                        },
                        "复制"
                    }
                }
            }
        }
    }
}

#[component]
fn JsonArrayItem(
    path: String,
    index: usize,
    value: Value,
    mode: JsonTreeMode,
    depth: usize,
    max_depth: Option<usize>,
    expanded_paths: Signal<HashSet<String>>,
    search_query: String,
    on_change: EventHandler<Value>,
    on_delete: EventHandler<()>,
    #[props(default)] should_expand_all: bool,
    #[props(default)] should_expand_first: bool,
) -> Element {
    let is_editable = mode == JsonTreeMode::Editable;
    let key = format!("[{}]", index);

    rsx! {
        div { class: "json-array-item",
            JsonNode {
                path: path.clone(),
                node_key: key.clone(),
                value,
                mode,
                depth,
                max_depth,
                expanded_paths,
                search_query,
                on_change,
                should_expand_all,
                should_expand_first,
            }
            if is_editable {
                button {
                    class: "json-node__action json-node__action--delete json-node__action--inline",
                    title: "删除此项",
                    onclick: move |_| on_delete.call(()),
                    "×"
                }
            }
        }
    }
}

#[component]
fn JsonObjectItem(
    path: String,
    item_key: String,
    value: Value,
    mode: JsonTreeMode,
    depth: usize,
    max_depth: Option<usize>,
    expanded_paths: Signal<HashSet<String>>,
    search_query: String,
    on_change: EventHandler<Value>,
    on_delete: EventHandler<()>,
    on_rename: EventHandler<String>,
    #[props(default)] should_expand_all: bool,
    #[props(default)] should_expand_first: bool,
) -> Element {
    let is_editable = mode == JsonTreeMode::Editable;
    let mut editing_key = use_signal(|| None::<String>);

    let is_editing_key = editing_key.read().is_some();
    let edit_key_val = editing_key
        .read()
        .clone()
        .unwrap_or_else(|| item_key.clone());

    let is_primitive = matches!(
        value,
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_)
    );
    let current_type = JsonValueType::from_value(&value);

    if is_primitive {
        let value_type = match &value {
            Value::Null => "null",
            Value::Bool(_) => "bool",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            _ => "null",
        };
        let type_class = match value_type {
            "null" => "json-node__value--null",
            "bool" => "json-node__value--bool",
            "number" => "json-node__value--number",
            "string" => "json-node__value--string",
            _ => "",
        };
        let value_display = match &value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            _ => String::new(),
        };

        let mut is_editing_value = use_signal(|| false);
        let mut edit_val = use_signal(|| value_display.clone());

        return rsx! {
            div { class: "json-object-item json-object-item--inline",
                if is_editing_key {
                    input {
                        class: "admin-input json-node__input json-node__input--key",
                        r#type: "text",
                        value: "{edit_key_val}",
                        autofocus: true,
                        oninput: move |e| editing_key.set(Some(e.value())),
                        onkeydown: {
                            let on_rename = on_rename.clone();
                            let edit_key_val = edit_key_val.clone();
                            move |e: KeyboardEvent| {
                                if e.key() == Key::Enter {
                                    on_rename.call(edit_key_val.clone());
                                    editing_key.set(None);
                                } else if e.key() == Key::Escape {
                                    editing_key.set(None);
                                }
                            }
                        },
                        onblur: {
                            let on_rename = on_rename.clone();
                            let edit_key_val = edit_key_val.clone();
                            move |_| {
                                on_rename.call(edit_key_val.clone());
                                editing_key.set(None);
                            }
                        },
                    }
                } else {
                    span {
                        class: if is_editable { "json-node__key json-node__key--editable" } else { "json-node__key" },
                        onclick: {
                            let item_key = item_key.clone();
                            move |_| {
                                if is_editable {
                                    editing_key.set(Some(item_key.clone()))
                                }
                            }
                        },
                        "{item_key}"
                    }
                }

                span { class: "json-node__colon", ":" }

                if is_editable {
                    JsonDropdownMenu {
                        trigger_class: "json-type-selector__button".to_string(),
                        items: vec![
                            JsonMenuItem::Type {
                                label: "null".to_string(),
                                value: JsonValueType::Null,
                                is_active: current_type == JsonValueType::Null,
                            },
                            JsonMenuItem::Type {
                                label: "boolean".to_string(),
                                value: JsonValueType::Bool,
                                is_active: current_type == JsonValueType::Bool,
                            },
                            JsonMenuItem::Type {
                                label: "number".to_string(),
                                value: JsonValueType::Number,
                                is_active: current_type == JsonValueType::Number,
                            },
                            JsonMenuItem::Type {
                                label: "string".to_string(),
                                value: JsonValueType::String,
                                is_active: current_type == JsonValueType::String,
                            },
                        ],
                        on_select: {
                            let on_change = on_change.clone();
                            let value = value.clone();
                            move |label: String| {
                                let target_type = match label.as_str() {
                                    "null" => JsonValueType::Null,
                                    "boolean" => JsonValueType::Bool,
                                    "number" => JsonValueType::Number,
                                    "string" => JsonValueType::String,
                                    _ => JsonValueType::String,
                                };
                                let converted = target_type.convert_value(&value);
                                on_change.call(converted);
                            }
                        },
                        "{current_type.label()}"
                    }
                }

                if *is_editing_value.read() {
                    if value_type == "bool" {
                        div { class: "json-bool-selector",
                            button {
                                class: if value_display == "true" { "json-bool-selector__item json-bool-selector__item--active" } else { "json-bool-selector__item" },
                                onclick: {
                                    let on_change = on_change.clone();
                                    move |e| {
                                        e.stop_propagation();
                                        on_change.call(Value::Bool(true));
                                        is_editing_value.set(false);
                                    }
                                },
                                "true"
                            }
                            button {
                                class: if value_display == "false" { "json-bool-selector__item json-bool-selector__item--active" } else { "json-bool-selector__item" },
                                onclick: {
                                    let on_change = on_change.clone();
                                    move |e| {
                                        e.stop_propagation();
                                        on_change.call(Value::Bool(false));
                                        is_editing_value.set(false);
                                    }
                                },
                                "false"
                            }
                        }
                    } else {
                        input {
                            class: "admin-input json-node__input",
                            r#type: "text",
                            value: "{edit_val}",
                            autofocus: true,
                            oninput: move |e| edit_val.set(e.value()),
                            onkeydown: {
                                let on_change = on_change.clone();
                                let edit_val = edit_val.read().clone();
                                move |e: KeyboardEvent| {
                                    if e.key() == Key::Enter {
                                        let new_val = edit_val.clone();
                                        let parsed = parse_primitive_value(&new_val);
                                        let converted = current_type.convert_value(&parsed);
                                        on_change.call(converted);
                                        is_editing_value.set(false);
                                    } else if e.key() == Key::Escape {
                                        is_editing_value.set(false);
                                    }
                                }
                            },
                            onblur: {
                                let on_change = on_change.clone();
                                let edit_val = edit_val.read().clone();
                                move |_| {
                                    let new_val = edit_val.clone();
                                    let parsed = parse_primitive_value(&new_val);
                                    let converted = current_type.convert_value(&parsed);
                                    on_change.call(converted);
                                    is_editing_value.set(false);
                                }
                            },
                        }
                    }
                } else if value_type == "string" {
                    span {
                        class: "json-node__value json-node__value--quoted {type_class}",
                        onclick: {
                            let value_display = value_display.clone();
                            move |_| {
                                if is_editable {
                                    edit_val.set(value_display.clone());
                                    is_editing_value.set(true);
                                }
                            }
                        },
                        "\"{value_display}\""
                    }
                } else {
                    span {
                        class: "json-node__value {type_class}",
                        onclick: {
                            let value_display = value_display.clone();
                            move |_| {
                                if is_editable {
                                    edit_val.set(value_display.clone());
                                    is_editing_value.set(true);
                                }
                            }
                        },
                        "{value_display}"
                    }
                }

                if is_editable {
                    button {
                        class: "json-node__action json-node__action--delete",
                        title: "删除此项",
                        onclick: move |_| on_delete.call(()),
                        "×"
                    }
                }
            }
        };
    }

    rsx! {
        div { class: "json-object-item",
            div { class: "json-object-item__header",
                if is_editing_key {
                    input {
                        class: "admin-input json-node__input json-node__input--key",
                        r#type: "text",
                        value: "{edit_key_val}",
                        autofocus: true,
                        oninput: move |e| editing_key.set(Some(e.value())),
                        onkeydown: {
                            let on_rename = on_rename.clone();
                            let edit_key_val = edit_key_val.clone();
                            move |e: KeyboardEvent| {
                                if e.key() == Key::Enter {
                                    on_rename.call(edit_key_val.clone());
                                    editing_key.set(None);
                                } else if e.key() == Key::Escape {
                                    editing_key.set(None);
                                }
                            }
                        },
                        onblur: {
                            let on_rename = on_rename.clone();
                            let edit_key_val = edit_key_val.clone();
                            move |_| {
                                on_rename.call(edit_key_val.clone());
                                editing_key.set(None);
                            }
                        },
                    }
                } else {
                    span {
                        class: "json-node__key json-node__key--editable",
                        onclick: {
                            let item_key = item_key.clone();
                            move |_| editing_key.set(Some(item_key.clone()))
                        },
                        "{item_key}"
                    }
                }
                if is_editable {
                    button {
                        class: "json-node__action json-node__action--delete json-node__action--inline",
                        title: "删除此项",
                        onclick: move |_| on_delete.call(()),
                        "×"
                    }
                }
            }
            JsonNode {
                path,
                node_key: if is_editing_key { edit_key_val } else { item_key.clone() },
                value,
                mode,
                depth,
                max_depth,
                expanded_paths,
                search_query,
                on_change,
                should_expand_all,
                should_expand_first,
            }
        }
    }
}

fn parse_primitive_value(s: &str) -> Value {
    let trimmed = s.trim();
    if trimmed == "null" {
        Value::Null
    } else if trimmed == "true" {
        Value::Bool(true)
    } else if trimmed == "false" {
        Value::Bool(false)
    } else if let Ok(n) = trimmed.parse::<i64>() {
        Value::Number(n.into())
    } else if let Ok(n) = trimmed.parse::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(n) {
            Value::Number(n)
        } else {
            Value::String(s.to_string())
        }
    } else {
        Value::String(s.to_string())
    }
}
