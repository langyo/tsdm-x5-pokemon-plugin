<?php
/** Pet sidebar badge rendering regression (pet_side_html.php). */
define('IN_DISCUZ', true);
require __DIR__ . '/../../plugin/pet_side_html.php';

$failures = 0;
function assert_same($expected, $actual, $label)
{
    global $failures;
    if ($expected !== $actual) {
        $failures++;
        echo "FAIL: $label\n  expected: " . var_export($expected, true) . "\n  actual:   " . var_export($actual, true) . "\n";
    }
}

function assert_contains($needle, $haystack, $label)
{
    global $failures;
    if (strpos($haystack, $needle) === false) {
        $failures++;
        echo "FAIL: $label\n  missing: $needle\n";
    }
}

// --- 图片回退链 ---
assert_same(
    ['https://img.tsdm39.com/Pokemon/pm/25.gif', 'source/plugin/pokemon/images/pm/25.png'],
    pm_side_big_chain(25, 0),
    'big chain normal uses CDN pm gif then local png'
);
assert_same(
    ['https://img.tsdm39.com/Pokemon/pmsg/25.gif', 'source/plugin/pokemon/images/pmsg/25.png'],
    pm_side_big_chain(25, 1),
    'big chain shiny uses CDN pmsg gif then local png'
);
assert_same(
    ['source/plugin/pokemon/images/spm/20200.gif', 'https://img.tsdm39.com/Pokemon/spm/20200.gif'],
    pm_side_small_chain(20200),
    'small chain uses local spm first then CDN spm'
);

// --- 数据形态兼容：X5 species_id/nickname 与 X3 pmno/nowname ---
assert_same(25, pm_side_species(['species_id' => '25']), 'X5 species_id shape');
assert_same(31, pm_side_species(['pmno' => '31']), 'X3 pmno shape');
assert_same(0, pm_side_species([]), 'missing species falls back to 0');
assert_same('皮卡丘', pm_side_name(['nickname' => '皮卡丘']), 'X5 nickname shape');
assert_same('小火龙', pm_side_name(['nowname' => '小火龙']), 'X3 nowname shape');

// --- 徽章渲染 ---
$badge = pm_side_badge_html([
    'first' => ['id' => 11, 'species_id' => 25, 'nickname' => '皮卡丘', 'level' => 50, 'is_shiny' => 0],
    'creeps' => [
        ['id' => 12, 'species_id' => 4, 'nickname' => '小火龙', 'level' => 12, 'is_shiny' => 1],
        ['id' => 13, 'pmno' => 7, 'nowname' => '杰尼龟', 'level' => 8],
    ],
]);
assert_contains('class="pm-side-badge"', $badge, 'badge wrapper present');
assert_contains('data-petid="11"', $badge, 'first pet carries petid');
assert_contains('https://img.tsdm39.com/Pokemon/pm/25.gif', $badge, 'first pet uses CDN big gif');
assert_contains('data-petid="12"', $badge, 'creep pet carries petid');
assert_contains('【闪】小火龙 Lv:12', $badge, 'shiny creep title has 闪 prefix');
assert_contains('data-pm-chain-lg="[&quot;https://img.tsdm39.com/Pokemon/pmsg/4.gif', $badge,
    'shiny creep large chain uses pmsg');
assert_contains('source/plugin/pokemon/images/spm/7.gif', $badge, 'creep small chain uses local spm');
assert_contains('pm-size-toggle', $badge, 'size toggle rendered when creeps exist');
assert_contains('data-pm-size="lg"', $badge, 'large size button present');
assert_contains('data-petid="13"', $badge, 'X3-legacy creep rendered');

// 闪光首发宠物：标题前缀 + 名字行「闪」徽标，大图走 pmsg
$shiny_first = pm_side_badge_html([
    'first' => ['id' => 14, 'species_id' => 25, 'nickname' => '皮卡丘', 'level' => 50, 'is_shiny' => 1],
]);
assert_contains('title="【闪】皮卡丘 Lv:50"', $shiny_first, 'shiny first title has 闪 prefix');
assert_contains('<span class="pm-mark pm-mark-shiny" title="异色（闪光）宠物">闪</span>',
    $shiny_first, 'shiny first name row carries 闪 badge');
assert_contains('https://img.tsdm39.com/Pokemon/pmsg/25.gif', $shiny_first,
    'shiny first uses CDN pmsg big gif');

// 无跟随宠物时不渲染切换按钮，但首发宠物仍在
$first_only = pm_side_badge_html([
    'first' => ['id' => 11, 'species_id' => 25, 'nickname' => '皮卡丘', 'level' => 50],
]);
assert_contains('pm-first', $first_only, 'first pet rendered');
if (strpos($first_only, 'pm-mark') !== false) {
    $failures++;
    echo "FAIL: normal pet must not carry the 闪 badge\n";
}
if (strpos($first_only, 'pm-size-toggle') !== false) {
    $failures++;
    echo "FAIL: size toggle must be hidden without creeps\n";
}

// 特殊字符昵称：title/属性需要正确转义
$quoted = pm_side_badge_html([
    'creeps' => [['id' => 1, 'species_id' => 25, 'nickname' => 'a"b<c', 'level' => 1]],
]);
assert_contains('title="a&quot;b&lt;c Lv:1"', $quoted, 'nickname escaped in title attribute');
assert_contains('alt="a&quot;b&lt;c Lv:1"', $quoted, 'nickname escaped in alt attribute');

// 空数据 / 脏数据
assert_same('', pm_side_badge_html([]), 'empty badge data renders nothing');
assert_same('', pm_side_badge_html(['first' => ['level' => 3], 'creeps' => [['nickname' => 'x']]]),
    'pets without species render nothing');

// --- 资产输出：样式与脚本一次性注入 ---
$assets = pm_side_assets();
assert_contains('window.__pmSideInit', $assets, 'assets embed the guard script');
assert_contains('.pm-size-toggle', $assets, 'assets embed toggle styles');
assert_contains('action=card', $assets, 'popup fetches the card endpoint');
assert_contains('.pm-mark', $assets, 'assets embed 闪 badge styles');
assert_contains('.pm-mark-shiny', $assets, 'assets embed shiny badge style');
assert_contains('data-pm-chain', $assets, 'assets embed fallback-chain logic');

if ($failures) {
    echo "pet side badge regression: $failures failure(s)\n";
    exit(1);
}
echo "pet side badge regression: all assertions passed\n";
