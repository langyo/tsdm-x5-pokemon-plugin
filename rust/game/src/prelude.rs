// 不依赖 dioxus 元 crate：元 crate 会把 fullstack/server/desktop 等可选依赖
// （axum、tower、quinn 等）整体锁入 Cargo.lock。本项目后端依托论坛 PHP，
// 不设 Rust 服务端，因此从细粒度子 crate 自行组装等效 prelude。
// 注意：rsx!/Routable 宏生成的代码以 dioxus_core 为路径根、以 dioxus::config_macros
// 调用 wasm_split 垫片，因此这些 crate 必须保持直接依赖且名字可达。
// dioxus_core 的 Ok/Result 是错误捕获专用别名，不能整包导入（会遮蔽 std 的 Ok）。
pub use dioxus_core::{
    consume_context, spawn, Component, Element, Event, EventHandler, Fragment, VNode,
};
pub use dioxus_core_macro::{component, rsx, Props};
pub use dioxus_elements::{
    events::*, extensions::*, global_attributes, keyboard_types, svg_attributes, traits::*, Code,
    GlobalAttributesExtension, Key, Location, Modifiers, SvgAttributesExtension,
};
pub use dioxus_hooks::*;
pub use dioxus_html as dioxus_elements;
pub use dioxus_router::{
    hooks::*, navigator, use_navigator, GoBackButton, GoForwardButton, Link, NavigationTarget,
    Outlet, Routable, Router,
};
pub use dioxus_signals::*;

// Routable 派生宏生成的代码硬编码 dioxus::config_macros::maybe_wasm_split! 路径，
// 用同名垫片模块满足解析（元 crate 不可用时的等价物）。
pub mod dioxus_shim {
    pub use dioxus_config_macros as config_macros;
}
pub use dioxus_shim as dioxus;
