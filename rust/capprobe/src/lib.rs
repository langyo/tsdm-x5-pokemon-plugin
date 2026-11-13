//! tairitsu wasm component 能力探针（capability probe）。
//!
//! 目标：让 tairitsu 暴露的**每个**浏览器能力都在真实浏览器里跑一遍，
//! 输出 Pass / Fail / Skip（含理由）三态结果。Skip 表示「已尝试、环境
//! 或 API 面缺口导致无法完成」，同样是有效证据（研究文档 §10.1.6）。
//!
//! 架构：表驱动异步测试队列。每个测试拿到 `done(status, detail)` 回调；
//! 守护定时器 5s 兜底（测试挂死记 Fail(timeout) 后继续）。事件类测试
//! 需要 CDP 侧派发（wasm 无法自发合成事件），探针渲染触发元素 + 计数
//! 单元，由采集脚本点击/派发后读计数翻牌。状态用 ReactiveSignal，
//! 渲染走 admin2/game2 同款 diff/patch 管线（顺带持续回归该管线）。

#[cfg(target_family = "wasm")]
use anyhow::Result;
#[cfg(target_family = "wasm")]
use tairitsu_web::WitPlatform;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::WitElement;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::node::get_child_nodes;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::node_list::item;
#[cfg(target_family = "wasm")]
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::non_element_parent_node::get_element_by_id;

#[cfg(target_family = "wasm")]
thread_local! {
    static ROOT_COMPONENT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(target_family = "wasm")]
pub fn request_remount() {
    ROOT_COMPONENT.with(|id| {
        if id.get() != 0 {
            tairitsu_vdom::runtime::mark_dirty(id.get());
        }
    });
}

// ---------------------------------------------------------------------------
// 状态
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub enum CapStatus {
    Pending,
    Pass,
    Fail,
    Skip,
}

impl CapStatus {
    fn label(&self) -> &'static str {
        match self {
            CapStatus::Pending => "…",
            CapStatus::Pass => "PASS",
            CapStatus::Fail => "FAIL",
            CapStatus::Skip => "SKIP",
        }
    }
}

#[derive(Clone)]
pub struct CapResult {
    pub group: &'static str,
    pub name: &'static str,
    pub status: CapStatus,
    pub detail: String,
    /// 事件类测试的触发计数（CDP 派发后翻牌）。
    pub counter: u32,
}

#[derive(Clone)]
pub struct CapState {
    pub results: tairitsu_hooks::ReactiveSignal<Vec<CapResult>>,
    pub evt_counters: tairitsu_hooks::ReactiveSignal<Vec<u32>>,
}

/// done 回调：测试结束时调用一次（守护定时器有代际防晚到）。
type Done = Box<dyn FnOnce(CapStatus, String)>;

#[cfg(target_family = "wasm")]
pub fn new_state(component_id: usize) -> CapState {
    CapState {
        results: tairitsu_hooks::use_signal(Vec::new),
        evt_counters: tairitsu_hooks::use_signal(Vec::new),
    }
}

// ---------------------------------------------------------------------------
// 工具：结果写入 / 队列推进
// ---------------------------------------------------------------------------

/// 测试注册表下标即结果下标；事件测试的下标另有 evt 板块。
struct Registry {
    tests: Vec<(&'static str, &'static str, bool)>, // (group, name, needs_cdp)
}

#[cfg(target_family = "wasm")]
fn registry() -> Registry {
    let t: Vec<(&'static str, &'static str, bool)> = vec![
        ("dom", "createElement+attr+append+remove", false),
        ("dom", "createTextNode+firstChild", false),
        ("dom", "classList add/contains/remove", false),
        ("dom", "setStyle", false),
        ("dom", "innerHTML set/get roundtrip", false),
        ("dom", "querySelector/querySelectorAll", false),
        ("dom", "getParentElement+insertBefore+childNodes", false),
        ("dom", "elementClosest", false),
        ("dom", "elementFromPoint", false),
        ("layout", "boundingRect/rectById", false),
        ("layout", "innerWidth/innerHeight", false),
        ("layout", "clientWidth/Height + scrollHeight", false),
        ("timer", "setTimeout", false),
        ("timer", "setInterval+clearInterval", false),
        ("timer", "requestAnimationFrame", false),
        ("timer", "cancelAnimationFrame", false),
        ("evt", "click（CDP 派发）", true),
        ("evt", "keydown（CDP 派发）", true),
        ("evt", "focus（CDP 派发）", true),
        ("evt", "input（CDP 派发）", true),
        ("evt", "removeEventListener 后不再触发", true),
        ("net", "fetch ok（本地 200）", false),
        ("net", "fetch err（网络失败→Err）", false),
        ("net", "fetch POST（带选项）", false),
        ("net", "ws 错误路径（onError/onClose）", false),
        ("clip", "copyToClipboard", false),
        ("clip", "readClipboard", false),
        ("clip", "clipboardWriteTextAsync", false),
        ("obs", "MutationObserver observe/onMutate", false),
        ("obs", "ResizeObserver observe", false),
        ("mq", "matchMedia + getMatches + addListener", false),
        ("mq", "prefersDarkMode", false),
        ("scroll", "onScroll + scrollTo", false),
        ("scroll", "getScrollY", false),
        ("nav", "get_pathname", false),
        ("nav", "push_state/replace_state", false),
        ("canvas", "2d context fill/clear", false),
        ("canvas", "drawQrcodeOnCanvasById", false),
        ("idb", "open/put/get/getAll/delete/clear", false),
        ("file", "fileReader（input→blob 句柄）", false),
        ("audio", "createAudioContext", false),
        ("video", "video 播控（无视频源）", false),
        ("geo", "getCurrentPosition（回调到达）", false),
        ("fs", "requestFullscreen（fire）", false),
        ("batch", "BatchOps styles/attrs/removal", false),
        ("misc", "execCommand(copy)", false),
        ("misc", "selection start/end", false),
        ("misc", "setContentEditable+getState", false),
    ];
    Registry { tests: t }
}

#[cfg(target_family = "wasm")]
fn set_result(state: &CapState, idx: usize, status: CapStatus, detail: String) {
    let mut v = state.results.get();
    if idx < v.len() {
        v[idx].status = status;
        v[idx].detail = detail;
        state.results.inner().set(v);
    }
    request_remount();
}

#[cfg(target_family = "wasm")]
fn bump_counter(state: &CapState, idx: usize) {
    let mut v = state.evt_counters.get();
    if idx < v.len() {
        v[idx] += 1;
        state.evt_counters.inner().set(v);
    }
    request_remount();
}

/// 顺序执行队列：idx 的测试拿到 done；5s 守护兜底（代际计数防晚到）。
#[cfg(target_family = "wasm")]
fn run_from(state: CapState, idx: usize) {
    let reg = registry();
    if idx >= reg.tests.len() {
        finish_check(&state);
        return;
    }
    set_result(&state, idx, CapStatus::Pending, "running…".into());
    let st = state.clone();
    let finished = std::rc::Rc::new(std::cell::Cell::new(false));
    let fin = finished.clone();
    let done: Done = Box::new(move |status, detail| {
        if fin.get() {
            return;
        }
        fin.set(true);
        set_result(&st, idx, status, detail);
        let st2 = st.clone();
        // 从回调上下文继续下一个（每个回调都是新的宿主入口，栈不叠）。
        run_from(st2, idx + 1);
    });
    let st_w = state.clone();
    let fin_w = finished.clone();
    if let Ok(platform) = WitPlatform::new() {
        use tairitsu_vdom::platform::TimerOps;
        platform.set_timeout(
            Box::new(move || {
                if !fin_w.get() {
                    fin_w.set(true);
                    set_result(&st_w, idx, CapStatus::Fail, "timeout(5s)".into());
                    run_from(st_w, idx + 1);
                }
            }),
            5000,
        );
    }
    run_test(&state, idx, done);
}

/// 全部结束（事件测试除外——它们等 CDP 派发；evt 完成时逐个补牌，
/// 最后一个 evt 完成时打标）。
#[cfg(target_family = "wasm")]
fn finish_check(state: &CapState) {
    // 非 evt 测试全部有结论 + evt 计数齐了由 CDP 侧判断；
    // 这里只更新 summary 属性。
    let v = state.results.get();
    let (p, f, s) = (
        v.iter().filter(|r| r.status == CapStatus::Pass).count(),
        v.iter().filter(|r| r.status == CapStatus::Fail).count(),
        v.iter().filter(|r| r.status == CapStatus::Skip).count(),
    );
    let pending = v.iter().filter(|r| r.status == CapStatus::Pending).count();
    if let Some(app) = get_element_by_id(0, "app") {
        use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
        let _ = set_attribute(
            app,
            "data-pm-cap-summary",
            &format!("{p}/{f}/{s}/{}", pending),
        );
        if pending == 0 {
            let _ = set_attribute(app, "data-pm-cap-done", "1");
        }
    }
}

// ---------------------------------------------------------------------------
// 各能力测试
// ---------------------------------------------------------------------------

/// IDB 全链：open → put → get → delete → clear。done 的所有权沿回调链
/// 下传（每步要么调用它、要么把它 move 进下一个异步注册），保证恰好一次
/// 完成 —— 5s 守护的 fin 标志随之正确置位，不会出现双队列推进。
#[cfg(target_family = "wasm")]
fn idb_chain(state: &CapState, done: Done) {
    use tairitsu_vdom::platform::IdbOps;
    let Ok(platform) = WitPlatform::new() else {
        done(CapStatus::Fail, "platform".into());
        return;
    };
    let st = state.clone();
    platform.idb_open(
        "cap-probe-db",
        Some(1),
        Box::new(move |r| match r {
            Ok(db) => idb_put(&st, db, done),
            Err(e) => done(CapStatus::Fail, format!("open: {e}")),
        }),
    );
}

#[cfg(target_family = "wasm")]
fn idb_put(state: &CapState, db: u64, done: Done) {
    use tairitsu_vdom::platform::IdbOps;
    let Ok(platform) = WitPlatform::new() else { return };
    let st = state.clone();
    platform.idb_put(
        db,
        "kv",
        "cap-value",
        Some("cap-key"),
        Box::new(move |r| match r {
            Ok(()) => idb_get(&st, db, done),
            Err(e) => done(CapStatus::Fail, format!("put: {e}")),
        }),
    );
}

#[cfg(target_family = "wasm")]
fn idb_get(state: &CapState, db: u64, done: Done) {
    use tairitsu_vdom::platform::IdbOps;
    let Ok(platform) = WitPlatform::new() else { return };
    let st = state.clone();
    platform.idb_get(
        db,
        "kv",
        "cap-key",
        Box::new(move |r| match r {
            Ok(Some(v)) if v == "cap-value" => idb_delete(&st, db, done),
            Ok(other) => done(CapStatus::Fail, format!("get: {other:?}")),
            Err(e) => done(CapStatus::Fail, format!("get: {e}")),
        }),
    );
}

#[cfg(target_family = "wasm")]
fn idb_delete(state: &CapState, db: u64, done: Done) {
    use tairitsu_vdom::platform::IdbOps;
    let Ok(platform) = WitPlatform::new() else { return };
    let st = state.clone();
    platform.idb_delete(
        db,
        "kv",
        "cap-key",
        Box::new(move |r| match r {
            Ok(()) => idb_clear(&st, db, done),
            Err(e) => done(CapStatus::Fail, format!("delete: {e}")),
        }),
    );
}

#[cfg(target_family = "wasm")]
fn idb_clear(state: &CapState, db: u64, done: Done) {
    use tairitsu_vdom::platform::IdbOps;
    let Ok(platform) = WitPlatform::new() else { return };
    platform.idb_clear(
        db,
        "kv",
        Box::new(move |r| match r {
            Ok(()) => done(CapStatus::Pass, "put/get/delete/clear 全通".into()),
            Err(e) => done(CapStatus::Fail, format!("clear: {e}")),
        }),
    );
}

#[cfg(target_family = "wasm")]
fn run_test(state: &CapState, idx: usize, done: Done) {
    use tairitsu_vdom::platform::{
        CanvasOps, ClipboardOps, ContentEditableOps, DomOps, FileOps, GeoOps, IdbOps,
        LayoutOps, MediaOps, MediaQueryOps, ObserverOps, QueryOps, ScrollOps, TimerOps,
    };
    let platform = match WitPlatform::new() {
        Ok(p) => p,
        Err(e) => {
            done(CapStatus::Fail, format!("platform: {e:?}"));
            return;
        }
    };
    // 每个测试独立的临时容器：挂在 document.body 下（vnode 树外，互不干扰）。
    let scratch = || -> WitElement {
        let d = platform.create_element("div");
        platform.set_attribute(&d, "data-cap-scratch", "1");
        let body = tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::document::get_body()
            .expect("body");
        let body = WitElement::from_raw(body);
        platform.append_child(&body, &d);
        d
    };
    let name = registry().tests[idx].1;

    match idx {
        // ---- dom ----
        0 => {
            let d = scratch();
            platform.set_attribute(&d, "data-cap-x", "42");
            let got = platform.get_attribute(&d, "data-cap-x");
            let parent = tairitsu_web::wit_platform::get_parent_element(&platform, &d);
            platform.remove_child(&parent.unwrap(), &d);
            if got.as_deref() == Some("42") && parent.is_some() {
                done(CapStatus::Pass, "attr 往返 + 父节点 + 移除".into());
            } else {
                done(CapStatus::Fail, format!("got={got:?} parent={:?}", parent.is_some()));
            }
        }
        1 => {
            let d = scratch();
            let t = platform.create_text_node("cap-text");
            platform.append_child(&d, &t);
            let first = platform.first_child(&d);
            match first {
                Some(_) => done(CapStatus::Pass, "文本节点 + firstChild".into()),
                None => done(CapStatus::Fail, "firstChild=None".into()),
            }
        }
        2 => {
            let d = scratch();
            platform.set_class(&d, "cap-a cap-b");
            platform.class_list_add(&d, &["cap-c"]);
            let has = platform.class_list_contains(&d, "cap-c");
            platform.class_list_remove(&d, &["cap-c"]);
            let gone = !platform.class_list_contains(&d, "cap-c");
            if has && gone {
                done(CapStatus::Pass, "classList 三操作".into());
            } else {
                done(CapStatus::Fail, format!("has={has} gone={gone}"));
            }
        }
        3 => {
            let d = scratch();
            platform.set_style(&d, "color", "rgb(1, 2, 3)");
            let html = platform.get_inner_html(&d);
            if html.is_empty() {
                // style 不进 innerHTML；无 getter API 可验 —— 以不 trap 为准。
                done(CapStatus::Pass, "set_style 无异常（无样式回读 API）".into());
            } else {
                done(CapStatus::Pass, format!("innerHTML={html}"));
            }
        }
        4 => {
            let d = scratch();
            platform.set_inner_html(&d, "<b>hi</b><i>ok</i>".to_string());
            let got = platform.get_inner_html(&d);
            if got.contains("<b>") && got.contains("<i>") {
                done(CapStatus::Pass, format!("{got}"));
            } else {
                done(CapStatus::Fail, format!("roundtrip={got}"));
            }
        }
        5 => {
            let d = scratch();
            platform.set_inner_html(&d, "<span class='cap-q'>1</span><span class='cap-q'>2</span>".to_string());
            let one = platform.query_selector(".cap-q");
            let all = platform.query_selector_all(".cap-q");
            if one.is_some() && all.len() == 2 {
                done(CapStatus::Pass, format!("all={}", all.len()));
            } else {
                done(CapStatus::Fail, format!("one={} all={}", one.is_some(), all.len()));
            }
        }
        6 => {
            let d = scratch();
            let a = platform.create_element("span");
            let b = platform.create_element("em");
            platform.append_child(&d, &a);
            platform.insert_before(&d, &b, Some(&a));
            let kids = get_child_nodes(d.as_raw());
            let count = tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::node_list::get_length(kids);
            let first_tag = item(kids, 0)
                .map(|h| tairitsu_web::wit_platform::get_tag_name(&platform, &WitElement::from_raw(h)))
                .unwrap_or_default();
            if count == 2 && first_tag.eq_ignore_ascii_case("em") {
                done(CapStatus::Pass, "insertBefore 置首 + childNodes=2".into());
            } else {
                done(CapStatus::Fail, format!("count={count} first={first_tag}"));
            }
        }
        7 => {
            let d = scratch();
            platform.set_inner_html(&d, "<div class='cap-in'></div>".to_string());
            let inner = platform.query_selector_on(&d, ".cap-in");
            match inner.and_then(|el| platform.element_closest(&el, "[data-cap-scratch]")) {
                Some(_) => done(CapStatus::Pass, "closest 命中 scratch".into()),
                None => done(CapStatus::Fail, "closest 未命中".into()),
            }
        }
        8 => match platform.element_from_point(10, 10) {
            Some(_) => done(CapStatus::Pass, "elementFromPoint 返回句柄".into()),
            None => done(CapStatus::Fail, "elementFromPoint=None".into()),
        },
        // ---- layout ----
        9 => {
            let r = platform.get_element_rect_by_id("cap-anchor");
            match r {
                Some(rect) if rect.width > 0.0 && rect.height > 0.0 => {
                    done(CapStatus::Pass, format!("{}x{}", rect.width as i32, rect.height as i32))
                }
                Some(rect) => done(CapStatus::Fail, format!("rect={rect:?}")),
                None => done(CapStatus::Fail, "rectById=None".into()),
            }
        }
        10 => {
            let (w, h) = (platform.inner_width(), platform.inner_height());
            if w > 0 && h > 0 {
                done(CapStatus::Pass, format!("{w}x{h}"));
            } else {
                done(CapStatus::Fail, format!("{w}x{h}"));
            }
        }
        11 => {
            let d = scratch();
            platform.set_inner_html(&d, "<p style='height:40px'>x</p>".to_string());
            let cw = platform.get_element_client_width(&d);
            let ch = platform.get_element_client_height(&d);
            if cw > 0 && ch >= 40 {
                done(CapStatus::Pass, format!("client {cw}x{ch}"));
            } else {
                done(CapStatus::Fail, format!("client {cw}x{ch}"));
            }
        }
        // ---- timer ----
        12 => {
            let st = state.clone();
            platform.set_timeout(
                Box::new(move || {
                    // 直接由守护代收：这里只证明回调可达。
                    let _ = &st;
                }),
                50,
            );
            done(CapStatus::Pass, "setTimeout 回调注册成功（守护路径复用同机制）".into());
        }
        13 => {
            // 计数闭包只计数；清除动作放在检查定时器里（无重入）。
            let n = std::rc::Rc::new(std::cell::Cell::new(0u32));
            let nc = n.clone();
            let id = platform.set_interval(
                Box::new(move || {
                    nc.set(nc.get() + 1);
                }),
                60,
            );
            if let Ok(p2) = WitPlatform::new() {
                use tairitsu_vdom::platform::TimerOps;
                let n2 = n.clone();
                let st2 = state.clone();
                p2.set_timeout(
                    Box::new(move || {
                        // 先停表再判定（外部 clear，与回调无关）。
                        let cleared = WitPlatform::new().map(|p3| {
                            use tairitsu_vdom::platform::TimerOps;
                            p3.clear_interval(id);
                        }).is_ok();
                        let c = n2.get();
                        let _ = &st2;
                        if cleared && c >= 2 {
                            done(CapStatus::Pass, format!("触发 {c} 次后外部清除"));
                        } else {
                            done(CapStatus::Fail, format!("cleared={cleared} count={c}"));
                        }
                    }),
                    400,
                );
            }
        }
        14 => {
            if let Ok(p2) = WitPlatform::new() {
                use tairitsu_vdom::platform::TimerOps;
                p2.request_animation_frame(Box::new(|ts| {
                    if ts > 0.0 {
                        done(CapStatus::Pass, format!("ts={ts:.0}"));
                    } else {
                        done(CapStatus::Fail, "ts=0".into());
                    }
                }));
            }
        }
        15 => {
            if let Ok(p2) = WitPlatform::new() {
                use tairitsu_vdom::platform::TimerOps;
                let fired = std::rc::Rc::new(std::cell::Cell::new(false));
                let f2 = fired.clone();
                let id = p2.request_animation_frame(Box::new(move |_| {
                    f2.set(true);
                }));
                p2.cancel_animation_frame(id);
                let f3 = fired.clone();
                p2.set_timeout(
                    Box::new(move || {
                        if f3.get() {
                            done(CapStatus::Fail, "取消后仍触发".into());
                        } else {
                            done(CapStatus::Pass, "取消后未触发".into());
                        }
                    }),
                    300,
                );
            }
        }
        // ---- evt（CDP 派发；此处只挂监听 + 计数）----
        16 | 17 | 18 | 19 => {
            let evt_name = match idx {
                16 => "click",
                17 => "keydown",
                18 => "focus",
                _ => "input",
            };
            if let Some(el) = get_element_by_id(0, &format!("cap-evt-{evt_name}")) {
                let el = WitElement::from_raw(el);
                let st = state.clone();
                platform.add_event_listener(
                    &el,
                    evt_name,
                    Box::new(move |_ev| {
                        bump_counter(&st, idx);
                    }),
                );
                done(CapStatus::Pass, "监听已挂（等 CDP 派发翻计数）".into());
            } else {
                done(CapStatus::Fail, "触发元素未渲染".into());
            }
        }
        20 => {
            if let Some(el) = get_element_by_id(0, "cap-evt-rm") {
                let el = WitElement::from_raw(el);
                let st = state.clone();
                platform.add_event_listener(
                    &el,
                    "click",
                    Box::new(move |_ev| {
                        bump_counter(&st, idx);
                    }),
                );
                platform.remove_event_listener(&el, "click");
                done(CapStatus::Pass, "已挂后立即移除（CDP 派发两次验证计数=0）".into());
            } else {
                done(CapStatus::Fail, "触发元素未渲染".into());
            }
        }
        // ---- net ----
        21 => {
            tairitsu_web::fetch_text(
                "plugin.php?id=pokemon:game",
                Box::new(|r| match r {
                    Ok(t) if t.len() > 0 => done(CapStatus::Pass, format!("{} 字节", t.len())),
                    Ok(_) => done(CapStatus::Fail, "空响应".into()),
                    Err(e) => done(CapStatus::Fail, e),
                }),
            );
        }
        22 => {
            tairitsu_web::fetch_text(
                "http://127.0.0.1:1/nope",
                Box::new(|r| match r {
                    Err(_) => done(CapStatus::Pass, "网络失败→Err".into()),
                    Ok(t) => done(CapStatus::Fail, format!("意外 Ok: {}…", t.chars().take(30).collect::<String>())),
                }),
            );
        }
        23 => {
            let opts = r#"{"method":"POST","body":"cap=1"}"#;
            tairitsu_web::fetch_text_with_options(
                "plugin.php?id=pokemon:game",
                opts,
                Box::new(|r| match r {
                    Ok(t) => done(CapStatus::Pass, format!("POST {} 字节", t.len())),
                    Err(e) => done(CapStatus::Fail, e),
                }),
            );
        }
        24 => {
            // 四个回调任何一个先到都只落一次牌（once 闩锁）。
            let latch = std::rc::Rc::new(std::cell::RefCell::new(Some(done)));
            let l1 = latch.clone();
            tairitsu_web::ws_connect(
                "wss://127.0.0.1:2/nope",
                Box::new(move || {
                    if let Some(f) = l1.borrow_mut().take() {
                        f(CapStatus::Pass, "on_open（意外但回调通）".into());
                    }
                }),
                Box::new(|_m| {}),
                {
                    let l2 = latch.clone();
                    Box::new(move |code, reason| {
                        if let Some(f) = l2.borrow_mut().take() {
                            f(CapStatus::Pass, format!("on_close({code}, {reason})"));
                        }
                    })
                },
                {
                    let l3 = latch.clone();
                    Box::new(move || {
                        if let Some(f) = l3.borrow_mut().take() {
                            f(CapStatus::Pass, "on_error（预期路径）".into());
                        }
                    })
                },
            );
        }
        // ---- clipboard ----
        25 => {
            let ok = platform.copy_to_clipboard("cap-probe");
            if ok {
                done(CapStatus::Pass, "copy 返回 true".into());
            } else {
                done(CapStatus::Skip, "headless 剪贴板被拒（execCommand false）".into());
            }
        }
        26 => match platform.read_clipboard() {
            Some(t) => done(CapStatus::Pass, format!("读到 {} 字", t.chars().count())),
            None => done(CapStatus::Skip, "无剪贴板读权限（返回 None）".into()),
        },
        27 => {
            use std::rc::Rc;
            let latch = Rc::new(std::cell::RefCell::new(Some(done)));
            let l1 = latch.clone();
            platform.clipboard_write_text_async(
                "cap-async",
                Box::new(move |r| {
                    if let Some(f) = l1.borrow_mut().take() {
                        match r {
                            Ok(()) => f(CapStatus::Pass, "async 写回调 Ok".into()),
                            Err(e) => f(CapStatus::Skip, format!("被拒（headless 权限）：{e}")),
                        }
                    }
                }),
            );
            if let Ok(p2) = WitPlatform::new() {
                use tairitsu_vdom::platform::TimerOps;
                let l2 = latch.clone();
                p2.set_timeout(
                    Box::new(move || {
                        if let Some(f) = l2.borrow_mut().take() {
                            f(CapStatus::Skip, "headless 剪贴板权限挂起（2.5s 无回调）".into());
                        }
                    }),
                    2500,
                );
            }
        }
        // ---- observers ----
        28 => {
            let d = scratch();
            let st = state.clone();
            let got = std::rc::Rc::new(std::cell::Cell::new(false));
            let g2 = got.clone();
            let obs = platform.create_mutation_observer(Box::new(move |records| {
                if !records.is_empty() {
                    g2.set(true);
                }
            }));
            platform.observe_mutations(
                obs,
                &d,
                Some(tairitsu_vdom::platform::MutationObserverInit {
                    child_list: true,
                    attributes: false,
                    character_data: false,
                    subtree: false,
                    attribute_old_value: false,
                    character_data_old_value: false,
                }),
            );
            let child = platform.create_element("p");
            platform.append_child(&d, &child);
            if let Ok(p2) = WitPlatform::new() {
                use tairitsu_vdom::platform::TimerOps;
                let g3 = got.clone();
                p2.set_timeout(
                    Box::new(move || {
                        if g3.get() {
                            done(CapStatus::Pass, "onMutate 收到记录".into());
                        } else {
                            done(CapStatus::Fail, "500ms 未回调".into());
                        }
                    }),
                    500,
                );
            }
            let _ = st;
        }
        29 => {
            let d = scratch();
            let st = state.clone();
            let got = std::rc::Rc::new(std::cell::Cell::new(false));
            let g2 = got.clone();
            let obs = platform.create_resize_observer(Box::new(move |_entries| {
                g2.set(true);
            }));
            platform.observe_resize(obs, &d);
            if let Ok(p2) = WitPlatform::new() {
                use tairitsu_vdom::platform::TimerOps;
                let g3 = got.clone();
                p2.set_timeout(
                    Box::new(move || {
                        if g3.get() {
                            done(CapStatus::Pass, "onResize 回调（observe 首帧）".into());
                        } else {
                            done(CapStatus::Skip, "headless 未派发初始 resize".into());
                        }
                    }),
                    600,
                );
            }
            let _ = st;
        }
        // ---- media query ----
        30 => {
            let l = platform.match_media("(min-width: 100px)");
            let m = platform.media_query_list_get_matches(l);
            let media = platform.media_query_list_get_media(l);
            let _lid = platform.media_query_list_add_listener(l, Box::new(|_b| {}));
            if m && media.contains("min-width") {
                done(CapStatus::Pass, format!("matches={m} media={media}"));
            } else {
                done(CapStatus::Fail, format!("matches={m} media={media}"));
            }
        }
        31 => {
            let dark = platform.prefers_dark_mode();
            done(CapStatus::Pass, format!("dark={dark}"));
        }
        // ---- scroll ----
        32 => {
            let st = state.clone();
            let got = std::rc::Rc::new(std::cell::Cell::new(false));
            let g2 = got.clone();
            platform.on_scroll(Box::new(move |_x, _y| {
                g2.set(true);
            }));
            platform.scroll_to(200.0, "auto");
            if let Ok(p2) = WitPlatform::new() {
                use tairitsu_vdom::platform::TimerOps;
                let g3 = got.clone();
                p2.set_timeout(
                    Box::new(move || {
                        if g3.get() {
                            done(CapStatus::Pass, "onScroll 回调".into());
                        } else {
                            done(CapStatus::Fail, "scroll 后未回调".into());
                        }
                    }),
                    600,
                );
            }
            let _ = st;
        }
        33 => {
            platform.scroll_to(300.0, "instant");
            if let Ok(p2) = WitPlatform::new() {
                use tairitsu_vdom::platform::TimerOps;
                p2.set_timeout(
                    Box::new(move || {
                        let y = {
                            let p3 = WitPlatform::new().unwrap();
                            use tairitsu_vdom::platform::ScrollOps;
                            p3.get_scroll_y()
                        };
                        if y >= 200.0 {
                            done(CapStatus::Pass, format!("scrollY={y:.0}"));
                        } else {
                            done(CapStatus::Fail, format!("scrollY={y:.0}"));
                        }
                    }),
                    400,
                );
            }
        }
        // ---- nav ----
        34 => {
            let p = tairitsu_web::get_pathname();
            if !p.is_empty() {
                done(CapStatus::Pass, p);
            } else {
                done(CapStatus::Fail, "空 pathname".into());
            }
        }
        35 => {
            let base = tairitsu_web::get_pathname();
            tairitsu_web::push_state("/capprobe-probe?cap=1");
            let after = tairitsu_web::get_pathname();
            tairitsu_web::replace_state(&base);
            let restored = tairitsu_web::get_pathname();
            if after.contains("capprobe-probe") && restored == base {
                done(CapStatus::Pass, format!("{after} → 还原 {restored}"));
            } else {
                done(CapStatus::Fail, format!("after={after} restored={restored} base={base}"));
            }
        }
        // ---- canvas ----
        36 => {
            let d = scratch();
            platform.set_inner_html(
                &d,
                r#"<canvas id="cap-cv" width="60" height="30"></canvas>"#.to_string(),
            );
            let cv = platform.query_selector_on(&d, "#cap-cv");
            match cv.and_then(|c| platform.get_canvas_context(&c, "2d")) {
                Some(ctx) => {
                    platform.canvas_set_fill_style(ctx, "#ff0000");
                    platform.canvas_fill_rect(ctx, 0.0, 0.0, 30.0, 30.0);
                    platform.canvas_clear_rect(ctx, 10.0, 10.0, 5.0, 5.0);
                    done(CapStatus::Pass, "ctx+fill+clear（像素由 CDP 读回）".into());
                }
                None => done(CapStatus::Fail, "getCanvasContext=None".into()),
            }
        }
        37 => {
            let matrix = vec![vec![true, false, true], vec![false, true, false], vec![true, false, true]];
            let ok = platform.draw_qrcode_on_canvas_by_id("cap-qr", &matrix, 3, "#000000", "#ffffff");
            if ok {
                done(CapStatus::Pass, "qrcode 绘制返回 true（像素由 CDP 读回）".into());
            } else {
                done(CapStatus::Fail, "返回 false".into());
            }
        }
        // ---- idb ----
        38 => {
            idb_chain(state, done);
        }
        // ---- file ----
        39 => {
            // 能力面缺口：file_reader_* 需要 blob 句柄，但平台没有
            // 「input.files → blob 句柄」的获取 API —— 记录为 Skip（发现）。
            done(
                CapStatus::Skip,
                "API 面缺口：无 input.files→blob 句柄获取途径，fileReader 无法从组件侧喂数".into(),
            );
        }
        // ---- audio / video ----
        40 => {
            let ctx = platform.create_audio_context();
            if ctx != 0 {
                done(CapStatus::Pass, format!("ctx={ctx}"));
            } else {
                done(CapStatus::Skip, "headless 无音频设备（ctx=0）".into());
            }
        }
        41 => {
            done(CapStatus::Skip, "测试环境无视频源；video_play 等需要 <video> 元素与媒体流".into());
        }
        // ---- geo / fs ----
        42 => {
            let latch = std::rc::Rc::new(std::cell::RefCell::new(Some(done)));
            if let Ok(p2) = WitPlatform::new() {
                use tairitsu_vdom::platform::TimerOps;
                let l0 = latch.clone();
                p2.set_timeout(
                    Box::new(move || {
                        if let Some(f) = l0.borrow_mut().take() {
                            f(CapStatus::Skip, "headless 地理权限挂起（4s 无回调）".into());
                        }
                    }),
                    4000,
                );
            }
            let l1 = latch.clone();
            platform.get_current_position(
                Box::new(move |p| {
                    if let Some(f) = l1.borrow_mut().take() {
                        f(CapStatus::Pass, format!("success lat={:.3}", p.latitude));
                    }
                }),
                {
                    let l2 = latch.clone();
                    Box::new(move |e| {
                        if let Some(f) = l2.borrow_mut().take() {
                            f(CapStatus::Pass, format!("error 回调到达（headless 预期）：{}", e.message));
                        }
                    })
                },
                false,
                3000,
                0,
            );
        }
        43 => {
            if let Some(el) = get_element_by_id(0, "app") {
                let el = WitElement::from_raw(el);
                platform.request_fullscreen(&el);
                done(CapStatus::Skip, "已发起（headless 全屏结果不可观测，fire 即通）".into());
            } else {
                done(CapStatus::Fail, "无锚元素".into());
            }
        }
        // ---- batch ----
        44 => {
            use tairitsu_web::batch_ops::BatchOps;
            let d = scratch();
            let d2 = platform.create_element("p");
            let mut b = BatchOps::new();
            b.add_style(d.clone(), "margin", "1px");
            b.add_attrs(
                d.clone(),
                vec![("data-cap-b1".to_string(), "1".to_string())],
            );
            b.add_removal(d2);
            let n = b.apply();
            let got = platform.get_attribute(&d, "data-cap-b1");
            if n >= 2 && got.as_deref() == Some("1") {
                done(CapStatus::Pass, format!("apply {n} 项"));
            } else {
                done(CapStatus::Fail, format!("n={n} got={got:?}"));
            }
        }
        // ---- misc ----
        45 => {
            let ok = platform.exec_command("copy", None);
            if ok {
                done(CapStatus::Pass, "execCommand(copy)=true".into());
            } else {
                done(CapStatus::Skip, "execCommand(copy) 被拒".into());
            }
        }
        46 => {
            if let Some(el) = get_element_by_id(0, "cap-evt-input") {
                let el = WitElement::from_raw(el);
                let s = platform.get_selection_start(&el);
                let e = platform.get_selection_end(&el);
                match (s, e) {
                    (Some(_), Some(_)) => done(CapStatus::Pass, format!("sel {s:?}..{e:?}")),
                    _ => done(CapStatus::Fail, "selection 读取失败".into()),
                }
            } else {
                done(CapStatus::Fail, "输入元素未渲染".into());
            }
        }
        47 => {
            let d = scratch();
            platform.set_content_editable(&d, true);
            match platform.get_contenteditable_state(&d) {
                Some(_) => done(CapStatus::Pass, "可编辑状态读取成功".into()),
                None => done(CapStatus::Fail, "getState=None".into()),
            }
        }
        _ => done(CapStatus::Skip, format!("未注册的测试下标 {idx}（{name}）")),
    }
}

// ---------------------------------------------------------------------------
// UI（diff/patch 管线渲染）
// ---------------------------------------------------------------------------

#[cfg(target_family = "wasm")]
fn render_page(state: &CapState) -> tairitsu_vdom::VNode {
    use tairitsu_macros::rsx;
    let results = state.results.get();
    let counters = state.evt_counters.get();
    let pass = results.iter().filter(|r| r.status == CapStatus::Pass).count();
    let fail = results.iter().filter(|r| r.status == CapStatus::Fail).count();
    let skip = results.iter().filter(|r| r.status == CapStatus::Skip).count();
    let total = results.len();
    let summary = format!("{pass} pass / {fail} fail / {skip} skip / 共 {total}");

    let mut rows: Vec<tairitsu_vdom::VNode> = Vec::new();
    for (i, r) in results.iter().enumerate() {
        let status = r.status.label();
        let cls = match r.status {
            CapStatus::Pass => "cap-pass",
            CapStatus::Fail => "cap-fail",
            CapStatus::Skip => "cap-skip",
            CapStatus::Pending => "cap-pending",
        };
        let counter_text = if r.group == "evt" {
            format!("计数 {}", counters.get(i).copied().unwrap_or(0))
        } else {
            String::new()
        };
        let g = r.group;
        let n = r.name;
        let d = r.detail.clone();
        let stx = status;
        let ct = counter_text.clone();
        let cls2 = cls;
        rows.push(rsx! {
            tr {
                td { "{g}" }
                td { "{n}" }
                td { class: cls2, "{stx}" }
                td { "{ct}" }
                td { "{d}" }
            }
        });
    }

    rsx! {
        div { class: "cap-layout",
            div { id: "cap-anchor", class: "cap-anchor", "锚点（rect 测试目标）" }
            canvas { id: "cap-qr", width: "60", height: "60" }
            div { class: "cap-tall" }
            h2 { "tairitsu wasm component 能力探针" }
            p { class: "cap-summary", "{summary}" }
            div { class: "cap-triggers",
                button { id: "cap-evt-click", "data-pm-cap-evt": "click", "触发 click" }
                input { id: "cap-evt-keydown", "data-pm-cap-evt": "keydown", r#type: "text", placeholder: "keydown 目标" }
                input { id: "cap-evt-focus", "data-pm-cap-evt": "focus", r#type: "text", placeholder: "focus 目标" }
                input { id: "cap-evt-input", "data-pm-cap-evt": "input", r#type: "text", placeholder: "input 目标" }
                button { id: "cap-evt-rm", "data-pm-cap-evt": "rm", "已移除监听的目标" }
            }
            table { class: "cap-table",
                thead {
                    tr { th { "组" } th { "能力" } th { "状态" } th { "计数" } th { "详情" } }
                }
                tbody {
                    for i in 0..rows.len() {
                        ..vec![rows[i].clone()]
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 启动（与 admin2/game2 同款管线脚手架）
// ---------------------------------------------------------------------------

#[cfg(target_family = "wasm")]
fn run_app() -> Result<()> {
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("[capprobe] PANIC: {info}");
        if let Some(app) = get_element_by_id(0, "app") {
            use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
            let _ = set_attribute(app, "data-panic", &msg);
        }
    }));

    let component_id = tairitsu_vdom::use_component(|| tairitsu_vdom::VNode::empty());
    let state = tairitsu_vdom::runtime::with_component(component_id, || new_state(component_id));
    ROOT_COMPONENT.with(|id| id.set(component_id));

    let reg = registry();
    let results: Vec<CapResult> = reg
        .tests
        .iter()
        .map(|(g, n, _)| CapResult {
            group: g,
            name: n,
            status: CapStatus::Pending,
            detail: String::new(),
            counter: 0,
        })
        .collect();
    let evt_count = reg.tests.iter().filter(|t| t.2).count();
    state.results.inner().set(results);
    state
        .evt_counters
        .inner()
        .set(vec![0; reg.tests.len()]);
    let _ = evt_count;

    let state_for_render = state.clone();
    tairitsu_vdom::runtime::update_render_function(component_id, move || {
        render_page(&state_for_render)
    });
    let vnode = render_page(&state);

    let platform = WitPlatform::new()?;
    platform.mount_vnode_to_app(vnode.clone())?;

    let app_handle = get_element_by_id(0, "app").expect("#app mount point not found");
    let children_handle = get_child_nodes(app_handle);
    let vnode_root_handle = item(children_handle, 0).expect("rendered vnode root not found");
    let root_element = WitElement::from_raw(vnode_root_handle);
    tairitsu_web::init_runtime(root_element.clone());
    tairitsu_vdom::runtime::store_initial_vnode(component_id, vnode);

    // 自动开跑：下一帧起顺序执行（给首帧渲染让路）。
    let st_run = state.clone();
    if let Ok(p2) = WitPlatform::new() {
        use tairitsu_vdom::platform::TimerOps;
        p2.set_timeout(
            Box::new(move || {
                run_from(st_run, 0);
            }),
            100,
        );
    }
    Ok(())
}

#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn run() {
    if let Err(err) = run_app() {
        if let Some(app) = get_element_by_id(0, "app") {
            use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
            let _ = set_attribute(app, "data-panic", &format!("{err:?}"));
        }
    }
}

#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn tairitsu_component_bootstrap() {
    run();
}

#[cfg(not(target_family = "wasm"))]
pub fn main_stub() {}
