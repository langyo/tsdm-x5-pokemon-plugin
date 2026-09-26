use crate::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct UIState {
    pub toasts: Vec<ToastInfo>,
    pub toast_counter: u64,
    pub popup: PopupState,
    pub modal: ModalState,
}

#[derive(Clone, PartialEq)]
pub struct ModalState {
    pub is_open: bool,
    pub title: String,
    pub content: Element,
}

impl Default for ModalState {
    fn default() -> Self {
        Self {
            is_open: false,
            title: String::new(),
            content: rsx! {},
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ToastInfo {
    pub id: u64,
    pub message: String,
    pub toast_type: ToastType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Info,
    Warning,
}

#[derive(Clone, PartialEq)]
pub struct PopupState {
    pub is_open: bool,
    pub position: (f64, f64),
    pub content: Element,
}

impl Default for PopupState {
    fn default() -> Self {
        Self {
            is_open: false,
            position: (0.0, 0.0),
            content: rsx! {},
        }
    }
}

pub fn show_toast(message: impl Into<String>, toast_type: ToastType) {
    let msg = message.into();

    let _type_str = match toast_type {
        ToastType::Success => "SUCCESS",
        ToastType::Error => "ERROR",
        ToastType::Info => "INFO",
        ToastType::Warning => "WARNING",
    };

    let mut state = crate::state::UI_STATE.write();

    // 如果 toast 数量达到 3 个，移除最旧的
    if state.toasts.len() >= 3 {
        state.toasts.remove(0);
    }

    let id = state.toast_counter;
    state.toast_counter += 1;
    state.toasts.push(ToastInfo {
        id,
        message: msg,
        toast_type,
    });

    let toast_id = id;
    // 错误/警告停留更久便于阅读和复制，但也要自动消失，
    // 否则会一直挡在页面顶部（此前错误 toast 永不消失）
    let timeout_ms = match toast_type {
        ToastType::Error => 8000,
        ToastType::Warning => 6000,
        _ => 3000,
    };

    spawn(async move {
        gloo_timers::future::TimeoutFuture::new(timeout_ms).await;
        hide_toast(toast_id);
    });
}

pub fn hide_toast(id: u64) {
    let mut state = crate::state::UI_STATE.write();
    state.toasts.retain(|t| t.id != id);
}

pub fn show_success(message: impl Into<String>) {
    show_toast(message, ToastType::Success);
}

pub fn show_error(message: impl Into<String>) {
    let mut state = crate::state::UI_STATE.write();
    state.toasts.retain(|t| t.toast_type != ToastType::Error);
    drop(state);
    show_toast(message, ToastType::Error);
}

pub fn show_info(message: impl Into<String>) {
    show_toast(message, ToastType::Info);
}

pub fn show_warning(message: impl Into<String>) {
    show_toast(message, ToastType::Warning);
}

pub fn open_popup(position: (f64, f64), content: Element) {
    let mut state = crate::state::UI_STATE.write();
    state.popup.is_open = true;
    state.popup.position = position;
    state.popup.content = content;
}

pub fn open_popup_at_element(rect: web_sys::DomRect, content: Element) {
    let menu_width = 120.0;
    let menu_height = 80.0;
    let window = web_sys::window().unwrap();
    let win_width = window.inner_width().unwrap().as_f64().unwrap_or(375.0);
    let win_height = window.inner_height().unwrap().as_f64().unwrap_or(700.0);

    let x = if rect.x() + menu_width > win_width {
        (rect.x() - menu_width - 10.0).max(10.0)
    } else {
        rect.x().max(10.0)
    };
    let y = if rect.y() + rect.height() + menu_height > win_height {
        (rect.y() - menu_height - 5.0).max(10.0)
    } else {
        (rect.y() + rect.height() + 5.0).max(10.0)
    };

    open_popup((x, y), content);
}

pub fn open_popup_at_mouse(client_x: f64, client_y: f64, content: Element) {
    let menu_width = 120.0;
    let menu_height = 80.0;
    let window = web_sys::window().unwrap();
    let win_width = window.inner_width().unwrap().as_f64().unwrap_or(375.0);
    let win_height = window.inner_height().unwrap().as_f64().unwrap_or(700.0);

    let x = if client_x + menu_width > win_width {
        (client_x - menu_width - 10.0).max(10.0)
    } else {
        (client_x - 40.0).max(10.0)
    };
    let y = if client_y + menu_height > win_height {
        (client_y - menu_height - 10.0).max(10.0)
    } else {
        (client_y + 10.0).max(10.0)
    };

    open_popup((x, y), content);
}

pub fn close_popup() {
    let mut state = crate::state::UI_STATE.write();
    state.popup = PopupState::default();
}

pub fn is_popup_open() -> bool {
    crate::state::UI_STATE.read().popup.is_open
}

// ============== Modal ==============

pub fn open_modal(title: impl Into<String>, content: Element) {
    let mut state = crate::state::UI_STATE.write();
    state.modal.is_open = true;
    state.modal.title = title.into();
    state.modal.content = content;
}

pub fn close_modal() {
    let mut state = crate::state::UI_STATE.write();
    state.modal = ModalState::default();
}

pub fn is_modal_open() -> bool {
    crate::state::UI_STATE.read().modal.is_open
}
