## [1.1.1](https://github.com/siviter-xyz/joyous-departures/compare/v1.1.0...v1.1.1) (2025-12-07)


### Bug Fixes

* verify version instead of always updating in publish workflow ([edb3b93](https://github.com/siviter-xyz/joyous-departures/commit/edb3b932b06c403314a578f6d16ee995a0cc9ed9))

# [1.1.0](https://github.com/siviter-xyz/joyous-departures/compare/v1.0.1...v1.1.0) (2025-12-07)


### Bug Fixes

* add artifact path debugging and check both wheel locations ([45d5109](https://github.com/siviter-xyz/joyous-departures/commit/45d51097780721caaede8a32a90e6f090494978a))
* complete publish workflow refactor with package existence checks ([f0581ea](https://github.com/siviter-xyz/joyous-departures/commit/f0581ea13c57e556190b9625ed1aed61e6db687e))
* remove python-source to suppress maturin warning ([e6e3edd](https://github.com/siviter-xyz/joyous-departures/commit/e6e3edd795f6c09614e869eacf19e675585bc0ee))
* simplify path detection logic for better compatibility ([4a719df](https://github.com/siviter-xyz/joyous-departures/commit/4a719df7e43f864b6fa70ed26d7e5fa685fe2017))


### Features

* add path-based test filtering and fix artifact paths ([cbd75a9](https://github.com/siviter-xyz/joyous-departures/commit/cbd75a9c8ae9e3b549babc267815694c9c39a528))

## [1.0.1](https://github.com/siviter-xyz/joyous-departures/compare/v1.0.0...v1.0.1) (2025-12-07)


### Bug Fixes

* configure release workflow to use PAT for tag pushes ([79e7c60](https://github.com/siviter-xyz/joyous-departures/commit/79e7c60e67df69bdeefd95e5c6a1c9e230898708))
* require GH_PAT with no fallback to GITHUB_TOKEN ([6bfc86c](https://github.com/siviter-xyz/joyous-departures/commit/6bfc86c0bcbd39f1b85c658377379e780a8f106c))
* use GH_PAT for semantic-release GITHUB_TOKEN ([e9fbbb7](https://github.com/siviter-xyz/joyous-departures/commit/e9fbbb71450a115cda6fb2011a713a217febb3ce))

# 1.0.0 (2025-12-07)


### Bug Fixes

* add include directive to maturin config for Python package ([edf1a96](https://github.com/siviter-xyz/joyous-departures/commit/edf1a96a80669ffe6238f55c916d99ac3c345749))
* add MANIFEST.in and PYTHONPATH for Python package import ([889b3c7](https://github.com/siviter-xyz/joyous-departures/commit/889b3c7f2f2edec8c03b12780bda0f3ad20feb26))
* add Node.js setup to release workflow ([440c2fc](https://github.com/siviter-xyz/joyous-departures/commit/440c2fc7404663cbc24c4059271e143c730c8174))
* add PYTHONPATH to pytest command in CI ([da38408](https://github.com/siviter-xyz/joyous-departures/commit/da384082a2c036837c98707ccb8ff0557b8a6bae))
* add sdist-include to maturin config and improve CI debugging ([0365d46](https://github.com/siviter-xyz/joyous-departures/commit/0365d46f0ac92b000e3f539fdcf81442854d254a))
* add sdist-include to maturin config and improve CI debugging ([6949de3](https://github.com/siviter-xyz/joyous-departures/commit/6949de3759228c0ecbec3335468b76f8685a687b))
* add sdist-include to maturin config for Python package ([2be7fdf](https://github.com/siviter-xyz/joyous-departures/commit/2be7fdf4370a12d692fcb0ae0b52ad05692b5317))
* address CI warnings and permission issues ([f5f39f2](https://github.com/siviter-xyz/joyous-departures/commit/f5f39f28995ffcc640dbc01c3be908919e19b770))
* address code review security and quality issues ([2003047](https://github.com/siviter-xyz/joyous-departures/commit/2003047ec42b20a49114a74aae7a18cf513e065b))
* change const to let for result variable in TypeScript ([0e030c2](https://github.com/siviter-xyz/joyous-departures/commit/0e030c2e14800861afce682b66459288d6758b2a))
* ci ([fa966a8](https://github.com/siviter-xyz/joyous-departures/commit/fa966a86de73d6f83b71bf951c3574b8a3fce3ff))
* configure maturin to include Python package directory ([ae4b9c9](https://github.com/siviter-xyz/joyous-departures/commit/ae4b9c9da8ae2bbafe87990d2369aa0d0f80b693))
* correct Python module import path and remove unused imports ([4179dd5](https://github.com/siviter-xyz/joyous-departures/commit/4179dd5fb88b865b5b2d7490f448c30067956160))
* improve corpus quality and fix test assertions ([b6a7aeb](https://github.com/siviter-xyz/joyous-departures/commit/b6a7aeb1f3b28808fadfed3060f1e998fe8482d5))
* improve Python package installation verification in CI ([d2e7cd5](https://github.com/siviter-xyz/joyous-departures/commit/d2e7cd5edcbc14659b6a2d0b4491112b7ec0143a))
* improve type safety and code clarity ([be8ed87](https://github.com/siviter-xyz/joyous-departures/commit/be8ed87d799a7dd6b74bdb4cadc8f9e63521314e))
* load corpus from embedded text file instead of compressed data ([b15a167](https://github.com/siviter-xyz/joyous-departures/commit/b15a167e4a883aa5825418139c8ef728abd6e9ad))
* make CI tests run independently and fix Python async tests ([a27afc1](https://github.com/siviter-xyz/joyous-departures/commit/a27afc14e1d144b3825985e91ef7dc7be732d805))
* resolve all clippy warnings ([5fd8709](https://github.com/siviter-xyz/joyous-departures/commit/5fd8709fccda457a8b46aa540a5b541e242ba616))
* resolve all clippy warnings ([1ee023e](https://github.com/siviter-xyz/joyous-departures/commit/1ee023e2ad56e724eba6cc5233ba07ec473916eb))
* resolve naming conflicts and update all references ([52953a5](https://github.com/siviter-xyz/joyous-departures/commit/52953a5f61bcb2b4e2ea25ed31e474c69944c05e))
* update CI to use new joy_generator module name ([652e74d](https://github.com/siviter-xyz/joyous-departures/commit/652e74d0a042dc45090bcd2b0742f906c6e4254a))
* update Python test script to await async function ([29e649d](https://github.com/siviter-xyz/joyous-departures/commit/29e649dfc67cfa080edef63796f92a9b5c06f363))
* update remaining Python import reference to joy_generator ([6311218](https://github.com/siviter-xyz/joyous-departures/commit/631121859ca9ca1d3577bf708489ea8eba344b5f))
* use pnpm-workspace.yaml for build script approval ([d5e2d2e](https://github.com/siviter-xyz/joyous-departures/commit/d5e2d2e0b4c1099ea961bfde4ebdd5cd1a0e6d44))


### Features

* add 10 messages with {date} and {time} template variables ([ab4d8cd](https://github.com/siviter-xyz/joyous-departures/commit/ab4d8cde8847929c1bd8db24ffcab797fd23a3fc))
* add corpus and fix TypeScript ES module compatibility ([b7fd2c6](https://github.com/siviter-xyz/joyous-departures/commit/b7fd2c604f99e08b5287cb5b63255b42d31d50cd))
* add date/time messages, fix static mut warning, and set up CI/CD ([4204e5a](https://github.com/siviter-xyz/joyous-departures/commit/4204e5a2713215d34b3b9f70eb7f878b91a2b3e3))
* add lint.sh script to match CI checks locally ([7f602d4](https://github.com/siviter-xyz/joyous-departures/commit/7f602d46273aefbb01cbd6f75c384fe4e63f3684))
* add pre-commit hook for automatic linting ([a9c86b0](https://github.com/siviter-xyz/joyous-departures/commit/a9c86b0c021b70b0b4fb7f27ff78e9ae28fa7284))
* add project structure and core Rust implementation ([86f772f](https://github.com/siviter-xyz/joyous-departures/commit/86f772f6492a27b3d1c8253b9a55f9d6297f5cf4))
* add semantic-release workflow for automated versioning and publishing ([f7ff441](https://github.com/siviter-xyz/joyous-departures/commit/f7ff441160c173d17af7970c0298c2af4c59995c))
* increase size of logo ([a14a18a](https://github.com/siviter-xyz/joyous-departures/commit/a14a18a6349260933039f0c0521e3571e0559403))
* initial commit ([355a147](https://github.com/siviter-xyz/joyous-departures/commit/355a14771d7b9c1e013978b491c219c81c468ba7))
* logo ([85c9213](https://github.com/siviter-xyz/joyous-departures/commit/85c9213b83b77d2a18e91f82de52b0c471f78f90))
