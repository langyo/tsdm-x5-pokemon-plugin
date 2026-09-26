use crate::dioxus::prelude::*;

#[component]
pub fn CodeEditor(
    id: String,
    value: String,
    language: String,
    height: Option<i32>,
    placeholder: Option<String>,
    onchange: EventHandler<String>,
) -> Element {
    let height = height.unwrap_or(200);
    let lines: Vec<usize> = value.lines().enumerate().map(|(i, _)| i + 1).collect();
    let line_count = lines.len().max(1);

    rsx! {
        div {
            class: "code-editor",
            style: "height: {height}px;",
            div {
                class: "code-editor__gutter",
                style: "height: {height}px;",
                for line_num in 1..=line_count {
                    div {
                        class: "code-editor__line-number",
                        "{line_num}"
                    }
                }
            }
            textarea {
                class: "code-editor__textarea",
                id: "{id}",
                style: "height: {height}px;",
                placeholder: placeholder.unwrap_or_default(),
                value: "{value}",
                spellcheck: "false",
                autocomplete: "off",
                autocorrect: "off",
                autocapitalize: "off",
                oninput: move |evt| {
                    onchange.call(evt.value());
                },
            }
        }
    }
}
