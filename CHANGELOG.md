## [1.6.4](https://github.com/siviter-xyz/joyous-departures/compare/v1.6.3...v1.6.4) (2025-12-08)


### Bug Fixes

* Fix WASM import error by removing default import and using top-level await ([ebd3443](https://github.com/siviter-xyz/joyous-departures/commit/ebd344382a65d4b610c818a7c5e1fe97c3fa24b2))

## [1.6.3](https://github.com/siviter-xyz/joyous-departures/compare/v1.6.2...v1.6.3) (2025-12-08)


### Bug Fixes

* Add required packages field to pnpm-workspace.yaml ([7f9587d](https://github.com/siviter-xyz/joyous-departures/commit/7f9587d0e768464b239ce2752a2c3ac426cdb286))
* Remove incorrect pip cache configuration ([b56343a](https://github.com/siviter-xyz/joyous-departures/commit/b56343a32edbe1d90c0276e0aabb9621a7affb03))
* Update publish workflow to use build script and add WASM target ([88cd525](https://github.com/siviter-xyz/joyous-departures/commit/88cd5250ef9d97146bf1eccea2d16f1ba28be123))
* Use pnpm/action-setup for proper pnpm installation and caching ([cd9019a](https://github.com/siviter-xyz/joyous-departures/commit/cd9019a9e56ab9495368462d8d5bdb1b94e31039))


### Performance Improvements

* Add comprehensive caching across all workflows ([2926097](https://github.com/siviter-xyz/joyous-departures/commit/29260976259706b4a2da3ddd57c3782996bbe503))

## [1.6.2](https://github.com/siviter-xyz/joyous-departures/compare/v1.6.1...v1.6.2) (2025-12-08)


### Bug Fixes

* Add wasm-opt installation to build job as well ([cfbc7e1](https://github.com/siviter-xyz/joyous-departures/commit/cfbc7e1e16867c372c2c65f66cb67af5920e6f53))
* Add wasm32-unknown-unknown target installation to CI ([6778b5e](https://github.com/siviter-xyz/joyous-departures/commit/6778b5e32f683fa9992be5d212d58d5c288d4eee))
* Add wasm32-unknown-unknown target to build job as well ([de46cdc](https://github.com/siviter-xyz/joyous-departures/commit/de46cdc8b9f3162a3736f1a974c12b7d07dcfad7))
* Also install dependencies before build in build job ([d815710](https://github.com/siviter-xyz/joyous-departures/commit/d815710e5b3d1130eb19d31bfb1beb4b690595bd))
* Install dependencies before build script and add wasm-opt to CI ([72ac0f7](https://github.com/siviter-xyz/joyous-departures/commit/72ac0f74e0d071f6ad09368ed0bd025b125abc3f))
* **typescript:** remove duplicate README.md from pkg directory ([449f0b8](https://github.com/siviter-xyz/joyous-departures/commit/449f0b88279a48d71b7f520d3b02415f27a091c5))
* Update CI workflow to use build-package.sh instead of wasm-pack ([ee04f2b](https://github.com/siviter-xyz/joyous-departures/commit/ee04f2b592a77daa16eb808159d6a9c3724835ee))
* Update pnpm-lock.yaml after removing wasm-pack dependency ([85260dd](https://github.com/siviter-xyz/joyous-departures/commit/85260dd824c134cf1d85773c7e1af8bc2eb8ddbe))


### Performance Improvements

* Add caching for cargo binaries in build job as well ([4b3ca3d](https://github.com/siviter-xyz/joyous-departures/commit/4b3ca3d178bf6c654bf1fa252e4c062d69408002))
* Add caching for wasm-bindgen-cli and wasm-opt in CI ([d24a154](https://github.com/siviter-xyz/joyous-departures/commit/d24a154cf833f9c44c499d57050da86876630c0c))
* Disable wasm-opt installation in CI (takes too long) ([6f29a22](https://github.com/siviter-xyz/joyous-departures/commit/6f29a22eb8f698bf41011f3756054979972733fa))
* Remove wasm-opt from build job as well ([60f0991](https://github.com/siviter-xyz/joyous-departures/commit/60f0991d7c879988ce82ec3fc66a830494ab49c1))

## [1.6.1](https://github.com/siviter-xyz/joyous-departures/compare/v1.6.0...v1.6.1) (2025-12-08)


### Bug Fixes

* **ci:** update Python test to use correct module name ([9e70e20](https://github.com/siviter-xyz/joyous-departures/commit/9e70e20074342c797f88af79ed4723ca28f76d01))
* modernize Python packaging and fix CI pipeline failures ([add7344](https://github.com/siviter-xyz/joyous-departures/commit/add734488cd08db8ea5ce3c3868034d202cf4dd0))

# [1.6.0](https://github.com/siviter-xyz/joyous-departures/compare/v1.5.2...v1.6.0) (2025-12-08)


### Bug Fixes

* missing lock bump ([26fae59](https://github.com/siviter-xyz/joyous-departures/commit/26fae59dbbc6534ee08691da656ab8c3bb00c97d))
* **python:** use maturin native mixed project support ([d965115](https://github.com/siviter-xyz/joyous-departures/commit/d96511554c91393e46e85069afc472dc98c2e773))
* **typescript:** add Cloudflare Workers compatibility ([0bced23](https://github.com/siviter-xyz/joyous-departures/commit/0bced23eeb97a7cfccf81eeb89d9d6d3bb8efaf8))


### Features

* add build scripts for local testing and verification ([b190078](https://github.com/siviter-xyz/joyous-departures/commit/b1900780ff60079e5b0f2c9133778d8d69013803))
* **examples:** add comprehensive tests and local package support ([46f68f5](https://github.com/siviter-xyz/joyous-departures/commit/46f68f5e5cb398a093a237e1721996353b36493a))
* **typescript:** migrate to tsdown for dual ESM/CommonJS builds with sourcemaps ([8f6407d](https://github.com/siviter-xyz/joyous-departures/commit/8f6407db38db6fe21bf1e8dc9f2cbbd72cefe6a8))

## [1.5.2](https://github.com/siviter-xyz/joyous-departures/compare/v1.5.1...v1.5.2) (2025-12-07)


### Bug Fixes

* correct YAML syntax for Python script in workflow ([8e73483](https://github.com/siviter-xyz/joyous-departures/commit/8e734832b3456f7a0e27bad388454d5b000957f9))
* trial fix ([d489367](https://github.com/siviter-xyz/joyous-departures/commit/d4893673ce5c360c6cd16850bfe8d6e3b7651aaa))

## [1.5.1](https://github.com/siviter-xyz/joyous-departures/compare/v1.5.0...v1.5.1) (2025-12-07)


### Bug Fixes

* inject joyous_departures package into wheel post-build ([bd77a06](https://github.com/siviter-xyz/joyous-departures/commit/bd77a06c8792439f3d0471b12df0d3525b95e8e0))

# [1.5.0](https://github.com/siviter-xyz/joyous-departures/compare/v1.4.0...v1.5.0) (2025-12-07)


### Bug Fixes

* include Python package directory in wheels via python-source ([23a1b08](https://github.com/siviter-xyz/joyous-departures/commit/23a1b08cfd507fb1edb0090c00bbcf554cffe33a))
* remove pkg/.gitignore to include WASM files in npm package ([af2f428](https://github.com/siviter-xyz/joyous-departures/commit/af2f428091b1b2a5dc73c0338dd9be37168aed34))


### Features

* update README ([0496bed](https://github.com/siviter-xyz/joyous-departures/commit/0496bed33e2a7ce25d4d349ac7679296770354d4))

# [1.4.0](https://github.com/siviter-xyz/joyous-departures/compare/v1.3.0...v1.4.0) (2025-12-07)


### Bug Fixes

* add debug output to PyPI publish check and update version ([7bf8c62](https://github.com/siviter-xyz/joyous-departures/commit/7bf8c6277e05884dc4b546d77a3abec53b6af878))
* correct WASM import paths to use ../pkg from src/ ([3036570](https://github.com/siviter-xyz/joyous-departures/commit/3036570d6ca3cb2b4f46f2a6a95f504e42abae20))
* improve PyPI 404 handling and add local publish script ([50e3dfb](https://github.com/siviter-xyz/joyous-departures/commit/50e3dfb9f4f330be13e1d95462c11cf7614c997a))
* improve PyPI check reliability and add better debugging ([e1618eb](https://github.com/siviter-xyz/joyous-departures/commit/e1618ebfed4a2826157796829e7b6626a68f2cc8))
* prevent publish on skipped release and fix PyPI wheel path ([15087bb](https://github.com/siviter-xyz/joyous-departures/commit/15087bbb00c19df9e52af36266251d24e54044de))
* resolve CI and PyPI publish issues ([2c36298](https://github.com/siviter-xyz/joyous-departures/commit/2c36298bcbda2bf9535c9aba6807cf996e2b6fda))


### Features

* update publish.sh with better build and error handling ([b7de606](https://github.com/siviter-xyz/joyous-departures/commit/b7de6062c7105b552a0357296dbd5bf208c47d5f))

# [1.3.0](https://github.com/siviter-xyz/joyous-departures/compare/v1.2.0...v1.3.0) (2025-12-07)


### Bug Fixes

* add verification step for pkg directory before npm publish ([23ba99b](https://github.com/siviter-xyz/joyous-departures/commit/23ba99b721e3a69b3e835e93bf054da04c6dc573))
* correct import paths in npm package and PyPI publish logic ([e5bbc2c](https://github.com/siviter-xyz/joyous-departures/commit/e5bbc2c1ed9bb1eb2f55a5c95736e1dacef7c8c0))
* correct PyPI publish check logic ([80ae3b2](https://github.com/siviter-xyz/joyous-departures/commit/80ae3b25f5677f78abf6e492cef1427857301edb))
* ts example ([3ebdc74](https://github.com/siviter-xyz/joyous-departures/commit/3ebdc7418ae0870fa668adb3b73a8f2cf51684a6))


### Features

* add prepublishOnly to ensure WASM bundle is built ([095c6e9](https://github.com/siviter-xyz/joyous-departures/commit/095c6e9e5488a7c86ac1ece9dd445812ba7930a3))

# [1.2.0](https://github.com/siviter-xyz/joyous-departures/compare/v1.1.1...v1.2.0) (2025-12-07)


### Bug Fixes

* copy README to Python package directory for maturin ([0d2cae4](https://github.com/siviter-xyz/joyous-departures/commit/0d2cae4a7cb6eb3e5b974f8a69f2ba075108a93f))
* correct MANIFEST.in README path ([caaabdd](https://github.com/siviter-xyz/joyous-departures/commit/caaabdd24b362ece3412cdcfb663fc9bec3b6212))


### Features

* add badges to README ([c87676f](https://github.com/siviter-xyz/joyous-departures/commit/c87676f5395876a76755f7e516548a0d63313c73))
* add example projects and fix npm package exports ([0772c55](https://github.com/siviter-xyz/joyous-departures/commit/0772c55c7e32c362c405f618847410f2938f2605))
* add support for npm Trusted Publishing via OIDC ([2fa68bb](https://github.com/siviter-xyz/joyous-departures/commit/2fa68bb02e5411d27dd99f51c14ee44c7474126d))
* include README in npm and PyPI packages ([3bcb2c2](https://github.com/siviter-xyz/joyous-departures/commit/3bcb2c2cb6f2bfaf4c800404faf699ce1a493dde))

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
