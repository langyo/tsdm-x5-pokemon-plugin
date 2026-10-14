//! 使用API的Shop Hook
//!
//! 迁移到 pokemon_system/api/shop.php

use crate::prelude::*;

use crate::utils::api_client::NewApiClient;
use _utils::types::api_shop::{ShopListResponse, ShopPetListResponse};

/// 名称筛选输入的防抖间隔（毫秒）：每次按键都会重启资源 future，
/// 停止输入该时长后才真正发起请求。
const SEARCH_DEBOUNCE_MS: u32 = 250;

pub fn use_shop(
    current_category: Signal<Option<u32>>,
    current_page: Signal<u32>,
    search_keyword: Signal<String>,
) -> Resource<Result<ShopListResponse, String>> {
    use_resource(move || async move {
        // 与服务端一致按 trim 后的关键字判断与发送，纯空格不算筛选。
        let keyword = search_keyword.read().trim().to_string();
        let category = *current_category.read();
        let page = *current_page.read();
        if !keyword.is_empty() {
            gloo_timers::future::TimeoutFuture::new(SEARCH_DEBOUNCE_MS).await;
        }
        let api = NewApiClient::new();
        api.get_shop_items(category, page, &keyword)
            .await
            .map_err(|e| format!("加载商品失败: {}", e))
    })
}

pub fn use_shop_pets(
    current_page: Signal<u32>,
    search_keyword: Signal<String>,
) -> Resource<Result<ShopPetListResponse, String>> {
    use_resource(move || async move {
        let keyword = search_keyword.read().trim().to_string();
        let page = *current_page.read();
        if !keyword.is_empty() {
            gloo_timers::future::TimeoutFuture::new(SEARCH_DEBOUNCE_MS).await;
        }
        let api = NewApiClient::new();
        api.get_shop_pets(page, &keyword)
            .await
            .map_err(|e| format!("加载宠物失败: {}", e))
    })
}
