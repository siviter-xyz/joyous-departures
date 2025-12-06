# Tasks: Goodbye Message Generator

**Input**: Design documents from `/specs/001-goodbye-generator/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are REQUIRED per specification - unit tests in Rust, E2E tests in Python/TypeScript with benchmarks.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

Based on plan.md structure:
- Rust core: `joy-generator/`
- Python bindings: `bindings/python/`
- TypeScript bindings: `bindings/typescript/`
- Corpus: `corpus/en-GB.txt`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create Rust workspace structure with Cargo.toml at repository root
- [x] T002 [P] Create `joy-generator/` directory structure with Cargo.toml
- [x] T003 [P] Create `bindings/python/` directory structure with Cargo.toml and pyproject.toml
- [x] T004 [P] Create `bindings/typescript/` directory structure with Cargo.toml, package.json, tsconfig.json
- [x] T005 [P] Create `corpus/` directory and create empty `corpus/en-GB.txt` file (will be populated in Phase 5)
- [x] T006 [P] Create `.github/workflows/` directory structure
- [x] T007 [P] Create `scripts/` directory with placeholder scripts
- [x] T008 [P] Initialize README.md with project overview

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T009 [US1] Create corpus loading infrastructure in `joy-generator/src/corpus.rs`
  - Read corpus/en-GB.txt at build time
  - Compress corpus using lz4
  - Embed compressed corpus in binary using include_bytes!
  - Decompress at runtime on first use
  - Cache decompressed corpus in Arc for thread-safety

- [x] T010 [US1] Create message template data structures in `joy-generator/src/corpus.rs`
  - MessageTemplate struct with template string, language_code, template_variables, has_emojis
  - MessageCorpus struct with messages Vec, compressed_data, language_index, initialized flag

- [x] T011 [US1] Create options structures in `joy-generator/src/options.rs`
  - CoreGoodbyeOptions struct with language_code, template_args, use_emojis, timezone
  - Default implementation with "en-GB", "Europe/London" defaults

- [x] T012 [US1] Create error types in `joy-generator/src/error.rs`
  - GoodbyeError enum with CorpusLoadError, InvalidLanguageCodeError, TemplateVariableError
  - Use thiserror for error handling

- [x] T013 [US1] Create datetime module in `joy-generator/src/datetime.rs`
  - Timezone handling using chrono-tz
  - Default date/time generation for {date} and {time} template variables
  - Format: YYYY-MM-DD for date, HH:MM for time

- [x] T014 [US1] Create template variable replacement in `joy-generator/src/template.rs`
  - Template variable detection and replacement
  - Support for {name}, {location}, {date}, {time}
  - Default value application: name="Good Soul", location="The World"
  - Date/time generation from timezone

- [x] T015 [US1] Create emoji handling in `joy-generator/src/emoji.rs`
  - Emoji detection using unicode-emoji crate
  - Emoji stripping functionality when use_emojis=false
  - Emoji position tracking for efficient processing

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Generate Basic Goodbye Message (Priority: P1) 🎯 MVP

**Goal**: Generate a random warm sign-off message from corpus with default values

**Independent Test**: Import library, call generate function with no arguments, receive valid goodbye message string

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T016 [P] [US1] Unit test for corpus loading in `joy-generator/tests/unit/corpus_test.rs`
  - Test corpus decompression
  - Test corpus initialization
  - Test message count (360 messages)

- [ ] T017 [P] [US1] Unit test for message generation in `joy-generator/tests/unit/generate_test.rs`
  - Test basic generation with defaults
  - Test randomness (multiple calls produce different results)
  - Test message format validation

- [ ] T018 [P] [US1] Benchmark test for generation speed in `joy-generator/benches/generate_bench.rs`
  - Measure generation time (target: <10ms)
  - Measure corpus loading time
  - Include results in README

- [ ] T019 [P] [US1] E2E test for Python bindings in `bindings/python/tests/test_e2e.py`
  - Test basic generate_goodbye() call
  - Test randomness verification
  - Benchmark performance

- [ ] T020 [P] [US1] E2E test for TypeScript bindings in `bindings/typescript/tests/e2e.test.ts`
  - Test basic generateGoodbye() call
  - Test randomness verification
  - Benchmark performance

### Implementation for User Story 1

- [x] T021 [US1] Implement core generation logic in `joy-generator/src/generate.rs`
  - Random message selection from corpus
  - Filter by language_code (default "en-GB")
  - Return selected message template

- [x] T022 [US1] Implement public API in `joy-generator/src/lib.rs`
  - Export generate_goodbye function
  - Export CoreGoodbyeOptions
  - Export GoodbyeError
  - Module organization

- [x] T023 [US1] Implement Python bindings in `bindings/python/src/lib.rs`
  - PyO3 setup and module definition
  - Wrap generate_goodbye function
  - Convert Python options to CoreGoodbyeOptions
  - Error handling and conversion

- [x] T024 [US1] Configure Python package in `bindings/python/pyproject.toml`
  - Package metadata (name, version, description)
  - Maturin build configuration
  - Dependencies

- [x] T025 [US1] Implement TypeScript bindings WASM setup in `bindings/typescript/`
  - Configure wasm-pack in Cargo.toml
  - Create WASM-compatible wrapper
  - Build script for WASM compilation

- [x] T026 [US1] Implement TypeScript wrapper in `bindings/typescript/src/index.ts`
  - Import WASM module
  - Wrap generate_goodbye function
  - Convert TypeScript options to CoreGoodbyeOptions
  - Error handling

- [x] T027 [US1] Create TypeScript type definitions in `bindings/typescript/src/index.d.ts`
  - GoodbyeOptions interface
  - GoodbyeTemplateArgs interface
  - Function signatures

- [x] T028 [US1] Configure TypeScript package in `bindings/typescript/package.json`
  - Package metadata (name, version, description)
  - pnpm configuration
  - Build scripts
  - WASM package dependencies

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Customize Message Generation (Priority: P1)

**Goal**: Customize generated messages with options (name, location, date, time, timezone, emojis)

**Independent Test**: Call function with various option combinations, verify output matches specified options

### Tests for User Story 2

- [ ] T029 [P] [US2] Unit test for template variable replacement in `joy-generator/tests/unit/template_test.rs`
  - Test {name} replacement with custom value
  - Test {name} replacement with default "Good Soul"
  - Test {location} replacement with custom value
  - Test {location} replacement with default "The World"
  - Test {date} and {time} generation with timezone
  - Test messages without template variables

- [ ] T030 [P] [US2] Unit test for emoji handling in `joy-generator/tests/unit/emoji_test.rs`
  - Test emoji detection
  - Test emoji stripping when use_emojis=false
  - Test emoji preservation when use_emojis=true

- [ ] T031 [P] [US2] Unit test for timezone handling in `joy-generator/tests/unit/datetime_test.rs`
  - Test date generation with different timezones
  - Test time generation with different timezones
  - Test default timezone "Europe/London"

- [ ] T032 [P] [US2] E2E test for Python customization in `bindings/python/tests/test_e2e.py`
  - Test custom name
  - Test custom location
  - Test timezone option
  - Test use_emojis=false

- [ ] T033 [P] [US2] E2E test for TypeScript customization in `bindings/typescript/tests/e2e.test.ts`
  - Test custom name
  - Test custom location
  - Test timezone option
  - Test use_emojis=false

### Implementation for User Story 2

- [ ] T034 [US2] Enhance template replacement in `joy-generator/src/template.rs`
  - Apply default values for missing template variables
  - Generate date/time from timezone when not provided
  - Support all template variables: name, location, date, time

- [ ] T035 [US2] Enhance Python bindings in `bindings/python/src/lib.rs`
  - Add timezone parameter support
  - Add template_args parameter with name, location, date, time
  - Convert timezone string to CoreGoodbyeOptions

- [ ] T036 [US2] Enhance TypeScript bindings in `bindings/typescript/src/index.ts`
  - Add timezone parameter support
  - Add templateArgs with name, location, date, time
  - Update type definitions

- [ ] T037 [US2] Update TypeScript type definitions in `bindings/typescript/src/index.d.ts`
  - Add timezone to GoodbyeOptions
  - Add location, date, time to GoodbyeTemplateArgs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: Polish & Cross-Cutting Concerns (Corpus Generation First)

**Purpose**: Improvements that affect multiple user stories, with corpus generation prioritized for testing/validation

- [ ] T038 [P] Generate initial corpus of 360 messages in `corpus/en-GB.txt`
  - Warm, heartfelt, whimsical sign-off messages
  - Mix of messages with and without template variables
  - Include emojis where appropriate
  - Examples: "Wishing you a liberated day, {name}❤️", "Until we meet again"

- [ ] T039 [P] Add comprehensive error handling across all modules
  - Clear error messages
  - Proper error propagation
  - User-friendly error types

- [ ] T040 [P] Add logging for debugging (if needed)
  - Corpus loading logs
  - Generation logs (optional, for debugging)

- [ ] T041 [P] Performance optimization
  - Optimize corpus loading (lazy vs eager)
  - Optimize template replacement
  - Optimize emoji stripping
  - Verify <10ms generation time

- [ ] T042 [P] Documentation updates
  - API documentation in code comments
  - README.md with examples
  - Quickstart guide validation
  - Benchmark results in README

- [ ] T043 [P] Code cleanup and refactoring
  - Review all code for consistency
  - Remove unused code
  - Optimize imports
  - Format code (cargo fmt, prettier, black)

- [ ] T044 [P] Security review
  - Validate all inputs
  - Check for potential vulnerabilities
  - Review dependencies

- [ ] T045 [P] Cross-platform testing
  - Test on Linux
  - Test on macOS
  - Test on Windows
  - Verify WASM works in browsers

- [ ] T046 [P] Validate quickstart.md examples
  - Test all TypeScript examples
  - Test all Python examples
  - Verify they work as documented

**Note**: User Story 3 (Translator) is deferred - async translator callback support is defined in contracts but implementation is deferred for later.

---

## Phase 6: User Story 4 - Install and Use from Package Registries (Priority: P2) ⏸️ PARTIALLY DEFERRED

**Status**: ⚠️ **PARTIALLY DEFERRED** - Some tasks can proceed without credentials; publishing tasks require credentials

**Goal**: Publish packages to npm and PyPI, enable installation via pnpm/uv

**Independent Test**: Install package from registry, import and use in new project

**Task Status**:
- ✅ **Can Proceed**: T047-T051, T053 (CI/testing/build scripts - no credentials needed)
- ⏸️ **Requires Credentials**: T052, T055, T056 (publishing workflows - need npm/PyPI tokens)
- ✅ **Can Proceed**: T054, T057 (versioning script and README - no credentials needed)

### Tests for User Story 4

- [ ] T047 [P] [US4] Test package installation in CI workflow `.github/workflows/ci.yml`
  - Test npm package can be installed
  - Test PyPI package can be installed
  - Test imports work correctly

### Implementation for User Story 4

**Infrastructure Tasks (Can Proceed Now)**:

- [ ] T048 [US4] Create versioning script in `scripts/version.sh`
  - Extract version from git tag
  - Update Cargo.toml versions
  - Update package.json version
  - Update pyproject.toml version
  - Validate semantic versioning format

- [ ] T049 [US4] Create build script in `scripts/build.sh`
  - Build Rust workspace
  - Build WASM package for TypeScript
  - Build Python package with maturin
  - Verify all builds succeed

- [ ] T050 [US4] Create test script in `scripts/test.sh`
  - Run Rust unit tests
  - Run Rust integration tests
  - Run Python E2E tests
  - Run TypeScript E2E tests
  - Run benchmarks

- [ ] T051 [US4] Create benchmark script in `scripts/bench.sh`
  - Run Rust benchmarks
  - Run Python benchmarks
  - Run TypeScript benchmarks
  - Generate benchmark report for README

- [ ] T053 [US4] Create CI workflow in `.github/workflows/ci.yml`
  - Test on multiple platforms (Linux, macOS, Windows)
  - Run all test suites
  - Build all packages
  - Check for errors

- [ ] T054 [US4] Create versioning workflow in `.github/workflows/version.yml`
  - Trigger on git tags (v*.*.*)
  - Extract version from tag
  - Run version.sh script
  - Commit version updates

- [ ] T057 [US4] Update README.md with installation instructions
  - pnpm installation for TypeScript
  - uv installation for Python
  - Basic usage examples
  - Benchmark results

**Publishing Tasks (Require Credentials - DEFERRED)**:

- [ ] T052 [US4] ⏸️ Create publish script in `scripts/publish.sh`
  - Publish to npm (pnpm publish) - **Requires npm token**
  - Publish to PyPI (maturin publish) - **Requires PyPI token**
  - Verify publication success

- [ ] T055 [US4] ⏸️ Create npm publish workflow in `.github/workflows/publish-npm.yml`
  - Trigger on version tag
  - Build TypeScript package
  - Publish to npm - **Requires NPM_TOKEN secret**
  - Verify publication

- [ ] T056 [US4] ⏸️ Create PyPI publish workflow in `.github/workflows/publish-pypi.yml`
  - Trigger on version tag
  - Build Python package
  - Publish to PyPI - **Requires PYPI_TOKEN secret**
  - Verify publication

**Checkpoint**: Infrastructure tasks (T047-T051, T053-T054, T057) can proceed. Publishing tasks (T052, T055-T056) deferred until credentials are configured.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - Extends US1 but independently testable
- **User Story 3 (P2)**: ⏸️ **DEFERRED** - Translator support deferred for later (contracts defined)
- **User Story 4 (P2)**: ⏸️ **DEFERRED** - Publishing deferred until credentials are configured

### Within Each User Story

- Tests (REQUIRED) MUST be written and FAIL before implementation
- Core Rust implementation before bindings
- Bindings before E2E tests
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, US1 and US2 can start in parallel (both P1)
- All tests for a user story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Complete Phase 5 (Polish) → Corpus generation, testing, optimization
5. User Story 3 (Translator) → ⏸️ **DEFERRED** for later
6. User Story 4 (Publishing) → ⏸️ **DEFERRED** until credentials configured
7. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Rust core + Python bindings)
   - Developer B: User Story 1 (TypeScript bindings)
   - Developer C: User Story 2 (extending US1)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- Benchmarks are REQUIRED and should be included in README
- Corpus generation (T038) is prioritized in Phase 5 for early testing/validation
- User Story 3 (Translator) is deferred - contracts defined but implementation later
- User Story 4 (Publishing) is deferred until npm/PyPI credentials are configured
- User Story 3 (Translator) is deferred - contracts defined but implementation later
- User Story 4 (Publishing) is deferred until npm/PyPI credentials are configured

