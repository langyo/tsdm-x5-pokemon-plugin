#[cfg(test)]
mod integration_tests {
    use serde_json::Value;

    const API: &str = "https://localhost:8443/plugin.php?id=pokemon:game&index=admin";

    async fn admin_api(action: &str, extra: &[(&str, &str)]) -> Result<Value, String> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| e.to_string())?;

        let mut body = serde_json::json!({ "action": action });
        if let Some(obj) = body.as_object_mut() {
            for (k, v) in extra {
                obj.insert(k.to_string(), serde_json::Value::String(v.to_string()));
            }
        }

        let resp = client
            .post(API)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let text = resp.text().await.map_err(|e| e.to_string())?;

        serde_json::from_str(&text)
            .map_err(|e| format!("JSON parse: {} — raw: {}", e, &text[..200.min(text.len())]))
    }

    async fn check(action: &str, extra: &[(&str, &str)]) -> bool {
        match admin_api(action, extra).await {
            Ok(v) => v.get("success").and_then(|s| s.as_bool()).unwrap_or(false),
            Err(e) => {
                eprintln!("  ERR: {e}");
                false
            }
        }
    }

    #[ignore = "requires local dev stack (just up) on localhost:8443"]
    #[tokio::test]
    async fn test_admin_crud_endpoints() {
        let tests = vec![
            "count::pokemon_type",
            "list::pokemon_type",
            "count::item_type",
            "list::item_type",
            "count::map_info",
            "list::map_info",
            "count::evolution_info",
            "list::evolution_info",
            "count::skill_type",
            "list::skill_type",
            "list::global_config",
            "count::user_info",
        ];

        for action in &tests {
            let ok = check(action, &[("from", "0"), ("count", "2")]).await;
            println!("  [{}] {}", if ok { "OK" } else { "FAIL" }, action);
            assert!(ok, "Failed: {action}");
        }
    }

    #[ignore = "requires local dev stack (just up) on localhost:8443"]
    #[tokio::test]
    async fn test_pokemon_info_endpoint() {
        let ok = check(
            "list::pokemon_info",
            &[("uid", "1"), ("from", "0"), ("count", "5")],
        )
        .await;
        assert!(ok, "pokemon_info failed");
    }

    #[ignore = "requires local dev stack (just up) on localhost:8443"]
    #[tokio::test]
    async fn test_item_info_endpoint() {
        let ok = check(
            "list::item_info",
            &[("uid", "1"), ("from", "0"), ("count", "5")],
        )
        .await;
        assert!(ok, "item_info failed");
    }
}
