pub fn get_random_points_0_to_31() -> [u64; 6] {
    match try_get_random_points_0_to_31() {
        Ok(randoms) => randoms,
        Err(error) => {
            // RNG 失败不再静默返回全 0（会把全 0 个体值当正常数据发放），
            // 记录错误后保留确定性回退，调用方可在控制台看到根因。
            log::error!("获取随机数失败，回退为全 0: {}", error);
            [0; 6]
        }
    }
}

pub fn try_get_random_points_0_to_31() -> Result<[u64; 6], getrandom::Error> {
    let mut randoms: [u8; 6] = Default::default();
    getrandom::fill(&mut randoms)?;

    // 生成六个 0 - 31 的数字
    for item in &mut randoms {
        *item %= 32;
    }

    // 将六个数字转换为 u64
    let mut randoms_u64: [u64; 6] = Default::default();
    for (i, item) in randoms.iter().enumerate() {
        randoms_u64[i] = *item as u64;
    }

    Ok(randoms_u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_points_returns_six_elements() {
        let result = get_random_points_0_to_31();
        assert_eq!(result.len(), 6, "应该返回6个随机数");
    }

    #[test]
    fn test_get_random_points_range() {
        let result = get_random_points_0_to_31();
        for (i, &value) in result.iter().enumerate() {
            assert!(value <= 31, "随机数{}应该<=31,实际值: {}", i, value);
        }
    }

    #[test]
    fn test_get_random_points_multiple_calls_different() {
        // 多次调用应该产生不同的结果(概率上)
        let result1 = get_random_points_0_to_31();
        let result2 = get_random_points_0_to_31();

        // 注意: 这个测试有极小概率失败(连续两次生成完全相同的随机数)
        // 在实际运行中,这种情况几乎不可能发生
        let mut different_count = 0;
        for i in 0..6 {
            if result1[i] != result2[i] {
                different_count += 1;
            }
        }

        assert!(
            different_count >= 3,
            "多次调用应该产生不同的结果,只有{}个位置不同",
            different_count
        );
    }

    #[test]
    fn test_get_random_points_distribution() {
        // 生成1000组随机数,检查分布是否均匀
        let mut counts = [0u32; 32];
        let _iterations = 100;

        for _ in 0.._iterations {
            let result = get_random_points_0_to_31();
            for &value in &result {
                if value < 32 {
                    counts[value as usize] += 1;
                }
            }
        }

        // 每个值应该至少出现一次(在100次迭代中,6个值/次)
        // 这个测试也可能偶尔失败,但概率很低
        let min_count = *counts.iter().min().unwrap();
        assert!(
            min_count > 0,
            "随机数分布测试失败,最小出现次数: {}",
            min_count
        );
    }
}
