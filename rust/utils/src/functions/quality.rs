//! 宠物品质体系：由六项个体值总和判定档位（E..SS+），并单独处理历史溢出数据。
//!
//! 档位沿用旧版截图中的阶梯（满值 186 = 6 × 31）：
//!   E 0-9、D 10-69、C 70-99、B 100-119、A 120-149、S 150-169、
//!   SS 170-185、SS+ 恰好 186。
//!
//! 历史宠物的个体值可能突破 186 上限（例如活动发放的雪拉比），此时档位
//! 记为 9 = UnKnow（未知），只用于展示，后台发放仍限定在 1-8 档。
//!
//! 与 PHP 侧 `plugin/api/pokemon_utils.php` 中的 `pm_quality_thresholds()`
//! / `pm_quality_grades()` / `pm_quality_rank()` / `pm_roll_ivs_for_rank()`
//! 保持同一套阈值与算法，两侧任何调整必须同步。

use crate::types::pokemon_type::PokemonAttributes;

/// 档位（1 = E .. 8 = SS+）进入该档所需的最低总个体值
pub const QUALITY_THRESHOLDS: [u64; 8] = [0, 10, 70, 100, 120, 150, 170, 186];

/// 档位名称，下标 = 档位 - 1
pub const QUALITY_GRADES: [&str; 8] = ["E", "D", "C", "B", "A", "S", "SS", "SS+"];

/// UnKnow 档位：历史个体值总和超过满值 186 时的档位号
pub const UNKNOWN_RANK: u8 = 9;

/// UnKnow 档位名称（沿用旧版截图的大小写）
pub const UNKNOWN_GRADE: &str = "UnKnow";

/// 六项个体值均为 31 时的总个体值满值
pub const IV_TOTAL_MAX: u64 = 186;

/// 总个体值 -> 品质档位（1 = E .. 8 = SS+，9 = UnKnow）
///
/// 超过满值 186 的历史个体值一律归入 UnKnow。
pub fn quality_rank(iv_total: u64) -> u8 {
    if iv_total > IV_TOTAL_MAX {
        return UNKNOWN_RANK;
    }
    let mut rank = 1;
    for (index, minimum) in QUALITY_THRESHOLDS.iter().enumerate() {
        if iv_total >= *minimum {
            rank = (index + 1) as u8;
        }
    }
    rank
}

/// 档位 -> 名称；0 或越界返回空串（表示无品质要求）
pub fn quality_grade(rank: u8) -> &'static str {
    if rank == UNKNOWN_RANK {
        return UNKNOWN_GRADE;
    }
    if rank == 0 || rank as usize > QUALITY_GRADES.len() {
        return "";
    }
    QUALITY_GRADES[(rank - 1) as usize]
}

/// 档位 -> 名称；None 用于 0（无要求）以区分「未设置」
pub fn quality_grade_option(rank: u8) -> Option<&'static str> {
    let grade = quality_grade(rank);
    if grade.is_empty() {
        None
    } else {
        Some(grade)
    }
}

/// 六项个体值总和
pub fn iv_total(attributes: &PokemonAttributes) -> u64 {
    attributes.hit_points
        + attributes.attack
        + attributes.defense
        + attributes.special_attack
        + attributes.special_defense
        + attributes.speed
}

/// 档位的总个体值区间（含端点）
///
/// 1-8 档首尾相接覆盖 0-186；UnKnow（9）为 187..=u64::MAX；
/// 其余非法档位回退到满值区间 (0, 186)。
pub fn rank_range(rank: u8) -> (u64, u64) {
    if rank == UNKNOWN_RANK {
        return (IV_TOTAL_MAX + 1, u64::MAX);
    }
    if rank == 0 || rank as usize > QUALITY_THRESHOLDS.len() {
        return (0, IV_TOTAL_MAX);
    }
    let minimum = QUALITY_THRESHOLDS[(rank - 1) as usize];
    let maximum = if rank as usize == QUALITY_THRESHOLDS.len() {
        IV_TOTAL_MAX
    } else {
        QUALITY_THRESHOLDS[rank as usize] - 1
    };
    (minimum, maximum)
}

fn random_below(bound: u64) -> u64 {
    // 区间宽度远小于 u64，取模偏差可忽略；失败回退 0 保持确定性
    let mut buffer = [0u8; 8];
    match getrandom::fill(&mut buffer) {
        Ok(()) => u64::from_le_bytes(buffer) % bound.max(1),
        Err(error) => {
            log::error!("品质掷取随机数失败，回退为下界: {}", error);
            0
        }
    }
}

/// 掷出一组落在指定品质档位内的六项个体值（0-31）。
///
/// 档位按旧版阶梯划分：E 0-9 ... SS 170-185、SS+ 恒为 186（六项全 31）。
/// 与 PHP 侧一致采用「定和分配 + 随机搬移」：先在档位区间内取目标总和并
/// 均分六项，再在保持总和不变、不越 0-31 界的前提下随机搬移点数，
/// 保证结果一定落在目标档位。
///
/// 档位参数会收敛到 1-8：UnKnow（9）只描述历史溢出数据，
/// 无法通过掷取生成，因此不会被发放。
pub fn roll_for_rank(rank: u8) -> PokemonAttributes {
    let rank = rank.clamp(1, 8);
    let (minimum, maximum) = rank_range(rank);
    let target = minimum + random_below(maximum - minimum + 1);

    let base = target / 6;
    let remainder = (target % 6) as usize;
    let mut values = [base; 6];
    for value in values.iter_mut().take(remainder) {
        *value += 1;
    }

    for _ in 0..24 {
        let from = random_below(6) as usize;
        let to = random_below(6) as usize;
        if from == to {
            continue;
        }
        let delta = 1 + random_below(4);
        if values[from] < delta || values[to] + delta > 31 {
            continue;
        }
        values[from] -= delta;
        values[to] += delta;
    }

    PokemonAttributes {
        hit_points: values[0],
        attack: values[1],
        defense: values[2],
        special_attack: values[3],
        special_defense: values[4],
        speed: values[5],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thresholds_match_the_documented_boundaries() {
        let cases = [
            (0u64, 1u8),
            (9, 1),
            (10, 2),
            (69, 2),
            (70, 3),
            (99, 3),
            (100, 4),
            (119, 4),
            (120, 5),
            (149, 5),
            (150, 6),
            (169, 6),
            (170, 7),
            (185, 7),
            (186, 8),
            (187, UNKNOWN_RANK),
            (190, UNKNOWN_RANK),
            (u64::MAX, UNKNOWN_RANK),
        ];
        for (total, rank) in cases {
            assert_eq!(
                quality_rank(total),
                rank,
                "总个体值 {total} 应为档位 {rank}"
            );
        }
    }

    #[test]
    fn grade_names_map_ranks() {
        assert_eq!(quality_grade(0), "");
        assert_eq!(quality_grade(1), "E");
        assert_eq!(quality_grade(2), "D");
        assert_eq!(quality_grade(7), "SS");
        assert_eq!(quality_grade(8), "SS+");
        assert_eq!(quality_grade(UNKNOWN_RANK), UNKNOWN_GRADE);
        assert_eq!(UNKNOWN_GRADE, "UnKnow");
        assert_eq!(quality_grade(10), "");
        assert_eq!(quality_grade_option(0), None);
        assert_eq!(quality_grade_option(7), Some("SS"));
        assert_eq!(quality_grade_option(UNKNOWN_RANK), Some("UnKnow"));
    }

    #[test]
    fn iv_total_sums_all_six_stats() {
        let attributes = PokemonAttributes {
            hit_points: 31,
            attack: 31,
            defense: 31,
            special_attack: 31,
            special_defense: 31,
            speed: 17,
        };
        assert_eq!(iv_total(&attributes), 172);
        assert_eq!(quality_rank(iv_total(&attributes)), 7);

        // 历史个体值可以突破单项 31 的上限，总和随之超过 186。
        let legacy_overflow = PokemonAttributes {
            hit_points: 40,
            attack: 31,
            defense: 31,
            special_attack: 31,
            special_defense: 31,
            speed: 31,
        };
        assert_eq!(iv_total(&legacy_overflow), 195);
        assert_eq!(quality_rank(iv_total(&legacy_overflow)), UNKNOWN_RANK);
        assert_eq!(
            quality_grade(quality_rank(iv_total(&legacy_overflow))),
            UNKNOWN_GRADE
        );
    }

    #[test]
    fn rank_ranges_are_contiguous() {
        let mut previous_end = 0u64;
        for rank in 1..=8u8 {
            let (minimum, maximum) = rank_range(rank);
            assert_eq!(
                minimum, previous_end,
                "档位 {rank} 的下界应与上一档上界相接"
            );
            assert!(minimum <= maximum);
            previous_end = maximum + 1;
        }
        assert_eq!(previous_end - 1, IV_TOTAL_MAX);

        assert_eq!(rank_range(UNKNOWN_RANK), (IV_TOTAL_MAX + 1, u64::MAX));
        assert_eq!(rank_range(0), (0, IV_TOTAL_MAX));
        assert_eq!(rank_range(10), (0, IV_TOTAL_MAX));
    }

    #[test]
    fn rolls_always_land_inside_the_requested_rank() {
        for rank in 1..=8u8 {
            let (minimum, maximum) = rank_range(rank);
            for _ in 0..40 {
                let attributes = roll_for_rank(rank);
                let total = iv_total(&attributes);
                assert!(
                    (minimum..=maximum).contains(&total),
                    "掷取总和 {total} 必须落在档位 {rank} 的区间 {minimum}..={maximum}"
                );
                assert_eq!(
                    quality_rank(total),
                    rank,
                    "掷取总和 {total} 必须落在档位 {rank}"
                );
                for value in [
                    attributes.hit_points,
                    attributes.attack,
                    attributes.defense,
                    attributes.special_attack,
                    attributes.special_defense,
                    attributes.speed,
                ] {
                    assert!(value <= 31, "单项个体值不得超过 31，实际 {value}");
                }
                if rank == 8 {
                    assert_eq!(
                        [
                            attributes.hit_points,
                            attributes.attack,
                            attributes.defense,
                            attributes.special_attack,
                            attributes.special_defense,
                            attributes.speed,
                        ],
                        [31u64; 6],
                        "SS+ 掷取必须为六项 31"
                    );
                }
            }
        }
    }

    #[test]
    fn roll_for_rank_clamps_out_of_range_ranks() {
        for _ in 0..20 {
            let lowest = roll_for_rank(0);
            assert_eq!(quality_rank(iv_total(&lowest)), 1);
            let unknown = roll_for_rank(UNKNOWN_RANK);
            assert_eq!(iv_total(&unknown), IV_TOTAL_MAX);
            assert_eq!(quality_rank(iv_total(&unknown)), 8);
        }
    }
}
