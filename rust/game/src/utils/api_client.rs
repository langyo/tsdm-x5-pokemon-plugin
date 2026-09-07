//! API 客户端
//!
//! 使用 pokemon_system/api/*.php 端点
//! URL格式: plugin.php?id=pokemon:pokemon&endpoint=xxx&action=yyy

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use gloo_net::http::Request;

// 导入API类型
use _utils::types::api_battle::{
    BattleScene, FleeRequest, StartBattleRequest, UseItemOnSkillRequest, UseSkillRequest,
};
use _utils::types::api_config::{GlobalConfigData, GlobalConfigResponse};
use _utils::types::api_evolution::{
    EvolutionCheckResponse, EvolutionPathResponse, EvolutionRequest, EvolutionResponse,
};
use _utils::types::api_map::MapsResponse;
use _utils::types::api_pokemon::{
    ApiResponse as PokemonApiResponse, EquipItemRequest, EquipItemResponse, EquipmentResponse,
    LearnSkillRequest, LearnSkillResponse, LearnableSkillsResponse, PokemonDetail, ReleaseRequest,
    RenameRequest, UnequipItemRequest, UnequipItemResponse, UpdateStateResponse,
};
use _utils::types::api_shop::{
    BuyItemRequest, BuyItemResponse, BuyPetRequest, BuyPetResponse, ShopListResponse,
    ShopPetListResponse,
};
use _utils::types::api_topics::TopicsResponse;
use _utils::types::api_user::{
    BadgeStatusResponse, InitializePlayerResponse, InventoryResponse, InventoryStatsResponse,
    OnlinePlayersResponse, UsablePokemonResponse, UseItemRequest, UseItemResponse,
    UserProfileResponse, UserStatsResponse,
};

/// API客户端
#[derive(Clone)]
pub struct NewApiClient {
    base_url: String,
}

impl NewApiClient {
    pub fn new() -> Self {
        Self {
            base_url: "/plugin.php?id=pokemon:pokemon".to_string(),
        }
    }

    // ============== Pokemon API ==============

    /// 获取宠物列表
    pub async fn get_pokemon_list(&self) -> Result<PokemonListResponse> {
        let url = format!("{}&endpoint=pokemon&action=list", self.base_url);
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<PokemonListResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 获取宠物详情
    pub async fn get_pokemon_detail(&self, id: u64) -> Result<PokemonDetail> {
        let url = format!(
            "{}&endpoint=pokemon&action=detail&pokemon_id={}",
            self.base_url, id
        );
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<PokemonDetail> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 重命名宠物
    pub async fn rename_pokemon(&self, id: u64, name: &str) -> Result<()> {
        let url = format!("{}&endpoint=pokemon&action=rename", self.base_url);
        let body = RenameRequest {
            id,
            name: name.to_string(),
        };

        let response = Request::post(&url)
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        // 尝试解析响应
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        // 检查 success 字段
        let success = result
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if success {
            Ok(())
        } else {
            let error = result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            Err(anyhow!("API error: {}", error))
        }
    }

    /// 放生宠物
    pub async fn release_pokemon(&self, id: u64) -> Result<()> {
        let url = format!("{}&endpoint=pokemon&action=release", self.base_url);
        let body = ReleaseRequest { id };

        let response = Request::post(&url)
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<()> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            Ok(())
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 列入首位 (设置为首宠)
    pub async fn set_first_pokemon(&self, pokemon_id: u64) -> Result<()> {
        let url = format!(
            "{}&endpoint=pokemon&action=set_first&pokemon_id={}",
            self.base_url, pokemon_id
        );

        let response = Request::post(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            return Err(anyhow!("HTTP error {}: {}", response.status(), error_text));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        let result: PokemonApiResponse<serde_json::Value> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            Ok(())
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 获取宠物可学习的技能列表
    pub async fn get_learnable_skills(&self, pokemon_id: u64) -> Result<LearnableSkillsResponse> {
        let url = format!(
            "{}&endpoint=pokemon&action=learnable_skills&pokemon_id={}",
            self.base_url, pokemon_id
        );
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<LearnableSkillsResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 遗忘宠物技能
    pub async fn forget_skill(&self, pokemon_id: u64, skill_id: u64) -> Result<()> {
        let url = format!("{}&endpoint=pokemon&action=forget_skill", self.base_url);
        let body = serde_json::json!({
            "pokemon_id": pokemon_id,
            "skill_id": skill_id
        });

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .map_err(|e| anyhow!("Body error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<()> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            Ok(())
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 学习新技能
    pub async fn learn_skill(
        &self,
        pokemon_id: u64,
        skill_id: u64,
        slot_index: Option<i32>,
    ) -> Result<LearnSkillResponse> {
        let url = format!("{}&endpoint=pokemon&action=learn_skill", self.base_url);
        let body = LearnSkillRequest {
            pokemon_id,
            skill_id,
            slot_index,
        };

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<LearnSkillResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 获取宝可梦装备信息
    pub async fn get_equipment(&self, pokemon_id: u64) -> Result<EquipmentResponse> {
        let url = format!(
            "{}&endpoint=pokemon&action=equipment&pokemon_id={}",
            self.base_url, pokemon_id
        );
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<EquipmentResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 装备物品
    pub async fn equip_item(
        &self,
        pokemon_id: u64,
        myitem_id: u64,
        slot_index: Option<i32>,
    ) -> Result<EquipItemResponse> {
        let url = format!("{}&endpoint=pokemon&action=equip_item", self.base_url);
        let body = EquipItemRequest {
            pokemon_id,
            myitem_id,
            slot_index,
        };

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<EquipItemResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 卸下装备
    pub async fn unequip_item(
        &self,
        pokemon_id: u64,
        slot_index: u32,
    ) -> Result<UnequipItemResponse> {
        let url = format!("{}&endpoint=pokemon&action=unequip_item", self.base_url);
        let body = UnequipItemRequest {
            pokemon_id,
            slot_index,
        };

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<UnequipItemResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 更新宠物状态（随机触发）
    pub async fn update_pokemon_state(&self) -> Result<UpdateStateResponse> {
        let url = format!("{}&endpoint=pokemon&action=update_state", self.base_url);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<UpdateStateResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    // ============== Battle API ==============

    /// 开始战斗
    pub async fn start_battle(
        &self,
        map_id: u64,
        boss_pokemon_type_id: Option<u64>,
    ) -> Result<BattleScene> {
        let url = format!("{}&endpoint=battle&action=start", self.base_url);
        let body = serde_json::to_string(&StartBattleRequest {
            map_id,
            boss_pokemon_type_id,
        })
        .map_err(|e| anyhow!("Serialize error: {}", e))?;

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .map_err(|e| anyhow!("Body error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());

            // 尝试解析 JSON 错误响应，提取 error 字段
            if let Ok(err_resp) =
                serde_json::from_str::<PokemonApiResponse<serde_json::Value>>(&error_text)
            {
                if let Some(error_msg) = err_resp.error {
                    return Err(anyhow!("{}", error_msg));
                }
            }

            // 如果无法解析或没有 error 字段，返回原始错误
            return Err(anyhow!("HTTP error {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        if response_text.trim().is_empty() {
            return Err(anyhow!("Empty response from server"));
        }

        let result: PokemonApiResponse<BattleScene> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response was: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 使用技能
    pub async fn use_skill(&self, battle_id: &str, skill_id: u64) -> Result<BattleScene> {
        let url = format!("{}&endpoint=battle&action=turn", self.base_url);
        let body = serde_json::to_string(&UseSkillRequest {
            battle_id: Some(battle_id.to_string()),
            skill_id,
        })
        .map_err(|e| anyhow!("Serialize error: {}", e))?;

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .map_err(|e| anyhow!("Body error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());

            // 尝试解析 JSON 错误响应，提取 error 字段
            if let Ok(err_resp) =
                serde_json::from_str::<PokemonApiResponse<serde_json::Value>>(&error_text)
            {
                if let Some(error_msg) = err_resp.error {
                    return Err(anyhow!("{}", error_msg));
                }
            }

            return Err(anyhow!("HTTP error {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        if response_text.trim().is_empty() {
            return Err(anyhow!("Empty response from server"));
        }

        let result: PokemonApiResponse<BattleScene> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response was: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 逃跑
    pub async fn flee(&self, battle_id: &str) -> Result<BattleScene> {
        let url = format!("{}&endpoint=battle&action=flee", self.base_url);
        let body = serde_json::to_string(&FleeRequest {
            battle_id: Some(battle_id.to_string()),
        })
        .map_err(|e| anyhow!("Serialize error: {}", e))?;

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .map_err(|e| anyhow!("Body error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());

            // 尝试解析 JSON 错误响应，提取 error 字段
            if let Ok(err_resp) =
                serde_json::from_str::<PokemonApiResponse<serde_json::Value>>(&error_text)
            {
                if let Some(error_msg) = err_resp.error {
                    return Err(anyhow!("{}", error_msg));
                }
            }

            return Err(anyhow!("HTTP error {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        if response_text.trim().is_empty() {
            return Err(anyhow!("Empty response from server"));
        }

        let result: PokemonApiResponse<BattleScene> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response was: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 切换上场宠物
    pub async fn switch_pokemon(&self, battle_id: &str) -> Result<BattleScene> {
        self.switch_pokemon_with_id(battle_id, None).await
    }

    /// 在战斗中切换宠物（指定宠物 ID）
    pub async fn switch_pokemon_with_id(
        &self,
        battle_id: &str,
        pokemon_id: Option<u64>,
    ) -> Result<BattleScene> {
        let url = format!("{}&endpoint=battle&action=switch_pokemon", self.base_url);
        let mut body_json = serde_json::json!({
            "battle_id": battle_id,
        });
        // 如果提供了 pokemon_id，添加到请求体中
        if let Some(pid) = pokemon_id {
            body_json["pokemon_id"] = serde_json::json!(pid);
        }

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&body_json)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());

            if let Ok(err_resp) =
                serde_json::from_str::<PokemonApiResponse<serde_json::Value>>(&error_text)
            {
                if let Some(error_msg) = err_resp.error {
                    return Err(anyhow!("{}", error_msg));
                }
            }

            return Err(anyhow!("HTTP error {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        if response_text.trim().is_empty() {
            return Err(anyhow!("Empty response from server"));
        }

        let result: PokemonApiResponse<BattleScene> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response was: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 被动替换宠物（宠物被打死后替换，不反击）
    pub async fn replace_pokemon(&self, _battle_id: &str, pokemon_id: u64) -> Result<BattleScene> {
        let url = format!("{}&endpoint=battle&action=replace_pokemon", self.base_url);
        let body_json = serde_json::json!({
            "pokemon_id": pokemon_id,
        });

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&body_json)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());

            if let Ok(err_resp) =
                serde_json::from_str::<PokemonApiResponse<serde_json::Value>>(&error_text)
            {
                if let Some(error_msg) = err_resp.error {
                    return Err(anyhow!("{}", error_msg));
                }
            }

            return Err(anyhow!("HTTP error {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        if response_text.trim().is_empty() {
            return Err(anyhow!("Empty response from server"));
        }

        let result: PokemonApiResponse<BattleScene> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response was: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 捕捉
    pub async fn capture(&self, ball_id: u64) -> Result<BattleScene> {
        let url = format!("{}&endpoint=battle&action=capture", self.base_url);
        let body = serde_json::json!({
            "ball_id": ball_id,
        });

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());

            // 尝试解析 JSON 错误响应，提取 error 字段
            if let Ok(err_resp) =
                serde_json::from_str::<PokemonApiResponse<serde_json::Value>>(&error_text)
            {
                if let Some(error_msg) = err_resp.error {
                    return Err(anyhow!("{}", error_msg));
                }
            }

            return Err(anyhow!("HTTP error {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        if response_text.trim().is_empty() {
            return Err(anyhow!("Empty response from server"));
        }

        let result: PokemonApiResponse<BattleScene> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response was: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 在战斗中使用物品（可能返回技能选择响应）
    ///
    /// 返回原始响应文本：PP 恢复类道具需要先让用户选择技能（requires_skill_selection），
    /// 其余物品直接返回战斗场景。调用方只需发起这一次请求——服务端在该请求中
    /// 已经完成扣道具、加血与野怪反击，再次调用会重复扣道具。
    pub async fn use_item_in_battle_raw(&self, item_id: u64) -> Result<String> {
        let url = format!("{}&endpoint=battle&action=use_item", self.base_url);
        let body = serde_json::json!({
            "item_id": item_id,
        });

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());

            // 解析 JSON 错误响应，提取 error 字段（如"您没有该物品"）
            if let Ok(err_resp) =
                serde_json::from_str::<PokemonApiResponse<serde_json::Value>>(&error_text)
            {
                if let Some(error_msg) = err_resp.error {
                    return Err(anyhow!("{}", error_msg));
                }
            }

            return Err(anyhow!("HTTP error {}", status));
        }

        response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))
    }

    /// 在战斗中对指定技能使用PP恢复道具
    pub async fn use_item_on_skill(&self, item_id: u64, skill_id: u64) -> Result<BattleScene> {
        let url = format!("{}&endpoint=battle&action=use_item_on_skill", self.base_url);
        let body = UseItemOnSkillRequest { item_id, skill_id };

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());

            // 尝试解析 JSON 错误响应，提取 error 字段
            if let Ok(err_resp) =
                serde_json::from_str::<PokemonApiResponse<serde_json::Value>>(&error_text)
            {
                if let Some(error_msg) = err_resp.error {
                    return Err(anyhow!("{}", error_msg));
                }
            }

            return Err(anyhow!("HTTP error {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        if response_text.trim().is_empty() {
            return Err(anyhow!("Empty response from server"));
        }

        let result: PokemonApiResponse<BattleScene> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response was: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    // ============== Map API ==============

    /// 获取地图列表
    pub async fn get_maps(
        &self,
        min_level: Option<u32>,
        max_level: Option<u32>,
    ) -> Result<MapsResponse> {
        let mut params = String::new();
        if let Some(min) = min_level {
            params.push_str(&format!("&min_level={}", min));
        }
        if let Some(max) = max_level {
            params.push_str(&format!("&max_level={}", max));
        }
        let url = format!("{}&endpoint=battle&action=maps{}", self.base_url, params);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        let result: _utils::types::api_pokemon::ApiResponse<MapsResponse> =
            serde_json::from_str(&response_text).map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response was: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 恢复战斗状态
    pub async fn recover_battle(&self) -> Result<BattleScene> {
        let url = format!("{}&endpoint=battle&action=recover", self.base_url);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        let result: _utils::types::api_pokemon::ApiResponse<BattleScene> =
            serde_json::from_str(&response_text).map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response was: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    // ============== Shop API ==============

    /// 获取商店物品
    pub async fn get_shop_items(
        &self,
        item_type: Option<u32>,
        page: u32,
    ) -> Result<ShopListResponse> {
        let type_param = item_type
            .map(|t| format!("&type={}", t))
            .unwrap_or_default();
        let url = format!(
            "{}&endpoint=shop&action=list&page={}{}",
            self.base_url, page, type_param
        );

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<ShopListResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 购买物品
    pub async fn buy_item(&self, item_id: u64, quantity: u32) -> Result<BuyItemResponse> {
        let url = format!("{}&endpoint=shop&action=buy", self.base_url);
        let body = BuyItemRequest {
            item_id,
            quantity: quantity as u64,
        };

        let response = Request::post(&url)
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // 尝试解析 JSON 错误响应，提取 error 字段
            if let Ok(err_resp) =
                serde_json::from_str::<PokemonApiResponse<serde_json::Value>>(&error_text)
            {
                if let Some(error_msg) = err_resp.error {
                    return Err(anyhow!("{}", error_msg));
                }
            }

            return Err(anyhow!("HTTP error {}", response.status()));
        }

        let result: PokemonApiResponse<BuyItemResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    pub async fn get_shop_pets(&self, page: u32) -> Result<ShopPetListResponse> {
        let url = format!("{}&endpoint=shop&action=pets&page={}", self.base_url, page);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<ShopPetListResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    pub async fn buy_pet(&self, pokemon_type_id: u64) -> Result<BuyPetResponse> {
        let url = format!("{}&endpoint=shop&action=buy_pet", self.base_url);
        let body = BuyPetRequest { pokemon_type_id };

        let response = Request::post(&url)
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            if let Ok(err_resp) =
                serde_json::from_str::<PokemonApiResponse<serde_json::Value>>(&error_text)
            {
                if let Some(error_msg) = err_resp.error {
                    return Err(anyhow!("{}", error_msg));
                }
            }

            return Err(anyhow!("HTTP error {}", response.status()));
        }

        let result: PokemonApiResponse<BuyPetResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    // ============== Evolution API ==============

    /// 检查进化
    pub async fn check_evolution(&self, pet_id: u64) -> Result<EvolutionCheckResponse> {
        let url = format!(
            "{}&endpoint=evolution&action=check&pet_id={}",
            self.base_url, pet_id
        );

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<EvolutionCheckResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 执行进化
    pub async fn evolve(&self, pet_id: u64) -> Result<EvolutionResponse> {
        let url = format!("{}&endpoint=evolution&action=evolve", self.base_url);
        let body = EvolutionRequest { pet_id };

        let response = Request::post(&url)
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<EvolutionResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    pub async fn get_evolution_path(&self, pmid: u64) -> Result<EvolutionPathResponse> {
        let url = format!(
            "{}&endpoint=evolution&action=evolution_path&pmid={}",
            self.base_url, pmid
        );

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<EvolutionPathResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    // ============== User API ==============

    /// 获取用户资料
    pub async fn get_user_profile(&self) -> Result<UserProfileResponse> {
        let url = format!("{}&endpoint=user&action=profile", self.base_url);

        let response_result = Request::get(&url).send().await;

        let response = response_result.map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        // 先获取响应文本以便调试
        let response_text_result = response.text().await;

        let response_text =
            response_text_result.map_err(|e| anyhow!("Failed to get response text: {}", e))?;

        // 尝试解析 JSON
        let result: PokemonApiResponse<UserProfileResponse> =
            serde_json::from_str(&response_text).map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            if let Some(ref _data) = result.data {}
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            let error_msg = result.error.as_deref().unwrap_or("Unknown error");
            Err(anyhow!("API error: {}", error_msg))
        }
    }

    /// 获取用户游戏统计
    pub async fn get_user_stats(&self) -> Result<UserStatsResponse> {
        let url = format!("{}&endpoint=user&action=stats", self.base_url);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<UserStatsResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 获取用户背包
    pub async fn get_user_inventory(
        &self,
        item_type: Option<u32>,
        page: u32,
    ) -> Result<InventoryResponse> {
        let type_param = item_type
            .map(|t| format!("&type={}", t))
            .unwrap_or_default();
        let url = format!(
            "{}&endpoint=user&action=inventory&page={}{}",
            self.base_url, page, type_param
        );

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "HTTP error: {} - {}",
                response.status(),
                error_text
            ));
        }

        let response_text = response.text().await.unwrap_or_else(|_| "".to_string());

        let result: PokemonApiResponse<InventoryResponse> = serde_json::from_str(&response_text)
            .map_err(|e| {
                let error_preview: String = response_text.chars().take(200).collect();
                anyhow!("Parse error: {} - response: {}", e, error_preview)
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 获取可以在战斗中使用的物品（只包含HP恢复和PP恢复道具）
    pub async fn get_battle_items(&self) -> Result<InventoryResponse> {
        let url = format!("{}&endpoint=battle&action=get_battle_items", self.base_url);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<InventoryResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 获取背包物品类型统计
    pub async fn get_inventory_stats(&self) -> Result<InventoryStatsResponse> {
        let url = format!("{}&endpoint=user&action=inventory_stats", self.base_url);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "HTTP error: {} - {}",
                response.status(),
                error_text
            ));
        }

        let response_text = response.text().await.unwrap_or_else(|_| "".to_string());

        let result: PokemonApiResponse<InventoryStatsResponse> =
            serde_json::from_str(&response_text).map_err(|e| {
                let error_preview: String = response_text.chars().take(200).collect();
                anyhow!("Parse error: {} - response: {}", e, error_preview)
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 治疗宝可梦
    pub async fn heal_pokemon(&self, pokemon_id: u64) -> Result<HealResponse> {
        let url = format!(
            "{}&endpoint=user&action=heal&pokemon_id={}",
            self.base_url, pokemon_id
        );

        let response = Request::post(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            return Err(anyhow!("HTTP error {}: {}", response.status(), error_text));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        let result: PokemonApiResponse<HealResponse> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 脱战并治疗宝可梦（宠物中心绿色通道）
    pub async fn heal_and_flee(&self, pokemon_id: u64) -> Result<HealResponse> {
        let url = format!(
            "{}&endpoint=user&action=heal_and_flee&pokemon_id={}",
            self.base_url, pokemon_id
        );

        let response = Request::post(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            return Err(anyhow!("HTTP error {}: {}", response.status(), error_text));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        let result: PokemonApiResponse<HealResponse> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 获取在线玩家
    pub async fn get_online_players(&self) -> Result<OnlinePlayersResponse> {
        let url = format!("{}&endpoint=user&action=online_players", self.base_url);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<OnlinePlayersResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 获取话题列表
    pub async fn get_topics(&self, limit: Option<u32>) -> Result<TopicsResponse> {
        let limit_param = limit.unwrap_or(6);
        let url = format!(
            "{}&endpoint=topics&action=list&limit={}",
            self.base_url, limit_param
        );

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<TopicsResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 获取全局配置
    pub async fn get_global_config(&self) -> Result<GlobalConfigData> {
        let url = format!("{}&endpoint=config&action=global_config", self.base_url);

        let response = Request::post(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        // 先获取响应文本进行调试
        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Text error: {}", e))?;

        let result: GlobalConfigResponse =
            serde_json::from_str(&response_text).map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            Ok(result.data)
        } else {
            Err(anyhow!("API error: failed to get global config"))
        }
    }

    /// 初始化新玩家
    pub async fn initialize_player(&self) -> Result<InitializePlayerResponse> {
        let url = format!("{}&endpoint=user&action=initialize", self.base_url);

        let response = Request::post(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!("HTTP error {}: {}", response.status(), error_text));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        if response_text.trim().is_empty() {
            return Err(anyhow!("Empty response from server"));
        }

        let result: PokemonApiResponse<InitializePlayerResponse> =
            serde_json::from_str(&response_text).map_err(|e| {
                let error_preview: String = response_text.chars().take(200).collect();
                anyhow!("Parse error: {} | Response: {}", e, error_preview)
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 刷新（hide=false，重新同步宠物数据并显示）或隐藏（hide=true）帖子宠物徽章
    pub async fn refresh_forum_badge(&self, hide: bool) -> Result<BadgeStatusResponse> {
        let url = format!("{}&endpoint=user&action=refresh_badge", self.base_url);
        let body = serde_json::json!({ "hide": hide });

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            return Err(anyhow!("HTTP error {}: {}", status, error_text));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Read error: {}", e))?;

        let result: PokemonApiResponse<BadgeStatusResponse> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 查询帖子宠物徽章当前是否隐藏
    pub async fn get_badge_status(&self) -> Result<BadgeStatusResponse> {
        let url = format!("{}&endpoint=user&action=badge_status", self.base_url);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Read error: {}", e))?;

        let result: PokemonApiResponse<BadgeStatusResponse> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }
}

impl Default for NewApiClient {
    fn default() -> Self {
        Self::new()
    }
}

// 导出类型别名以保持兼容性
pub use _utils::types::api_pokemon::PokemonListResponse;

/// 治疗响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealResponse {
    pub cost: i64,
    pub message: String,
    pub pokemon_id: u64,
    pub current_hp: i64,
    pub max_hp: i64,
}

impl NewApiClient {
    /// 使用物品
    pub async fn use_item(&self, item_id: u64, pokemon_id: Option<u64>) -> Result<UseItemResponse> {
        let url = format!("{}&endpoint=user&action=use_item", self.base_url);

        let body = UseItemRequest {
            item_id,
            pokemon_id,
        };

        let response = Request::post(&url)
            .json(&body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<UseItemResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 获取可以使用指定物品的宠物列表
    pub async fn get_usable_pokemon(&self, item_id: u64) -> Result<UsablePokemonResponse> {
        let url = format!(
            "{}&endpoint=user&action=get_usable_pokemon&item_id={}",
            self.base_url, item_id
        );

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let result: PokemonApiResponse<UsablePokemonResponse> = response
            .json()
            .await
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        if result.success {
            result.data.ok_or_else(|| anyhow!("No data returned"))
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 移动宝可梦到指定位置 (site: 2=背包, 3=仓库)
    pub async fn move_pokemon_to_site(&self, pokemon_id: u64, site: u8) -> Result<()> {
        let url = format!(
            "{}&endpoint=pokemon&action=move_pokemon&pokemon_id={}&site={}",
            self.base_url, pokemon_id, site
        );

        let response = Request::post(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            return Err(anyhow!("HTTP error {}: {}", response.status(), error_text));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        let result: PokemonApiResponse<serde_json::Value> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            Ok(())
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }

    /// 交换两个宝可梦的位置
    pub async fn swap_pokemon_position(&self, pokemon_id_1: u64, pokemon_id_2: u64) -> Result<()> {
        let url = format!(
            "{}&endpoint=pokemon&action=swap_pokemon&pokemon_id_1={}&pokemon_id_2={}",
            self.base_url, pokemon_id_1, pokemon_id_2
        );

        let response = Request::post(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if !response.ok() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            return Err(anyhow!("HTTP error {}: {}", response.status(), error_text));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        let result: PokemonApiResponse<serde_json::Value> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response: {}",
                    e,
                    response_text.chars().take(200).collect::<String>()
                )
            })?;

        if result.success {
            Ok(())
        } else {
            Err(anyhow!("API error: {}", result.error.unwrap_or_default()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_client_creation() {
        let client = NewApiClient::new();
        assert_eq!(client.base_url, "/plugin.php?id=pokemon:pokemon");
    }

    #[test]
    fn test_api_client_clone() {
        let client1 = NewApiClient::new();
        let client2 = client1.clone();
        assert_eq!(client1.base_url, client2.base_url);
    }
}
