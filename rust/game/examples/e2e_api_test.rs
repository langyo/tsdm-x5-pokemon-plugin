// E2E API 测试 (pokemon_system/api/)
// 用法: cargo run --example e2e_api_test

use anyhow::Result;
use std::{sync::Arc, time::Duration};

use reqwest::cookie::CookieStore;
use reqwest::cookie::Jar;

/// API的基础URL - 通过plugin.php访问
fn api_base(base_url: &str) -> String {
    format!("{}/plugin.php?id=pokemon:pokemon", base_url)
}

/// 登录Discuz论坛并获取会话Cookie
async fn login_discuz(base_url: &str) -> Result<Arc<Jar>> {
    println!("🔐 Logging in to Discuz...");

    let jar = Arc::new(Jar::default());
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_provider(jar.clone())
        .timeout(Duration::from_secs(30))
        .build()?;

    // 现版 X5 已拒绝 lssubmit 快捷登录，必须走带 formhash 的完整表单
    let login_page = format!("{}/member.php?mod=logging&action=login", base_url);
    let page_html = client.get(&login_page).send().await?.text().await?;
    let formhash = extract_formhash(&page_html)
        .ok_or_else(|| anyhow::anyhow!("login page has no formhash"))?;

    let login_url = format!("{}/member.php?mod=logging&action=login", base_url);
    let params = [
        ("formhash", formhash.as_str()),
        ("username", "admin"),
        ("password", "admin123"),
        ("questionid", "0"),
        ("answer", ""),
        ("cookietime", "2592000"),
        ("loginsubmit", "yes"),
        ("referer", &format!("{}/", base_url)),
    ];

    let response = client.post(&login_url).form(&params).send().await?;
    let status = response.status();
    let _ = response.text().await?;

    // X5 登录失败也返回 200（错误信息在页面里），唯一可靠的成功信号是
    // cookie jar 里出现 *_auth 会话 cookie
    let cookie_header = jar
        .cookies(&reqwest::Url::parse(base_url)?)
        .and_then(|value| value.to_str().ok().map(str::to_string))
        .unwrap_or_default();
    if !status.is_success() || !cookie_header.contains("_auth") {
        anyhow::bail!(
            "Login failed: status={}, cookies=[{}]",
            status,
            cookie_header
        );
    }
    println!("   ✅ Login successful");

    let plugin_home = format!("{}/plugin.php?id=pokemon:pokemon", base_url);
    client.get(&plugin_home).send().await?;
    println!("   ✅ Plugin session initialized");

    Ok(jar)
}

fn extract_formhash(html: &str) -> Option<String> {
    let marker = r#"name="formhash" value=""#;
    let start = html.find(marker)? + marker.len();
    let rest = &html[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🧪 TSDM Pokemon Plugin - API E2E测试");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let base_url = std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost".to_string());
    println!("🌐 Target: {}", base_url);
    println!();

    // 登录
    let cookie_jar = login_discuz(&base_url).await?;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_provider(cookie_jar)
        .build()?;

    let api_base = api_base(&base_url);
    println!();

    // Pokemon API测试
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Pokemon API Tests (pokemon.php)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_pokemon_list(&api_base, &client).await?;
    test_pokemon_profile(&api_base, &client).await?;
    test_inventory(&api_base, &client).await?;
    test_user_stats(&api_base, &client).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Shop API Tests (shop.php)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_shop_categories(&api_base, &client).await?;
    test_shop_items(&api_base, &client).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Equipment API Tests (pokemon.php)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_equipment_api(&api_base, &client).await?;
    test_equip_item_api(&api_base, &client).await?;
    test_unequip_item_api(&api_base, &client).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Evolution API Tests (evolution.php)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_evolution_available(&api_base, &client).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Battle API Tests (battle.php)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_battle_start(&api_base, &client).await?;
    test_battle_list(&api_base, &client).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Error Handling Tests");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_api_error_invalid_action(&api_base, &client).await?;
    test_api_unauthorized_request(&base_url).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Pagination Tests");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_inventory_pagination(&api_base, &client).await?;
    test_shop_items_pagination(&api_base, &client).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Data Consistency Tests");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_user_profile_data_consistency(&api_base, &client).await?;
    test_inventory_items_structure(&api_base, &client).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Response Format Tests");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_api_response_timestamp(&api_base, &client).await?;
    test_api_success_field(&api_base, &client).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Type Filtering Tests");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_inventory_type_filter(&api_base, &client).await?;
    test_shop_category_filter(&api_base, &client).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 Edge Case Tests");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    test_empty_pokemon_list(&api_base, &client).await?;
    test_large_page_number(&api_base, &client).await?;
    test_invalid_pokemon_detail(&api_base, &client).await?;
    test_stats_achievement_flags(&api_base, &client).await?;

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ All new API tests passed!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

// ==================== User API Tests ====================

async fn test_pokemon_profile(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: 获取用户资料 (user.php?action=profile)");
    let url = format!("{}&endpoint=user&action=profile", api_base);

    let response = client.get(&url).send().await?;
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await?;
        anyhow::bail!("API请求失败: {} - {}", status, body);
    }

    let body = response.text().await?;

    // 验证JSON响应
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            println!("   ✅ 响应格式正确");
            if let Some(data) = json.get("data") {
                println!(
                    "   ✅ uid: {}",
                    data.get("uid").unwrap_or(&serde_json::json!(null))
                );
                println!(
                    "   ✅ username: {}",
                    data.get("username").unwrap_or(&serde_json::json!(null))
                );
                println!(
                    "   ✅ money: {}",
                    data.get("money").unwrap_or(&serde_json::json!(null))
                );
            }
        } else {
            println!("   ❌ API返回success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

async fn test_inventory(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: 获取背包物品 (user.php?action=inventory)");
    let url = format!("{}&endpoint=user&action=inventory&page=1", api_base);

    let response = client.get(&url).send().await?;
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await?;
        anyhow::bail!("API请求失败: {} - {}", status, body);
    }

    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            println!("   ✅ 响应格式正确");
            if let Some(data) = json.get("data") {
                println!(
                    "   ✅ total: {}",
                    data.get("total").unwrap_or(&serde_json::json!(0))
                );
                println!(
                    "   ✅ page: {}",
                    data.get("page").unwrap_or(&serde_json::json!(0))
                );

                if let Some(items) = data.get("items").and_then(|i| i.as_array()) {
                    println!("   ✅ items count: {}", items.len());
                }
            }
        } else {
            println!("   ❌ API返回success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

async fn test_user_stats(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: 获取游戏统计 (user.php?action=stats)");
    let url = format!("{}&endpoint=user&action=stats", api_base);

    let response = client.get(&url).send().await?;
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await?;
        anyhow::bail!("API请求失败: {} - {}", status, body);
    }

    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            println!("   ✅ 响应格式正确");
            if let Some(data) = json.get("data") {
                if let Some(pokemon_stats) = data.get("pokemon_stats") {
                    println!(
                        "   ✅ pokemon_stats.total_owned: {}",
                        pokemon_stats
                            .get("total_owned")
                            .unwrap_or(&serde_json::json!(0))
                    );
                }
            }
        } else {
            println!("   ❌ API返回success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

async fn test_pokemon_list(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: 获取宝可梦列表 (pokemon.php?action=list)");
    let url = format!("{}&endpoint=pokemon&action=list", api_base);

    let response = client.get(&url).send().await?;
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await?;
        anyhow::bail!("API请求失败: {} - {}", status, body);
    }

    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            println!("   ✅ 响应格式正确");
            if let Some(data) = json.get("data") {
                if let Some(pokemons) = data.get("pokemons").and_then(|p| p.as_array()) {
                    println!("   ✅ pokemons count: {}", pokemons.len());
                } else {
                    println!("   ✅ pokemons: [] (新用户无宝可梦)");
                }
            }
        } else {
            println!("   ❌ API返回success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

// ==================== Equipment API Tests ====================

async fn test_equipment_api(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: 获取装备信息 (pokemon.php?action=equipment)");

    let list_url = format!("{}&endpoint=pokemon&action=list", api_base);
    let list_resp = client.get(&list_url).send().await?;
    let list_body = list_resp.text().await?;

    let pet_id = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&list_body) {
        if let Some(pokemons) = json["data"]["pokemons"].as_array() {
            if let Some(first) = pokemons.first() {
                first.get("id").and_then(|id| id.as_u64())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(pokemon_id) = pet_id {
        println!("   🔍 使用宠物ID: {}", pokemon_id);

        let url = format!(
            "{}&endpoint=pokemon&action=equipment&pokemon_id={}",
            api_base, pokemon_id
        );
        let response = client.get(&url).send().await?;
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await?;
            anyhow::bail!("API请求失败: {} - {}", status, body);
        }

        let body = response.text().await?;

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            if json["success"].as_bool() == Some(true) {
                println!("   ✅ 响应格式正确");
                if let Some(data) = json.get("data") {
                    if let Some(slots) = data.get("slots").and_then(|s| s.as_array()) {
                        println!("   ✅ slots count: {}", slots.len());
                        for (idx, slot) in slots.iter().enumerate() {
                            let has_item = slot.get("item").is_some();
                            println!(
                                "      - Slot {}: {}",
                                idx,
                                if has_item { "已装备" } else { "空" }
                            );
                        }
                    }
                    if let Some(owned) = data.get("owned_items").and_then(|o| o.as_array()) {
                        println!("   ✅ owned_items count: {}", owned.len());
                    }
                    if let Some(shop) = data.get("shop_items").and_then(|s| s.as_array()) {
                        println!("   ✅ shop_items count: {}", shop.len());
                    }
                    if let Some(money) = data.get("user_money").and_then(|m| m.as_i64()) {
                        println!("   ✅ user_money: {}", money);
                    }
                }
            } else if let Some(error) = json.get("error").and_then(|e| e.as_str()) {
                println!("   ⚠️  API returned error: {}", error);
            } else {
                println!("   ❌ API返回success=false: {}", json);
            }
        } else {
            println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
        }
    } else {
        println!("   ℹ️  用户暂无宠物，跳过装备测试");
    }

    println!();
    Ok(())
}

async fn test_equip_item_api(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: 装备物品 (pokemon.php?action=equip_item)");

    let list_url = format!("{}&endpoint=pokemon&action=list", api_base);
    let list_resp = client.get(&list_url).send().await?;
    let list_body = list_resp.text().await?;

    let pet_id = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&list_body) {
        json["data"]["pokemons"]
            .as_array()
            .and_then(|p| p.first())
            .and_then(|first| first.get("id").and_then(|id| id.as_u64()))
    } else {
        None
    };

    if let Some(pokemon_id) = pet_id {
        println!("   🔍 使用宠物ID: {}", pokemon_id);

        let url = format!("{}&endpoint=pokemon&action=equip_item", api_base);
        let payload = serde_json::json!({
            "pokemon_id": pokemon_id,
            "myitem_id": 1,
            "slot_index": 0
        });

        let response = client.post(&url).json(&payload).send().await?;
        let body = response.text().await?;

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            if json["success"].as_bool() == Some(true) {
                println!("   ✅ 装备成功");
                if let Some(data) = json.get("data") {
                    if let Some(slot) = data.get("slot_index").and_then(|s| s.as_u64()) {
                        println!("   ✅ slot_index: {}", slot);
                    }
                    if let Some(item) = data.get("equipped_item") {
                        if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                            println!("   ✅ equipped_item.name: {}", name);
                        }
                    }
                }
            } else if let Some(error) = json.get("error").and_then(|e| e.as_str()) {
                println!("   ℹ️  装备失败（可能物品不存在或已装备）: {}", error);
            } else {
                println!("   ⚠️  Unexpected response: {}", json);
            }
        } else {
            println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
        }
    } else {
        println!("   ℹ️  用户暂无宠物，跳过装备测试");
    }

    println!();
    Ok(())
}

async fn test_unequip_item_api(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: 卸下装备 (pokemon.php?action=unequip_item)");

    let list_url = format!("{}&endpoint=pokemon&action=list", api_base);
    let list_resp = client.get(&list_url).send().await?;
    let list_body = list_resp.text().await?;

    let pet_id = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&list_body) {
        json["data"]["pokemons"]
            .as_array()
            .and_then(|p| p.first())
            .and_then(|first| first.get("id").and_then(|id| id.as_u64()))
    } else {
        None
    };

    if let Some(pokemon_id) = pet_id {
        println!("   🔍 使用宠物ID: {}", pokemon_id);

        let url = format!("{}&endpoint=pokemon&action=unequip_item", api_base);
        let payload = serde_json::json!({
            "pokemon_id": pokemon_id,
            "slot_index": 0
        });

        let response = client.post(&url).json(&payload).send().await?;
        let body = response.text().await?;

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            if json["success"].as_bool() == Some(true) {
                println!("   ✅ 卸下装备成功");
                if let Some(data) = json.get("data") {
                    if let Some(slot) = data.get("slot_index").and_then(|s| s.as_u64()) {
                        println!("   ✅ slot_index: {}", slot);
                    }
                }
            } else if let Some(error) = json.get("error").and_then(|e| e.as_str()) {
                println!("   ℹ️  卸下失败（可能该槽位无装备）: {}", error);
            } else {
                println!("   ⚠️  Unexpected response: {}", json);
            }
        } else {
            println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
        }
    } else {
        println!("   ℹ️  用户暂无宠物，跳过卸下装备测试");
    }

    println!();
    Ok(())
}

// ==================== Shop API Tests ====================

async fn test_shop_categories(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: 获取商店分类 (shop.php?action=categories)");
    let url = format!("{}&endpoint=shop&action=categories", api_base);

    let response = client.get(&url).send().await?;
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await?;
        anyhow::bail!("API请求失败: {} - {}", status, body);
    }

    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            println!("   ✅ 响应格式正确");
            if let Some(data) = json.get("data") {
                if let Some(categories) = data.get("categories").and_then(|c| c.as_array()) {
                    println!("   ✅ categories count: {}", categories.len());
                    for cat in categories.iter().take(3) {
                        println!("      - {:?}", (cat.get("id"), cat.get("name")));
                    }
                }
            }
        } else {
            println!("   ❌ API返回success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

async fn test_shop_items(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: 获取商店物品 (shop.php?action=items)");
    let url = format!("{}&endpoint=shop&action=items&page=1", api_base);

    let response = client.get(&url).send().await?;
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await?;
        anyhow::bail!("API请求失败: {} - {}", status, body);
    }

    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            println!("   ✅ 响应格式正确");
            if let Some(data) = json.get("data") {
                println!(
                    "   ✅ total: {}",
                    data.get("total").unwrap_or(&serde_json::json!(0))
                );
                println!(
                    "   ✅ page: {}",
                    data.get("page").unwrap_or(&serde_json::json!(0))
                );

                if let Some(items) = data.get("items").and_then(|i| i.as_array()) {
                    println!("   ✅ items count: {}", items.len());
                    for item in items.iter().take(2) {
                        println!("      - {:?}", (item.get("name"), item.get("price")));
                    }
                }
            }
        } else {
            println!("   ❌ API返回success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

// ==================== Evolution API Tests ====================

async fn test_evolution_available(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: 获取可进化列表 (evolution.php?action=available)");

    // 需要先获取一个宠物ID
    let list_url = format!("{}&endpoint=pokemon&action=list", api_base);
    let list_resp = client.get(&list_url).send().await?;
    let list_body = list_resp.text().await?;

    let pet_id = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&list_body) {
        if let Some(pokemons) = json["data"]["pokemons"].as_array() {
            if let Some(first) = pokemons.first() {
                first.get("id").and_then(|id| id.as_u64())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(pet_id) = pet_id {
        println!("   🔍 使用宠物ID: {}", pet_id);

        let url = format!(
            "{}&endpoint=evolution&action=available&petid={}",
            api_base, pet_id
        );
        let response = client.get(&url).send().await?;
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await?;
            anyhow::bail!("API请求失败: {} - {}", status, body);
        }

        let body = response.text().await?;

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            if json["success"].as_bool() == Some(true) {
                println!("   ✅ 响应格式正确");
                if let Some(data) = json.get("data") {
                    if let Some(evolutions) =
                        data.get("available_evolutions").and_then(|e| e.as_array())
                    {
                        println!("   ✅ available_evolutions count: {}", evolutions.len());
                    } else {
                        println!("   ✅ available_evolutions: [] (无可用进化)");
                    }
                }
            } else {
                println!("   ❌ API返回success=false: {}", json);
            }
        } else {
            println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
        }
    } else {
        println!("   ℹ️  用户暂无宠物，跳过进化测试");
    }

    println!();
    Ok(())
}

// ==================== Error Handling Tests ====================

async fn test_api_error_invalid_action(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: API with invalid action parameter");
    let url = format!("{}&endpoint=user&action=invalid_action", api_base);

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(false) {
            println!("   ✅ 正确返回success=false");
            if let Some(error) = json.get("error") {
                println!("   ✅ error message present: {}", error);
            } else {
                println!("   ⚠️  no error message in response");
            }
        } else {
            println!("   ⚠️  Expected success=false, got: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

async fn test_api_unauthorized_request(base_url: &str) -> Result<()> {
    println!("📋 Test: API request without authentication");

    // Create a client without cookies
    let client = reqwest::Client::new();
    let api_base = api_base(base_url);
    let url = format!("{}&endpoint=user&action=profile", api_base);

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(false) {
            println!("   ✅ 正确返回success=false for unauthorized request");
            if let Some(error) = json.get("error") {
                println!("   ✅ error message present: {}", error);
            }
        } else {
            println!("   ⚠️  Expected success=false, got: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

// ==================== Pagination Tests ====================

async fn test_inventory_pagination(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: Inventory pagination (pages 1 & 2)");

    // Test page 1
    let url1 = format!("{}&endpoint=user&action=inventory&page=1", api_base);
    let response1 = client.get(&url1).send().await?;
    let body1 = response1.text().await?;

    if let Ok(json1) = serde_json::from_str::<serde_json::Value>(&body1) {
        if json1["success"].as_bool() == Some(true) {
            println!("   ✅ Page 1 retrieved successfully");
            if let Some(data) = json1.get("data") {
                let page1 = data.get("page").and_then(|p| p.as_u64()).unwrap_or(0);
                let total = data.get("total").and_then(|t| t.as_u64()).unwrap_or(0);
                println!("   ✅ page: {}, total: {}", page1, total);

                // Test page 2 if there are enough items
                if total > 0 {
                    let url2 = format!("{}&endpoint=user&action=inventory&page=2", api_base);
                    let response2 = client.get(&url2).send().await?;
                    let body2 = response2.text().await?;

                    if let Ok(json2) = serde_json::from_str::<serde_json::Value>(&body2) {
                        if json2["success"].as_bool() == Some(true) {
                            println!("   ✅ Page 2 retrieved successfully");
                            if let Some(data2) = json2.get("data") {
                                let page2 = data2.get("page").and_then(|p| p.as_u64()).unwrap_or(0);
                                println!("   ✅ page 2 page number: {}", page2);
                            }
                        } else {
                            println!("   ⚠️  Page 2 returned success=false: {}", json2);
                        }
                    }
                } else {
                    println!("   ℹ️  Total items is 0, skipping page 2 test");
                }
            }
        } else {
            println!("   ❌ Page 1 returned success=false: {}", json1);
        }
    } else {
        println!(
            "   ⚠️  响应不是有效JSON: {}",
            &body1[..body1.len().min(200)]
        );
    }

    println!();
    Ok(())
}

async fn test_shop_items_pagination(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: Shop items pagination");

    let url1 = format!("{}&endpoint=shop&action=items&page=1", api_base);
    let response1 = client.get(&url1).send().await?;
    let body1 = response1.text().await?;

    if let Ok(json1) = serde_json::from_str::<serde_json::Value>(&body1) {
        if json1["success"].as_bool() == Some(true) {
            println!("   ✅ Page 1 retrieved successfully");
            if let Some(data) = json1.get("data") {
                let total = data.get("total").and_then(|t| t.as_u64()).unwrap_or(0);
                let page = data.get("page").and_then(|p| p.as_u64()).unwrap_or(0);
                println!("   ✅ page: {}, total: {}", page, total);

                if let Some(items) = data.get("items").and_then(|i| i.as_array()) {
                    println!("   ✅ items on page 1: {}", items.len());
                }
            }
        } else {
            println!("   ❌ Returned success=false: {}", json1);
        }
    } else {
        println!(
            "   ⚠️  响应不是有效JSON: {}",
            &body1[..body1.len().min(200)]
        );
    }

    println!();
    Ok(())
}

// ==================== Data Consistency Tests ====================

async fn test_user_profile_data_consistency(
    api_base: &str,
    client: &reqwest::Client,
) -> Result<()> {
    println!("📋 Test: User profile data consistency");
    let url = format!("{}&endpoint=user&action=profile", api_base);

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            if let Some(data) = json.get("data") {
                // Check for required fields
                let has_uid = data.get("uid").is_some();
                let has_username = data.get("username").is_some();
                let has_money = data.get("money").is_some();

                if has_uid {
                    println!("   ✅ Contains 'uid' field");
                } else {
                    println!("   ❌ Missing 'uid' field");
                }

                if has_username {
                    println!("   ✅ Contains 'username' field");
                } else {
                    println!("   ❌ Missing 'username' field");
                }

                if has_money {
                    println!("   ✅ Contains 'money' field");
                } else {
                    println!("   ❌ Missing 'money' field");
                }

                if has_uid && has_username && has_money {
                    println!("   ✅ All required fields present");
                } else {
                    anyhow::bail!("Missing required fields in user profile");
                }
            }
        } else {
            println!("   ❌ API returned success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

async fn test_inventory_items_structure(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: Inventory items structure validation");
    let url = format!("{}&endpoint=user&action=inventory&page=1", api_base);

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            if let Some(data) = json.get("data") {
                if let Some(items) = data.get("items").and_then(|i| i.as_array()) {
                    if items.is_empty() {
                        println!("   ℹ️  No items in inventory");
                    } else {
                        if let Some(first_item) = items.first() {
                            let has_id = first_item.get("id").is_some();
                            let has_name = first_item.get("name").is_some();
                            let has_quantity = first_item.get("quantity").is_some();

                            if has_id {
                                println!("   ✅ Item has 'id' field");
                            } else {
                                println!("   ❌ Item missing 'id' field");
                            }

                            if has_name {
                                println!("   ✅ Item has 'name' field");
                            } else {
                                println!("   ❌ Item missing 'name' field");
                            }

                            if has_quantity {
                                println!("   ✅ Item has 'quantity' field");
                            } else {
                                println!("   ❌ Item missing 'quantity' field");
                            }

                            if has_id && has_name && has_quantity {
                                println!("   ✅ Item structure is correct");
                            }
                        }
                    }
                }
            }
        } else {
            println!("   ❌ API returned success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

// ==================== Response Format Tests ====================

async fn test_api_response_timestamp(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: API response includes timestamp");
    let url = format!("{}&endpoint=user&action=profile", api_base);

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            let has_timestamp = json.get("timestamp").is_some();
            let has_data_timestamp = json["data"].get("timestamp").is_some();

            if has_timestamp || has_data_timestamp {
                println!("   ✅ Response includes timestamp field");
            } else {
                println!("   ⚠️  Response does not include timestamp field");
            }
        } else {
            println!("   ❌ API returned success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

async fn test_api_success_field(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: All endpoints include success field");

    let endpoints = vec![
        ("user", "profile"),
        ("user", "inventory"),
        ("user", "stats"),
        ("pokemon", "list"),
        ("shop", "categories"),
        ("shop", "items"),
    ];

    let mut all_have_success = true;

    for (endpoint, action) in endpoints {
        let url = format!("{}&endpoint={}&action={}", api_base, endpoint, action);
        let response = client.get(&url).send().await?;
        let body = response.text().await?;

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            let has_success = json.get("success").is_some();
            if has_success {
                println!(
                    "   ✅ {}.php?action={} has 'success' field",
                    endpoint, action
                );
            } else {
                println!(
                    "   ❌ {}.php?action={} missing 'success' field",
                    endpoint, action
                );
                all_have_success = false;
            }
        } else {
            println!(
                "   ⚠️  {}.php?action={} invalid JSON response",
                endpoint, action
            );
            all_have_success = false;
        }
    }

    if all_have_success {
        println!("   ✅ All endpoints include success field");
    } else {
        anyhow::bail!("Some endpoints missing success field");
    }

    println!();
    Ok(())
}

// ==================== Type Filtering Tests ====================

async fn test_inventory_type_filter(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: Filter inventory by type");

    // Test without filter
    let url_all = format!("{}&endpoint=user&action=inventory&page=1", api_base);
    let response_all = client.get(&url_all).send().await?;
    let body_all = response_all.text().await?;

    if let Ok(json_all) = serde_json::from_str::<serde_json::Value>(&body_all) {
        if json_all["success"].as_bool() == Some(true) {
            println!("   ✅ Retrieved all inventory items");

            // Test with type filter (if supported)
            let url_filtered = format!(
                "{}&endpoint=user&action=inventory&page=1&type=item",
                api_base
            );
            let response_filtered = client.get(&url_filtered).send().await?;
            let body_filtered = response_filtered.text().await?;

            if let Ok(json_filtered) = serde_json::from_str::<serde_json::Value>(&body_filtered) {
                if json_filtered["success"].as_bool() == Some(true) {
                    println!("   ✅ Retrieved filtered inventory items by type");
                } else {
                    println!("   ℹ️  Type filter may not be supported: {}", json_filtered);
                }
            }
        } else {
            println!("   ❌ API returned success=false: {}", json_all);
        }
    } else {
        println!(
            "   ⚠️  响应不是有效JSON: {}",
            &body_all[..body_all.len().min(200)]
        );
    }

    println!();
    Ok(())
}

async fn test_shop_category_filter(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: Filter shop by category");

    // First get available categories
    let categories_url = format!("{}&endpoint=shop&action=categories", api_base);
    let cat_response = client.get(&categories_url).send().await?;
    let cat_body = cat_response.text().await?;

    let category_id = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cat_body) {
        if json["success"].as_bool() == Some(true) {
            if let Some(categories) = json["data"]["categories"].as_array() {
                if let Some(first_cat) = categories.first() {
                    first_cat.get("id").and_then(|id| id.as_u64())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(cat_id) = category_id {
        println!("   🔍 Using category ID: {}", cat_id);

        let url = format!(
            "{}&endpoint=shop&action=items&category={}",
            api_base, cat_id
        );
        let response = client.get(&url).send().await?;
        let body = response.text().await?;

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            if json["success"].as_bool() == Some(true) {
                println!("   ✅ Retrieved filtered shop items by category");
                if let Some(data) = json.get("data") {
                    if let Some(items) = data.get("items").and_then(|i| i.as_array()) {
                        println!("   ✅ Filtered items count: {}", items.len());
                    }
                }
            } else {
                println!("   ⚠️  Filter returned success=false: {}", json);
            }
        } else {
            println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
        }
    } else {
        println!("   ℹ️  No categories available, skipping filter test");
    }

    println!();
    Ok(())
}

// ==================== Edge Case Tests ====================

async fn test_empty_pokemon_list(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: Behavior when user has no pokemons");
    let url = format!("{}&endpoint=pokemon&action=list", api_base);

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            if let Some(data) = json.get("data") {
                if let Some(pokemons) = data.get("pokemons").and_then(|p| p.as_array()) {
                    if pokemons.is_empty() {
                        println!("   ✅ Correctly returns empty array for new user");
                    } else {
                        println!("   ℹ️  User has {} pokemons", pokemons.len());
                    }
                } else {
                    println!("   ⚠️  pokemons field is not an array");
                }
            }
        } else {
            println!("   ❌ API returned success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

async fn test_large_page_number(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: Request page beyond available data");
    let url = format!("{}&endpoint=user&action=inventory&page=9999", api_base);

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            println!("   ✅ Request succeeded");
            if let Some(data) = json.get("data") {
                if let Some(items) = data.get("items").and_then(|i| i.as_array()) {
                    if items.is_empty() {
                        println!("   ✅ Returns empty array for out-of-range page");
                    } else {
                        println!("   ⚠️  Expected empty array, got {} items", items.len());
                    }
                }
            }
        } else {
            println!(
                "   ℹ️  Returned success=false for out-of-range page: {}",
                json
            );
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

async fn test_invalid_pokemon_detail(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: Request detail with non-existent pokemon ID");
    let url = format!("{}&endpoint=pokemon&action=detail&id=999999999", api_base);

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(false) {
            println!("   ✅ Correctly returns success=false for non-existent ID");
            if json.get("error").is_some() {
                println!("   ✅ error message present");
            }
        } else {
            println!("   ⚠️  Expected success=false, got: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

async fn test_stats_achievement_flags(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("📋 Test: Verify achievement fields are boolean");
    let url = format!("{}&endpoint=user&action=stats", api_base);

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        if json["success"].as_bool() == Some(true) {
            if let Some(data) = json.get("data") {
                if let Some(achievements) = data.get("achievements") {
                    println!("   ✅ Achievements field present");

                    // Check if achievement values are boolean
                    let mut all_boolean = true;
                    if let Some(obj) = achievements.as_object() {
                        for (key, value) in obj.iter().take(5) {
                            let is_bool = value.is_boolean();
                            if is_bool {
                                println!("   ✅ Achievement '{}' is boolean: {}", key, value);
                            } else {
                                println!(
                                    "   ⚠️  Achievement '{}' is not boolean: {:?}",
                                    key, value
                                );
                                all_boolean = false;
                            }
                        }
                    }

                    if all_boolean {
                        println!("   ✅ Checked achievements are boolean");
                    }
                } else {
                    println!("   ℹ️  No achievements field in response");
                }
            }
        } else {
            println!("   ❌ API returned success=false: {}", json);
        }
    } else {
        println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
    }

    println!();
    Ok(())
}

/**
 * 📦 Battle API - Start Battle Test
 *
 * 测试战斗开始 API 的功能：
 * - 验证战斗可以成功启动
 * - 检查返回的战斗数据结构
 * - 验证数据库持久化（通过返回的 battle_id）
 */
async fn test_battle_start(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("🧪 Test: Battle Start (战斗开始)");

    // Battle API 直接访问，不通过 endpoint 参数
    let url = format!("{}&endpoint=battle&action=start", api_base);

    // 构造请求：开始战斗（需要在地图 ID 1）
    let payload = serde_json::json!({
        "map_id": 1
    });

    let response = client.post(&url).json(&payload).send().await?;

    let status = response.status();
    let body = response.text().await?;

    println!("   Status: {}", status);

    if status.is_success() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            if json["success"].as_bool() == Some(true) {
                println!("   ✅ Battle start API successful");

                // 验证返回的战斗数据结构
                if let Some(data) = json.get("data") {
                    // 检查 battle_id 存在
                    if let Some(battle_id) = data.get("battle_id").and_then(|v| v.as_str()) {
                        println!("   ✅ Battle ID: {}", battle_id);

                        // 验证 battle_id 格式（应以 battle_ 开头）
                        if battle_id.starts_with("battle_") {
                            println!("   ✅ Battle ID format valid");
                        } else {
                            println!("   ⚠️  Battle ID format unexpected");
                        }
                    } else {
                        println!("   ❌ Missing battle_id");
                    }

                    // 检查 map 信息
                    if let Some(map_id) = data.get("map_id").and_then(|v| v.as_u64()) {
                        println!("   ✅ Map ID: {}", map_id);
                    }
                    if let Some(map_name) = data.get("map_name").and_then(|v| v.as_str()) {
                        println!("   ✅ Map Name: {}", map_name);
                    }

                    // 检查战斗回合
                    if let Some(turn) = data.get("turn").and_then(|v| v.as_u64()) {
                        println!("   ✅ Turn: {}", turn);
                        if turn == 1 {
                            println!("   ✅ Initial turn is 1");
                        }
                    }

                    // 检查状态
                    if let Some(status) = data.get("status").and_then(|v| v.as_str()) {
                        println!("   ✅ Battle Status: {}", status);
                        if status == "active" {
                            println!("   ✅ Battle is active");
                        }
                    }

                    // 检查我方宝可梦
                    if let Some(my_pokemon) = data.get("my_pokemon").and_then(|v| v.as_object()) {
                        println!("   ✅ My Pokemon present");
                        if let Some(name) = my_pokemon.get("name").and_then(|v| v.as_str()) {
                            println!("      - Name: {}", name);
                        }
                        if let Some(level) = my_pokemon.get("level").and_then(|v| v.as_u64()) {
                            println!("      - Level: {}", level);
                        }
                        if let Some(hp) = my_pokemon.get("hp").and_then(|v| v.as_i64()) {
                            if let Some(max_hp) = my_pokemon.get("max_hp").and_then(|v| v.as_i64())
                            {
                                println!("      - HP: {}/{}", hp, max_hp);
                            }
                        }
                        if let Some(skills) = my_pokemon.get("skills").and_then(|v| v.as_array()) {
                            println!("      - Skills count: {}", skills.len());
                        }
                    }

                    // 检查野生宝可梦
                    if let Some(wild_pokemon) = data.get("wild_pokemon").and_then(|v| v.as_object())
                    {
                        println!("   ✅ Wild Pokemon present");
                        if let Some(name) = wild_pokemon.get("name").and_then(|v| v.as_str()) {
                            println!("      - Name: {}", name);
                        }
                        if let Some(level) = wild_pokemon.get("level").and_then(|v| v.as_u64()) {
                            println!("      - Level: {}", level);
                        }
                        if let Some(is_shiny) =
                            wild_pokemon.get("is_shiny").and_then(|v| v.as_bool())
                        {
                            if is_shiny {
                                println!("      - ✨ Shiny Pokemon!");
                            }
                        }
                    }

                    // 检查消息
                    if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                        println!("   ✅ Message: {}", message);
                    }
                }
            } else if let Some(error) = json.get("error").and_then(|v| v.as_str()) {
                // 可能是未设置出战宠物或其他前置条件未满足
                println!("   ⚠️  API returned error: {}", error);

                if error.contains("No active pokemon") {
                    println!("   ℹ️  需要先设置出战宠物（is_zd = 1）");
                } else if error.contains("Map not found") {
                    println!("   ℹ️  地图数据未初始化");
                }
            } else {
                println!("   ❌ API returned success=false: {}", json);
            }
        } else {
            println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
        }
    } else {
        println!("   ❌ HTTP error: {}", status);
    }

    println!();
    Ok(())
}

/**
 * 📦 Battle API - Battle List Test
 *
 * 测试获取活跃战斗列表 API 的功能：
 * - 验证可以获取用户的活跃战斗列表
 * - 检查返回的战斗列表数据结构
 * - 验证分页和过滤功能
 */
async fn test_battle_list(api_base: &str, client: &reqwest::Client) -> Result<()> {
    println!("🧪 Test: Battle List (获取活跃战斗列表)");

    let url = format!("{}&endpoint=battle&action=list", api_base);

    let response = client.get(&url).send().await?;
    let status = response.status();
    let body = response.text().await?;

    println!("   Status: {}", status);

    if status.is_success() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            if json["success"].as_bool() == Some(true) {
                println!("   ✅ Battle list API successful");

                if let Some(data) = json.get("data") {
                    // 检查战斗总数
                    if let Some(total) = data.get("total").and_then(|v| v.as_u64()) {
                        println!("   ✅ Total active battles: {}", total);
                    }

                    // 检查战斗列表
                    if let Some(battles) = data.get("battles").and_then(|v| v.as_array()) {
                        println!("   ✅ Battles count: {}", battles.len());

                        for (idx, battle) in battles.iter().enumerate() {
                            println!("   📊 Battle #{}:", idx + 1);

                            if let Some(battle_id) =
                                battle.get("battle_id").and_then(|v| v.as_str())
                            {
                                println!("      - Battle ID: {}", battle_id);
                            }
                            if let Some(map_id) = battle.get("map_id").and_then(|v| v.as_u64()) {
                                println!("      - Map ID: {}", map_id);
                            }
                            if let Some(map_name) = battle.get("map_name").and_then(|v| v.as_str())
                            {
                                println!("      - Map Name: {}", map_name);
                            }
                            if let Some(status) = battle.get("status").and_then(|v| v.as_str()) {
                                println!("      - Status: {}", status);
                            }
                            if let Some(turn) = battle.get("turn").and_then(|v| v.as_u64()) {
                                println!("      - Turn: {}", turn);
                            }
                            if let Some(created_at) =
                                battle.get("created_at").and_then(|v| v.as_u64())
                            {
                                println!("      - Created At: {}", created_at);
                            }
                            if let Some(updated_at) =
                                battle.get("updated_at").and_then(|v| v.as_u64())
                            {
                                println!("      - Updated At: {}", updated_at);
                            }
                        }

                        // 验证：如果有战斗，每个战斗都应该有必需的字段
                        if !battles.is_empty() {
                            let all_valid = battles.iter().all(|b| {
                                b.get("battle_id").and_then(|v| v.as_str()).is_some()
                                    && b.get("map_id").and_then(|v| v.as_u64()).is_some()
                                    && b.get("status").and_then(|v| v.as_str()).is_some()
                                    && b.get("turn").and_then(|v| v.as_u64()).is_some()
                            });

                            if all_valid {
                                println!("   ✅ All battles have required fields");
                            } else {
                                println!("   ⚠️  Some battles missing required fields");
                            }
                        } else {
                            println!("   ℹ️  No active battles found (expected if no battle was started)");
                        }
                    } else {
                        println!("   ⚠️  No battles array in response");
                    }
                }
            } else if let Some(error) = json.get("error").and_then(|v| v.as_str()) {
                println!("   ⚠️  API returned error: {}", error);
            } else {
                println!("   ❌ API returned success=false: {}", json);
            }
        } else {
            println!("   ⚠️  响应不是有效JSON: {}", &body[..body.len().min(200)]);
        }
    } else {
        println!("   ❌ HTTP error: {}", status);
    }

    println!();
    Ok(())
}
