# Tasks: Native Dual-Language Library (v2.0)

**Feature**: Native Dual-Language Library  
**Branch**: `003-native-dual-lib`  
**Spec**: [spec.md](./spec.md)  
**Plan**: [plan.md](./plan.md)  
**Generated**: 2025-12-29

## Summary

| Metric | Value |
|--------|-------|
| Total Tasks | 48 |
| Setup Phase | 8 tasks |
| Foundational Phase | 6 tasks |
| US1 (P1) | 8 tasks |
| US2 (P1) | 4 tasks |
| US3 (P2) | 4 tasks |
| US4 (P2) | 4 tasks |
| US5 (P2) | 5 tasks |
| Polish Phase | 9 tasks |
| Parallel Opportunities | 20 tasks |

## User Story Mapping

| Story | Priority | Description | Independent Test |
|-------|----------|-------------|------------------|
| US1 | P1 | Platform-Agnostic Message Generation | Import and call `generateGoodbye()` in any environment |
| US2 | P1 | Consistent API Across Languages | Compare API signatures and behaviors between TS and Python |
| US3 | P2 | Specification-Driven Development | Modify corpus and verify both implementations update |
| US4 | P2 | Easy Installation Without Build Tools | Install from fresh environment without Rust/clang |
| US5 | P2 | Unified Test Suite | Run both test suites and compare coverage |

---

## Phase 1: Setup

**Goal**: Restructure the repository for dual-package architecture.

- [ ] T001 Create packages directory structure at `/packages/typescript/src/` and `/packages/python/src/joyous_departures/`
- [ ] T002 [P] Create packages/typescript/tests/ directory
- [ ] T003 [P] Create packages/python/tests/ directory
- [ ] T004 [P] Archive Rust artifacts by moving `joy-generator/` to `_archive/joy-generator/`
- [ ] T005 [P] Archive bindings by moving `bindings/` to `_archive/bindings/`
- [ ] T006 [P] Archive Cargo files by moving `Cargo.toml` and `Cargo.lock` to `_archive/`
- [ ] T007 Update `.gitignore` for new packages structure at `/.gitignore`
- [ ] T008 Create root `pnpm-workspace.yaml` for monorepo support at `/pnpm-workspace.yaml`

**Completion Criteria**: Directory structure matches plan.md specification.

---

## Phase 2: Foundational

**Goal**: Implement the code generator that both implementations depend on.

**Blocking**: All user story phases depend on this phase.

- [ ] T009 Create code generator script at `/scripts/generate-corpus.ts`
- [ ] T010 Implement parseCorpus function to parse and deduplicate messages in `/scripts/generate-corpus.ts`
- [ ] T011 Implement generateTypeScript function to output const array in `/scripts/generate-corpus.ts`
- [ ] T012 Implement generatePython function to output tuple in `/scripts/generate-corpus.ts`
- [ ] T013 Run generator and commit initial generated files to `/packages/typescript/src/corpus.generated.ts` and `/packages/python/src/joyous_departures/corpus.py`
- [ ] T014 Update pre-commit hook to run generator in `/.githooks/pre-commit`

**Completion Criteria**: Running `npx tsx scripts/generate-corpus.ts` generates valid corpus files for both languages.

---

## Phase 3: User Story 1 - Platform-Agnostic Message Generation (P1)

**Goal**: Implement core message generation that works in all environments.

**Independent Test**: Import library and call `generateGoodbye()` in Node.js, Deno, browser, and Cloudflare Workers.

### TypeScript Implementation

- [ ] T015 [P] [US1] Create TypeScript types (TemplateArgs, GoodbyeOptions) in `/packages/typescript/src/types.ts`
- [ ] T016 [P] [US1] Implement template substitution utility in `/packages/typescript/src/templates.ts`
- [ ] T017 [P] [US1] Implement emoji stripping utility in `/packages/typescript/src/emoji.ts`
- [ ] T018 [US1] Implement generateGoodbye function in `/packages/typescript/src/index.ts`

### Python Implementation

- [ ] T019 [P] [US1] Implement template substitution utility in `/packages/python/src/joyous_departures/templates.py`
- [ ] T020 [P] [US1] Implement emoji stripping utility in `/packages/python/src/joyous_departures/emoji.py`
- [ ] T021 [US1] Implement generate_goodbye_sync function in `/packages/python/src/joyous_departures/__init__.py`
- [ ] T022 [US1] Implement async generate_goodbye wrapper in `/packages/python/src/joyous_departures/__init__.py`

**Completion Criteria**: Both `generateGoodbye()` (TS) and `generate_goodbye_sync()` (Python) return valid messages.

---

## Phase 4: User Story 2 - Consistent API Across Languages (P1)

**Goal**: Ensure API semantics are identical between TypeScript and Python.

**Independent Test**: Compare function signatures, option names, and behavior for edge cases.

**Note**: Timezone handling is included in T016/T019 (templates utilities). Validation is separate.

- [ ] T023 [US2] Implement input validation (truncation) for templateArgs in `/packages/typescript/src/validation.ts`
- [ ] T024 [P] [US2] Implement input validation (truncation) for template_args in `/packages/python/src/joyous_departures/validation.py`
- [ ] T025 [US2] Implement generateGoodbyeAsync for translator support in `/packages/typescript/src/index.ts`
- [ ] T026 [US2] Verify API parity by comparing function signatures in `/specs/003-native-dual-lib/contracts/`

**Completion Criteria**: Both implementations accept the same options and produce equivalent outputs.

---

## Phase 5: User Story 3 - Specification-Driven Development (P2)

**Goal**: Enable corpus changes to automatically update both implementations.

**Independent Test**: Add a message to corpus, run generator, verify tests pass.

- [ ] T027 [US3] Create shared test case definitions at `/specs/003-native-dual-lib/test-cases.json`
- [ ] T028 [P] [US3] Document generator workflow in `/scripts/README.md`
- [ ] T029 [P] [US3] Add corpus validation to generator (non-empty, no duplicates) in `/scripts/generate-corpus.ts`
- [ ] T030 [US3] Add CI job to verify generated files are up-to-date in `/.github/workflows/ci.yml`

**Completion Criteria**: Modifying `corpus/en-GB.txt` and running generator updates both implementations.

---

## Phase 6: User Story 4 - Easy Installation (P2)

**Goal**: Enable zero-dependency installation from package registries.

**Independent Test**: Install from npm/PyPI in clean environment without build tools.

- [ ] T031 [US4] Create package.json with correct exports and metadata in `/packages/typescript/package.json`
- [ ] T032 [P] [US4] Create tsconfig.json for ESM/CJS dual output in `/packages/typescript/tsconfig.json`
- [ ] T033 [P] [US4] Create pyproject.toml with hatchling build system in `/packages/python/pyproject.toml`
- [ ] T034 [US4] Verify zero runtime dependencies in both package manifests

**Completion Criteria**: `pnpm add` and `uv pip install` work without compilation.

---

## Phase 7: User Story 5 - Unified Test Suite (P2)

**Goal**: Implement comprehensive tests for both implementations.

**Independent Test**: Run both test suites and verify >90% coverage.

- [ ] T035 [US5] Create TypeScript test suite (corpus, generator, templates) in `/packages/typescript/tests/`
- [ ] T036 [P] [US5] Create Python test suite (corpus, generator, templates) in `/packages/python/tests/`
- [ ] T037 [US5] Create test runner script for both languages at `/scripts/test.sh`
- [ ] T038 [US5] Create Cloudflare Workers integration test at `/packages/typescript/tests/workers.test.ts` (validates SC-001)
- [ ] T039 [P] [US5] Add performance benchmark test to verify <10ms generation at `/packages/typescript/tests/benchmark.test.ts` (validates SC-004)

**Completion Criteria**: Both test suites pass with >90% coverage; Cloudflare Workers test passes.

---

## Phase 8: Polish & Cross-Cutting

**Goal**: Finalize CI/CD, documentation, and verification.

- [ ] T040 Update CI workflow for new package structure in `/.github/workflows/ci.yml`
- [ ] T041 [P] Update publish workflow for npm and PyPI in `/.github/workflows/publish.yml`
- [ ] T042 [P] Update README.md with v2.0 installation and usage at `/README.md`
- [ ] T043 Create CHANGELOG.md with v2.0.0 release notes at `/CHANGELOG.md`
- [ ] T044 [P] Create MIGRATION.md for v1.x → v2.0 migration guide at `/MIGRATION.md`
- [ ] T045 Add package size verification script at `/scripts/check-size.sh` (validates SC-005: <100KB TS, <50KB Python)
- [ ] T046 [P] Add Cloudflare Workers CI job with wrangler dry-run in `/.github/workflows/ci.yml`
- [ ] T047 Run final verification: all tests pass, package sizes meet targets
- [ ] T048 Tag v2.0.0 release after all checks pass

**Completion Criteria**: CI passes, documentation complete, package sizes verified, Cloudflare Workers validated.

---

## Dependency Graph

```
Phase 1 (Setup: T001-T008)
    │
    ▼
Phase 2 (Foundational: T009-T014)
    │
    ├──────────────────┐
    ▼                  ▼
Phase 3 (US1)      Phase 4 (US2)
T015-T022          T023-T026
    │                  │
    └────────┬─────────┘
             ▼
    Phase 5 (US3: T027-T030)
             │
             ▼
    Phase 6 (US4: T031-T034)
             │
             ▼
    Phase 7 (US5: T035-T039)
    [Includes Cloudflare Workers test]
             │
             ▼
    Phase 8 (Polish: T040-T048)
    [CI/CD, docs, size verification]
```

## Parallel Execution Opportunities

### Within Phase 1 (Setup)
Tasks T002-T006 can run in parallel after T001.

### Within Phase 3 (US1)
TypeScript tasks (T015-T018) can run in parallel with Python tasks (T019-T022).

### Within Phase 4 (US2)
T023 (TS validation) can run in parallel with T024 (Python validation).

### Within Phase 7 (US5)
T035 (TS tests), T036 (Python tests), T039 (benchmark) can run in parallel.

### Within Phase 8 (Polish)
T041, T042, T044, T046 can run in parallel.

### Across Phases
- Phase 3 (US1) and Phase 4 (US2) can start in parallel after Phase 2
- US3, US4, US5 form a sequential chain (each builds on previous)

## Implementation Strategy

### MVP Scope (Minimum Viable Product)
**Phases 1-3 only**: Repository reorganization, code generator, and basic message generation in both languages.

**MVP Deliverables**:
- Working `generateGoodbye()` in TypeScript
- Working `generate_goodbye_sync()` in Python
- Both using generated corpus constants

**Estimated Duration**: 4 days

### Incremental Delivery
1. **Week 1**: Phases 1-3 (MVP)
2. **Week 1-2**: Phases 4-5 (API parity, spec-driven)
3. **Week 2**: Phases 6-8 (Installation, tests, polish)

---

## Task Execution Checklist

Use this checklist to track progress:

```
Setup Phase:          □ □ □ □ □ □ □ □  (8 tasks: T001-T008)
Foundational Phase:   □ □ □ □ □ □      (6 tasks: T009-T014)
US1 - Generation:     □ □ □ □ □ □ □ □  (8 tasks: T015-T022)
US2 - API Parity:     □ □ □ □          (4 tasks: T023-T026)
US3 - Spec-Driven:    □ □ □ □          (4 tasks: T027-T030)
US4 - Installation:   □ □ □ □          (4 tasks: T031-T034)
US5 - Test Suite:     □ □ □ □ □        (5 tasks: T035-T039)
Polish Phase:         □ □ □ □ □ □ □ □ □ (9 tasks: T040-T048)
                      ─────────────────
Total:                                 (48 tasks)
```

---

## Next Steps

1. Begin **Phase 1: Setup** with task T001
2. After T001 completes, parallelize T002-T006
3. Proceed to **Phase 2: Foundational** (code generator)
4. Once generator works, start **Phase 3** and **Phase 4** in parallel

**Suggested First Command**:
```bash
mkdir -p packages/typescript/src packages/typescript/tests packages/python/src/joyous_departures packages/python/tests
```

