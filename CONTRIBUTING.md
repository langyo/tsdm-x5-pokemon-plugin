# Contributing to TSDM Pokemon Plugin

> AI agents: read [AGENTS.md](AGENTS.md) first — it carries the full binding rule
> set (commit format, PR workflow, merge policy, secrets red line, CI policy).
> This guide covers the same conventions for human contributors.

## Branch Strategy

- `master` — stable, deployable branch. All changes land here via **squash-merge
  PR**. Direct pushes to `master` are prohibited.
- Feature branches — created from `master`, named `feat/<description>`,
  `fix/<description>`, `chore/<description>`, or `refactor/<description>`.

## Merge Policy

**All changes must be merged into `master` via Pull Request.** Direct pushes to
`master` are prohibited.

A PR may be merged (squash) once all of the following hold:

1. It targets the `master` branch.
2. Its title follows the commit message convention below, with the `(#<PR_ID>)`
   suffix appended once the number is known.
3. The local gates pass: `just fmt-check`, `just lint`, `just test`,
   `just test-php`.
4. CI shows no code-level failure (compile/test/lint). Purely environmental
   failures (network hiccups, queueing) may be recorded in the PR and waived
   after the local gates pass — see AGENTS.md §6/§8.

Squash merge only — merge commits and rebase merges are not used, even though
the repository settings permit them. Delete your feature branch after merging.

## Commit Message Convention

All commits follow the [gitmoji](https://gitmoji.dev) convention based on
Celestia Island standards:

```
<gitmoji> <Capitalized English summary.>
```

| Rule | Example (pass) | Example (fail) |
|---|---|---|
| Starts with a gitmoji | `🐛 Fix the parser crash.` | `Fix the parser crash.` |
| No Conventional Commits prefix | `🐛 Fix the parser crash.` | `🐛 fix: crash.` |
| No colon-prefix subject of any shape | `♻️ Drop dead queries after audit round 23.` | `♻️ Audit round 23: drop dead queries.` |
| First letter after emoji is uppercase | `🐛 Fix the parser crash.` | `🐛 fix the parser crash.` |
| Ends with a period `.` | `🐛 Fix the parser crash.` | `🐛 Fix the parser crash` |
| English only | `🐛 Fix the parser crash.` | `🐛 修复崩溃。` |
| Must not start with version number or filler | `⬆️ Upgrade dependencies.` | `⬆️ 0.3.0` / `⬆️ Bump version to 0.3.0.` |
| One plain sentence; details go in the body | `✨ Add evolution chain data table.` | `✨ Phase 6: Add evolution chain data table.` |

The summary is ONE sentence describing the change; detailed context belongs in
the commit body (blank line + bullets), never in the subject line. The gitmoji
already conveys the change type — never repeat it as a `type:` prefix.

### PR Title (becomes the squash merge commit)

PR titles follow the exact same rule as commit summaries — **without** any PR
number suffix:

```
<gitmoji> <Capitalized English summary.>
```

Example: `✨ Add evolution chain data table.`

GitHub appends ` (#N)` automatically when squashing (repo setting: commit-or-PR
title). Never write the number yourself — a manual `(#N)` produces a doubled
`(#N) (#N)` subject (observed in #17). The linter tolerates both forms, but
the convention is: no number in the title.

### Exemptions

- `Revert "..."` commits (produced by `git revert`) are automatically exempt.
- `Merge branch ...` / `Merge pull request ...` subjects are **rejected** — this
  repo squash-merges only, so a merge-commit subject is a violation, not an
  exemption.

### Enforcement

The rules above are enforced in CI by
[`celestia-devtools commit-msg-lint`](https://github.com/celestia-island/celestia-devtools)
(PR title + every commit in the PR). To install the local commit-msg hook:

```bash
just install
just commit-msg-hook-install
```

### Quick Reference

| Gitmoji | When to use |
|---|---|
| ✨ | New feature |
| 🐛 | Bug fix |
| 📝 | Documentation |
| ♻️ | Refactor |
| 🎨 | Format / code style |
| ✅ | Add or update tests |
| 🔧 | Configuration changes |
| 🚀 | Deploy / release / initial commit |
| ⬆️ | Upgrade dependencies |
| 🔥 | Remove code or files |
| 🚑 | Critical hotfix |
| 🔨 | Development scripts or tooling |
| 🌐 | Internationalization |
| 💡 | Add or update comments |

## Changelog

This repository deliberately maintains **no CHANGELOG file**. The merged PRs are
the changelog (squash subject + PR description); release notes live on git tags
and the GitHub Releases page. Do not add a changelog file or reference one from
templates.

## Development Setup

```bash
# Install tools (celestia-devtools: markdown formatting + commit-msg lint)
just install

# Format code (Markdown + Rust)
just fmt

# Check formatting without writing
just fmt-check

# Lint Rust (clippy, warnings as errors)
just lint

# Run Rust tests (live-stack integration tests are #[ignore]d)
just test

# Run static PHP tests (schema consistency, stale columns, php -l)
just test-php

# Start the full dev stack (Discuz X5 + MariaDB + plugin auto-install)
just up
```

## Database Migrations

Migration scripts live in `migrations/from-x3/`:

1. `001_x3_to_x5_migration.sql` — engine/charset upgrade, table normalization
   (idempotent; safe to re-run)

Always back up the database before running migration scripts.
