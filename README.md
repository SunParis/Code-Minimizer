# Code Minimizer

Code Minimizer is a Rust command-line reducer for single-file compiler and
runtime fuzzing cases. It takes one source file, a language id, two run commands
A/B, optional build commands, and reduces the source while preserving a
result-observable A/B difference.

The first implementation supports Java and JavaScript through tree-sitter.

## Goal

Fuzzing inputs are often large random programs. Code Minimizer turns such a
program into a smaller proof of concept while keeping the configured
interestingness condition true. In the current implementation that condition is
an A/B difference on stdout, stderr, both streams, or exit-status class, with an
optional exit-status preservation policy.

This is a structure-aware reducer. It parses the source, builds a
language-neutral snapshot, generates phase-specific candidate groups, applies
edits, reparses candidates, and asks the oracle whether the candidate still
triggers the configured result difference. It is not a whitespace or comment
stripper.

## Non-Goals

- It reduces one source file at a time.
- It does not prove the semantic root cause of a bug.
- It does not guarantee a mathematical global minimum.
- It does not sandbox user-provided commands.
- It does not use runtime speed as an acceptance signal for non-hang bugs.
- Identifier renaming is only attempted by the experimental `weighted-random`
  algorithm as a conservative late cleanup pass.

## Install

```bash
cargo build --release
```

The binary is written to `target/release/code-minimizer`.

## CLI

JavaScript:

```bash
code-minimizer reduce \
  --lang js \
  --input ./case.js \
  --run-a "node {input}" \
  --run-b "node --jitless {input}" \
  --timeout 5s \
  --output ./case.min.js
```

Java with direct source-file launch:

```bash
code-minimizer reduce \
  --lang java \
  --input ./Test.java \
  --run-a "/path/to/java-a --add-modules jdk.incubator.vector {input}" \
  --run-b "/path/to/java-b --add-modules jdk.incubator.vector {input}" \
  --timeout 120s \
  --max-trials 0 \
  --output ./Test.min.java
```

Java with a shared build command:

```bash
code-minimizer reduce \
  --lang java \
  --input ./Test.java \
  --build "javac {input}" \
  --run-a "java -cp {dir} Test" \
  --run-b "/path/to/other-java -cp {dir} Test" \
  --output ./Test.min.java
```

Java with side-specific build commands:

```bash
code-minimizer reduce \
  --lang java \
  --input ./Test.java \
  --build-a "javac-a {input}" \
  --build-b "javac-b {input}" \
  --run-a "java-a -cp {dir} Test" \
  --run-b "java-b -cp {dir} Test"
```

## Placeholders

Command templates support these placeholders:

- `{input}`: the source file path inside the current side-specific trial
  directory.
- `{dir}`: the directory containing `{input}`.
- `{stem}`: the original input file stem.
- `{output}`: the side-specific output directory reserved for build artifacts.

Expanded placeholder values are shell-quoted. The final command is executed
through `sh -c`.

## Important Options

- `--lang <java|js|javascript>` selects the parser and language adapter.
- `--timeout <duration>` sets the per-command timeout. Values such as `120s`,
  `5s`, `250ms`, and `5` are accepted. For the current result-only oracle,
  timeout means the candidate is rejected.
- `--max-output-bytes <n>` caps captured stdout and stderr per command.
- `--confirm-runs <n>` repeats baseline, accepted-candidate, and final
  confirmation checks. Every confirmation must preserve the configured
  difference. The default is `1`.
- `--preserve-exit <same-class|none|exact>` controls exit-status preservation.
  The default is `same-class`.
- `--diff-mode <any-channel|stdout|stderr|both>` controls which stdout/stderr
  stream difference must remain. The default is `any-channel`.
  Side-specific trial paths are normalized before stdout/stderr comparison so
  compiler diagnostic paths do not become false diffs.
- `--max-trials <n>` bounds oracle trials after baseline validation. Use `0`
  for no explicit trial limit.
- `--max-rounds <n>` bounds fixed-point stage rounds. Use `0` for no explicit
  round limit.
- `--algorithm <name>` selects the reducer algorithm. `structured` is the
  deterministic staged reducer and remains the default. `weighted-random`
  samples deletable statement points by adaptive weights, then runs cleanup and
  rename sweeps. All algorithms finish with the shared blank-line cleanup stage.
- `--jobs <n>` is reserved for future parallel trials. The current reducer runs
  sequentially.
- `--keep-temp` keeps the session workspace for debugging. By default, the
  workspace is created next to the output file and removed when reduction
  finishes.
- `--json-report <path>` writes a machine-readable reduction report.

## Reduction Model

Every accepted source is represented as a `ProgramSnapshot`:

```text
source text
  -> ParsedProgram
  -> ProgramIndex
  -> ProgramAnalysis
  -> ComplexityScore
```

Node ids, symbol ids, candidate ids, and byte ranges are valid only inside the
snapshot that created them. After any accepted edit, the reducer rebuilds the
snapshot before generating more candidates.

The reducer has pluggable scheduling algorithms. The default `structured`
algorithm runs fixed-point stages in this order:

1. `RuntimeCostReduction`
2. `AggressiveFunctionElimination`
3. `AggressiveBlockElimination`
4. `StatementAndSiblingReduction`
5. `DeadDeclarationAndOutputCleanup`
6. `ExpressionLiteralTypeCleanup`

After the fixed-point loop, the reducer runs a final one-minimal sweep. A normal
fixed-point round stops only when no candidate is accepted, not when the byte
size happens to stay unchanged. The final sweep forces single-candidate attempts
across the enabled stages. If it accepts anything, it reparses and restarts the
sweep. When the sweep accepts nothing, the output is one-minimal for the current
candidate generator.

After the selected algorithm finishes, the engine always runs `BlankLineCleanup`
as the last stage. It tries to delete blank and whitespace-only lines one at a
time. Each deletion is parsed and checked by the oracle like any other
candidate; if the configured difference is not preserved, the previous accepted
source remains current.

The experimental `weighted-random` algorithm reuses the same parser, candidate
generators, oracle, cache, workspace, and reports. It collects deletable
statement candidates as weighted points, randomly samples one point according to
its current weight, and tests that deletion. Invalid edits, parse failures,
compile/build failures, exit-status changes, and timeouts remove that point from
the current point list. Oracle rejections that keep the program valid lower the
point's neighborhood weights. Accepted deletions are kept, the snapshot is
rebuilt, and nearby point weights are increased with distance decay. The loop
stops after a patience limit based on the remaining point count. It then runs
single-candidate cleanup for dead declarations/output-only code, conservative
long-identifier renames. The shared engine cleanup removes blank lines
afterward.

## Acceptance and Rollback

A candidate is accepted only when all of these are true:

- the edit applies cleanly to the current snapshot source;
- the candidate parses without diagnostics;
- the candidate satisfies the active stage objective, such as lower static
  runtime cost for loop-bound shrinking or lower structural complexity for
  deletion/cleanup phases;
- optional build commands complete successfully;
- A/B run commands complete without timeout;
- output does not exceed the capture limit;
- the configured result diff still exists;
- the exit-status preservation policy still holds;
- every `--confirm-runs` confirmation passes.

Rejected candidates are discarded. Rollback is implicit: the current accepted
source remains in memory and is mirrored under the workspace `accepted/`
directory. Full candidate sources are kept only in the fixed
`trials/current/a` and `trials/current/b` inputs, while every trial also gets a
historical diff under `history/`.

Reducer log lines are prefixed with local time in `[YYYY-MM-DD HH:MM:SS]`
format. The reducer prints the fixed current trial directory once at startup.
Rejection logs use readable phase names plus stable enum names, and include the
group, attempt, candidate count, and oracle reason.

## Java and JavaScript Support

JavaScript candidates include comment cleanup, output-call deletion, top-level
declaration deletion, declaration deletion, block and statement reduction,
control-flow replacement, expression replacement, literal shrinking, and cleanup
edits.

Java candidates include comment cleanup, import deletion, member and local
declaration deletion, output-call deletion, block and method-body replacement,
control-flow replacement, expression replacement, literal shrinking, empty
nested block deletion, and cleanup edits. The Java adapter protects the only
public top-level class by default.

Output noise is prioritized. Java `System.out.print*`, `System.err.print*`,
`java.lang.System.out/err.print*`, and `printStackTrace(System.out/System.err)`
are treated as high-value removal candidates. JavaScript `console.log`,
`console.error`, `process.stdout.write`, and `process.stderr.write` are treated
similarly.

## Reports

`--json-report` serializes session metadata, including:

- input/output paths and language;
- reducer algorithm;
- original and final source sizes;
- total, accepted, and rejected trial counts;
- trial-cache hits;
- baseline and final stdout/stderr diff state plus reported exit-status
  difference metadata;
- per-phase candidate, trial, accept, reject, size, and static runtime-cost
  statistics;
- configured `max_rounds`, `max_trials`, `confirm_runs`, `timeout_ms`, and
  `max_output_bytes`;
- whether the trial limit stopped reduction;
- the kept workspace directory when `--keep-temp` is used.

All human-readable report strings are English.

## Safety

User-provided build and run commands execute on the local machine through
`sh -c`. Code Minimizer is not a sandbox. Run untrusted fuzzing inputs inside a
disposable VM, container, or other external sandbox.

Each trial uses separate A and B side directories so build artifacts do not
cross-contaminate. Commands should write only under `{dir}` or `{output}`.

On Ctrl+C or SIGTERM, Code Minimizer sends SIGTERM to active command process
groups, waits one second, then sends SIGKILL before exiting. Per-command
timeouts use the same cleanup sequence.

## Testing

Run the Rust test suite:

```bash
cargo test
```

Run documentation tests:

```bash
cargo test --doc
```

The real Java fixture under `test_cases/java/**` should be validated with a
full reduction command that uses `--timeout 120s` and `--max-trials 0`. A
successful full validation must confirm that the final source still preserves
the A/B result difference and that removable output noise such as irrelevant
`System.out.println(...)` statements is gone.

## Extending Languages

New languages implement `lang::LanguageAdapter` and register the adapter in
`lang::adapter_for`. An adapter must parse source, build or enrich a
`ProgramIndex`, generate phase-specific `CandidateGroup` values, and optionally
normalize source after accepted edits.

For full reducer behavior, the adapter or shared index builder should expose enough
structure for function/call-site, loop, block, def-use, output, and literal
analysis.

Adapters must not execute commands, compare A/B outputs, manage trial
directories, or decide final interestingness. Those responsibilities belong to
the shared reducer and oracle.

## Project Policies

All runtime output, log text, CLI errors, panic text, test failure messages, and
human-readable JSON report strings in source code must be English.

Public modules, public types, public functions, traits, important enum
variants, and non-trivial implementation paths must have English comments or
doc comments. Comments should explain responsibility, state flow, invariants,
edge cases, and failure handling.

The project documentation is self-contained. Maintainers should be able to
understand usage, architecture, public API, language extension rules, safety
boundaries, and testing requirements from `README.md`, `README.zh-CN.md`,
`docs/API.md`, `docs/API.zh-CN.md`, `docs/ARCHITECTURE.md`, and
`docs/ARCHITECTURE.zh-CN.md` without relying on any design draft.
