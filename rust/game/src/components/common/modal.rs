use crate::prelude::*;

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
