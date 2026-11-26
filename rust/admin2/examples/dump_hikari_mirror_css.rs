//! 构建期 helper：把 hikari Rust 镜像自带的组件样式按选择器打印到 stdout。
//!
//! 用法（由 `scripts/publish/dump_hikari_mirror_css.py` 调用）：
//!
//! ```text
//! cargo run --example dump_hikari_mirror_css [-- <selector>]
//! ```
//!
//! 不带参数时打印整份 bundle（`StyleRegistry::css_bundle()`）；带选择器时只
//! 打印选择器命中的扁平规则块。构建脚本先编译 Vue 那半边，再检查缺哪些
//! `.hk-*` 前缀，只向这里要缺的那几段——整份 bundle 与 Vue 表大面积重叠，
//! 全量拼进去会让同一批类名被定义两遍。
//!
//! 这个文件是固定源码（Cargo 的 example 需要参与索引），构建脚本只读它。

fn main() {
    let selector = std::env::args().nth(1);
    let mut registry = hikari_components::StyleRegistry::default();
    registry.register_all();
    let bundle = registry.css_bundle();
    match selector {
        None => print!("{bundle}"),
        Some(selector) => match extract_blocks(&bundle, &selector) {
            Some(blocks) => print!("{blocks}"),
            None => {
                eprintln!("no rule block matches {selector:?}");
                std::process::exit(1);
            }
        },
    }
}

/// 取出 bundle 里所有「选择器头包含 `needle`」的扁平规则块。
///
/// bundle 已经展开（没有嵌套规则），所以按 `{}` 配对切块即可。选择器头是
/// 「上一个块结束后第一个非空白位置到 `{` 之前」的那段——这样 `@media` 的
/// 头部也算作内层块的选择器头（宁可多带一条外层媒体查询，也不漏掉里面的
/// 规则）。`/* … */` 注释在切块前先剥掉，免得注释文字里的 `{`/`}` 干扰配对。
fn extract_blocks(bundle: &str, needle: &str) -> Option<String> {
    let stripped = strip_comments(bundle);
    let mut out = String::new();
    let mut depth = 0usize;
    let mut head_start = 0usize;
    let mut body_start = 0usize;
    for (i, ch) in stripped.char_indices() {
        match ch {
            '{' => {
                if depth == 0 {
                    body_start = i + 1;
                }
                depth += 1;
            }
            '}' => {
                if depth == 0 {
                    continue;
                }
                depth -= 1;
                if depth == 0 {
                    let head = &stripped[head_start..body_start - 1];
                    if head.contains(needle) {
                        out.push_str(head.trim());
                        out.push('{');
                        out.push_str(&stripped[body_start..i]);
                        out.push_str("}\n");
                    }
                    head_start = i + 1;
                }
            }
            _ => {}
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

/// 去掉 `/* … */` 注释；未闭合的注释按「吃到结尾」处理。
fn strip_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(open) = rest.find("/*") {
        out.push_str(&rest[..open]);
        match rest[open + 2..].find("*/") {
            Some(close) => rest = &rest[open + 2 + close + 2..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}
