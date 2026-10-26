//! API 客户端
//!
//! 使用 pokemon_system/api/*.php 端点
//! URL格式: plugin.php?id=pokemon:pokemon&endpoint=xxx&action=yyy
//!
//! 所有请求走统一的核心封装：`send_ok` 负责发送与非 2xx 处理（优先解析
//! 响应体信封中的 error 字段），`parse_envelope` 负责读取正文并按
//! `ApiResponse<T>` 信封解包 data。各端点方法只声明 endpoint/action、
//! 查询串与请求体。

use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, task::Poll};

use gloo_net::http::{Request, Response};

// 导入API类型
use _utils::types::api_battle::{
    BattleItemsResponse, BattleLogResponse, BattleMutationResponse, BattleScene, FleeRequest,
    PendingBattleAction, StartBattleRequest, UseItemOnSkillRequest, UseSkillRequest,
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
#[derive(Clone, Copy)]
pub struct NewApiClient {
    base_url: &'static str,
}

#[derive(Debug)]
pub struct BattleActionError {
    pub message: String,
    /// 网络断线或响应不完整时，服务端可能已执行，必须保留原 request_id。
    pub uncertain: bool,
}

pub(crate) async fn battle_timeout<T>(future: impl Future<Output = T>) -> Result<T> {
    let mut operation = Box::pin(future);
    let mut timeout = Box::pin(gloo_timers::future::TimeoutFuture::new(15_000));
    std::future::poll_fn(move |context| {
        if let Poll::Ready(value) = operation.as_mut().poll(context) {
            return Poll::Ready(Ok(value));
        }
        if timeout.as_mut().poll(context).is_ready() {
            Poll::Ready(Err(anyhow!("连接超时，请重试确认结果")))
        } else {
            Poll::Pending
        }
    })
    .await
}

/// RFC 3986 百分号编码：除未保留字符外全部转义，保证中文与保留字符
/// 出现在查询串时原样送达服务端（与 PHP 端字面子串搜索配套）。
pub(crate) fn encode_query_component(value: &str) -> String {
    value
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

impl NewApiClient {
    pub fn new() -> Self {
        Self {
            base_url: "/plugin.php?id=pokemon:pokemon",
        }
    }

    async fn player_mutation<T>(&self, operation: impl Future<Output = Result<T>>) -> Result<T> {
        battle_timeout(operation)
            .await
            .map_err(|_| anyhow!("请求超时，操作结果尚未确认。请先检查最新资料，避免重复操作。"))?
    }

    // ============== 请求核心 ==============

    fn url(&self, endpoint: &str, action: &str, query: &str) -> String {
        format!(
            "{}&endpoint={}&action={}{}",
            self.base_url, endpoint, action, query
        )
    }

    /// 发送请求并处理非 2xx：优先解析响应体信封中的 error 字段（如"您没有该物品"），
    /// 解析不出时回退为带状态码与正文预览的 HTTP 错误。
    /// 接收 `request.send()` 产生的 future，兼容带体与不带体两类请求。
    async fn send_ok(
        &self,
        send_fut: impl std::future::Future<Output = Result<Response, gloo_net::Error>>,
    ) -> Result<Response> {
        let response = send_fut
            .await
            .map_err(|e| anyhow!("Network error: {}", e))?;

        if response.ok() {
            return Ok(response);
        }

        let status = response.status();
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
        Err(anyhow!(
            "HTTP error {}: {}",
            status,
            error_text.chars().take(200).collect::<String>()
        ))
    }

    /// 读取正文并按 `ApiResponse<T>` 信封解包 data。
    async fn parse_envelope<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        if response_text.trim().is_empty() {
            return Err(anyhow!("Empty response from server"));
        }

        let result: PokemonApiResponse<T> = serde_json::from_str(&response_text).map_err(|e| {
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

    /// 读取正文并只校验信封 success，不关心 data（用于 data 可为空的写操作）。
    async fn parse_envelope_ok(&self, response: Response) -> Result<()> {
        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        let result: PokemonApiResponse<serde_json::Value> = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow!(
                    "Parse error: {} | Response was: {}",
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

    /// GET 并解包 data。
    async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        action: &str,
        query: &str,
    ) -> Result<T> {
        let response = self
            .send_ok(Request::get(&self.url(endpoint, action, query)).send())
            .await?;
        self.parse_envelope(response).await
    }

    /// 携 JSON 请求体的 POST 并解包 data。
    async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        action: &str,
        query: &str,
        body: &B,
    ) -> Result<T> {
        let request = Request::post(&self.url(endpoint, action, query))
            .json(body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?;
        let response = self.send_ok(request.send()).await?;
        self.parse_envelope(response).await
    }

    /// 无请求体的 POST 并解包 data。
    async fn post_empty<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        action: &str,
        query: &str,
    ) -> Result<T> {
        let response = self
            .send_ok(Request::post(&self.url(endpoint, action, query)).send())
            .await?;
        self.parse_envelope(response).await
    }

    /// 携 JSON 请求体的 POST，只校验 success。
    async fn post_unit<B: Serialize>(
        &self,
        endpoint: &str,
        action: &str,
        query: &str,
        body: &B,
    ) -> Result<()> {
        let request = Request::post(&self.url(endpoint, action, query))
            .json(body)
            .map_err(|e| anyhow!("Serialize error: {}", e))?;
        let response = self.send_ok(request.send()).await?;
        self.parse_envelope_ok(response).await
    }

    /// 无请求体的 POST，只校验 success。
    async fn post_empty_unit(&self, endpoint: &str, action: &str, query: &str) -> Result<()> {
        let response = self
            .send_ok(Request::post(&self.url(endpoint, action, query)).send())
            .await?;
        self.parse_envelope_ok(response).await
    }

    // ============== Pokemon API ==============

    /// 获取宠物列表
    pub async fn get_pokemon_list(&self) -> Result<PokemonListResponse> {
        battle_timeout(self.get("pokemon", "list", "")).await?
    }

    /// 获取宠物详情
    pub async fn get_pokemon_detail(&self, id: u64) -> Result<PokemonDetail> {
        self.get("pokemon", "detail", &format!("&pokemon_id={}", id))
            .await
    }

    /// 重命名宠物
    pub async fn rename_pokemon(&self, id: u64, name: &str) -> Result<()> {
        self.player_mutation(self.post_unit(
            "pokemon",
            "rename",
            "",
            &RenameRequest {
                id,
                name: name.to_string(),
            },
        ))
        .await
    }

    /// 放生宠物
    pub async fn release_pokemon(&self, id: u64) -> Result<()> {
        self.post_unit("pokemon", "release", "", &ReleaseRequest { id })
            .await
    }

    /// 列入首位 (设置为首宠)
    pub async fn set_first_pokemon(&self, pokemon_id: u64) -> Result<()> {
        self.post_empty_unit(
            "pokemon",
            "set_first",
            &format!("&pokemon_id={}", pokemon_id),
        )
        .await
    }

    /// 获取宠物可学习的技能列表
    pub async fn get_learnable_skills(&self, pokemon_id: u64) -> Result<LearnableSkillsResponse> {
        battle_timeout(self.get(
            "pokemon",
            "learnable_skills",
            &format!("&pokemon_id={}", pokemon_id),
        ))
        .await?
    }

    /// 遗忘宠物技能
    pub async fn forget_skill(&self, pokemon_id: u64, skill_id: u64) -> Result<()> {
        self.player_mutation(self.post_unit(
            "pokemon",
            "forget_skill",
            "",
            &serde_json::json!({
                "pokemon_id": pokemon_id,
                "skill_id": skill_id
            }),
        ))
        .await
    }

    /// 学习新技能（pm_myskill 无槽位列，技能按插入顺序生效；
    /// 替换技能需先调用遗忘接口）
    pub async fn learn_skill(&self, pokemon_id: u64, skill_id: u64) -> Result<LearnSkillResponse> {
        self.player_mutation(self.post(
            "pokemon",
            "learn_skill",
            "",
            &LearnSkillRequest {
                pokemon_id,
                skill_id,
            },
        ))
        .await
    }

    /// 获取宝可梦装备信息
    pub async fn get_equipment(&self, pokemon_id: u64) -> Result<EquipmentResponse> {
        battle_timeout(self.get(
            "pokemon",
            "equipment",
            &format!("&pokemon_id={}", pokemon_id),
        ))
        .await?
    }

    /// 装备物品
    pub async fn equip_item(
        &self,
        pokemon_id: u64,
        myitem_id: u64,
        slot_index: Option<i32>,
    ) -> Result<EquipItemResponse> {
        self.player_mutation(self.post(
            "pokemon",
            "equip_item",
            "",
            &EquipItemRequest {
                pokemon_id,
                myitem_id,
                slot_index,
            },
        ))
        .await
    }

    /// 卸下装备
    pub async fn unequip_item(
        &self,
        pokemon_id: u64,
        slot_index: u32,
    ) -> Result<UnequipItemResponse> {
        self.player_mutation(self.post(
            "pokemon",
            "unequip_item",
            "",
            &UnequipItemRequest {
                pokemon_id,
                slot_index,
            },
        ))
        .await
    }

    /// 更新宠物状态（随机触发）
    pub async fn update_pokemon_state(&self) -> Result<UpdateStateResponse> {
        self.get("pokemon", "update_state", "").await
    }

    /// 移动宝可梦到指定位置 (site: 2=背包, 3=仓库)
    pub async fn move_pokemon_to_site(&self, pokemon_id: u64, site: u8) -> Result<()> {
        self.post_empty_unit(
            "pokemon",
            "move_pokemon",
            &format!("&pokemon_id={}&site={}", pokemon_id, site),
        )
        .await
    }

    /// 交换两个宝可梦的位置
    pub async fn swap_pokemon_position(&self, pokemon_id_1: u64, pokemon_id_2: u64) -> Result<()> {
        self.post_empty_unit(
            "pokemon",
            "swap_pokemon",
            &format!(
                "&pokemon_id_1={}&pokemon_id_2={}",
                pokemon_id_1, pokemon_id_2
            ),
        )
        .await
    }

    // ============== Battle API ==============

    /// 开始战斗
    pub async fn start_battle(
        &self,
        map_id: u64,
        boss_pokemon_type_id: Option<u64>,
    ) -> Result<BattleScene> {
        self.start_battle_at_index(map_id, boss_pokemon_type_id, None)
            .await
    }

    /// 按地图配置索引挑战 Boss，避免同物种的多个配置被当成同一项。
    pub async fn start_battle_at_index(
        &self,
        map_id: u64,
        boss_pokemon_type_id: Option<u64>,
        boss_index: Option<u64>,
    ) -> Result<BattleScene> {
        self.post(
            "battle",
            "start",
            "",
            &StartBattleRequest {
                map_id,
                boss_pokemon_type_id,
                boss_index,
            },
        )
        .await
    }

    /// 使用技能
    pub async fn use_skill(&self, battle_id: &str, skill_id: u64) -> Result<BattleScene> {
        self.post(
            "battle",
            "turn",
            "",
            &UseSkillRequest {
                battle_id: Some(battle_id.to_string()),
                skill_id,
            },
        )
        .await
    }

    /// 逃跑
    pub async fn flee(&self, battle_id: &str) -> Result<BattleScene> {
        self.post(
            "battle",
            "flee",
            "",
            &FleeRequest {
                battle_id: Some(battle_id.to_string()),
            },
        )
        .await
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
        let mut body_json = serde_json::json!({
            "battle_id": battle_id,
        });
        // 如果提供了 pokemon_id，添加到请求体中
        if let Some(pid) = pokemon_id {
            body_json["pokemon_id"] = serde_json::json!(pid);
        }

        self.post("battle", "switch_pokemon", "", &body_json).await
    }

    /// 被动替换宠物（宠物被打死后替换，不反击）
    pub async fn replace_pokemon(&self, _battle_id: &str, pokemon_id: u64) -> Result<BattleScene> {
        self.post(
            "battle",
            "replace_pokemon",
            "",
            &serde_json::json!({
                "pokemon_id": pokemon_id,
            }),
        )
        .await
    }

    /// 捕捉
    pub async fn capture(&self, ball_id: u64) -> Result<BattleScene> {
        self.post(
            "battle",
            "capture",
            "",
            &serde_json::json!({
                "ball_id": ball_id,
            }),
        )
        .await
    }

    /// 在战斗中使用物品（可能返回技能选择响应）
    ///
    /// 返回原始响应文本：PP 恢复类道具需要先让用户选择技能（requires_skill_selection），
    /// 其余物品直接返回战斗场景。调用方只需发起这一次请求——服务端在该请求中
    /// 已经完成扣道具、加血与野怪反击，再次调用会重复扣道具。
    pub async fn use_item_in_battle_raw(&self, item_id: u64) -> Result<String> {
        let request = Request::post(&self.url("battle", "use_item", ""))
            .json(&serde_json::json!({
                "item_id": item_id,
            }))
            .map_err(|e| anyhow!("Serialize error: {}", e))?;
        let response = self.send_ok(request.send()).await?;
        response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))
    }

    /// 在战斗中对指定技能使用PP恢复道具
    pub async fn use_item_on_skill(
        &self,
        item_id: u64,
        skill_record_id: u64,
    ) -> Result<BattleScene> {
        self.post(
            "battle",
            "use_item_on_skill",
            "",
            &UseItemOnSkillRequest {
                item_id,
                skill_record_id,
            },
        )
        .await
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
        battle_timeout(self.get("battle", "maps", &params)).await?
    }

    /// 恢复战斗状态
    pub async fn recover_battle(&self) -> Result<BattleScene> {
        battle_timeout(self.get("battle", "recover", "")).await?
    }

    pub async fn recover_current_battle(&self) -> Result<Option<BattleScene>> {
        battle_timeout(async {
            let response = Request::get(&self.url("battle", "recover", ""))
                .send()
                .await
                .map_err(|_| anyhow!("无法连接服务器"))?;
            if response.status() == 404 {
                return Ok(None);
            }
            let response = self.send_ok(async { Ok(response) }).await?;
            self.parse_envelope(response).await.map(Some)
        })
        .await?
    }

    pub async fn get_battle_log(&self, battle_id: u64) -> Result<BattleLogResponse> {
        battle_timeout(self.get("battle", "battle_log", &format!("&battle_id={battle_id}"))).await?
    }

    pub async fn perform_battle_action(
        &self,
        pending: &PendingBattleAction,
    ) -> std::result::Result<BattleMutationResponse, BattleActionError> {
        let uncertain = |message: String| BattleActionError {
            message,
            uncertain: true,
        };
        let request = Request::post(&self.url("battle", &pending.action, ""))
            .json(&pending.request)
            .map_err(|error| BattleActionError {
                message: format!("无法建立请求：{error}"),
                uncertain: false,
            })?;
        let (status, text) = battle_timeout(async {
            let response = request.send().await?;
            let status = response.status();
            response.text().await.map(|text| (status, text))
        })
        .await
        .map_err(|error| uncertain(error.to_string()))?
        .map_err(|_| uncertain("连接中断，操作结果尚未确认。".into()))?;
        if status == 401 || status == 403 {
            return Err(uncertain(
                "登录或验证已过期，请重新登录并刷新页面，再重试确认原操作结果。".into(),
            ));
        }
        let envelope: PokemonApiResponse<serde_json::Value> = serde_json::from_str(&text)
            .map_err(|_| uncertain("服务器响应不完整，操作结果尚未确认。".into()))?;
        if status >= 500 {
            return Err(uncertain("服务器暂时无法回应，操作结果尚未确认。".into()));
        }
        if !envelope.success || status >= 400 {
            return Err(BattleActionError {
                message: envelope
                    .error
                    .unwrap_or_else(|| "战斗状态已改变，请重新同步。".into()),
                uncertain: false,
            });
        }
        serde_json::from_value(envelope.data.unwrap_or(serde_json::Value::Null))
            .map_err(|_| uncertain("服务器返回的战斗资料不完整。".into()))
    }

    /// 获取可以在战斗中使用的物品（只包含HP恢复和PP恢复道具）
    pub async fn get_battle_items(&self) -> Result<BattleItemsResponse> {
        battle_timeout(self.get("battle", "get_battle_items", "")).await?
    }

    pub async fn get_battle_balls(&self) -> Result<InventoryResponse> {
        battle_timeout(self.get_user_inventory(Some(2), 1)).await?
    }

    // ============== Shop API ==============

    /// 获取商店物品（keyword 为名称字面子串筛选，空串表示不筛选）
    pub async fn get_shop_items(
        &self,
        item_type: Option<u32>,
        page: u32,
        keyword: &str,
    ) -> Result<ShopListResponse> {
        let type_param = item_type
            .map(|t| format!("&type={}", t))
            .unwrap_or_default();
        let keyword_param = if keyword.is_empty() {
            String::new()
        } else {
            format!("&keyword={}", encode_query_component(keyword))
        };
        battle_timeout(self.get(
            "shop",
            "list",
            &format!("&page={}{}{}", page, type_param, keyword_param),
        ))
        .await?
    }

    /// 购买物品
    pub async fn buy_item(&self, item_id: u64, quantity: u32) -> Result<BuyItemResponse> {
        self.player_mutation(self.post(
            "shop",
            "buy",
            "",
            &BuyItemRequest {
                item_id,
                quantity: quantity as u64,
            },
        ))
        .await
    }

    /// 获取商店宠物（keyword 为名称字面子串筛选，空串表示不筛选）
    pub async fn get_shop_pets(&self, page: u32, keyword: &str) -> Result<ShopPetListResponse> {
        let keyword_param = if keyword.is_empty() {
            String::new()
        } else {
            format!("&keyword={}", encode_query_component(keyword))
        };
        battle_timeout(self.get("shop", "pets", &format!("&page={}{}", page, keyword_param)))
            .await?
    }

    pub async fn buy_pet(&self, pokemon_type_id: u64) -> Result<BuyPetResponse> {
        self.player_mutation(self.post("shop", "buy_pet", "", &BuyPetRequest { pokemon_type_id }))
            .await
    }

    // ============== Evolution API ==============

    /// 检查进化
    pub async fn check_evolution(&self, pet_id: u64) -> Result<EvolutionCheckResponse> {
        self.get("evolution", "check", &format!("&pet_id={}", pet_id))
            .await
    }

    /// 执行进化（多条路线都满足时可用 to_id 指定目标形态）
    pub async fn evolve(&self, pet_id: u64, to_id: Option<u64>) -> Result<EvolutionResponse> {
        self.post(
            "evolution",
            "evolve",
            "",
            &EvolutionRequest { pet_id, to_id },
        )
        .await
    }

    pub async fn get_evolution_path(&self, pmid: u64) -> Result<EvolutionPathResponse> {
        self.get("evolution", "evolution_path", &format!("&pmid={}", pmid))
            .await
    }

    // ============== User API ==============

    /// 获取用户资料
    pub async fn get_user_profile(&self) -> Result<UserProfileResponse> {
        battle_timeout(self.get("user", "profile", "")).await?
    }

    /// 获取用户游戏统计
    pub async fn get_user_stats(&self) -> Result<UserStatsResponse> {
        self.get("user", "stats", "").await
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
        self.get(
            "user",
            "inventory",
            &format!("&page={}{}", page, type_param),
        )
        .await
    }

    pub async fn get_user_inventory_search(
        &self,
        item_type: Option<u32>,
        page: u32,
        search: &str,
    ) -> Result<InventoryResponse> {
        let encoded = encode_query_component(search);
        let category = item_type
            .map(|value| format!("&type={value}"))
            .unwrap_or_default();
        self.get(
            "user",
            "inventory",
            &format!("&page={page}{category}&search={encoded}"),
        )
        .await
    }

    /// 获取背包物品类型统计
    pub async fn get_inventory_stats(&self) -> Result<InventoryStatsResponse> {
        self.get("user", "inventory_stats", "").await
    }

    /// 治疗宝可梦
    pub async fn heal_pokemon(&self, pokemon_id: u64) -> Result<HealResponse> {
        self.player_mutation(self.post_empty(
            "user",
            "heal",
            &format!("&pokemon_id={}", pokemon_id),
        ))
        .await
    }

    /// 脱战并治疗宝可梦（宠物中心绿色通道）
    pub async fn heal_and_flee(&self, pokemon_id: u64) -> Result<HealResponse> {
        self.player_mutation(self.post_empty(
            "user",
            "heal_and_flee",
            &format!("&pokemon_id={}", pokemon_id),
        ))
        .await
    }

    /// 获取在线玩家
    pub async fn get_online_players(&self) -> Result<OnlinePlayersResponse> {
        self.get("user", "online_players", "").await
    }

    /// 初始化新玩家
    pub async fn initialize_player(&self) -> Result<InitializePlayerResponse> {
        self.player_mutation(self.post_empty("user", "initialize", ""))
            .await
    }

    /// 刷新（hide=false，重新同步宠物数据并显示）或隐藏（hide=true）帖子宠物徽章
    pub async fn refresh_forum_badge(&self, hide: bool) -> Result<BadgeStatusResponse> {
        self.player_mutation(self.post(
            "user",
            "refresh_badge",
            "",
            &serde_json::json!({ "hide": hide }),
        ))
        .await
    }

    /// 查询帖子宠物徽章当前是否隐藏
    pub async fn get_badge_status(&self) -> Result<BadgeStatusResponse> {
        battle_timeout(self.get("user", "badge_status", "")).await?
    }

    /// 使用物品
    pub async fn use_item(&self, item_id: u64, pokemon_id: Option<u64>) -> Result<UseItemResponse> {
        self.post(
            "user",
            "use_item",
            "",
            &UseItemRequest {
                item_id,
                pokemon_id,
            },
        )
        .await
    }

    /// 获取可以使用指定物品的宠物列表
    pub async fn get_usable_pokemon(&self, item_id: u64) -> Result<UsablePokemonResponse> {
        self.get(
            "user",
            "get_usable_pokemon",
            &format!("&item_id={}", item_id),
        )
        .await
    }

    // =============== Config API ===============

    /// 获取全局配置（该信封的 data 非可空，走独立解析）
    pub async fn get_global_config(&self) -> Result<GlobalConfigData> {
        let response = self
            .send_ok(Request::post(&self.url("config", "global_config", "")).send())
            .await?;
        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;
        let result: GlobalConfigResponse =
            serde_json::from_str(&response_text).map_err(|e| anyhow!("Parse error: {}", e))?;
        if result.success {
            Ok(result.data)
        } else {
            Err(anyhow!("API error: failed to get global config"))
        }
    }

    // =============== Topics API ===============

    /// 获取话题列表
    pub async fn get_topics(&self, limit: Option<u32>) -> Result<TopicsResponse> {
        let limit_param = limit.unwrap_or(6);
        self.get("topics", "list", &format!("&limit={}", limit_param))
            .await
    }
}

impl Default for NewApiClient {
    fn default() -> Self {
        Self::new()
    }
}

// 导出类型别名以保持兼容性
pub use _utils::types::api_pokemon::PokemonListResponse;
pub use _utils::types::api_user::HealResponse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_client_creation() {
        let client = NewApiClient::new();
        assert_eq!(client.base_url, "/plugin.php?id=pokemon:pokemon");
    }

    #[test]
    fn test_api_client_copy() {
        // NewApiClient 是 Copy 句柄，按值复制即可共享
        let client1 = NewApiClient::new();
        let client2 = client1;
        assert_eq!(client1.base_url, client2.base_url);
    }
}
