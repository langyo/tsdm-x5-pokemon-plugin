//! 宠物品质体系：由六项个体值总和判定档位（E..SS+）。
//!
//! 与 PHP 侧 `plugin/api/pokemon_utils.php` 中的 `pm_quality_thresholds()`
//! / `pm_quality_grades()` / `pm_roll_ivs_for_rank()` 保持同一套阈值与算法，
//! 两侧任何调整必须同步。

use crate::types::pokemon_type::PokemonAttributes;

/// 档位（1 = E .. 8 = SS+）进入该档所需的最低总个体值
pub const QUALITY_THRESHOLDS: [u64; 8] = [0, 60, 84, 105, 125, 145, 160, 172];

/// 档位名称，下标 = 档位 - 1
pub const QUALITY_GRADES: [&str; 8] = ["E", "D", "C", "B", "A", "S", "SS", "SS+"];

/// 六项个体值均为 31 时的总个体值满值
pub const IV_TOTAL_MAX: u64 = 186;

/// 总个体值 -> 品质档位（1 = E .. 8 = SS+）
pub fn quality_rank(iv_total: u64) -> u8 {
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
pub fn rank_range(rank: u8) -> (u64, u64) {
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
    // 档位区间宽度最大 186，u64 取模偏差可忽略；失败回退 0 保持确定性
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
/// 与 PHP 侧一致采用「定和分配 + 随机搬移」：先在档位区间内取目标总和并
/// 均分六项，再在保持总和不变、不越 0-31 界的前提下随机搬移点数，
/// 保证结果一定落在目标档位。
pub fn roll_for_rank(rank: u8) -> PokemonAttributes {
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
            (59, 1),
            (60, 2),
            (83, 2),
            (84, 3),
            (104, 3),
            (105, 4),
            (124, 4),
            (125, 5),
            (144, 5),
            (145, 6),
            (159, 6),
            (160, 7),
            (171, 7),
            (172, 8),
            (186, 8),
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
        assert_eq!(quality_grade(8), "SS+");
        assert_eq!(quality_grade(9), "");
        assert_eq!(quality_grade_option(0), None);
        assert_eq!(quality_grade_option(7), Some("SS"));
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
        assert_eq!(quality_rank(iv_total(&attributes)), 8);
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
    }

    #[test]
    fn rolls_always_land_inside_the_requested_rank() {
        for rank in 1..=8u8 {
            for _ in 0..40 {
                let attributes = roll_for_rank(rank);
                let total = iv_total(&attributes);
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
            }
        }
    }
}
