<?php
/** Pet quality system regression: IV totals, grade thresholds, graded rolls and grant plumbing. */
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

// 档位命名与边界（阈值 0/60/84/105/125/145/160/172，满值 186）
$names = [1 => 'E', 2 => 'D', 3 => 'C', 4 => 'B', 5 => 'A', 6 => 'S', 7 => 'SS', 8 => 'SS+'];
foreach ($names as $rank => $grade) {
    check(pm_quality_name($rank) === $grade, "Rank $rank should be grade $grade");
}
check(pm_quality_name(0) === '' && pm_quality_name(9) === '', 'Out-of-range ranks have no grade name');

$boundaries = [
    [0, 1], [59, 1], [60, 2], [83, 2], [84, 3], [104, 3], [105, 4], [124, 4],
    [125, 5], [144, 5], [145, 6], [159, 6], [160, 7], [171, 7], [172, 8], [186, 8],
];
foreach ($boundaries as [$total, $rank]) {
    check(pm_quality_rank($total) === $rank, "Total IV $total should be rank $rank");
}

// 实例品质摘要
$pm = ['hpg' => 31, 'atkg' => 31, 'defg' => 31, 'spatkg' => 31, 'spdefg' => 31, 'sdg' => 17];
check(pm_quality_of_pokemon($pm) === ['iv_total' => 172, 'rank' => 8, 'grade' => 'SS+'],
    'A 172-total instance is exactly SS+');
$pm['sdg'] = 16;
check(pm_quality_of_pokemon($pm)['grade'] === 'SS', '171 totals fall back to SS');

// 按档位掷取：数值范围与档位归属必须始终成立
foreach ([1, 2, 3, 4, 5, 6, 7, 8] as $rank) {
    for ($i = 0; $i < 40; $i++) {
        $ivs = pm_roll_ivs_for_rank($rank);
        foreach (['hp', 'atk', 'def', 'spatk', 'spdef', 'speed'] as $key) {
            check($ivs[$key] >= 0 && $ivs[$key] <= 31, "Rolled $key stays within 0-31");
        }
        $total = array_sum($ivs);
        check(pm_quality_rank($total) === $rank, "Rolled total $total must belong to requested rank $rank");
    }
}

// 发放链路：显式个体值必须原样落库，缺省时随机
$species = ['id' => 1, 'name' => 'Fixture', 'xs' => 'l', 'sex' => -1,
    'hp' => 50, 'atk' => 50, 'def' => 50, 'spatk' => 50, 'spdef' => 50, 'speed' => 50];
$explicit = pm_roll_ivs_for_rank(8);
$granted = create_new_pokemon_data($species, 10, 7, null, $explicit);
check([$granted['hpg'], $granted['atkg'], $granted['defg'], $granted['spatkg'], $granted['spdefg'], $granted['sdg']]
    === [$explicit['hp'], $explicit['atk'], $explicit['def'], $explicit['spatk'], $explicit['spdef'], $explicit['speed']],
    'Explicit IVs are stored verbatim on grant');
check(pm_quality_of_pokemon($granted)['rank'] === 8, 'Granted pet quality matches the requested rank');

$random = create_new_pokemon_data($species, 10, 7);
foreach (['hpg', 'atkg', 'defg', 'spatkg', 'spdefg', 'sdg'] as $column) {
    check($random[$column] >= 0 && $random[$column] <= 31, "Random grant keeps $column within 0-31");
}

echo "$passed pet quality assertions passed\n";
