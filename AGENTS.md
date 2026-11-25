# lib-crypto

`crypto` — cryptographic hashing for the handl runtime: `crypto.sha1`,
`crypto.sha256`, `crypto.md5` — pure-`.hnl` arrows over the
`math`/`bytes`/`typeof` substrate ops (u32 arithmetic, `rotl32`,
`bit_*`).

## Layout

| path | role |
| --- | --- |
| `handl-packages/crypto/crypto.hnl` | the package surface — `pkg.hnl` `entry` points here |
| `examples/` | usage examples |
| `pkg.hnl` | manifest — `modules: ["crypto"]` |
| `scripts/hnl-gate.sh` | `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Pure `.hnl` only — consumers vendor `pkg.hnl` + `src/**/*.hnl` +
  `handl-packages/**/*.hnl` via `handl-pkg-vendor` and embed with
  `include_str!` (`plugin-lib-stdlib` does this).
- No Rust crate and no native crypto deps — everything is handl code
  over substrate ops.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
