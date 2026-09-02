//! 使用API的Shop Hook
//!
//! 迁移到 pokemon_system/api/shop.php

use dioxus::prelude::*;

use crate::utils::api_client::NewApiClient;
use _utils::types::api_shop::{ShopListResponse, ShopPetListResponse};

pub fn use_shop(
    current_category: Signal<Option<u32>>,
    current_page: Signal<u32>,
) -> Resource<Result<ShopListResponse, String>> {
    use_resource(move || async move {
        let api = NewApiClient::new();
        let category = *current_category.read();
        let page = *current_page.read();
        api.get_shop_items(category, page)
            .await
            .map_err(|e| format!("加载商品失败: {}", e))
    })
}

pub fn use_shop_pets(current_page: Signal<u32>) -> Resource<Result<ShopPetListResponse, String>> {
    use_resource(move || async move {
        let api = NewApiClient::new();
        let page = *current_page.read();
        api.get_shop_pets(page)
            .await
            .map_err(|e| format!("加载宠物失败: {}", e))
    })
}
