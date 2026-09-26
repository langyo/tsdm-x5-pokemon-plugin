use crate::prelude::*;

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

#[derive(Clone)]
pub struct ModalContext {
    pub state: Signal<ModalState>,
}

impl ModalContext {
    pub fn open(&mut self, title: impl Into<String>, content: Element) {
        self.state.set(ModalState {
            is_open: true,
            title: title.into(),
            content,
        });
    }

    pub fn close(&mut self) {
        self.state.set(ModalState::default());
    }
}

pub fn use_modal() -> ModalContext {
    let state = use_signal(ModalState::default);
    ModalContext { state }
}

#[component]
pub fn ModalProvider(children: Element) -> Element {
    let modal = use_modal();

    use_context_provider(|| modal.clone());

    let modal_state = modal.state.read();
    let is_open = modal_state.is_open;
    let title = modal_state.title.clone();
    let content = modal_state.content.clone();

    rsx! {
        {children}
        if is_open {
            {
                let ctx = consume_context::<ModalContext>();
                let ctx2 = ctx.clone();
                rsx! {
                    div {
                        class: "modal-overlay",
                        onclick: move |_| {
                            let mut c = ctx.clone();
                            c.close();
                        },
                        div { class: "modal-container", onclick: move |e| e.stop_propagation(),
                            div { class: "modal-header",
                                h3 { "{title}" }
                                button {
                                    class: "modal-close-btn",
                                    onclick: move |_| {
                                        let mut c = ctx2.clone();
                                        c.close();
                                    },
                                    "×"
                                }
                            }
                            div { class: "modal-body custom-scrollbar", {content} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Modal(
    is_open: bool,
    #[props(default = true)] close_on_overlay: bool,
    on_close: EventHandler<()>,
    title: String,
    children: Element,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let overlay_handler = move |_| {
        if close_on_overlay {
            on_close.call(());
        }
    };

    rsx! {
        div { class: "modal-overlay", onclick: overlay_handler,
            div {
                class: "modal-container",
                onclick: move |e| e.stop_propagation(),
                if !title.is_empty() {
                    div { class: "modal-header",
                        h3 { "{title}" }
                        button {
                            class: "modal-close-btn",
                            onclick: move |_| on_close.call(()),
                            "×"
                        }
                    }
                }
                div { class: "modal-body custom-scrollbar", {children} }
            }
        }
    }
}

#[component]
pub fn ModalLg(
    is_open: bool,
    #[props(default = true)] close_on_overlay: bool,
    on_close: EventHandler<()>,
    title: String,
    children: Element,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let overlay_handler = move |_| {
        if close_on_overlay {
            on_close.call(());
        }
    };

    rsx! {
        div { class: "modal-overlay", onclick: overlay_handler,
            div {
                class: "modal-container modal-lg",
                onclick: move |e| e.stop_propagation(),
                if !title.is_empty() {
                    div { class: "modal-header",
                        h3 { "{title}" }
                        button {
                            class: "modal-close-btn",
                            onclick: move |_| on_close.call(()),
                            "×"
                        }
                    }
                }
                div { class: "modal-body custom-scrollbar", {children} }
            }
        }
    }
}
