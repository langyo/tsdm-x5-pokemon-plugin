<?php
/** Pet quality system regression: IV totals, legacy grade ladder, graded rolls and grant plumbing. */
error_reporting(E_ALL);
set_error_handler(function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/api/pokemon_utils.php';

$passed = 0;
function check($condition, $message)
{
    if (!$condition) throw new RuntimeException($message);
    $GLOBALS['passed']++;
}

// 旧版阶梯：E 0-9、D 10-69、C 70-99、B 100-119、A 120-149、S 150-169、
// SS 170-185、SS+ 恰好 186；历史溢出数据（>186）为 9=UnKnow
$names = [1 => 'E', 2 => 'D', 3 => 'C', 4 => 'B', 5 => 'A', 6 => 'S', 7 => 'SS', 8 => 'SS+', 9 => 'UnKnow'];
foreach ($names as $rank => $grade) {
    check(pm_quality_name($rank) === $grade, "Rank $rank should be grade $grade");
}
check(pm_quality_name(0) === '' && pm_quality_name(10) === '', 'Out-of-range ranks have no grade name');

check(pm_quality_thresholds() === [1 => 0, 2 => 10, 3 => 70, 4 => 100, 5 => 120, 6 => 150, 7 => 170, 8 => 186],
    'Thresholds follow the legacy individual-value ladder');
check(array_keys(pm_quality_grades()) === [1, 2, 3, 4, 5, 6, 7, 8, 9],
    'Grades cover E..SS+ plus UnKnow');

$boundaries = [
    [0, 1], [9, 1], [10, 2], [69, 2], [70, 3], [99, 3], [100, 4], [119, 4],
    [120, 5], [149, 5], [150, 6], [169, 6], [170, 7], [185, 7], [186, 8],
    [187, 9], [190, 9], [999, 9],
];
foreach ($boundaries as [$total, $rank]) {
    check(pm_quality_rank($total) === $rank, "Total IV $total should be rank $rank");
}
// 字符串入参先 intval，UnKnow 判定对字符串同样生效
check(pm_quality_rank('186') === 8 && pm_quality_rank('190') === 9, 'String totals are coerced with intval');

// 实例品质摘要
$pm = ['hpg' => 31, 'atkg' => 31, 'defg' => 31, 'spatkg' => 31, 'spdefg' => 31, 'sdg' => 31];
check(pm_quality_of_pokemon($pm) === ['iv_total' => 186, 'rank' => 8, 'grade' => 'SS+'],
    'A 186-total instance is exactly SS+');
$pm['sdg'] = 30;
check(pm_quality_of_pokemon($pm) === ['iv_total' => 185, 'rank' => 7, 'grade' => 'SS'],
    'A 185-total instance is SS');
$pm['sdg'] = 14;
check(pm_quality_of_pokemon($pm) === ['iv_total' => 169, 'rank' => 6, 'grade' => 'S'],
    'A 169-total instance is S');
$low = ['hpg' => 10, 'atkg' => 0, 'defg' => 0, 'spatkg' => 0, 'spdefg' => 0, 'sdg' => 0];
check(pm_quality_of_pokemon($low) === ['iv_total' => 10, 'rank' => 2, 'grade' => 'D'],
    'A 10-total instance is D');
$low['hpg'] = 9;
check(pm_quality_of_pokemon($low) === ['iv_total' => 9, 'rank' => 1, 'grade' => 'E'],
    'A 9-total instance is E');
// 历史溢出：个体值突破单项 31 / 总和 186 时归入 UnKnow
$legacy = ['hpg' => 40, 'atkg' => 31, 'defg' => 31, 'spatkg' => 31, 'spdefg' => 31, 'sdg' => 31];
check(pm_quality_of_pokemon($legacy) === ['iv_total' => 195, 'rank' => 9, 'grade' => 'UnKnow'],
    'A legacy 195-total instance is UnKnow');

// 按档位掷取：数值范围与档位归属必须始终成立
foreach ([1, 2, 3, 4, 5, 6, 7, 8] as $rank) {
    for ($i = 0; $i < 40; $i++) {
        $ivs = pm_roll_ivs_for_rank($rank);
        foreach (['hp', 'atk', 'def', 'spatk', 'spdef', 'speed'] as $key) {
            check($ivs[$key] >= 0 && $ivs[$key] <= 31, "Rolled $key stays within 0-31");
        }
        $total = array_sum($ivs);
        check(pm_quality_rank($total) === $rank, "Rolled total $total must belong to requested rank $rank");
        if ($rank === 8) {
            check($total === 186 && count(array_filter($ivs, fn($iv) => $iv === 31)) === 6,
                'SS+ rolls are always six 31s (total 186)');
        }
    }
}
// UnKnow（9）无法掷取：档位参数收敛到 1-8
foreach ([0, 9, 42] as $requested) {
    $ivs = pm_roll_ivs_for_rank($requested);
    check(count($ivs) === 6, 'Six IVs are rolled even for out-of-range ranks');
    $clamped = max(1, min(8, intval($requested)));
    check(pm_quality_rank(array_sum($ivs)) === $clamped,
        "Requested rank $requested clamps to rank $clamped");
}

// 发放链路：显式个体值必须原样落库，缺省时随机
$species = ['id' => 1, 'name' => 'Fixture', 'xs' => 'l', 'sex' => -1,
    'hp' => 50, 'atk' => 50, 'def' => 50, 'spatk' => 50, 'spdef' => 50, 'speed' => 50];
$explicit = pm_roll_ivs_for_rank(8);
check(array_sum($explicit) === 186, 'SS+ explicit roll totals the 186 cap');
$granted = create_new_pokemon_data($species, 10, 7, null, $explicit);
check([$granted['hpg'], $granted['atkg'], $granted['defg'], $granted['spatkg'], $granted['spdefg'], $granted['sdg']]
    === [$explicit['hp'], $explicit['atk'], $explicit['def'], $explicit['spatk'], $explicit['spdef'], $explicit['speed']],
    'Explicit IVs are stored verbatim on grant');
check(pm_quality_of_pokemon($granted) === ['iv_total' => 186, 'rank' => 8, 'grade' => 'SS+'],
    'Granted pet quality matches the requested SS+ rank');

$random = create_new_pokemon_data($species, 10, 7);
foreach (['hpg', 'atkg', 'defg', 'spatkg', 'spdefg', 'sdg'] as $column) {
    check($random[$column] >= 0 && $random[$column] <= 31, "Random grant keeps $column within 0-31");
}

echo "$passed pet quality assertions passed\n";
