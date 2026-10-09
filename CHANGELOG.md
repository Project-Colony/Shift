# Changelog

## [0.2.0](https://github.com/Project-Colony/Shift/compare/v0.1.0...v0.2.0) (2026-10-09)


### Features

* add bracket aliases for image navigation ([b42bb58](https://github.com/Project-Colony/Shift/commit/b42bb5825b2ebe19489cbb9fe28a33cdfc4d62a3))
* add fit mode keyboard shortcut ([b4b1c79](https://github.com/Project-Colony/Shift/commit/b4b1c79498f903dd31d7fde0bcb92fe8a5097aa9))
* add letter aliases for image navigation ([c7210eb](https://github.com/Project-Colony/Shift/commit/c7210ebdf974b63ee9084eeaec1739c85881bfcf))
* add reset zoom keyboard alias ([496f48a](https://github.com/Project-Colony/Shift/commit/496f48a4908f7590a0253f885fab9b9cc7da4650))
* bootstrap Shift_Private image viewer ([d41e123](https://github.com/Project-Colony/Shift/commit/d41e123b55650793b5f1f3ab7145e9d95a740fbc))
* improve viewer navigation and zoom ergonomics ([f11918a](https://github.com/Project-Colony/Shift/commit/f11918a536ae30b885455a5f2cbef9a2b79fb9c9))
* improve viewer UI and folder flow ([909948e](https://github.com/Project-Colony/Shift/commit/909948e31f0cf7622d29d1a1f0c5e172a05fc5a1))
* ship the binary as shift and answer --version ([#14](https://github.com/Project-Colony/Shift/issues/14)) ([9f984a3](https://github.com/Project-Colony/Shift/commit/9f984a33c78155f3ac23842673422a7eac47d513))
* **viewer:** add basic fullscreen toggle ([7b2ba87](https://github.com/Project-Colony/Shift/commit/7b2ba876ba424a87a95f363cc4513c02561e8799))
* **viewer:** add quick help panel ([d30c9fd](https://github.com/Project-Colony/Shift/commit/d30c9fd3ca8bb3359399171a327cdc7409f6f422))
* **viewer:** clarify metadata hierarchy ([725a14e](https://github.com/Project-Colony/Shift/commit/725a14e1b51f2a5b7e6b37c740ad894abb9fe285))
* **viewer:** improve loading status feedback ([0ca0818](https://github.com/Project-Colony/Shift/commit/0ca081845c01f6ce16bf153e5bc57c2232406f39))
* **viewer:** preserve manual zoom across navigation ([8e2e31b](https://github.com/Project-Colony/Shift/commit/8e2e31b6e051739966b1e59b365e7d0dd0b4a411))
* **viewer:** respect image aspect in fit mode ([8d35189](https://github.com/Project-Colony/Shift/commit/8d3518930cf6c92a061c23ef8fd490c8e70f1350))
* **viewer:** show image dimensions in metadata ([e14671e](https://github.com/Project-Colony/Shift/commit/e14671e2f4dcbc57d305c4ea3c92b3b4fd1861c5))
* **viewer:** smooth scroll zoom response ([4a76636](https://github.com/Project-Colony/Shift/commit/4a766361e4065f37be0e5fa0161965d6d0ca2244))


### Fixes

* show the read error for an unreadable first image and fit large images to the window ([#13](https://github.com/Project-Colony/Shift/issues/13)) ([f7f1360](https://github.com/Project-Colony/Shift/commit/f7f1360c8c712924d8a62d4707644ea631986cbf))
* **viewer:** preserve open failure status ([8c9b7b9](https://github.com/Project-Colony/Shift/commit/8c9b7b915db82f425005d8e275bcfa215749f4a8))
* **viewer:** reset state on empty folder ([e1d652b](https://github.com/Project-Colony/Shift/commit/e1d652b6bd97876bb1ef5b060ccd9f6c77a9aa3e))
* **viewer:** surface unreadable image failures ([179bc55](https://github.com/Project-Colony/Shift/commit/179bc55bb2ef40ab5cd0c8b7a5a1697ba33dd0e0))
* **viewer:** target active window for fullscreen ([4adb6d9](https://github.com/Project-Colony/Shift/commit/4adb6d9b6a107653ef1e639bbcc7e7491b988302))


### Internals

* **fullscreen:** hide chrome around image ([30cd82c](https://github.com/Project-Colony/Shift/commit/30cd82c43e435bc18ce2db54a2b1efecec248c3d))
* **status:** simplify folder load messaging ([1c55903](https://github.com/Project-Colony/Shift/commit/1c5590351e8243eb09106eeb0610987a1febf430))
* **ui:** calm the empty state ([e642c27](https://github.com/Project-Colony/Shift/commit/e642c275106b9e6f40ca647ebad02cf680db05be))
* **ui:** group top bar controls ([ede60a8](https://github.com/Project-Colony/Shift/commit/ede60a8def4638711f54ba37a6dbf5be5e53cbb4))
* **ui:** hide idle footer ([cf46aa3](https://github.com/Project-Colony/Shift/commit/cf46aa32a702e1ca4ac7275216b7c9ea52941fae))
* **ui:** reduce metadata noise ([b6e86a5](https://github.com/Project-Colony/Shift/commit/b6e86a55173948e0306d436b4f1c9beb91a86989))
* **ui:** reduce viewer padding noise ([0da2346](https://github.com/Project-Colony/Shift/commit/0da23460d3aebcd664d6c34252c751aff0840e08))
* **ui:** separate controls from metadata ([6a8f0ec](https://github.com/Project-Colony/Shift/commit/6a8f0ec0f70f5c23fa8d221cc8dfd98eee499e7c))
* **ui:** simplify viewer chrome ([b5cf223](https://github.com/Project-Colony/Shift/commit/b5cf223c5147f044333990b0e918edd9a6349a72))
* **ui:** soften empty status messaging ([24aa5a1](https://github.com/Project-Colony/Shift/commit/24aa5a1c5f51262cfd78240909fb2551c75c7679))
* **ui:** stabilize controls row width ([1411144](https://github.com/Project-Colony/Shift/commit/141114401739d28fbc62aecfd3f96a34c23d734f))
* **ui:** tighten empty state spacing ([2c5453c](https://github.com/Project-Colony/Shift/commit/2c5453cb80d673c283fb4b50946bbb5226a28254))
* **viewer:** remove dormant help toggle ([0ec703d](https://github.com/Project-Colony/Shift/commit/0ec703d0615daa2e596b6143ae8b42dbc9481c83))


### Documentation

* polish README presentation ([ff9102b](https://github.com/Project-Colony/Shift/commit/ff9102b3480e9ebb94c704366225aadf6e04bda0))
* rewrite project README ([6b8da3b](https://github.com/Project-Colony/Shift/commit/6b8da3be9f027859e255a33522b49999e413ace7))
* rewrite the README with installation, privacy and code signing policy ([#16](https://github.com/Project-Colony/Shift/issues/16)) ([5ed7cac](https://github.com/Project-Colony/Shift/commit/5ed7cacc2960e946829c031bd78bf1f7543fb0a7))
