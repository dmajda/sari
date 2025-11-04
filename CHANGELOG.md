# Changelog

## 2.0.0

**Released:** November 4, 2025

### Changes

* **Breaking:** Add position information to errors
  ([`aeb6909..b971453`](https://github.com/dmajda/sari/compare/aeb6909334e3b84b331425c3772452ac0354a0cc...b971453faab396b8a0ca144f51f82d7c7b88c027))
* Implement `+` and `-` unary operators
  ([`10059df`](https://github.com/dmajda/sari/commit/10059df59812e37ad8a8fae70e0587082954e7ff))
* Derive `Eq` for `Error`
  ([`5d7c621`](https://github.com/dmajda/sari/commit/5d7c621cb1b61327656a22d79628b2f42c47fbb2))
* Change the `message` parameter of `Error::new` from `&str` to `impl Into<String>`
  ([`96a35e8`](https://github.com/dmajda/sari/commit/96a35e87a727ce1d9722fe2e0758810123072d24))
* Change token and AST structure, rewrite the scanner, parser, and evaluator
  ([`5d7c621..9980994`](https://github.com/dmajda/sari/compare/5d7c621cb1b61327656a22d79628b2f42c47fbb2...99809946bcbb392c8f5bcd434321afc368408d0b))
* Miscellaneous documentation improvements and a fix
  ([`3ef5530`](https://github.com/dmajda/sari/commit/3ef5530be20bdf7334115441886f27f8fb8c13a3), [`aeb6909`](https://github.com/dmajda/sari/commit/aeb6909334e3b84b331425c3772452ac0354a0cc), [`dacf834`](https://github.com/dmajda/sari/commit/dacf834351cad4bf35e87c64004549cd08f760d1))
* Require Rust >= 1.88.0
  ([`74f0e25`](https://github.com/dmajda/sari/commit/74f0e25c63866e184ec4a6b94c220a2ca6bf5478))

[Complete list of changes](https://github.com/dmajda/sari/compare/v1.0.0...v2.0.0)

## 1.0.0

**Released**: January 4, 2025

Initial release.
