//! 宠物品质体系（只读部分）。
//!
//! 移植自 `rust/utils/src/functions/quality.rs`，只保留展示所需：总个体值
//! 到档位的判定与档位名称。掷取（`roll_for_rank`）依赖 getrandom 与本页
//! 无关，故不移植；阈值的唯一真源仍是 PHP `pm_quality_thresholds()` 与
//! 上述 Rust 模块，三处调整必须同步。

/// 档位（1 = E .. 8 = SS+）进入该档所需的最低总个体值。
pub const QUALITY_THRESHOLDS: [u64; 8] = [0, 10, 70, 100, 120, 150, 170, 186];

/// 档位名称，下标 = 档位 - 1。
pub const QUALITY_GRADES: [&str; 8] = ["E", "D", "C", "B", "A", "S", "SS", "SS+"];

/// UnKnow 档位：历史个体值总和超过满值 186 时的档位号。
pub const UNKNOWN_RANK: u8 = 9;

/// UnKnow 档位名称（沿用旧版截图的大小写）。
pub const UNKNOWN_GRADE: &str = "UnKnow";

/// 六项个体值均为 31 时的总个体值满值。
pub const IV_TOTAL_MAX: u64 = 186;

/// 总个体值 -> 品质档位（1 = E .. 8 = SS+，9 = UnKnow）。
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

/// 档位 -> 名称；0 或越界返回空串（表示无品质要求）。
pub fn quality_grade(rank: u8) -> &'static str {
    if rank == UNKNOWN_RANK {
        return UNKNOWN_GRADE;
    }
    if rank == 0 || rank as usize > QUALITY_GRADES.len() {
        return "";
    }
    QUALITY_GRADES[(rank - 1) as usize]
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
        assert_eq!(quality_grade(7), "SS");
        assert_eq!(quality_grade(8), "SS+");
        assert_eq!(quality_grade(UNKNOWN_RANK), "UnKnow");
        assert_eq!(quality_grade(10), "");
    }
}
