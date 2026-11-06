//! 仓库筛选/排序纯逻辑。
//!
//! 移植自 `rust/game/src/utils/storage_filters.rs`（Dioxus 版），含原测试。
//! 规则唯一真源是这里与上述模块，两侧调整必须同步。

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StorageSort {
    #[default]
    Oldest,
    Newest,
    LevelAscending,
    LevelDescending,
}

impl StorageSort {
    pub fn from_value(value: &str) -> Self {
        match value {
            "newest" => Self::Newest,
            "level_asc" => Self::LevelAscending,
            "level_desc" => Self::LevelDescending,
            _ => Self::Oldest,
        }
    }

    pub fn value(self) -> &'static str {
        match self {
            Self::Oldest => "oldest",
            Self::Newest => "newest",
            Self::LevelAscending => "level_asc",
            Self::LevelDescending => "level_desc",
        }
    }
}

#[derive(Default)]
pub struct StorageFilters<'a> {
    pub name: &'a str,
    pub pokemon_type: &'a str,
    pub min_level: Option<u64>,
    pub max_level: Option<u64>,
    pub shiny: Option<bool>,
    pub sort: StorageSort,
}

pub struct StorageEntry<'a> {
    pub id: u64,
    pub name: &'a str,
    pub nickname: Option<&'a str>,
    pub type_1: &'a str,
    pub type_2: Option<&'a str>,
    pub level: u64,
    pub shiny: bool,
}

pub fn parse_level_range(min: &str, max: &str) -> Result<(Option<u64>, Option<u64>), &'static str> {
    let parse = |value: &str| {
        if value.trim().is_empty() {
            Ok(None)
        } else {
            value
                .trim()
                .parse::<u64>()
                .ok()
                .filter(|level| *level > 0)
                .map(Some)
                .ok_or("等级请输入大于 0 的整数")
        }
    };
    let (min, max) = (parse(min)?, parse(max)?);
    if matches!((min, max), (Some(a), Some(b)) if a > b) {
        return Err("最低等级不能高于最高等级");
    }
    Ok((min, max))
}

pub fn matching_storage_ids(
    entries: &[StorageEntry<'_>],
    filters: &StorageFilters<'_>,
) -> Vec<u64> {
    let keyword = filters.name.trim().to_lowercase();
    let mut matched: Vec<_> = entries
        .iter()
        .filter(|entry| {
            (keyword.is_empty()
                || entry.name.to_lowercase().contains(&keyword)
                || entry
                    .nickname
                    .unwrap_or_default()
                    .to_lowercase()
                    .contains(&keyword))
                && (filters.pokemon_type.is_empty()
                    || entry.type_1 == filters.pokemon_type
                    || entry.type_2 == Some(filters.pokemon_type))
                && filters.min_level.map_or(true, |min| entry.level >= min)
                && filters.max_level.map_or(true, |max| entry.level <= max)
                && filters.shiny.map_or(true, |shiny| entry.shiny == shiny)
        })
        .collect();
    matched.sort_by(|a, b| match filters.sort {
        StorageSort::Oldest => a.id.cmp(&b.id),
        StorageSort::Newest => b.id.cmp(&a.id),
        StorageSort::LevelAscending => a.level.cmp(&b.level).then(a.id.cmp(&b.id)),
        StorageSort::LevelDescending => b.level.cmp(&a.level).then(a.id.cmp(&b.id)),
    });
    matched.into_iter().map(|entry| entry.id).collect()
}

/// 批量动作执行时重核筛选结果（含刷新后失效的 id）。
pub fn selected_matching_ids(selected: &[u64], matching: &[u64]) -> Vec<u64> {
    let allowed: HashSet<_> = matching.iter().copied().collect();
    selected
        .iter()
        .copied()
        .filter(|id| allowed.contains(id))
        .collect()
}

/// 仓库页 HP 辅助（对齐 Dioxus 版 utils::pokemon）。
pub fn hp_class_storage(hp: i64, max_hp: i64) -> &'static str {
    if max_hp <= 0 || hp <= 0 {
        return "hp-bar-critical";
    }
    let pct = hp as f64 / max_hp as f64;
    if pct > 0.5 {
        "hp-bar-high"
    } else if pct > 0.2 {
        "hp-bar-mid"
    } else {
        "hp-bar-low"
    }
}

pub fn hp_health_status(hp: i64, max_hp: i64) -> &'static str {
    if max_hp <= 0 || hp <= 0 {
        "critical"
    } else {
        let pct = hp as f64 / max_hp as f64;
        if pct <= 0.2 {
            "danger"
        } else if pct <= 0.5 {
            "warning"
        } else {
            "healthy"
        }
    }
}

/// 虚弱状态（20-22）与濒危同样需要特殊标记。
pub fn is_weak_state(state: u8) -> bool {
    matches!(state, 20..=22)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries() -> Vec<StorageEntry<'static>> {
        vec![
            StorageEntry {
                id: 3,
                name: "喷火龙",
                nickname: Some("Flame"),
                type_1: "火",
                type_2: Some("飞行"),
                level: 50,
                shiny: true,
            },
            StorageEntry {
                id: 1,
                name: "皮卡丘",
                nickname: None,
                type_1: "电",
                type_2: None,
                level: 20,
                shiny: false,
            },
            StorageEntry {
                id: 2,
                name: "小火龙",
                nickname: Some("小火"),
                type_1: "火",
                type_2: None,
                level: 20,
                shiny: false,
            },
        ]
    }

    #[test]
    fn combined_filters_match_secondary_type_and_inclusive_levels() {
        let filters = StorageFilters {
            name: " flaME ",
            pokemon_type: "飞行",
            min_level: Some(50),
            max_level: Some(50),
            shiny: Some(true),
            ..Default::default()
        };
        assert_eq!(matching_storage_ids(&entries(), &filters), [3]);
        assert!(matching_storage_ids(
            &entries(),
            &StorageFilters {
                shiny: Some(false),
                ..filters
            }
        )
        .is_empty());
    }

    #[test]
    fn name_filter_searches_both_names_without_a_nickname() {
        assert_eq!(
            matching_storage_ids(
                &entries(),
                &StorageFilters {
                    name: "皮卡",
                    ..Default::default()
                }
            ),
            [1]
        );
        assert_eq!(
            matching_storage_ids(
                &entries(),
                &StorageFilters {
                    name: "小火",
                    ..Default::default()
                }
            ),
            [2]
        );
    }

    #[test]
    fn type_and_level_filters_apply_together() {
        assert_eq!(
            matching_storage_ids(
                &entries(),
                &StorageFilters {
                    pokemon_type: "火",
                    max_level: Some(20),
                    ..Default::default()
                }
            ),
            [2]
        );
        assert_eq!(
            matching_storage_ids(
                &entries(),
                &StorageFilters {
                    min_level: Some(21),
                    ..Default::default()
                }
            ),
            [3]
        );
    }

    #[test]
    fn sorting_is_stable_for_equal_levels_and_tracks_acquisition_ids() {
        for (sort, expected) in [
            (StorageSort::Oldest, vec![1, 2, 3]),
            (StorageSort::Newest, vec![3, 2, 1]),
            (StorageSort::LevelAscending, vec![1, 2, 3]),
            (StorageSort::LevelDescending, vec![3, 1, 2]),
        ] {
            assert_eq!(
                matching_storage_ids(
                    &entries(),
                    &StorageFilters {
                        sort,
                        ..Default::default()
                    }
                ),
                expected
            );
            assert_eq!(StorageSort::from_value(sort.value()), sort);
        }
    }

    #[test]
    fn range_validation_keeps_invalid_input_from_matching_everything() {
        assert_eq!(parse_level_range("", " "), Ok((None, None)));
        assert_eq!(parse_level_range(" 20 ", "50"), Ok((Some(20), Some(50))));
        for (min, max) in [
            ("0", ""),
            ("-1", ""),
            ("1.5", ""),
            ("51", "50"),
            ("", "wrong"),
        ] {
            assert!(parse_level_range(min, max).is_err());
        }
    }

    #[test]
    fn selection_excludes_filtered_and_removed_pokemon() {
        assert_eq!(selected_matching_ids(&[1, 2, 3, 99], &[2, 3]), [2, 3]);
        assert!(selected_matching_ids(&[1, 2], &[]).is_empty());
    }

    #[test]
    fn filtering_and_sorting_happen_before_the_render_limit() {
        let entries: Vec<_> = (1..=130)
            .map(|id| StorageEntry {
                id,
                name: "皮卡丘",
                nickname: None,
                type_1: "电",
                type_2: None,
                level: id,
                shiny: id > 60,
            })
            .collect();
        let matching = matching_storage_ids(
            &entries,
            &StorageFilters {
                shiny: Some(true),
                sort: StorageSort::Newest,
                ..Default::default()
            },
        );
        assert_eq!(matching.len(), 70);
        assert_eq!(matching[0], 130);
        assert_eq!(matching[59], 71);
        assert_eq!(selected_matching_ids(&[1, 61, 130], &matching), [61, 130]);
    }

    #[test]
    fn hp_helpers_classify_health_states() {
        assert_eq!(hp_health_status(0, 10), "critical");
        assert_eq!(hp_health_status(1, 10), "danger");
        assert_eq!(hp_health_status(5, 10), "warning");
        assert_eq!(hp_health_status(9, 10), "healthy");
        assert_eq!(hp_class_storage(9, 10), "hp-bar-high");
        assert!(is_weak_state(21));
        assert!(!is_weak_state(1));
    }
}
