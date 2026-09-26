use crate::prelude::*;

/// 页面布局模式
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LayoutMode {
    /// 常规布局：左侧内容 + 右侧边栏
    Default,
    /// 全宽布局：内容占满整个区域（无侧边栏）
    FullWidth,
}

#[derive(Clone, Props, PartialEq)]
pub struct PageContainerProps {
    /// 布局模式
    pub mode: LayoutMode,
    /// 页面内容
    pub children: Element,
    /// 可选的类名
    #[props(default)]
    pub class: String,
}

/// 通用页面容器组件
///
/// 提供统一的布局容器，支持常规和全宽两种模式
/// - 常规模式：用于个人中心、商店、宠物中心等带侧边栏的页面
/// - 全宽模式：用于冒险、首页等需要完整空间的页面
#[component]
pub fn PageContainer(props: PageContainerProps) -> Element {
    let container_class = match props.mode {
        LayoutMode::FullWidth => "page-container page-container--full-width",
        LayoutMode::Default => "page-container page-container--default",
    };

    let extra_class = if !props.class.is_empty() {
        format!(" {}", props.class)
    } else {
        String::new()
    };

    rsx! {
        div {
            class: "{container_class}{extra_class}",
            {props.children}
        }
    }
}
