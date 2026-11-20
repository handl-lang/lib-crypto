# lib-crypto

`handl-plugin-lib-crypto` — cryptographic hashing for the handl runtime:
`crypto.sha256` and `crypto.md5` are native Rust ops; `crypto.sha1` is
implemented in pure Handl over the `math`/`bytes`/`typeof` atoms.

## Layout

```text
handl-packages/crypto/   the .hnl package
src/                     the Rust plugin crate
scripts/hnl-gate.sh      the gate
ci.sh                    bupha entry
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
