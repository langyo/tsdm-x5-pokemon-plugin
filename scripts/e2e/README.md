# E2E Tests

End-to-end test suite for Pokemon Plugin admin API.

## Usage

```bash
python scripts/e2e/run.py           # dispatch-level tests
python scripts/e2e/run.py --http    # additionally run HTTP-level tests
```

Requires Docker/Podman containers `tsdm-app` and `tsdm-db` to be running.
With `--http`, the app must serve HTTPS at `$E2E_BASE_URL`
(default `https://localhost:8443`).

## Test Coverage

Currently 63 dispatch-level tests plus the HTTP-level suite, covering:
- 9 of the 10 admin entities (pokemon, items, maps, evolutions, skills, config, users, etc.; `effect_data` is not yet covered)
- CRUD operations: count, list, get, set, filter
- Special operations: pokemon_info, item_info per-user queries
- Global config read/write

## Adding Tests

Add new test cases in `run.py` using the `test()` and `run_api()` helpers:

```python
test("label", run_api("count::entity"), expect_count=42)
test("label", run_api("list::entity", {"from":"0","count":"10"}))
```
