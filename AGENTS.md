# AGENTS.md — Agent Rules for tsdm-x5-pokemon-plugin

> All AI agents and tools operating in this repository MUST follow these rules.
> Derived from the Celestia workspace convention (2026-09 snapshot), adapted to this
> repository. Workspace-infrastructure-specific rules (shared runners, NFS worktrees,
> proxy discipline) intentionally do **not** apply here and are omitted; everything
> below is binding.

---

## 1. Commit Message Format

```
<gitmoji> <Capitalized English summary ending with period.>
```

- Must start with one gitmoji. The whitelist is the one enforced by the CI linter
  (`celestia-devtools commit-msg-lint`): the gitmoji.dev canonical set plus the org
  additions 🔗 (symlink/copilot), 🔄 (sync/refresh), 📜 (license), 🛡️ (shield).
  Commonly used: ✨ 🐛 🔧 ♻️ 🔥 📝 🎨 ✅ 🚀 🌐 ⬆️ 🎉 📦.
- Summary in English, capitalized, ending with `.`. No CJK characters.
- No Conventional Commits prefixes (`feat:` / `fix:` / `chore(scope):` …). The
  gitmoji **is** the type marker.
- **No colon-prefix subjects of any shape** — not only `type: details` but also
  "Topic phrase: details" forms such as `♻️ Audit round 23: drop dead queries.` or
  `✨ Phase 6: Theme bridge.` are rejected by the CI linter (rule 7). Write the
  whole change as one plain sentence: `♻️ Drop dead queries after audit round 23.`
- The summary is ONE plain sentence. Detailed context belongs in the commit BODY
  (blank line + bullets), never in the summary line.
- Must not start with a bare version number or filler phrase (`0.3.0`, `Bump
  version`, `Update to` …).
- No `Merge branch xxx` subjects — this repo uses squash merges only;
  merge-commit subjects are **rejected** by the linter, not exempted.
- `Revert "..."` subjects (produced by `git revert`) are exempt from the gitmoji
  requirement.
- GitHub appends ` (#N)` when squashing; the linter strips that suffix before
  checking the trailing period.

### 1.1 CHANGELOG Policy

- **Never maintain a CHANGELOG / revision-history file in this repo.** The merged
  PRs ARE the changelog: the squash commit (gitmoji + one-sentence summary) plus
  the PR description form the complete change history, and `git log` serves any
  granularity. Hand-curated changelog files always drift stale.
- Release notes belong on the git tag + GitHub Releases page (written per
  release, not per commit), never in a tracked file.
- Do not recreate a CHANGELOG, and if a template or workflow references one,
  remove that reference in the same PR that touches it.

---

## 2. Repository Layout

| Path | Purpose |
|------|---------|
| `plugin/` | The Discuz X5 plugin itself (PHP): game logic, admin panel, API, i18n, images. `_x5-ref/` inside is read-only Discuz reference code — never modify it. |
| `rust/` | Rust/WASM frontends (`_admin`, `_game`, `_utils`), built into `plugin/wasm/` by `scripts/publish/build.py`. |
| `template/` | Discuz X5 page templates. |
| `migrations/from-x3/` | Sequential X3→X5 SQL migration scripts. |
| `docker/` | Dev-stack Dockerfile + MariaDB init SQL (`init.d/`). |
| `scripts/` | Python tooling: `docker/` (dev stack), `test/run.py` (static test suite), `publish/build.py` (WASM build + packaging), `migrate/`, `backup/`, `e2e/`. |
| `dist/` | Published X5-installable plugin zips (build output). |

---

## 3. PR Workflow

Every change — human or agent — lands through this exact pattern:

1. **Create a feature branch** off `master` (`feat/<name>` / `fix/<name>` /
   `chore/<name>` / `refactor/<name>`). Never commit directly on `master`. If you
   use git worktrees, keep them outside the main checkout and remove them after
   the branch is merged and deleted.
2. **3-Round Verify Cycle** for each change:
   - Round 1: Analyze → Improve → Verify
   - Round 2: Analyze again → Improve → Verify
   - Round 3: Final analysis → Polish → Verify
   - If ANY round fails, restart the 3-round count from zero.
3. **Commit** with the gitmoji message format (§1).
4. **Push** the branch.
5. **Create the PR** via `gh pr create`. The PR title follows the exact same
   rule as a commit subject (`<gitmoji> <one-sentence description.>`) —
   **without** any `(#<PR_ID>)` suffix: GitHub appends the number automatically
   when squashing (repo setting: commit-or-PR title), and a manually added
   number doubles it (see the `(#17) (#17)` subject of #17).
6. **Squash merge** (autonomous per §6). Never use a merge commit or rebase
   merge through the UI/API — squash only, even though the repo settings permit
   the other styles.
7. **Delete** the feature branch after merge.

### Agent Working Patterns

- All non-trivial tasks MUST use subagents (general or explore type) to prevent
  context pollution. Give each subagent a complete, self-contained brief: exact
  file paths, conventions (check existing code patterns first), verification
  criteria, and the commit message format.
- Launch independent sub-tasks as parallel subagent calls; chain sequential
  tasks (wait for the result, then proceed).
- Each subagent MUST verify its own work before returning; cross-verify with a
  different subagent where practical.
- Before committing, the relevant gates must pass locally:
  `just fmt-check`, `just lint`, `just test`, `just test-php` (§5).

---

## 4. Branch Naming

- `master` — production. Squash-merge only. Direct pushes prohibited.
- `feat/<name>` — new features.
- `fix/<name>` — bug fixes.
- `chore/<name>` — maintenance.
- `refactor/<name>` — restructuring without behavior change.

### Git Push Rules

- **NEVER use bare `git push --force`** without explicit human authorization.
  Hard rule, no exceptions for "convenience".
- Prefer `git push --force-with-lease` for rebase/amend recovery on feature
  branches.
- If `--force-with-lease` is rejected (stale remote-tracking ref), **STOP
  immediately and NEVER fall back to `--force`**: fetch the remote branch,
  review `git log origin/<branch>..HEAD` and `git log HEAD..origin/<branch>`,
  confirm no unaccounted commits, and ask the user before retrying any push.
- `--force` of any kind on `master` is **ABSOLUTELY FORBIDDEN**. Master only
  ever advances via squash merge.
- When in doubt, do not force push — create a new branch, re-commit, or ask.

---

## 5. Build & Test

Local gates (all must pass before committing):

- `just fmt-check` — Markdown formatting (`celestia-devtools format-markdown --check`)
- `cargo fmt --all -- --check` — Rust formatting
- `just lint` — `cargo clippy --workspace --all-targets -- -D warnings`
- `just test` — `cargo test --workspace` (live-stack integration tests are
  `#[ignore]`d; run them manually with `cargo test --workspace -- --ignored`
  while the dev stack is up)
- `just test-php` — static test suite: schema/PHP consistency, stale-column
  scan, seed validation, and `php -l` over every file in `plugin/`

Dev stack (needed only for live testing, not for the gates above):

- `just install` (once) — installs `celestia-devtools`
- `just up` / `just down` / `just logs` — Discuz X5 + MariaDB stack via Podman
- `just publish` — rebuild WASM frontends, package the X5-installable zip into
  `dist/`, bump the patch version (`VERSION`), tag and push

Version bumps belong in the feature/fix PR itself (or are applied automatically
by `just publish`); do NOT create standalone version-bump PRs.

---

## 6. Merge & Release Rules

- **PRs may be merged autonomously** (no per-PR human confirmation required),
  subject to §1, §4 and all of the following:
  1. **Message compliance**: the squash subject must be
     `<gitmoji> <one English sentence ending with .>` with no colon-prefix; the
     PR title follows the same rule.
  2. **Checks gate**: CI checks should pass before merging. This repo is a
     private repo on a GitHub Free plan — branch protection and required checks
     are unavailable, so the gate is enforced by this file, not by GitHub. A
     check that fails for a documented environment/infra reason (e.g. `pip
     install` of `celestia-devtools` failing on a network hiccup, runner
     queueing) may be bypassed only when the failure is recorded in the PR and
     the change passes the local gates (§5). **Never merge over a genuine code
     failure** (compile/test/lint).
  3. **PR economy**: do NOT create a PR per trivial change. A PR should bundle a
     meaningful batch of mergeable functionality. Small PRs are allowed only
     when there is genuinely nothing else to bundle (urgent hotfix, single
     rule/CLI change).
- Never create PRs outside an approved workflow: only create a PR when
  explicitly asked or as part of an approved work plan. Unsolicited PRs require
  permission.

---

## 7. Secrets Red Line

> Violating any item below is treated as an incident, on par with credential
> leakage.

1. **Never commit real passwords / keys / tokens / internal IPs** into the git
   tree — any branch, any file, including comments, examples, defaults, test
   data, README, docs, and dev-stack configs.
2. When code needs a credential:
   - use environment variables or untracked config files, or placeholders
     (`<your-password>` / `CHANGE_ME`);
   - use RFC 5737 documentation addresses (192.0.2.x / 198.51.100.x /
     203.0.113.x) and obviously fake values (`test-password` / `sk-xxx`) in
     examples.
3. If a real credential is genuinely required, **ask the user first**; never
   write it in on your own judgment.
4. **Pre-commit self-check**: for changes touching config / deployment / dev
   scripts / seed data, grep the diff for `password|secret|token|api_key` and
   confirm no real values before committing.
5. Credentials that live in workspace-local files outside the repo must never
   be copied into any repo file — including this one.
6. If a leak happens anyway:
   1. remove the credential from the current branch/PR immediately;
   2. assess the exposure (tags / branches / forks / clones);
   3. report to the user, who decides on history rewriting;
   4. regardless of rewriting, treat the credential as public and rotate it.

---

## 8. CI Policy

CI runs on GitHub-hosted runners (`ubuntu-latest`) — the Celestia self-hosted
runner fleet belongs to another organization and cannot serve this repository.
Three workflows exist:

- `ci.yml` — the **basic gate**: markdown format check, PHP syntax + static
  tests, Rust fmt/clippy/test. Triggers: `pull_request` types `[opened,
  reopened, ready_for_review]`, `push` to `master` (post-merge verification),
  and manual `workflow_dispatch`. PR-push rebuilds are intentionally NOT
  triggered (PR numbers and minutes are finite); use `workflow_dispatch`
  before merging when a re-run is needed. **A PR must not merge until every
  `ci.yml` job is green** (or a failure is demonstrably environmental, see
  rule 1 below).
- `e2e.yml` — the **full check**: boots the live dev stack
  (podman-compose, seeded DB, self-signed TLS) and runs the admin e2e suite
  (`scripts/e2e/run.py --http`) plus the game e2e suite
  (`cargo run --example e2e_full_test`). Cost control: it triggers ONLY on
  `push` to `master` and on manual `workflow_dispatch` — never on PRs or
  ordinary branch pushes. Dispatch it on a branch (`gh workflow run e2e.yml
  --ref <branch>`) when a pre-merge full run is warranted, e.g. for changes
  that touch the PHP API surface, the serde type models, or the stack itself.
- `commit-msg-lint.yml` — lints the PR title and every commit in the PR against
  §1 via `celestia-devtools commit-msg-lint`.

Operating rules:

1. **CI is a reference, not a gate**: before merging, check whether any check
   shows a **code-level failure** (compile/test/lint). If yes — fix it. If all
   failures are environmental (network, queueing) — record in the PR and merge
   after local gates pass. Do not stare at CI for more than ~15 minutes.
2. Redundant runs (e.g. superseded pushes) may be cancelled:
   `gh run cancel <id> --repo langyo/tsdm-x5-pokemon-plugin`.
3. `concurrency` + `cancel-in-progress` is configured in the workflows to reap
   superseded runs automatically.
