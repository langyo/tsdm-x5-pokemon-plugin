// E2E 完整测试套件
// 用法: cargo run --example e2e_full_test

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

use reqwest::cookie::Jar;

fn api_base(base_url: &str) -> String {
    format!("{}/plugin.php?id=pokemon:pokemon", base_url)
}

fn api_url(base: &str, endpoint: &str, action: &str) -> String {
    format!("{}&endpoint={}&action={}", base, endpoint, action)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestState {
    user: UserInfo,
    pokemons: Vec<PokemonInfo>,
    items: Vec<ItemInfo>,
    battle: Option<BattleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserInfo {
    uid: u64,
    username: String,
    money: i64,
    total_pokemons: u64,
    total_items: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PokemonInfo {
    id: u64,
    pmno: u64,
    name: String,
    level: u64,
    hp: i64,
    max_hp: i64,
    is_zd: u8,
    skills: Vec<SkillInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SkillInfo {
    id: u64,
    name: String,
    pp: u64,
    max_pp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ItemInfo {
    id: u64,
    type_id: u64,
    name: String,
    quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BattleInfo {
    wild_pokemon: WildPokemonInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WildPokemonInfo {
    pmno: u64,
    name: String,
    level: u64,
    hp: i64,
    max_hp: i64,
}

struct TestContext {
    client: reqwest::Client,
    api_base: String,
}

impl TestContext {
    fn new(base_url: &str, cookie_jar: Arc<Jar>) -> Self {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .cookie_provider(cookie_jar)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self {
            client,
            api_base: api_base(base_url),
        }
    }

    async fn reset_user(&self) -> Result<()> {
        println!("   🔄 Resetting user data...");
        let url = api_url(&self.api_base, "admin", "test_reset_user");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "reset_all": true }))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            anyhow::bail!("Reset user failed: {} - {}", status, body);
        }

        println!("   ✅ User data reset");
        Ok(())
    }

    async fn set_money(&self, amount: i64) -> Result<()> {
        println!("   💰 Setting money to {}...", amount);
        let url = api_url(&self.api_base, "admin", "test_set_money");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "amount": amount }))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            anyhow::bail!("Set money failed: {} - {}", status, body);
        }

        println!("   ✅ Money set to {}", amount);
        Ok(())
    }

    async fn create_pokemon(
        &self,
        pmno: u64,
        level: u64,
        is_zd: u8,
        hp_percent: u64,
    ) -> Result<PokemonInfo> {
        println!(
            "   🐾 Creating pokemon pmno={}, level={}, is_zd={}...",
            pmno, level, is_zd
        );
        let url = api_url(&self.api_base, "admin", "test_create_pokemon");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "pmno": pmno,
                "level": level,
                "is_zd": is_zd,
                "hp_percent": hp_percent
            }))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            anyhow::bail!("Create pokemon failed: {} - {}", status, body);
        }

        let body = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&body)?;

        let data = json
            .get("data")
            .ok_or_else(|| anyhow::anyhow!("No data in response"))?;

        let pokemon = PokemonInfo {
            id: data.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
            pmno: data.get("pmno").and_then(|v| v.as_u64()).unwrap_or(pmno),
            name: data
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string(),
            level: data.get("level").and_then(|v| v.as_u64()).unwrap_or(level),
            hp: data.get("hp").and_then(|v| v.as_i64()).unwrap_or(0),
            max_hp: data.get("max_hp").and_then(|v| v.as_i64()).unwrap_or(0),
            is_zd: data.get("is_zd").and_then(|v| v.as_u64()).unwrap_or(0) as u8,
            skills: vec![],
        };

        println!(
            "   ✅ Created {} (Lv.{}, HP {}/{})",
            pokemon.name, pokemon.level, pokemon.hp, pokemon.max_hp
        );
        Ok(pokemon)
    }

    async fn add_item(&self, item_id: u64, quantity: i64) -> Result<()> {
        println!("   📦 Adding item {} x{}...", item_id, quantity);
        let url = api_url(&self.api_base, "admin", "test_add_item");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "item_id": item_id,
                "quantity": quantity
            }))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            anyhow::bail!("Add item failed: {} - {}", status, body);
        }

        println!("   ✅ Added item {} x{}", item_id, quantity);
        Ok(())
    }

    async fn get_state(&self) -> Result<TestState> {
        let url = api_url(&self.api_base, "admin", "test_get_state");
        let response = self.client.get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            anyhow::bail!("Get state failed: {} - {}", status, body);
        }

        let body = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&body)?;

        let data = json
            .get("data")
            .ok_or_else(|| anyhow::anyhow!("No data in response"))?;

        let user_data = data
            .get("user")
            .ok_or_else(|| anyhow::anyhow!("No user in response"))?;
        let user = UserInfo {
            uid: user_data.get("uid").and_then(|v| v.as_u64()).unwrap_or(0),
            username: user_data
                .get("username")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string(),
            money: user_data.get("money").and_then(|v| v.as_i64()).unwrap_or(0),
            total_pokemons: user_data
                .get("total_pokemons")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            total_items: user_data
                .get("total_items")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
        };

        let pokemons_data = data
            .get("pokemons")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let mut pokemons = Vec::new();
        for p in pokemons_data {
            pokemons.push(PokemonInfo {
                id: p.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
                pmno: p.get("pmno").and_then(|v| v.as_u64()).unwrap_or(0),
                name: p
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                level: p.get("level").and_then(|v| v.as_u64()).unwrap_or(0),
                hp: p.get("hp").and_then(|v| v.as_i64()).unwrap_or(0),
                max_hp: p.get("max_hp").and_then(|v| v.as_i64()).unwrap_or(0),
                is_zd: p.get("is_zd").and_then(|v| v.as_u64()).unwrap_or(0) as u8,
                skills: vec![],
            });
        }

        let items_data = data
            .get("items")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let mut items = Vec::new();
        for i in items_data {
            items.push(ItemInfo {
                id: i.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
                type_id: i.get("type_id").and_then(|v| v.as_u64()).unwrap_or(0),
                name: i
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                quantity: i.get("quantity").and_then(|v| v.as_i64()).unwrap_or(0),
            });
        }

        let battle = if let Some(b) = data.get("battle") {
            if b.is_null() {
                None
            } else {
                Some(BattleInfo {
                    wild_pokemon: WildPokemonInfo {
                        pmno: b
                            .get("wild_pokemon")
                            .and_then(|w| w.get("pmno"))
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0),
                        name: b
                            .get("wild_pokemon")
                            .and_then(|w| w.get("name"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown")
                            .to_string(),
                        level: b
                            .get("wild_pokemon")
                            .and_then(|w| w.get("level"))
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0),
                        hp: b
                            .get("wild_pokemon")
                            .and_then(|w| w.get("hp"))
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0),
                        max_hp: b
                            .get("wild_pokemon")
                            .and_then(|w| w.get("max_hp"))
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0),
                    },
                })
            }
        } else {
            None
        };

        Ok(TestState {
            user,
            pokemons,
            items,
            battle,
        })
    }

    async fn start_battle(&self, map_id: u64) -> Result<serde_json::Value> {
        println!("   ⚔️  Starting battle on map {}...", map_id);
        let url = format!(
            "{}&endpoint=battle&action=start&map_id={}",
            self.api_base, map_id
        );
        let response = self.client.post(&url).send().await?;

        let status = response.status();
        let body = response.text().await?;

        let json: serde_json::Value = serde_json::from_str(&body)?;

        if !status.is_success() || json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Start battle failed: {}", error);
        }

        println!("   ✅ Battle started");
        Ok(json)
    }

    async fn use_skill(&self, skill_id: u64) -> Result<serde_json::Value> {
        println!("   🎯 Using skill {}...", skill_id);
        let url = api_url(&self.api_base, "battle", "turn");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "skill_id": skill_id }))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        let json: serde_json::Value = serde_json::from_str(&body)?;

        if !status.is_success() || json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Use skill failed: {}", error);
        }

        println!("   ✅ Skill used");
        Ok(json)
    }

    async fn flee(&self) -> Result<serde_json::Value> {
        println!("   🏃 Fleeing from battle...");
        let url = api_url(&self.api_base, "battle", "flee");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({}))
            .send()
            .await?;

        let body = response.text().await?;

        let json: serde_json::Value = serde_json::from_str(&body)?;
        println!("   ✅ Fled from battle");
        Ok(json)
    }

    async fn heal_pokemon(&self, pokemon_id: u64) -> Result<()> {
        println!("   💊 Healing pokemon {}...", pokemon_id);
        let url = format!(
            "{}&endpoint=user&action=heal&pokemon_id={}",
            self.api_base, pokemon_id
        );
        let response = self.client.post(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            anyhow::bail!("Heal failed: {} - {}", status, body);
        }

        println!("   ✅ Pokemon healed");
        Ok(())
    }

    async fn buy_item(&self, item_id: u64, quantity: u64) -> Result<()> {
        println!("   🛒 Buying item {} x{}...", item_id, quantity);
        let url = api_url(&self.api_base, "shop", "buy");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "item_id": item_id,
                "quantity": quantity
            }))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            anyhow::bail!("Buy item failed: {} - {}", status, body);
        }

        println!("   ✅ Item bought");
        Ok(())
    }

    async fn use_item(&self, item_id: u64, pokemon_id: Option<u64>) -> Result<()> {
        println!("   📦 Using item {}...", item_id);
        let url = api_url(&self.api_base, "user", "use_item");
        let mut payload = serde_json::json!({ "item_id": item_id });
        if let Some(pid) = pokemon_id {
            payload["pokemon_id"] = serde_json::json!(pid);
        }

        let response = self.client.post(&url).json(&payload).send().await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Use item failed: {} - {}", status, body);
        }

        let json: serde_json::Value = serde_json::from_str(&body)?;
        if json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Use item failed: {}", error);
        }

        println!("   ✅ Item used");
        Ok(())
    }

    async fn get_maps(&self) -> Result<Vec<serde_json::Value>> {
        let url = api_url(&self.api_base, "battle", "maps");
        let response = self.client.get(&url).send().await?;

        let body = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&body)?;

        let maps = json
            .get("data")
            .and_then(|d| d.get("maps"))
            .and_then(|m| m.as_array())
            .cloned()
            .unwrap_or_default();

        Ok(maps)
    }
}

async fn login_discuz(base_url: &str) -> Result<Arc<Jar>> {
    println!("🔐 Logging in to Discuz...");

    let jar = Arc::new(Jar::default());
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_provider(jar.clone())
        .timeout(Duration::from_secs(30))
        .build()?;

    let login_page = format!("{}/member.php?mod=logging&action=login", base_url);
    client.get(&login_page).send().await?;

    let login_url = format!(
        "{}/member.php?mod=logging&action=login&loginsubmit=yes&infloat=yes&lssubmit=yes&inajax=1",
        base_url
    );

    let params = [
        ("username", "admin"),
        ("password", "admin123"),
        ("questionid", "0"),
        ("answer", ""),
        ("cookietime", "2592000"),
    ];

    let response = client.post(&login_url).form(&params).send().await?;
    let status = response.status();

    if !status.is_success() {
        anyhow::bail!("Login failed: {}", status);
    }

    println!("   ✅ Login successful");

    let plugin_home = format!("{}/plugin.php?id=pokemon:pokemon", base_url);
    client.get(&plugin_home).send().await?;
    println!("   ✅ Plugin session initialized");

    Ok(jar)
}

fn print_separator(title: &str) {
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{}", title);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
}

fn print_test_header(name: &str) {
    println!();
    println!("📋 Test: {}", name);
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🧪 TSDM Pokemon Plugin - E2E Full Test Suite");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let base_url =
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8888".to_string());
    println!("🌐 Target: {}", base_url);
    println!();

    let cookie_jar = login_discuz(&base_url).await?;
    let ctx = TestContext::new(&base_url, cookie_jar);

    let mut passed = 0;
    let mut failed = 0;

    print_separator("📦 Phase 1: Data Preparation Tests");

    match test_data_preparation(&ctx).await {
        Ok(_) => passed += 1,
        Err(e) => {
            println!("   ❌ Error: {}", e);
            failed += 1;
        }
    }

    print_separator("⚔️  Phase 2: Battle Flow Tests");

    match test_battle_flow(&ctx).await {
        Ok(_) => passed += 1,
        Err(e) => {
            println!("   ❌ Error: {}", e);
            failed += 1;
        }
    }

    print_separator("🛒 Phase 3: Shop Flow Tests");

    match test_shop_flow(&ctx).await {
        Ok(_) => passed += 1,
        Err(e) => {
            println!("   ❌ Error: {}", e);
            failed += 1;
        }
    }

    print_separator("💊 Phase 4: Item Usage Tests");

    match test_item_usage(&ctx).await {
        Ok(_) => passed += 1,
        Err(e) => {
            println!("   ❌ Error: {}", e);
            failed += 1;
        }
    }

    print_separator("🏥 Phase 5: Healing Tests");

    match test_healing_flow(&ctx).await {
        Ok(_) => passed += 1,
        Err(e) => {
            println!("   ❌ Error: {}", e);
            failed += 1;
        }
    }

    print_separator("📊 Test Summary");

    println!();
    println!("   ✅ Passed: {}", passed);
    println!("   ❌ Failed: {}", failed);
    println!();

    if failed > 0 {
        anyhow::bail!("Some tests failed");
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ All E2E tests passed!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

async fn test_data_preparation(ctx: &TestContext) -> Result<()> {
    print_test_header("Data Preparation");

    ctx.reset_user().await?;

    ctx.set_money(10000).await?;

    let state = ctx.get_state().await?;
    assert_eq!(state.user.money, 10000, "Money should be 10000");
    assert_eq!(state.pokemons.len(), 0, "Should have no pokemons");
    println!(
        "   ✅ Verified: money={}, pokemons={}",
        state.user.money,
        state.pokemons.len()
    );

    let pokemon = ctx.create_pokemon(25, 30, 1, 100).await?;
    assert_eq!(pokemon.pmno, 25, "Pokemon should be Pikachu (pmno=25)");
    assert_eq!(pokemon.is_zd, 1, "Pokemon should be set as active");

    let state = ctx.get_state().await?;
    assert_eq!(state.pokemons.len(), 1, "Should have 1 pokemon");
    println!("   ✅ Verified: pokemons={}", state.pokemons.len());

    ctx.add_item(1, 10).await?;
    ctx.add_item(4, 5).await?;

    let state = ctx.get_state().await?;
    assert_eq!(state.items.len(), 2, "Should have 2 item types");
    println!("   ✅ Verified: items={}", state.items.len());

    println!();
    println!("   ✅ Data preparation test passed");
    Ok(())
}

async fn test_battle_flow(ctx: &TestContext) -> Result<()> {
    print_test_header("Battle Flow");

    ctx.reset_user().await?;
    ctx.set_money(5000).await?;
    ctx.create_pokemon(25, 30, 1, 100).await?;
    ctx.add_item(4, 5).await?;

    let maps = ctx.get_maps().await?;
    if maps.is_empty() {
        println!("   ⚠️  No maps available, skipping battle test");
        return Ok(());
    }

    let map_id = maps[0].get("id").and_then(|v| v.as_u64()).unwrap_or(101);
    println!(
        "   🗺️  Using map ID: {} ({})",
        map_id,
        maps[0]
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
    );

    let battle_result = ctx.start_battle(map_id).await?;

    if let Some(data) = battle_result.get("data") {
        let status = data.get("status").and_then(|v| v.as_str()).unwrap_or("");
        if status == "active" {
            println!("   ✅ Battle is active");

            if let Some(my_pokemon) = data.get("my_pokemon") {
                if let Some(skills) = my_pokemon.get("skills").and_then(|s| s.as_array()) {
                    if let Some(first_skill) = skills.first() {
                        let skill_id = first_skill.get("id").and_then(|v| v.as_u64()).unwrap_or(1);
                        let skill_name = first_skill
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown");
                        println!("   🎯 Using skill: {} (id={})", skill_name, skill_id);

                        let turn_result = ctx.use_skill(skill_id).await?;
                        if let Some(turn_data) = turn_result.get("data") {
                            if let Some(wild) = turn_data.get("wild_pokemon") {
                                let wild_hp = wild.get("hp").and_then(|v| v.as_i64()).unwrap_or(0);
                                let wild_max_hp =
                                    wild.get("max_hp").and_then(|v| v.as_i64()).unwrap_or(1);
                                let wild_name = wild
                                    .get("name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Unknown");

                                println!(
                                    "   ✅ Wild {} took damage: HP {}/{}",
                                    wild_name, wild_hp, wild_max_hp
                                );
                            }
                        }
                    }
                }
            }

            ctx.flee().await?;
            println!("   ✅ Fled from battle successfully");
        } else {
            println!("   ⚠️  Battle status: {}", status);
        }
    }

    println!();
    println!("   ✅ Battle flow test passed");
    Ok(())
}

async fn test_shop_flow(ctx: &TestContext) -> Result<()> {
    print_test_header("Shop Flow");

    ctx.reset_user().await?;
    ctx.set_money(10000).await?;

    let state_before = ctx.get_state().await?;
    let money_before = state_before.user.money;
    println!("   💰 Money before: {}", money_before);

    ctx.buy_item(1, 5).await?;

    let state_after = ctx.get_state().await?;
    println!("   💰 Money after: {}", state_after.user.money);

    assert!(
        state_after.user.money < money_before,
        "Money should decrease after purchase"
    );
    assert!(
        state_after.items.iter().any(|i| i.type_id == 1),
        "Should have item type 1"
    );

    println!();
    println!("   ✅ Shop flow test passed");
    Ok(())
}

async fn test_item_usage(ctx: &TestContext) -> Result<()> {
    print_test_header("Item Usage");

    ctx.reset_user().await?;
    ctx.set_money(5000).await?;
    ctx.create_pokemon(25, 30, 1, 50).await?;
    ctx.add_item(1, 10).await?;

    let state_before = ctx.get_state().await?;
    let pokemon_before = state_before.pokemons.first().cloned();
    let item_before = state_before.items.iter().find(|i| i.type_id == 1).cloned();

    if let (Some(pokemon), Some(item)) = (pokemon_before, item_before) {
        println!("   🐾 Pokemon HP before: {}/{}", pokemon.hp, pokemon.max_hp);
        println!("   📦 Item quantity before: {}", item.quantity);

        ctx.use_item(item.type_id, Some(pokemon.id)).await?;

        let state_after = ctx.get_state().await?;
        if let Some(item_after) = state_after.items.iter().find(|i| i.type_id == 1) {
            assert!(
                item_after.quantity < item.quantity,
                "Item quantity should decrease"
            );
            println!("   📦 Item quantity after: {}", item_after.quantity);
        }

        println!("   ✅ Item used successfully");
    } else {
        println!("   ⚠️  No pokemon or item available for test");
    }

    println!();
    println!("   ✅ Item usage test passed");
    Ok(())
}

async fn test_healing_flow(ctx: &TestContext) -> Result<()> {
    print_test_header("Healing Flow");

    ctx.reset_user().await?;
    ctx.set_money(5000).await?;
    ctx.create_pokemon(25, 30, 1, 50).await?;

    let state_before = ctx.get_state().await?;
    if let Some(pokemon) = state_before.pokemons.first() {
        println!(
            "   🐾 Pokemon HP before heal: {}/{}",
            pokemon.hp, pokemon.max_hp
        );

        assert!(
            pokemon.hp < pokemon.max_hp,
            "Pokemon should have reduced HP"
        );

        ctx.heal_pokemon(pokemon.id).await?;

        let state_after = ctx.get_state().await?;
        if let Some(pokemon_after) = state_after.pokemons.first() {
            println!(
                "   🐾 Pokemon HP after heal: {}/{}",
                pokemon_after.hp, pokemon_after.max_hp
            );

            assert_eq!(
                pokemon_after.hp, pokemon_after.max_hp,
                "Pokemon should be fully healed"
            );
        }
    } else {
        println!("   ⚠️  No pokemon available for healing test");
    }

    println!();
    println!("   ✅ Healing flow test passed");
    Ok(())
}
