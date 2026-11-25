# lib-crypto

`crypto` — cryptographic hashing for the handl runtime: `crypto.sha1`,
`crypto.sha256`, and `crypto.md5` are pure-`.hnl` arrows over the
`math`/`bytes`/`typeof` substrate ops (u32 arithmetic, `rotl32`,
`bit_*`).

The package surface is pure `.hnl`: consumers vendor `pkg.hnl` +
`src/**/*.hnl` + `handl-packages/**/*.hnl` via `handl-pkg-vendor` and
embed the sources with `include_str!` at their own build time
(`plugin-lib-stdlib` does this — the `handl-plugin-lib-crypto` crate's
`CRYPTO_HNL`/`CRYPTO_FUNCS` consts are no longer consumed).

## Layout

```text
handl-packages/crypto/   the .hnl package (crypto.hnl + tests)
examples/crypto/         a runnable example
pkg.hnl                  package manifest — modules, entry, license
scripts/hnl-gate.sh      the gate
workflows/gate.yaml                    bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
