use crate::prelude::*;

use crate::state::{close_popup, open_popup, open_popup_at_element, open_popup_at_mouse, UI_STATE};

#[derive(Clone, Copy, PartialEq)]
pub struct PopupContext;

impl PopupContext {
    pub fn open(&mut self, position: (f64, f64), content: Element) {
        open_popup(position, content);
    }

    pub fn open_at_element(&mut self, rect: web_sys::DomRect, content: Element) {
        open_popup_at_element(rect, content);
    }

    pub fn open_at_mouse(&mut self, client_x: f64, client_y: f64, content: Element) {
        open_popup_at_mouse(client_x, client_y, content);
    }

    pub fn close(&mut self) {
        close_popup();
    }
}

pub fn use_popup() -> PopupContext {
    PopupContext
}

#[component]
pub fn PopupProvider(children: Element) -> Element {
    let ui_state = UI_STATE.read();
    let is_open = ui_state.popup.is_open;
    let position = ui_state.popup.position;
    let content = ui_state.popup.content.clone();

    rsx! {
        {children}
        if is_open {
            div {
                class: "popup-overlay",
                onclick: move |_| {
                    close_popup();
                },
                div {
                    class: "popup-container",
                    style: "left: {position.0}px; top: {position.1}px;",
                    onclick: move |e| e.stop_propagation(),
                    {content}
                }
            }
        }
    }
}

#[component]
pub fn PopupMenu(children: Element) -> Element {
    rsx! {
        div { class: "popup-menu", {children} }
    }
}

#[component]
pub fn PopupMenuItem(
    onclick: EventHandler<MouseEvent>,
    children: Element,
    #[props(default = false)] danger: bool,
    #[props(default = false)] primary: bool,
    #[props(default = false)] disabled: bool,
) -> Element {
    let class = match (danger, primary) {
        (true, _) => "popup-menu-item danger",
        (_, true) => "popup-menu-item primary",
        _ => "popup-menu-item",
    };

    rsx! {
        button {
            class: "{class}",
            disabled,
            onclick: move |e| onclick.call(e),
            {children}
        }
    }
}

// ============== Global Modal Provider ==============

#[component]
pub fn GlobalModalProvider(children: Element) -> Element {
    let ui_state = UI_STATE.read();
    let is_open = ui_state.modal.is_open;
    let title = ui_state.modal.title.clone();
    let content = ui_state.modal.content.clone();

    rsx! {
        {children}
        if is_open {
            div {
                class: "modal-overlay",
                onclick: move |_| {
                    crate::state::close_modal();
                },
                div {
                    class: "modal-container",
                    onclick: move |e| e.stop_propagation(),
                    if !title.is_empty() {
                        div { class: "modal-header",
                            h3 { "{title}" }
                            button {
                                class: "modal-close-btn",
                                onclick: move |_| {
                                    crate::state::close_modal();
                                },
                                "×"
                            }
                        }
                    }
                    div { class: "modal-body custom-scrollbar", {content} }
                }
            }
        }
    }
}
