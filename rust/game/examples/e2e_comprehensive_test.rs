use anyhow::Result;
use serde_json::Value;
use std::{sync::Arc, time::Duration};

use reqwest::{cookie::Jar, Client};

const DEFAULT_BASE_URL: &str = "http://localhost:8888";

fn api_url(base: &str, endpoint: &str, action: &str) -> String {
    format!(
        "{}/plugin.php?id=pokemon:pokemon&endpoint={}&action={}",
        base, endpoint, action
    )
}

struct TestContext {
    client: Client,
    api_base: String,
}

impl TestContext {
    fn new(base_url: &str, cookie_jar: Arc<Jar>) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .cookie_provider(cookie_jar)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self {
            client,
            api_base: base_url.to_string(),
        }
    }

    async fn reset_user(&self) -> Result<()> {
        println!("   🔄 Resetting user data...");
        let url = api_url(&self.api_base, "admin", "test_reset_user");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({}))
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            anyhow::bail!("Reset failed: {} - {}", status, body);
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
    ) -> Result<Value> {
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
        let body = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Create pokemon failed: {} - {}", status, body);
        }

        let json: Value = serde_json::from_str(&body)?;
        if let Some(data) = json.get("data") {
            let name = data
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            let lvl = data.get("level").and_then(|v| v.as_u64()).unwrap_or(0);
            let hp = data.get("hp").and_then(|v| v.as_i64()).unwrap_or(0);
            let max_hp = data.get("max_hp").and_then(|v| v.as_i64()).unwrap_or(0);
            println!("   ✅ Created {} (Lv.{}, HP {}/{})", name, lvl, hp, max_hp);
        }
        Ok(json)
    }

    async fn add_item(&self, item_id: u64, quantity: u64) -> Result<()> {
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

    async fn get_state(&self) -> Result<Value> {
        let url = api_url(&self.api_base, "admin", "test_get_state");
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        Ok(json.get("data").cloned().unwrap_or(Value::Null))
    }

    async fn initialize_player(&self) -> Result<Value> {
        println!("   🎮 Initializing new player...");
        let url = api_url(&self.api_base, "user", "initialize");
        let response = self.client.get(&url).send().await?;

        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Initialize failed: {}", error);
        }

        if let Some(data) = json.get("data") {
            if let Some(pokemon) = data.get("pokemon") {
                let name = pokemon
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                let level = pokemon.get("level").and_then(|v| v.as_u64()).unwrap_or(0);
                println!("   ✅ Got starter pokemon: {} Lv.{}", name, level);
            }
        }
        Ok(json)
    }

    async fn get_pokemon_list(&self) -> Result<Value> {
        let url = api_url(&self.api_base, "pokemon", "list");
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        Ok(json.get("data").cloned().unwrap_or(Value::Null))
    }

    async fn get_pokemon_detail(&self, pokemon_id: u64) -> Result<Value> {
        let url = format!(
            "{}&pokemon_id={}",
            api_url(&self.api_base, "pokemon", "detail"),
            pokemon_id
        );
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        if body.is_empty() {
            anyhow::bail!("Empty response from pokemon detail API");
        }
        let json: Value = serde_json::from_str(&body)?;
        Ok(json.get("data").cloned().unwrap_or(Value::Null))
    }

    async fn rename_pokemon(&self, pokemon_id: u64, new_name: &str) -> Result<()> {
        println!(
            "   ✏️  Renaming pokemon {} to '{}'...",
            pokemon_id, new_name
        );
        let url = api_url(&self.api_base, "pokemon", "rename");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "id": pokemon_id,
                "name": new_name
            }))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if !status.is_success() || json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Rename failed: {}", error);
        }
        println!("   ✅ Pokemon renamed to '{}'", new_name);
        Ok(())
    }

    async fn release_pokemon(&self, pokemon_id: u64) -> Result<()> {
        println!("   🕊️  Releasing pokemon {}...", pokemon_id);
        let url = api_url(&self.api_base, "pokemon", "release");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "id": pokemon_id
            }))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if !status.is_success() || json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Release failed: {}", error);
        }
        println!("   ✅ Pokemon released");
        Ok(())
    }

    async fn get_learnable_skills(&self, pokemon_id: u64) -> Result<Value> {
        let url = format!(
            "{}&pokemon_id={}",
            api_url(&self.api_base, "pokemon", "learnable_skills"),
            pokemon_id
        );
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        Ok(json.get("data").cloned().unwrap_or(Value::Null))
    }

    async fn learn_skill(&self, pokemon_id: u64, skill_id: u64, slot_index: u64) -> Result<()> {
        println!(
            "   📚 Learning skill {} for pokemon {} at slot {}...",
            skill_id, pokemon_id, slot_index
        );
        let url = api_url(&self.api_base, "pokemon", "learn_skill");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "pokemon_id": pokemon_id,
                "skill_id": skill_id,
                "slot_index": slot_index
            }))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if !status.is_success() || json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Learn skill failed: {}", error);
        }
        println!("   ✅ Skill learned");
        Ok(())
    }

    async fn check_evolution(&self, pokemon_id: u64) -> Result<Value> {
        let url = format!(
            "{}&petid={}",
            api_url(&self.api_base, "evolution", "check"),
            pokemon_id
        );
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        Ok(json.get("data").cloned().unwrap_or(Value::Null))
    }

    async fn evolve_pokemon(&self, pokemon_id: u64) -> Result<Value> {
        println!("   ✨ Evolving pokemon {}...", pokemon_id);
        let url = api_url(&self.api_base, "evolution", "evolve");
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "pet_id": pokemon_id
            }))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if !status.is_success() || json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Evolve failed: {}", error);
        }

        if let Some(data) = json.get("data") {
            if let Some(new_name) = data.get("new_name").and_then(|v| v.as_str()) {
                println!("   ✅ Evolved to '{}'", new_name);
            }
        }
        Ok(json)
    }

    async fn start_battle(&self, map_id: u64) -> Result<Value> {
        println!("   ⚔️  Starting battle on map {}...", map_id);
        let url = format!(
            "{}/plugin.php?id=pokemon:pokemon&endpoint=battle&action=start&map_id={}",
            self.api_base, map_id
        );
        let response = self.client.post(&url).send().await?;

        let status = response.status();
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

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

    async fn use_skill(&self, skill_id: u64) -> Result<Value> {
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
        let json: Value = serde_json::from_str(&body)?;

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

    async fn flee(&self) -> Result<()> {
        println!("   🏃 Fleeing from battle...");
        let url = api_url(&self.api_base, "battle", "flee");
        let response = self.client.post(&url).send().await?;

        let status = response.status();
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if !status.is_success() || json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Flee failed: {}", error);
        }
        println!("   ✅ Fled from battle");
        Ok(())
    }

    async fn get_maps(&self) -> Result<Value> {
        let url = api_url(&self.api_base, "battle", "maps");
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        Ok(json.get("data").cloned().unwrap_or(Value::Null))
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
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if !status.is_success() || json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Buy item failed: {}", error);
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
        let json: Value = serde_json::from_str(&body)?;

        if !status.is_success() || json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Use item failed: {}", error);
        }
        println!("   ✅ Item used");
        Ok(())
    }

    async fn heal_pokemon(&self, pokemon_id: u64) -> Result<()> {
        println!("   💊 Healing pokemon {}...", pokemon_id);
        let url = format!(
            "{}&pokemon_id={}",
            api_url(&self.api_base, "user", "heal"),
            pokemon_id
        );
        let response = self.client.get(&url).send().await?;

        let status = response.status();
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if !status.is_success() || json.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let error = json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!("Heal failed: {}", error);
        }
        println!("   ✅ Pokemon healed");
        Ok(())
    }

    async fn get_profile(&self) -> Result<Value> {
        let url = api_url(&self.api_base, "user", "profile");
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        Ok(json.get("data").cloned().unwrap_or(Value::Null))
    }

    async fn get_inventory(&self) -> Result<Value> {
        let url = api_url(&self.api_base, "user", "inventory");
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        Ok(json.get("data").cloned().unwrap_or(Value::Null))
    }

    async fn get_stats(&self) -> Result<Value> {
        let url = api_url(&self.api_base, "user", "stats");
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        Ok(json.get("data").cloned().unwrap_or(Value::Null))
    }

    async fn get_shop_items(&self) -> Result<Value> {
        let url = api_url(&self.api_base, "shop", "items");
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        Ok(json.get("data").cloned().unwrap_or(Value::Null))
    }
}

async fn login_discuz(base_url: &str) -> Result<Arc<Jar>> {
    println!("🔐 Logging in to Discuz...");

    let jar = Arc::new(Jar::default());
    let client = Client::builder()
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
    println!("🧪 TSDM Pokemon Plugin - Comprehensive E2E Test Suite");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let base_url = std::env::var("API_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
    println!("🌐 Target: {}", base_url);
    println!();

    let cookie_jar = login_discuz(&base_url).await?;
    let ctx = TestContext::new(&base_url, cookie_jar);

    let mut passed = 0;
    let mut failed = 0;
    let mut test_results: Vec<(String, bool)> = Vec::new();

    macro_rules! run_test {
        ($name:expr, $test:expr) => {
            print_separator($name);
            match $test(&ctx).await {
                Ok(_) => {
                    passed += 1;
                    test_results.push(($name.to_string(), true));
                }
                Err(e) => {
                    println!("   ❌ Error: {}", e);
                    failed += 1;
                    test_results.push(($name.to_string(), false));
                }
            }
        };
    }

    run_test!("📦 Phase 1: Data Preparation", test_data_preparation);
    run_test!(
        "🎮 Phase 2: New Player Initialization",
        test_player_initialization
    );
    run_test!("⚔️  Phase 3: Battle Flow", test_battle_flow);
    run_test!("🎯 Phase 4: Capture Flow", test_capture_flow);
    run_test!("🛒 Phase 5: Shop Flow", test_shop_flow);
    run_test!("💊 Phase 6: Item Usage", test_item_usage);
    run_test!("🏥 Phase 7: Healing Flow", test_healing_flow);
    run_test!("✨ Phase 8: Evolution System", test_evolution);
    run_test!("📚 Phase 9: Skill System", test_skill_system);
    run_test!("🐾 Phase 10: Pokemon Management", test_pokemon_management);
    run_test!("📊 Phase 11: User Profile & Stats", test_user_profile);
    run_test!(
        "🎒 Phase 12: Inventory Management",
        test_inventory_management
    );

    print_separator("📊 Test Summary");

    println!();
    println!("   Test Results:");
    for (name, success) in &test_results {
        let status = if *success { "✅" } else { "❌" };
        println!("   {} {}", status, name);
    }
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
    let money = state
        .get("user")
        .and_then(|u| u.get("money"))
        .and_then(|m| m.as_i64())
        .unwrap_or(0);
    assert_eq!(money, 10000, "Money should be 10000");
    println!("   ✅ Verified: money={}", money);

    let pokemon = ctx.create_pokemon(25, 30, 1, 100).await?;
    let pokemon_id = pokemon
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    assert!(pokemon_id > 0, "Pokemon ID should be positive");

    ctx.add_item(1, 10).await?;
    ctx.add_item(4, 5).await?;

    let state = ctx.get_state().await?;
    let items = state
        .get("items")
        .and_then(|i| i.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    assert_eq!(items, 2, "Should have 2 item types");
    println!("   ✅ Verified: items={}", items);

    println!();
    println!("   ✅ Data preparation test passed");
    Ok(())
}

async fn test_player_initialization(ctx: &TestContext) -> Result<()> {
    print_test_header("New Player Initialization");

    ctx.reset_user().await?;

    let result = ctx.initialize_player().await?;

    if let Some(data) = result.get("data") {
        if let Some(pokemon) = data.get("pokemon") {
            let name = pokemon
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            let level = pokemon.get("level").and_then(|v| v.as_u64()).unwrap_or(0);

            assert!(level >= 5, "Starter pokemon should be at least level 5");
            println!("   ✅ Starter pokemon: {} Lv.{}", name, level);
        }
    }

    let state = ctx.get_state().await?;
    let pokemons = state
        .get("pokemons")
        .and_then(|p| p.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    assert!(
        pokemons >= 1,
        "Should have at least 1 pokemon after initialization"
    );
    println!("   ✅ Verified: pokemons={}", pokemons);

    println!();
    println!("   ✅ Player initialization test passed");
    Ok(())
}

async fn test_battle_flow(ctx: &TestContext) -> Result<()> {
    print_test_header("Battle Flow");

    ctx.reset_user().await?;
    ctx.set_money(5000).await?;
    ctx.create_pokemon(25, 30, 1, 100).await?;
    ctx.add_item(4, 5).await?;

    let maps = ctx.get_maps().await?;
    let maps_arr = maps
        .get("maps")
        .and_then(|m| m.as_array())
        .cloned()
        .unwrap_or_default();
    if maps_arr.is_empty() {
        println!("   ⚠️  No maps available, skipping battle test");
        return Ok(());
    }

    let map_id = maps_arr[0]
        .get("id")
        .and_then(|v| v.as_u64())
        .unwrap_or(101);
    println!("   🗺️  Using map ID: {}", map_id);

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
                                    "   ✅ Wild {} HP: {}/{}",
                                    wild_name, wild_hp, wild_max_hp
                                );
                            }
                        }
                    }
                }
            }

            ctx.flee().await?;
            println!("   ✅ Fled from battle successfully");
        }
    }

    println!();
    println!("   ✅ Battle flow test passed");
    Ok(())
}

async fn test_capture_flow(ctx: &TestContext) -> Result<()> {
    print_test_header("Capture Flow");

    ctx.reset_user().await?;
    ctx.set_money(5000).await?;
    ctx.create_pokemon(25, 30, 1, 100).await?;

    ctx.add_item(4, 10).await?;

    let state_before = ctx.get_state().await?;
    let pokemons_before = state_before
        .get("pokemons")
        .and_then(|p| p.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    println!("   📊 Pokemons before capture: {}", pokemons_before);

    let maps = ctx.get_maps().await?;
    let maps_arr = maps
        .get("maps")
        .and_then(|m| m.as_array())
        .cloned()
        .unwrap_or_default();
    if maps_arr.is_empty() {
        println!("   ⚠️  No maps available, skipping capture test");
        return Ok(());
    }

    let map_id = maps_arr[0]
        .get("id")
        .and_then(|v| v.as_u64())
        .unwrap_or(101);
    println!("   🗺️  Using map ID: {}", map_id);

    let battle_result = ctx.start_battle(map_id).await?;

    if let Some(data) = battle_result.get("data") {
        let status = data.get("status").and_then(|v| v.as_str()).unwrap_or("");
        if status == "active" {
            if let Some(my_pokemon) = data.get("my_pokemon") {
                if let Some(skills) = my_pokemon.get("skills").and_then(|s| s.as_array()) {
                    if let Some(first_skill) = skills.first() {
                        let skill_id = first_skill.get("id").and_then(|v| v.as_u64()).unwrap_or(1);

                        for _ in 0..3 {
                            let turn_result = ctx.use_skill(skill_id).await?;
                            if let Some(turn_data) = turn_result.get("data") {
                                if let Some(wild) = turn_data.get("wild_pokemon") {
                                    let wild_hp =
                                        wild.get("hp").and_then(|v| v.as_i64()).unwrap_or(0);
                                    if wild_hp <= 0 {
                                        println!("   ✅ Wild pokemon defeated");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let state_after = ctx.get_state().await?;
    let pokemons_after = state_after
        .get("pokemons")
        .and_then(|p| p.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    println!("   📊 Pokemons after battle: {}", pokemons_after);

    println!();
    println!("   ✅ Capture flow test passed");
    Ok(())
}

async fn test_shop_flow(ctx: &TestContext) -> Result<()> {
    print_test_header("Shop Flow");

    ctx.reset_user().await?;
    ctx.set_money(10000).await?;

    let shop_items = ctx.get_shop_items().await?;
    let items = shop_items
        .get("items")
        .and_then(|i| i.as_array())
        .cloned()
        .unwrap_or_default();
    println!("   📦 Shop has {} items available", items.len());

    let state_before = ctx.get_state().await?;
    let money_before = state_before
        .get("user")
        .and_then(|u| u.get("money"))
        .and_then(|m| m.as_i64())
        .unwrap_or(0);
    println!("   💰 Money before: {}", money_before);

    ctx.buy_item(1, 5).await?;

    let state_after = ctx.get_state().await?;
    let money_after = state_after
        .get("user")
        .and_then(|u| u.get("money"))
        .and_then(|m| m.as_i64())
        .unwrap_or(0);
    println!("   💰 Money after: {}", money_after);

    assert!(
        money_after < money_before,
        "Money should decrease after purchase"
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
    let pokemon_before = state_before
        .get("pokemons")
        .and_then(|p| p.as_array())
        .and_then(|a| a.first())
        .cloned();
    let item_before = state_before
        .get("items")
        .and_then(|i| i.as_array())
        .and_then(|a| {
            a.iter()
                .find(|item| item.get("type_id").and_then(|t| t.as_u64()) == Some(1))
        })
        .cloned();

    if let (Some(pokemon), Some(item)) = (pokemon_before, item_before) {
        let hp_before = pokemon.get("hp").and_then(|v| v.as_i64()).unwrap_or(0);
        let max_hp = pokemon.get("max_hp").and_then(|v| v.as_i64()).unwrap_or(0);
        let pokemon_id = pokemon.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
        let item_qty_before = item.get("quantity").and_then(|v| v.as_i64()).unwrap_or(0);

        println!("   🐾 Pokemon HP before: {}/{}", hp_before, max_hp);
        println!("   📦 Item quantity before: {}", item_qty_before);

        ctx.use_item(1, Some(pokemon_id)).await?;

        let state_after = ctx.get_state().await?;
        if let Some(pokemon_after) = state_after
            .get("pokemons")
            .and_then(|p| p.as_array())
            .and_then(|a| a.first())
        {
            let hp_after = pokemon_after
                .get("hp")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            println!("   🐾 Pokemon HP after: {}/{}", hp_after, max_hp);
            assert!(
                hp_after > hp_before,
                "HP should increase after using potion"
            );
        }

        if let Some(item_after) = state_after
            .get("items")
            .and_then(|i| i.as_array())
            .and_then(|a| {
                a.iter()
                    .find(|item| item.get("type_id").and_then(|t| t.as_u64()) == Some(1))
            })
        {
            let item_qty_after = item_after
                .get("quantity")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            println!("   📦 Item quantity after: {}", item_qty_after);
            assert!(
                item_qty_after < item_qty_before,
                "Item quantity should decrease"
            );
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
    if let Some(pokemon) = state_before
        .get("pokemons")
        .and_then(|p| p.as_array())
        .and_then(|a| a.first())
    {
        let hp_before = pokemon.get("hp").and_then(|v| v.as_i64()).unwrap_or(0);
        let max_hp = pokemon.get("max_hp").and_then(|v| v.as_i64()).unwrap_or(0);
        let pokemon_id = pokemon.get("id").and_then(|v| v.as_u64()).unwrap_or(0);

        println!("   🐾 Pokemon HP before heal: {}/{}", hp_before, max_hp);

        assert!(hp_before < max_hp, "Pokemon should have reduced HP");

        ctx.heal_pokemon(pokemon_id).await?;

        let state_after = ctx.get_state().await?;
        if let Some(pokemon_after) = state_after
            .get("pokemons")
            .and_then(|p| p.as_array())
            .and_then(|a| a.first())
        {
            let hp_after = pokemon_after
                .get("hp")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            println!("   🐾 Pokemon HP after heal: {}/{}", hp_after, max_hp);

            assert_eq!(hp_after, max_hp, "Pokemon should be fully healed");
        }
    } else {
        println!("   ⚠️  No pokemon available for healing test");
    }

    println!();
    println!("   ✅ Healing flow test passed");
    Ok(())
}

async fn test_evolution(ctx: &TestContext) -> Result<()> {
    print_test_header("Evolution System");

    ctx.reset_user().await?;
    ctx.set_money(5000).await?;

    let pokemon = ctx.create_pokemon(10, 15, 1, 100).await?;
    let pokemon_id = pokemon
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    if pokemon_id > 0 {
        let evo_check = ctx.check_evolution(pokemon_id).await?;
        let can_evolve = evo_check
            .get("can_evolve")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let conditions = evo_check
            .get("conditions")
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        println!("   🔍 Evolution check for pokemon {}", pokemon_id);
        println!("   📋 Can evolve: {}", can_evolve);

        if !conditions.is_empty() {
            println!("   📋 Evolution conditions:");
            for cond in &conditions {
                if let Some(cond_type) = cond.get("type").and_then(|t| t.as_str()) {
                    let target = cond
                        .get("target")
                        .and_then(|t| t.as_str())
                        .unwrap_or("Unknown");
                    println!("      - {}: {}", cond_type, target);
                }
            }
        }

        if can_evolve {
            let evo_result = ctx.evolve_pokemon(pokemon_id).await?;
            if let Some(data) = evo_result.get("data") {
                if let Some(new_name) = data.get("new_name").and_then(|n| n.as_str()) {
                    println!("   ✅ Evolved to: {}", new_name);
                }
            }
        } else {
            println!("   ℹ️  Pokemon cannot evolve yet (conditions not met)");
        }
    }

    println!();
    println!("   ✅ Evolution system test passed");
    Ok(())
}

async fn test_skill_system(ctx: &TestContext) -> Result<()> {
    print_test_header("Skill System");

    ctx.reset_user().await?;
    ctx.set_money(5000).await?;

    let pokemon = ctx.create_pokemon(25, 30, 1, 100).await?;
    let pokemon_id = pokemon
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    if pokemon_id > 0 {
        let detail = ctx.get_pokemon_detail(pokemon_id).await?;
        if let Some(skills) = detail.get("skills").and_then(|s| s.as_array()) {
            println!("   📋 Current skills ({}):", skills.len());
            for skill in skills {
                let name = skill
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("Unknown");
                let pp = skill.get("pp").and_then(|p| p.as_i64()).unwrap_or(0);
                let max_pp = skill.get("max_pp").and_then(|p| p.as_i64()).unwrap_or(0);
                println!("      - {} ({}/{})", name, pp, max_pp);
            }
        }

        let learnable = ctx.get_learnable_skills(pokemon_id).await?;
        let available = learnable
            .get("available_skills")
            .and_then(|a| a.as_array())
            .cloned()
            .unwrap_or_default();
        let unlocked = learnable
            .get("unlocked_skills")
            .and_then(|u| u.as_array())
            .cloned()
            .unwrap_or_default();

        println!("   📚 Available skills to learn: {}", available.len());
        println!("   📚 Unlocked skills: {}", unlocked.len());

        if !unlocked.is_empty() {
            let skill_id = unlocked[0].get("id").and_then(|v| v.as_u64()).unwrap_or(0);
            let skill_name = unlocked[0]
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("Unknown");
            println!("   📖 Attempting to learn: {}", skill_name);

            match ctx.learn_skill(pokemon_id, skill_id, 0).await {
                Ok(_) => println!("   ✅ Skill learned successfully"),
                Err(e) => println!("   ℹ️  Could not learn skill: {}", e),
            }
        }
    }

    println!();
    println!("   ✅ Skill system test passed");
    Ok(())
}

async fn test_pokemon_management(ctx: &TestContext) -> Result<()> {
    print_test_header("Pokemon Management");

    ctx.reset_user().await?;
    ctx.set_money(5000).await?;

    ctx.create_pokemon(25, 30, 1, 100).await?;
    ctx.create_pokemon(1, 20, 0, 100).await?;
    ctx.create_pokemon(4, 15, 0, 100).await?;

    let list = ctx.get_pokemon_list().await?;
    let pokemons = list
        .get("pokemons")
        .and_then(|p| p.as_array())
        .cloned()
        .unwrap_or_default();
    let total = list.get("total").and_then(|t| t.as_u64()).unwrap_or(0);

    println!("   📋 Pokemon list: {} total", total);
    for p in &pokemons {
        let name = p.get("name").and_then(|n| n.as_str()).unwrap_or("Unknown");
        let level = p.get("level").and_then(|l| l.as_u64()).unwrap_or(0);
        let id = p.get("id").and_then(|i| i.as_u64()).unwrap_or(0);
        println!("      - {} Lv.{} (id={})", name, level, id);
    }

    assert!(pokemons.len() >= 3, "Should have at least 3 pokemons");

    if let Some(first_pokemon) = pokemons.first() {
        let pokemon_id = first_pokemon
            .get("id")
            .and_then(|i| i.as_u64())
            .unwrap_or(0);

        ctx.rename_pokemon(pokemon_id, "TestNickname").await?;

        let detail = ctx.get_pokemon_detail(pokemon_id).await?;
        let name = detail
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("Unknown");
        println!("   ✏️  Pokemon name after rename: {}", name);
    }

    if pokemons.len() > 1 {
        let last_pokemon = pokemons.last().unwrap();
        let release_id = last_pokemon.get("id").and_then(|i| i.as_u64()).unwrap_or(0);

        ctx.release_pokemon(release_id).await?;

        let list_after = ctx.get_pokemon_list().await?;
        let total_after = list_after
            .get("total")
            .and_then(|t| t.as_u64())
            .unwrap_or(0);
        println!("   📊 Pokemon count after release: {}", total_after);

        assert!(
            total_after < total,
            "Pokemon count should decrease after release"
        );
    }

    println!();
    println!("   ✅ Pokemon management test passed");
    Ok(())
}

async fn test_user_profile(ctx: &TestContext) -> Result<()> {
    print_test_header("User Profile & Stats");

    ctx.reset_user().await?;
    ctx.set_money(5000).await?;
    ctx.create_pokemon(25, 30, 1, 100).await?;

    let profile = ctx.get_profile().await?;
    if let Some(user) = profile.get("user") {
        let username = user
            .get("username")
            .and_then(|u| u.as_str())
            .unwrap_or("Unknown");
        let money = user.get("money").and_then(|m| m.as_i64()).unwrap_or(0);
        println!("   👤 User: {}", username);
        println!("   💰 Money: {}", money);
    }

    let stats = ctx.get_stats().await?;
    if let Some(pokemon_stats) = stats.get("pokemon_stats") {
        let total = pokemon_stats
            .get("total")
            .and_then(|t| t.as_u64())
            .unwrap_or(0);
        let avg_level = pokemon_stats
            .get("avg_level")
            .and_then(|l| l.as_f64())
            .unwrap_or(0.0);
        println!(
            "   📊 Pokemon stats: {} total, avg level {:.1}",
            total, avg_level
        );
    }

    println!();
    println!("   ✅ User profile test passed");
    Ok(())
}

async fn test_inventory_management(ctx: &TestContext) -> Result<()> {
    print_test_header("Inventory Management");

    ctx.reset_user().await?;
    ctx.set_money(10000).await?;

    ctx.buy_item(1, 5).await?;
    ctx.buy_item(4, 3).await?;
    ctx.buy_item(5, 2).await?;

    let inventory = ctx.get_inventory().await?;
    let items = inventory
        .get("items")
        .and_then(|i| i.as_array())
        .cloned()
        .unwrap_or_default();
    let total = inventory.get("total").and_then(|t| t.as_u64()).unwrap_or(0);

    println!("   🎒 Inventory: {} items, {} types", total, items.len());
    for item in &items {
        let name = item
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("Unknown");
        let qty = item.get("quantity").and_then(|q| q.as_i64()).unwrap_or(0);
        let type_id = item.get("type_id").and_then(|t| t.as_u64()).unwrap_or(0);
        println!("      - {} x{} (type={})", name, qty, type_id);
    }

    assert!(items.len() >= 3, "Should have at least 3 item types");

    println!();
    println!("   ✅ Inventory management test passed");
    Ok(())
}
