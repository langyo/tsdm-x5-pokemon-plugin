//! 后台外壳：hikari 的 `AdminShell`（文档模式）+ `AdminHeader` + `NavSidebar`
//! + 状态栏 + Toast 宿主。
//!
//! 与 shittim-chest 的 `AdminLayout`、ayane 的 `ShellLayout` 是同一种壳——壳
//! 独占整个视口（后台是独立页面，见 `plugin/admin_v2.inc.php`）：
//!
//! ```text
//! AdminShell                        = hikari FaceShell，文档模式（内容区自带滚动）
//!   ├─ header    AdminHeader        标题 = 当前页；头像菜单：返回论坛 / 宠物中心 / 旧版后台 / 退出
//!   ├─ sidebar   NavSidebar         八个数据页，一行一条路由
//!   ├─ content   当前页
//!   ├─ footer    状态栏三个区块      插件身份与版本 · 连接状态 · 界面切换
//!   └─ overlays  头像下拉的遮罩、弹窗、Toast
//! ```
//!
//! hikari 的壳组件自己不存状态（`FaceShell` / `AdminHeader` 头注的
//! STATE CONTRACT）：抽屉开合、头像下拉、视口宽度全是受控 props，由
//! [`ShellState`] 持有，这里只做两件事——把状态映射成 props、把回调写回状态。
//!
//! 桌面 / 移动由 `viewport_width` 对 `MOBILE_BREAKPOINT` 决定：桌面挂侧栏 +
//! 头像下拉；移动端侧栏进抽屉（`mobile_open`），头像变成抽屉的开关，账号
//! 菜单挪到抽屉底部的 `user_panel`——与 shittim-chest 的两张脸一致。

// rsx! 的子节点位置上，VNode 要走 `..vec![node]` 展开，函数调用式组件
// （`{Icon(IconProps { .. })}`）要带花括号；纯标识符裸写会被当成属性。

use hikari_components::feedback::popover::PopoverAnchorRect;
use hikari_components::prelude::{Icon, IconProps, MdiIcon};
use hikari_components::{
    AdminHeader, AdminHeaderAvatarAction, AdminHeaderGroup, AdminHeaderProps,
    admin_header_avatar_initial, AdminShell, AdminShellProps, Badge, BadgeProps, BadgeSize,
    BadgeVariant, NavSidebar, NavSidebarItem, NavSidebarProps, StatusBar, StatusBarConnection,
    StatusBarProps, Toast, ToastProps,
};
use tairitsu_macros::rsx;
use tairitsu_vdom::{Callback, EventHandler, VNode, VText};

use crate::state::{AdminRoute, AppState, MenuAnchor, ShellState};
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::{
    location, platform_helpers,
};

/// hikari 壳的桌面 / 移动分界（`FaceShell` 的 `mobile_breakpoint` 默认值）。
const MOBILE_BREAKPOINT: f64 = 1024.0;

fn is_desktop(viewport_width: f64) -> bool {
    // 0 = 尚未测量：按桌面渲染，首帧不闪抽屉（hikari `face_shell_is_desktop`）。
    viewport_width <= 0.0 || viewport_width >= MOBILE_BREAKPOINT
}

fn go(url: &str) {
    location::set_href(url);
}

// ---------------------------------------------------------------------------
// 导航
// ---------------------------------------------------------------------------

/// 侧栏一行 = 一条路由。图标是 hikari `nav_sidebar_icon` 表里的名字键。
fn nav_items() -> Vec<NavSidebarItem> {
    const ROWS: [(AdminRoute, &str); 8] = [
        (AdminRoute::GlobalConfig, "settings"),
        (AdminRoute::PokemonData, "bot"),
        (AdminRoute::ItemData, "box"),
        (AdminRoute::MapData, "chart"),
        (AdminRoute::UserData, "panels"),
        (AdminRoute::EvolutionData, "link"),
        (AdminRoute::SkillType, "zap"),
        (AdminRoute::EffectData, "share"),
    ];
    ROWS.iter()
        .map(|(route, icon)| NavSidebarItem {
            key: route.key().to_string(),
            icon: icon.to_string(),
            label: route.title().to_string(),
            route: route.key().to_string(),
            ..Default::default()
        })
        .collect()
}

fn route_of_key(key: &str) -> Option<AdminRoute> {
    const ALL: [AdminRoute; 8] = [
        AdminRoute::GlobalConfig,
        AdminRoute::PokemonData,
        AdminRoute::ItemData,
        AdminRoute::MapData,
        AdminRoute::UserData,
        AdminRoute::EvolutionData,
        AdminRoute::SkillType,
        AdminRoute::EffectData,
    ];
    ALL.into_iter().find(|route| route.key() == key)
}

/// 侧栏面板。`NavSidebar` 只渲染导航带（一行一条路由），不自带面板外壳
/// （见其头注的 delta）；面板的宽度、底色、分隔线由 `FaceShell` 的 `<aside>`
/// 给，这里补的是内边距与滚动。
fn sidebar(state: &AppState) -> VNode {
    let nav_state = state.clone();
    let sidebar = NavSidebar(NavSidebarProps {
        items: nav_items(),
        current_route: state.route.get().key().to_string(),
        on_navigate: Some(EventHandler::new(move |key: String| {
            if let Some(target) = route_of_key(&key) {
                crate::pages::switch_route(&nav_state, target);
            }
            // 移动端：选中一条即收起抽屉（桌面端这个标志无人读）。
            nav_state.shell.mobile_open.inner().set(false);
            crate::request_remount();
        })),
        ..Default::default()
    });
    rsx! {
        div { class: "admin-nav-panel", ..vec![sidebar] }
    }
}

// ---------------------------------------------------------------------------
// 账号菜单
// ---------------------------------------------------------------------------

/// 一行账号菜单项，沿用 hikari `HkMenu` 的行语法
/// （`button.hk-menu-row > span.hk-menu-item-icon + span.hk-menu-label`），
/// 头像下拉与移动端抽屉共用。
fn menu_row(
    icon: MdiIcon,
    label: &'static str,
    danger: bool,
    on_pick: impl Fn() + 'static,
) -> VNode {
    rsx! {
        button {
            "type": "button",
            class: "hk-menu-row",
            "data-danger": if danger { Some("true") } else { None },
            onclick: move |event: tairitsu_vdom::MouseEvent| {
                event.stop_propagation();
                on_pick();
            },
            span {
                class: "hk-menu-item-icon",
                {Icon(IconProps {
                    icon,
                    size: 16,
                    ..Default::default()
                })}
            }
            span { class: "hk-menu-label", "{label}" }
        }
    }
}

/// 账号菜单的四个去处，点了即整页跳转。`in_drawer` 只决定跳走之前先收起
/// 哪一面——下拉里收下拉，抽屉里合抽屉。
fn account_rows(state: &AppState, in_drawer: bool) -> Vec<VNode> {
    let boot = state.shell.boot.clone();
    let leave = |url: String, shell: ShellState| {
        move || {
            if in_drawer {
                shell.mobile_open.inner().set(false);
            } else {
                shell.user_menu_open.inner().set(false);
            }
            crate::request_remount();
            go(&url);
        }
    };
    let shell = state.shell.clone();
    vec![
        menu_row(
            MdiIcon::ArrowExpandHorizontal,
            "返回论坛",
            false,
            leave(boot.forum_url.clone(), shell.clone()),
        ),
        menu_row(
            MdiIcon::Home,
            "宠物中心",
            false,
            leave(boot.game_url.clone(), shell.clone()),
        ),
        menu_row(
            MdiIcon::Book,
            "旧版后台",
            false,
            leave(boot.legacy_url.clone(), shell.clone()),
        ),
        menu_row(
            MdiIcon::ArrowRight,
            "退出登录",
            true,
            leave(boot.logout_url.clone(), shell.clone()),
        ),
    ]
}

/// 身份块（头像 + 用户名 + 用户组），与 `AdminHeader` 下拉顶部的同一套
/// 标记；抽屉里没有那个下拉，由宿主自己渲染一份。
fn identity_block(state: &AppState) -> VNode {
    let boot = state.shell.boot.clone();
    let group = boot.user_group.clone();
    let name = boot.user_name.clone();
    let avatar = boot.avatar.clone();
    let failed = state.shell.avatar_failed.get();
    let show_img = !avatar.is_empty() && !failed;
    let initial = admin_header_avatar_initial(&name);
    let badge = if group.is_empty() {
        VNode::empty()
    } else {
        Badge(BadgeProps {
            variant: BadgeVariant::Primary,
            size: BadgeSize::Sm,
            children: VNode::Text(VText::new(group.as_str())),
            ..Default::default()
        })
    };
    rsx! {
        div { class: "s-user-header s-user-header--profile",
            span { class: "s-user-avatar-chip", "aria-hidden": "true",
                if show_img {
                    img { class: "s-user-avatar-img", src: avatar, alt: "" }
                } else {
                    span { class: "s-user-avatar", "{initial}" }
                }
            }
            div { class: "s-user-header-body",
                div { class: "s-user-header-name", "{name}" }
                div { class: "s-user-header-groups", ..vec![badge] }
            }
        }
    }
}

/// 移动端抽屉底部的账号区：身份块 + 菜单行（Vue 的 `userPanel` 槽）。
fn user_panel(state: &AppState) -> VNode {
    let rows = account_rows(state, true);
    let identity = identity_block(state);
    rsx! {
        div { class: "admin-drawer-user",
            div { class: "hk-menu-slot hk-menu-slot--head", ..vec![identity] }
            div { class: "hk-menu-level", "role": "menu", ..rows }
        }
    }
}

fn close_menu(shell: &ShellState) {
    shell.user_menu_open.inner().set(false);
    crate::request_remount();
}

fn open_drawer(shell: &ShellState) {
    shell.mobile_open.inner().set(true);
    crate::request_remount();
}

/// 打开下拉前量一次头像按钮的矩形：hikari 的下拉是 `position: fixed`，
/// 锚点由宿主给（`AdminHeaderProps::menu_anchor`），没有锚点就只能贴在
/// 头部左缘。
fn measure_avatar_anchor() -> Option<MenuAnchor> {
    crate::hk::measure(".s-admin-header-avatar")
}

fn set_menu_open(shell: &ShellState, open: bool) {
    if open {
        shell.menu_anchor.inner().set(measure_avatar_anchor());
    }
    shell.user_menu_open.inner().set(open);
    crate::request_remount();
}

/// 头部：标题 = 当前页（「我在哪」，不是「我是谁」——身份在头像菜单里）。
/// 下拉里的菜单行由宿主给（`user_menu` 覆盖内置行：hikari 内置的是头像
/// 编辑 / 语言 / 退出，这里要的是论坛侧的四个去处）。
fn header(state: &AppState, desktop: bool) -> VNode {
    let sh = state.shell.clone();
    let boot = sh.boot.clone();
    let group = boot.user_group.clone();

    let hamburger_state = sh.clone();
    let drawer_avatar_state = sh.clone();
    let menu_state = sh.clone();
    let avatar_error_state = sh.clone();
    let menu_avatar_error_state = sh.clone();

    let rows = account_rows(state, false);
    let user_menu = rsx! {
        div { class: "hk-menu-level", "role": "menu", ..rows }
    };

    AdminHeader(AdminHeaderProps {
        title: if desktop {
            state.route.get().title().to_string()
        } else {
            String::new()
        },
        compact: !desktop,
        show_hamburger: !desktop,
        on_hamburger: Some(EventHandler::new(move |_event: ()| {
            open_drawer(&hamburger_state);
        })),
        username: boot.user_name.clone(),
        avatar_url: boot.avatar.clone(),
        user_groups: if group.is_empty() {
            Vec::new()
        } else {
            vec![AdminHeaderGroup {
                id: "staff".to_string(),
                name: group,
            }]
        },
        avatar_action: if desktop {
            AdminHeaderAvatarAction::Menu
        } else {
            AdminHeaderAvatarAction::Drawer
        },
        avatar_trigger_label: Some("账号菜单".to_string()),
        avatar_failed: sh.avatar_failed.get(),
        on_avatar_error: Some(EventHandler::new(move |_event: ()| {
            avatar_error_state.avatar_failed.inner().set(true);
            crate::request_remount();
        })),
        user_menu_open: sh.user_menu_open.get() && desktop,
        menu_anchor: sh.menu_anchor.get().map(|anchor| PopoverAnchorRect {
            x: anchor.x,
            y: anchor.y,
            width: anchor.width,
            height: anchor.height,
        }),
        on_avatar_click: Some(EventHandler::new(move |_event: ()| {
            open_drawer(&drawer_avatar_state);
        })),
        on_user_menu_change: Some(EventHandler::new(move |open: bool| {
            set_menu_open(&menu_state, open);
        })),
        on_menu_avatar_error: Some(EventHandler::new(move |_event: ()| {
            menu_avatar_error_state.avatar_failed.inner().set(true);
            crate::request_remount();
        })),
        user_menu: Some(user_menu),
        ..Default::default()
    })
}

// ---------------------------------------------------------------------------
// 状态栏
// ---------------------------------------------------------------------------

/// 状态栏三个区块：左 = 插件身份与版本，中空，右 = 连接状态 + 界面切换。
/// 带子本身由 `FaceShell` 渲染（`footer.s-status-bar`），这里只填内容。
fn status_footer(state: &AppState) -> VNode {
    let boot = state.shell.boot.clone();
    let version = if boot.plugin_version.is_empty() {
        String::new()
    } else {
        format!("v{}", boot.plugin_version)
    };
    let status_version = if boot.plugin_version.is_empty() {
        "宠物插件".to_string()
    } else {
        format!("宠物插件 {}", boot.plugin_version)
    };
    let legacy_url = boot.legacy_url.clone();

    let left = rsx! {
        div { class: "s-status-bar-left",
            span { class: "admin-status-brand", "天使动漫 · 宠物插件后台" }
            if !version.is_empty() {
                span { class: "admin-status-version", "{version}" }
            }
        }
    };
    let center = rsx! {
        div { class: "s-status-bar-center" }
    };
    let right = rsx! {
        div { class: "s-status-bar-right",
            {StatusBar(StatusBarProps {
                standalone: false,
                version: status_version,
                connection_status: StatusBarConnection::Connected,
                ..Default::default()
            })}
            // 迁移期的逃生口：Dioxus 旧版后台（套论坛版式）。
            a {
                class: "admin-status-link",
                href: legacy_url,
                "data-test-adm-legacy-link": "1",
                "旧版界面"
            }
        }
    };
    VNode::Fragment(vec![left, center, right])
}

// ---------------------------------------------------------------------------
// 外壳
// ---------------------------------------------------------------------------

/// 提示宿主：hikari 的共享提示栈（`use_toast()` 推入）在这里统一渲染。
fn toast_host() -> VNode {
    Toast(ToastProps {
        on_copy: Some(Callback::new(|text: String| {
            platform_helpers::copy_to_clipboard(&text);
        })),
        ..Default::default()
    })
}

/// 把当前页与浮层装进壳。`content` 是页面正文，`overlays` 是浮在壳之上的
/// 弹窗（各页的编辑器、删除确认）。
pub fn render(state: &AppState, content: VNode, mut overlays: Vec<VNode>) -> VNode {
    let sh = state.shell.clone();
    let desktop = is_desktop(sh.viewport_width.get());

    // 头像下拉打开时铺一层透明遮罩：点到别处即收起。hikari 的下拉没有
    // 「点外部关闭」引擎（见 AdminHeader 头注），这一步留给宿主；层级压在
    // 头部之下，所以头部自己的控件照常可点。
    if desktop && sh.user_menu_open.get() {
        let backdrop_state = sh.clone();
        overlays.insert(
            0,
            rsx! {
                div {
                    class: "admin-menu-backdrop",
                    "data-test-adm-menu-backdrop": "1",
                    onclick: move |_| close_menu(&backdrop_state),
                }
            },
        );
    }
    overlays.push(toast_host());

    let mobile_state = sh.clone();
    AdminShell(AdminShellProps {
        viewport_width: sh.viewport_width.get(),
        mobile_open: sh.mobile_open.get(),
        nav_title: Some("宠物插件后台".to_string()),
        // 桌面侧栏不折叠成图标栏：折叠要宿主自己实现悬停 peek 状态机
        // （见 FaceShell 头注），先按 ayane 的做法保持展开。
        sidebar_collapsible: false,
        sidebar_width: "224px".to_string(),
        header: Some(header(state, desktop)),
        sidebar: Some(sidebar(state)),
        user_panel: Some(user_panel(state)),
        content,
        footer: Some(status_footer(state)),
        on_mobile_open_change: Some(EventHandler::new(move |open: bool| {
            mobile_state.mobile_open.inner().set(open);
            crate::request_remount();
        })),
        overlays: Some(VNode::Fragment(overlays)),
        ..Default::default()
    })
}
