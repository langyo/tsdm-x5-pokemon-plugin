# Game reliability and interface tests

## Behavior covered

| Area | Expected behavior | Verification |
| --- | --- | --- |
| Battle reconnect | An interrupted action retains its request ID; recovery reads the current battle; retry replays a committed result without repeating it. | PHP replay tests, MariaDB concurrency, Chromium lost-response scenarios |
| Database transactions | Money, inventory, HP, PP, events and action receipts commit or roll back together. | Independent PHP CGI processes against MariaDB |
| Purchase capacity | A bulk purchase writes its quantity once. Full or invalid inventory rejects the whole purchase without keeping the debit, including in non-strict SQL mode. | Both MariaDB SQL modes, final-capacity races and injected failures |
| Player mutations | Concurrent starter claims, item uses, releases and equipment changes preserve account and inventory limits. Failed writes roll back all effects. | MariaDB request races and injected database failures |
| Read repairs | A stale list or detail request cannot replace newer healing, battle damage, state or equipment-dependent HP values. Uncontested reads still repair invalid legacy values. | PHP snapshot regressions and MariaDB barriers around real CGI reads |
| Admin filters | All conditions combine regardless of order. Names resolve to matching IDs, exclusions retain their meaning, and Chinese sale flags match the selected value. | Actual admin dispatch and SQL against MariaDB |
| Admin inventory | Grants and edits preserve valid owners, item types, quantity limits and equipped references. Concurrent grants serialize, and failed writes roll back. | PHP integrity checks and MariaDB transactions |
| Admin Pokemon | Grants, edits and releases preserve one leader and the last Pokemon. Failed multi-table writes roll back; concurrent releases serialize with player actions. | Actual admin dispatch against MariaDB, failure injection and concurrent workers |
| Admin catalog identities | New records use database-generated IDs, including after deleting the highest record and during concurrent inserts. Existing references cannot silently attach to unrelated replacement records. | Actual catalog handlers against MariaDB in both SQL modes |
| Configuration capacity | Long announcement lists and quoted skill text survive storage. Old configuration columns expand before any settings are written; explicit upgrades can run again. | MariaDB strict and non-strict SQL modes, upgrade failure checks and legacy announcement reads |
| Configuration concurrency | Concurrent first reads preserve defaults and newer administrator values. Explicit saves still apply if another reader creates the key first. Only lock conflicts retry, with a fixed attempt limit. | MariaDB barriers and native Discuz error-contract regressions |
| Legacy export text | Archived X2 SQL strings retain whitespace, escapes, Unicode and NULL values. Invalid input leaves existing output intact. | Export CLI tests and original-versus-converted MariaDB imports |
| Admin saves and user selection | A pending save submits once, failed saves retain the draft, and completed saves release the loading state. Related Pokemon and inventory responses remain bound to the selected user. | Actual admin WASM with delayed requests and rejected saves |
| Maintenance | Either closed switch blocks player operations; named staff, authorized administration and public announcements retain access. | Actual router regressions and PHP CGI requests against MariaDB |
| Legacy announcements | The first config, admin or topics read migrates legacy notices. An explicitly cleared list stays empty; concurrent readers cannot overwrite a new administrator notice. | Admin input round trips and MariaDB barriers before migration inserts |
| Forum topics | The preview checks native forum permissions, passwords, paid access and group membership before reading thread metadata. Public announcements remain visible. | Actual topics handler, isolated forum tables and optional native Discuz permission helpers |
| Avatar compatibility | Standard and legacy two-digit UID filenames resolve in priority order. Unsupported sizes and missing files use safe fallbacks. | Actual avatar routes against isolated temporary forum directories |
| Skills and evolution | Concurrent learning preserves four unique skills. Learning and forgetting respect battles that start while waiting for a lock; the active Pokemon cannot evolve during battle. | MariaDB races and PHP mutation regressions |
| Experience rewards | Experience beyond the last threshold remains capped at level 100. Large rewards can reach the cap; responses match the level stored for legacy pets. Explicit zero gold remains zero. | Real experience helper and reward handler regression |
| Random Pokemon state | State, experience, HP and timestamps update together with valid SQL. Updates reject active battles and roll back failed writes. | Deterministic state transitions against MariaDB |
| Center treatment | Healing and abandoning a battle commit together; reconnect cannot restore the abandoned battle. | MariaDB strict SQL mode and Chromium success/failure scenarios |
| Names and input | Unicode names use character limits, quotes round-trip unchanged, malformed JSON returns 400, and unexpected errors hide server details. | Real PHP requests and persisted MariaDB values |
| PHP/Rust contracts | Real PHP responses deserialize into the Rust models used by the game. | Generated fixtures decoded by the Rust integration test |
| Battle reports | Players can open persisted turns and copy the existing BBCode report. | PHP event grouping and Chromium report/retry/clipboard scenarios |
| Inventory and storage | Item search runs before pagination; storage filters and ordering apply to the whole collection; hidden selections cannot be released. | PHP search tests, Rust filter tests, Chromium collections over 60 entries |
| Touch and keyboard items | Opening an item displays its details; only explicit confirmation consumes it; pending actions prevent double submission. | Chromium touch, Enter/Space and pending-request checks |
| Player navigation | Pending purchases, starter claims and skill changes cannot be submitted repeatedly. Switching pets resets equipment selection; equipment and skill load errors can retry. Profile and pet refreshes survive navigation, and old responses cannot replace fresh data. | Chromium player action scenarios |
| Navigation and equipment | Main navigation supports keyboard use, toasts expire across page changes, and badge reads cannot undo newer changes. Occupied inventory rows cannot be selected for reuse. Equipment operations finish after navigation and refresh even after a partially completed replacement. | Chromium delayed-request, keyboard and navigation scenarios |
| Encounter and damage rules | Encounters use exact map membership and reject disabled maps. Current rules respect type immunities and resistance; legacy rules retain their original behavior. | PHP encounter, endpoint and deterministic battle-core regression tests |
| Boss encounters | Same-species variants keep their configured identity after sorting and reconnect. Configured IVs apply, action responses retain Boss metadata, mixed maps allow ordinary encounters, and new Boss defaults match missing-field defaults. | PHP encounters and receipts, real CGI requests against MariaDB, Rust form/wire models and Chromium Boss choices |

The server protocol, compatibility behavior and retention rules are described in
[Battle request replay](battle-request-replay.md).

## Fast checks

```sh
python scripts/test/run.py
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo clippy -p _game --lib --target wasm32-unknown-unknown -- -D warnings
```

## Real MariaDB and contract checks

Requirements: MariaDB 10.11, PHP 8.3 or later with `mysqli`, PHP CGI, and Rust.
Use a disposable local database server. The configured database user needs
permission to create and drop databases. Each run generates its own
`tsdm_test_<random>` database, creates the real plugin schema, and removes only
that database in its cleanup block. No existing Discuz database is selected.

```sh
export TSDM_DB_HOST=127.0.0.1
export TSDM_DB_PORT=3306
export TSDM_DB_USER=root
export TSDM_DB_PASSWORD='<your-local-test-password>'
export TSDM_PHP_CGI=php-cgi
mkdir -p target
export TSDM_API_CONTRACT_FIXTURES="$PWD/target/api-contract-fixtures.json"
php scripts/test/live_database.php
php scripts/test/pokemon_state_database.php
php scripts/test/admin_filter_database.php
php scripts/test/admin_item_database.php
php scripts/test/admin_pokemon_database.php
php scripts/test/admin_catalog_database.php
php scripts/test/admin_identity_database.php
php scripts/test/config_concurrency_database.php
TSDM_TOPICS_REAL_DB=1 php scripts/test/topics_permissions.php
cargo test -p _utils --test php_api_contract --locked -- --ignored
```

On Windows, set the same environment variables in PowerShell, point
`TSDM_PHP_CGI` to `php-cgi.exe`, and pass `-c <test-php.ini>` to PHP if needed.
Workers inherit that configuration. The test uses the real API dispatcher,
request parsing, response hooks and endpoint SQL. A small Discuz boundary supplies
the fixture account and forwards database calls to `mysqli`; it does not test
forum authentication or the Discuz SQL safety parser.

The parent holds the account row lock until both worker connections reach their
own lock attempt. This checks actual database serialization. An injected failure
on receipt insertion verifies that preceding gameplay writes also roll back.
The CI job repeats these checks with an isolated MariaDB service and passes the
fresh PHP response file directly to Rust. The contract test is explicitly ignored
in the ordinary Rust suite because it requires those generated responses.

The random-state suite uses its own `tsdm_test_state_<random>` database and the
actual state handler with fixed random seeds. It checks persisted state changes,
battle restrictions, owner isolation and rollback after an injected write failure.

The admin suites use independent `tsdm_test_filters_<random>` and
`tsdm_test_admin_item_<random>` databases. They exercise the actual dispatcher,
filter queries, inventory transactions, equipped references and injected failure
rollback. The filter fixtures include combined conditions in both orders, names
shared by multiple rows, unmatched names and invalid operators.

The topics suite runs without a database by default; `TSDM_TOPICS_REAL_DB=1`
adds an isolated `tsdm_test_topics_<random>` database. Table adapters supply the
Discuz boundary. To also exercise unmodified native helpers from a local Discuz
checkout, set `TSDM_DISCUZ_FORUMPERM` to `helper_forumperm.php` and
`TSDM_DISCUZ_GROUP` to `function_group.php`. These optional checks do not install
the forum or verify its login flow.

Forums using legacy `formulaperm` rules are omitted from the game preview for
non-moderators because that native evaluator can exit with an HTML message.
Readers can open the forum directly to use its normal permission flow. Native
X5 rules stored in `viewperm` are evaluated by Discuz's own permission helper.
Configured public announcements are unaffected by this preview restriction.

## Seed and upgrade checks

With the same `TSDM_DB_*` environment variables, run:

```sh
php scripts/test/seed_upgrade_database.php
```

This test requires MariaDB and PHP with `mysqli`; it does not require PHP CGI,
Rust or an installed forum. It creates two independent databases named
`tsdm_test_seed_<random>`, records ownership only after successful creation, and
removes only those databases in its cleanup block.

The suite imports all 14 Docker seed files in order, including the minimal forum
tables needed by theme and plugin registration. It checks game references,
callable item modules and actual item evolution results. Separate populated
fixtures exercise complete and partial X3 migrations, preservation of already
migrated values, and repeated migration runs. It also verifies that the targeted
item evolution repair restores original seed rows, leaves customized rules
unchanged, and can be run again without changes.

This checks SQL and game data behavior; it does not install Discuz or test forum
authentication. For existing sites affected by the original item evolution seed,
see [Seed data repairs](../migrations/seed-fixes/README.md).

The archived X2 dump converter has a separate text-preservation check:

```sh
python scripts/test/x2_export_regressions.py
python scripts/test/x2_export_regressions.py --database
```

The second command uses the same `TSDM_DB_*` settings and requires PHP with
`mysqli`. Set `TSDM_PHP_EXECUTABLE` and `TSDM_PHP_INI` for a custom PHP runtime.
It imports both a generated legacy dump and the converted SQL into an isolated
database, then compares the stored values with independent expected values.
The converter accepts archived X2 fixed-column-order dumps; see its module
documentation for supported literals and SQL modes. Unsupported input fails
before replacing the requested output file.

## Browser checks

See [Game and admin browser regressions](../scripts/browser/README.md). These tests
load the actual compiled game and admin WASM with an isolated HTTP fixture server. The optional
live suite connects that WASM to the real PHP routes and MariaDB, then checks
purchases, item use, battle turns, reconnect and reports against stored values.
Both suites supply the Discuz login boundary and use no production player data.

The repository's complete Sass build requires the maintainer's ignored
`styles-core/variables` dependency. When it is unavailable, preserve the shipped
base CSS and compile only added rules that do not depend on those private
variables. Do not replace the shipped CSS with Sass's generated error page.
