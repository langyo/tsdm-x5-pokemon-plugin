//! 商店页纯逻辑（native 可编译、可测试）。
//!
//! 从 shop.rs 拆出：分页窗口算法、悬浮详情定位。两处都与 Dioxus 版
//! 逐行为对齐（rust/game/src/pages/shop.rs），测试也一并移植，
//! `cargo test -p _game2` 直接跑。

/// 页码窗口：总页数不多时全部展示，否则保留首尾页并给出省略号。
pub fn windowed_pages(current: usize, total: usize) -> Vec<Option<usize>> {
    let total = total.max(1);
    let current = current.clamp(1, total);
    if total <= 7 {
        return (1..=total).map(Some).collect();
    }
    let window_start = current.saturating_sub(1).max(2);
    let window_end = (current + 1).min(total - 1);
    let mut pages: Vec<Option<usize>> = vec![Some(1)];
    if window_start > 2 {
        pages.push(None);
    }
    pages.extend((window_start..=window_end).map(Some));
    if window_end < total - 1 {
        pages.push(None);
    }
    pages.push(Some(total));
    pages
}

/// 分页器条目：页码或省略号（windowed_pages 的返回元素）。
#[derive(Clone, Copy, PartialEq)]
pub enum PagerEntry {
    Page(usize),
    Ellipsis,
}

/// 页码窗口（返回 PagerEntry 形态，供 rsx 逐项 match 渲染）。
pub fn windowed_entries(current: usize, total: usize) -> Vec<PagerEntry> {
    windowed_pages(current, total)
        .into_iter()
        .map(|p| match p {
            Some(n) => PagerEntry::Page(n),
            None => PagerEntry::Ellipsis,
        })
        .collect()
}

/// 悬浮详情定位（视口坐标，配合 .shop-tile-tooltip 的 position:fixed）：
/// 显示在光标上方，避免盖住当前卡片下方的价格与购买按钮；上边距不足时
/// 翻到光标下方，横向按估算宽度收拢在视口内。`tooltip_h` 为按内容高度的
/// 估算上限，需与 .shop-tile-tooltip 的 CSS 尺寸保持一致。
///
/// 与 Dioxus 版的差异：视口宽度来自 tairitsu 平台接口；高度接口未导入，
/// 纵向不做下限钳制（tooltip 本身 fixed 定位，超出视口可滚动查看）。
pub fn shop_tooltip_position(client_x: f64, client_y: f64, tooltip_h: f64, win_w: f64) -> (f64, f64) {
    const TOOLTIP_W: f64 = 230.0; // CSS max-width
    const GAP: f64 = 12.0;
    const MARGIN: f64 = 8.0;

    let mut x = client_x - TOOLTIP_W / 2.0;
    x = x.max(MARGIN).min((win_w - TOOLTIP_W - MARGIN).max(MARGIN));

    let mut y = client_y - tooltip_h - GAP;
    if y < MARGIN {
        y = client_y + GAP;
    }
    (x, y)
}

/// wasm 环境读视口宽度；native（测试）下返回 None，由调用方用默认值。
#[cfg(target_family = "wasm")]
pub fn viewport_width() -> Option<f64> {
    use tairitsu_vdom::platform::LayoutOps;
    tairitsu_web::WitPlatform::new().ok().map(|p| p.inner_width() as f64)
}

#[cfg(not(target_family = "wasm"))]
pub fn viewport_width() -> Option<f64> {
    None
}

#[cfg(test)]
mod tests {
    use super::{shop_tooltip_position, windowed_pages};

    fn compact(pages: &[Option<usize>]) -> Vec<String> {
        pages
            .iter()
            .map(|p| p.map(|n| n.to_string()).unwrap_or_else(|| "…".to_string()))
            .collect()
    }

    #[test]
    fn few_pages_are_all_listed() {
        assert_eq!(compact(&windowed_pages(1, 1)), ["1"]);
        assert_eq!(compact(&windowed_pages(3, 7)), ["1", "2", "3", "4", "5", "6", "7"]);
    }

    #[test]
    fn long_lists_keep_edges_with_ellipsis() {
        assert_eq!(compact(&windowed_pages(1, 12)), ["1", "2", "…", "12"]);
        assert_eq!(compact(&windowed_pages(5, 12)), ["1", "…", "4", "5", "6", "…", "12"]);
        assert_eq!(compact(&windowed_pages(12, 12)), ["1", "…", "11", "12"]);
    }

    #[test]
    fn zero_total_falls_back_to_single_page() {
        assert_eq!(compact(&windowed_pages(3, 0)), ["1"]);
    }

    #[test]
    fn tooltip_sits_above_cursor_centered() {
        // 宽视口中央：水平居中于光标，垂直在光标上方留 GAP。
        let (x, y) = shop_tooltip_position(640.0, 400.0, 130.0, 1280.0);
        assert_eq!(x, 640.0 - 115.0);
        assert_eq!(y, 400.0 - 130.0 - 12.0);
    }

    #[test]
    fn tooltip_flips_below_near_top_edge() {
        let (_x, y) = shop_tooltip_position(640.0, 50.0, 130.0, 1280.0);
        assert_eq!(y, 50.0 + 12.0);
    }

    #[test]
    fn tooltip_clamps_horizontally_into_viewport() {
        // 光标贴左缘：不小于 MARGIN；贴右缘：不超过 win_w - W - MARGIN。
        let (x_left, _) = shop_tooltip_position(10.0, 400.0, 130.0, 1280.0);
        assert_eq!(x_left, 8.0);
        let (x_right, _) = shop_tooltip_position(1270.0, 400.0, 130.0, 1280.0);
        assert_eq!(x_right, 1280.0 - 230.0 - 8.0);
    }
}
