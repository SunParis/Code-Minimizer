# API

This document describes the Rust API boundaries used by Code Minimizer. It is
self-contained and should remain useful even when design drafts are removed.

## Configuration

The implementation lives under `app::config`; `config::*` remains a
compatibility re-export from the crate root. `config::ReduceConfig` is the
immutable configuration for one reduction session. It contains:

- language id;
- input and output paths;
- A/B run command templates;
- optional shared or side-specific build command templates;
- per-command timeout;
- output capture limit;
- confirmation count;
- diff mode;
- exit-status preservation policy;
- output-local workspace and JSON report options;
- reducer limits;
- reducer algorithm.

`ReduceConfig::oracle_fingerprint()` returns the stable fingerprint used in
trial cache keys. It includes all oracle-affecting settings, including
`confirm_runs`.

`config::BuildConfig` supports:

- `None`: no build command;
- `Shared(String)`: both sides use the same build template;
- `PerSide { a, b }`: side-specific build templates.

`config::DiffMode` supports `AnyChannel`, `Stdout`, `Stderr`, and `Both`.
Exit status is controlled separately by `config::PreserveExit`.

`config::PreserveExit` supports `None`, `SameClass`, and `Exact`.

`config::ReducerLimits` stores `max_rounds`, `max_trials`, and an optional
`SizeStopConfig`. The CLI converts a user value of `0` for rounds/trials to
`usize::MAX`, meaning no explicit limit.

`config::SizeStopConfig` supports two independent target-size stop conditions:

- `bytes`: stop when the accepted source is at or below this byte count;
- `percent`: stop when accepted/original size is at or below this whole-number
  percentage.

If both are configured, reaching either target stops further candidate
scheduling. Final output writing and final oracle confirmation still run.

`config::parse_byte_size()` accepts plain bytes and `B`, `KB`, `MB`, or `GB`
suffixes. `config::parse_size_percent()` accepts whole percentages written as
`50` or `50%`.

`config::ReductionAlgorithm` supports `Structured` and `WeightedRandom`. The CLI
spells them as `structured` and `weighted-random`.

## Command Templates

The implementation lives under `execution::command_template`;
`command_template::*` remains a compatibility re-export.
`command_template::expand_template()` expands:

- `{input}`;
- `{dir}`;
- `{stem}`;
- `{output}`.

Unknown or unclosed placeholders are rejected before execution. Expanded values
are shell-quoted and the final command is executed through `sh -c`.

## Runner

The implementation lives under `execution::runner`; `runner::*` remains a
compatibility re-export. `runner::CommandRunner` executes one shell command in a
chosen working directory. It returns `CommandOutcome`:

```rust
pub struct CommandOutcome {
    pub status: ExitStatusSummary,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub timed_out: bool,
    pub duration: Duration,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}
```

`InvocationOutcome` groups an optional build outcome and the required run
outcome. The runner drains stdout and stderr on separate reader threads. It
retains only the configured byte cap but continues draining the pipes so verbose
children do not block.

On Unix, command shells are spawned into their own process group. Timeouts send
SIGTERM to that process group, wait one second, then send SIGKILL. The binary
also installs Ctrl+C/SIGTERM handlers that apply the same cleanup sequence to
all active command process groups. After a shutdown signal, the reducer stops
starting new child processes and writes the last accepted source plus the
optional JSON report before exiting with `128 + signal`.

## Oracle

`oracle::Oracle` establishes the baseline and evaluates candidate sources. The
implementation is split into focused files for command execution, result checks,
output path normalization, and decision types, but these are internal module
details.

Baseline validation runs A and B `confirm_runs` times. Every confirmation must
finish without timeout or truncation and must satisfy the configured diff mode.
Reduction does not start unless the baseline is stable.

Candidate evaluation also runs up to `confirm_runs` confirmations. A candidate
is accepted only when every confirmation satisfies:

- build commands succeed when configured;
- run commands do not timeout;
- output is not truncated;
- required result diff remains true;
- exit-status preservation remains true.

Before stdout/stderr comparison, the oracle normalizes side-specific `{input}`,
`{dir}`, and `{output}` trial paths to common placeholders. This prevents
compiler diagnostics from creating a false diff only because A ran in an `a/`
directory and B ran in a `b/` directory.

If any confirmation fails, the candidate is rejected and the rejection reason
mentions the failed confirmation when useful.

`OracleDecision` records acceptance, rejection reason, diff state, and command
outcomes when available.

The session workspace reuses `trials/current` for baseline confirmations,
candidate trials, and final confirmation. Full source files are retained only
for `accepted/<input>`, `trials/current/a/<input>`, and
`trials/current/b/<input>`. Each attempt writes `trials/current/current.diff`
and a historical `history/<trial>.diff` record.

The reducer creates this workspace under the output file's parent directory
rather than under the system temporary directory. It is removed at the end of a
successful or failed session unless `--keep-temp` is enabled.

The implementation lives under `source::output_diff`; `output_diff::*` remains a
compatibility re-export. `output_diff::OutputDiff` is the JSON-reportable diff
state:

```rust
pub struct OutputDiff {
    pub stdout_differs: bool,
    pub stderr_differs: bool,
    pub exit_differs: bool,
}
```

`exit_differs` is report metadata. `--diff-mode` only accepts stdout/stderr
stream modes; use `--preserve-exit` when exit status must remain stable.

## Text Edits

The implementation lives under `source::edit`; `edit::*` remains a
compatibility re-export. `edit::TextRange` is a half-open UTF-8 byte range.

`edit::Edit` supports:

- `Delete(TextRange)`;
- `Replace { range, replacement }`;
- `Multi(Vec<Edit>)`.

`Edit::apply()` rejects invalid ranges, overlapping ranges, and ranges that are
not UTF-8 character boundaries.

Adapters must generate ranges against the exact `ParsedProgram::source` they
received. After any accepted edit, ranges are invalid and must be regenerated
from a fresh snapshot.

## Program Snapshots and IR

`ir::ProgramSnapshot` is the reducer's durable current-program representation:

```rust
pub struct ProgramSnapshot {
    pub version: SnapshotId,
    pub source: String,
    pub file_name: String,
    pub parsed: ParsedProgram,
    pub index: ProgramIndex,
    pub score: ComplexityScore,
    pub analysis: ProgramAnalysis,
    pub source_hash: SourceHash,
    pub structure_hash: StructureHash,
}
```

`SnapshotId` is monotonic. `NodeId`, `ScopeId`, `SymbolId`, candidate ids, and
byte ranges are valid only inside the snapshot that produced them.

`ir::ProgramIndex` contains flattened nodes, scopes, symbols, references, call
edges, entry points, sibling lists, and a conservative dependency graph. The
index is advisory: it improves candidate generation and ordering, but the
oracle remains the final correctness gate.

`ir::ProgramAnalysis` contains advisory summaries for functions, call sites,
loops, blocks, def-use records, and a static `RuntimeCostEstimate`. It is rebuilt
with every accepted snapshot.

Important analysis records include:

- `FunctionSummary`: function node, best-effort name, entry/protected flags,
  body bytes, statement/loop/call counts, and syntactic call sites.
- `CallSiteSummary`: call node, enclosing function, best-effort callee name, and
  context such as statement, assignment, declaration initializer, condition,
  return, or nested expression.
- `LoopSummary`: loop node/kind, body bytes, nested loop count, call/output
  counts, depth, optional estimated trip count, and runtime priority score.
- `BlockSummary`: block node, optional owner construct, owner/block bytes,
  statement/nested-block/loop/call counts, and block priority score.
- `DefUseSummary`: declaration node, best-effort name, reference count,
  output-only reference count, and whether the initializer may have side
  effects.
- `RuntimeCostEstimate`: static loop, call-density, literal, allocation, and
  output-noise cost buckets. It is a scheduling and pre-oracle filter signal,
  not an oracle decision.

`ir::ComplexityScore` is a lexicographic structural score with a static runtime
cost total. Measured runtime duration is not part of acceptance. Most candidates
must produce a strictly lower structural score before the oracle is allowed to
run; `RuntimeCostReduction` candidates may instead pass the pre-oracle filter
when their static runtime-cost total decreases.

High-priority dimensions include output operations, top-level declarations,
functions, types, imports, statements, control-flow nodes, loops, nesting,
expressions, literal size, dependency edges, AST node count, and source bytes.

## Language Adapter

New languages implement:

```rust
pub trait LanguageAdapter: Send + Sync {
    fn language_id(&self) -> &'static str;

    fn parse(&self, source: &str, file_name: &str) -> anyhow::Result<ParsedProgram>;

    fn build_index(&self, parsed: &ParsedProgram) -> anyhow::Result<ProgramIndex>;

    fn generate_groups(
        &self,
        stage: StageKind,
        parsed: &ParsedProgram,
        index: &ProgramIndex,
        score: &ComplexityScore,
    ) -> anyhow::Result<Vec<CandidateGroup>>;

    fn normalize_after_accept(&self, source: &str, file_name: &str) -> anyhow::Result<String>;
}
```

`build_index` has a default implementation that creates a conservative
language-neutral index from the parsed tree.

`generate_groups` is the stage-level candidate interface. The default
implementation returns no adapter-specific groups, which is appropriate when the
language-neutral generator fully covers a stage. Concrete adapters can return
syntax-aware `CandidateGroup` values for the active `StageKind`. The engine also
augments adapter groups with language-neutral groups for loop-bound shrinking,
call-site neutralization, block/statement chunks, dead declarations, and literal
cleanup. Function call-site neutralization is conservative: only call sites with
an exact best-effort callee/function-name match are grouped with a function, and
function declaration deletion is generated later as single-candidate work after
exact external call sites disappear.

Adapters must not execute commands, compare A/B outputs, manage workspaces, or
decide final interestingness.

## Parsed Program

`lang::ParsedProgram` contains:

- current source text;
- trial file name;
- optional tree-sitter tree;
- a lifetime-free `NodeSummary` root;
- parse diagnostics.

Reducer logic uses `NodeSummary` and `ProgramIndex`, not tree-sitter node
references. This keeps parser lifetimes out of the reducer.

`ParseDiagnostic` messages must be English.

## Candidates and Edit Plans

`reducer::candidate::Candidate` contains:

- stable id;
- snapshot id;
- owning `StageKind`;
- optional target `NodeId`;
- English description;
- semantic `EditPlan`;
- compatibility `Edit`;
- priority and estimated byte delta;
- expected effect and risk;
- dependency/conflict/invalidation metadata.
- semantic `TransformKind`.

`TransformKind` values explain the semantic category of a candidate:

- `ShrinkLoopBound`;
- `RemoveCallSite`;
- `ReplaceCallWithValue`;
- `DeleteFunctionDecl`;
- `DeleteWholeConstruct`;
- `EmptyBlockBody`;
- `UnwrapBlock`;
- `DeleteStatementChunk`;
- `DeleteDeadDeclaration`;
- `RemoveOutputOnlyVariable`;
- `ShrinkLiteral`;
- `Cleanup`.

`reducer::edit_plan::EditPlan` records the semantic meaning of the low-level
edits:

```rust
pub struct EditPlan {
    pub id: EditPlanId,
    pub snapshot: SnapshotId,
    pub primary_target: Option<NodeId>,
    pub edits: Vec<Edit>,
    pub intent: EditIntent,
    pub safety: EditSafety,
    pub affected_nodes: Vec<NodeId>,
    pub affected_symbols: Vec<SymbolId>,
    pub expected_effect: CandidateEffect,
}
```

`Edit::Multi` is only a low-level edit container. `EditPlan` is the API that
explains why the edit exists and which snapshot it belongs to.

Candidate descriptions and attempt descriptions are user/report-facing strings
and must be English.

## Candidate Groups

`reducer::group::CandidateGroup` is the unit scheduled by the reducer:

```rust
pub struct CandidateGroup {
    pub id: CandidateGroupId,
    pub snapshot: SnapshotId,
    pub stage: StageKind,
    pub region: Option<NodeId>,
    pub description: String,
    pub kind: CandidateGroupKind,
    pub candidates: Vec<Candidate>,
    pub strategy: GroupStrategy,
    pub chunk_plan: ChunkPlan,
    pub priority: i32,
}
```

Group kinds include `Atomic`, `SiblingList`, `DeclarationFamily`,
`OutputNoise`, `DependencyClosure`, `LiteralShrinkSet`, and `ControlPathSet`.

`ChunkPlan` schedules whole-group, chunk, complement, and single-candidate
attempts. The plan is rebuilt from the current snapshot, so stale ranges are not
reused after acceptance.

Groups that delete function declarations use `SinglesOnly` scheduling. This
keeps declaration removal separate from call-site neutralization and avoids
large mixed chunks whose failures are usually predictable compiler rejections.

## Reducer Engine

`reducer::engine::ReducerEngine` owns the end-to-end workflow:

1. read input source;
2. build the initial `ProgramSnapshot`;
3. reject invalid initial parse diagnostics;
4. establish a stable baseline oracle observation;
5. run the selected reducer algorithm;
6. run the shared final `BlankLineCleanup` stage;
7. write the minimized output;
8. run final confirmation when no shutdown signal was received;
9. write an optional JSON report.

If Ctrl+C or SIGTERM arrives, the engine stops scheduling new candidates, skips
final confirmation, writes the last accepted snapshot to the output path, writes
the optional JSON report, and returns a summary marked with the received signal.

The engine is language-independent. It only sees `LanguageAdapter`,
`ProgramSnapshot`, `CandidateGroup`, `EditPlan`, `ComplexityScore`, and oracle
decisions. Runtime implementation lives under `reducer::runtime`: `engine` owns
the session lifecycle, `context` owns the algorithm-facing API, `trial_runner`
owns one candidate attempt, and `objective` owns pre-oracle simplification
checks. Candidate data types live under `reducer::model`; candidate planning,
retargeting, ordering, and regional regrouping live under `reducer::planning`.
Algorithm implementations live under `reducer::algorithms` and use
`ReductionContext` to access shared candidate generation, oracle trials,
accepted-source updates, limit checks, and stage report recording.

Limit checks are centralized in `ReductionContext`. Algorithms call the same
stop check they use for `max_trials`; it also observes configured size targets
against the current accepted snapshot. When a size target is reached, algorithms
stop scheduling new candidates and the engine proceeds to final write and final
oracle confirmation.

## Algorithms

`reducer::algorithms::ReducerAlgorithm` is the scheduling interface:

```rust
pub trait ReducerAlgorithm {
    fn run(&self, context: &mut ReductionContext<'_>) -> anyhow::Result<()>;
}
```

The `structured` implementation runs the deterministic phase order and final
single-candidate sweep. The `weighted-random` implementation is split into
`point_loop`, `points`, `weights`, `rng`, `cleanup`, and `rename`; together they
sample deletable statement points by adaptive weights, then run cleanup and
conservative long-name rename sweeps. Blank-line removal is shared engine
cleanup that runs after every algorithm.

## Structured Phase Order

Normal fixed-point stages are:

1. `RuntimeCostReduction`
2. `AggressiveFunctionElimination`
3. `AggressiveBlockElimination`
4. `StatementAndSiblingReduction`
5. `DeadDeclarationAndOutputCleanup`
6. `ExpressionLiteralTypeCleanup`

The final one-minimal sweep reuses these stages but forces single-candidate
attempts.

After any algorithm-specific work, `BlankLineCleanup` tries blank and
whitespace-only line deletions one at a time. Each candidate is still parsed and
checked by the oracle; rejection means the current accepted source is left
unchanged.

## Report Format

The implementation lives under `app::report`; `report::*` remains a
compatibility re-export. `report::ReductionReport` is serialized when
`--json-report` is used. It contains:

- input and output paths;
- language;
- reducer algorithm;
- original and final sizes;
- total, accepted, and rejected trial counts;
- cache hits;
- configured `max_rounds`, `max_trials`, `confirm_runs`, `timeout_ms`, and
  `max_output_bytes`;
- configured `stop_size_bytes` and `stop_size_percent`;
- baseline and final diff state;
- per-phase reports;
- trial-limit, size-limit, and shutdown-signal status;
- kept workspace directory when available.

`interrupted_by_signal` is `null` for normal runs. When it is an integer, the
output source is the last accepted snapshot already validated before shutdown;
the report may omit final confirmation effects because no new child process is
started after the signal is observed.

`StageReport` records round, stage, generated candidates, trials, accepted,
rejected, size before/after, and static runtime-cost total before/after.

## Adding a Language

To add a language:

1. create `src/lang/<language>.rs`;
2. implement `LanguageAdapter`;
3. use a real parser or structured parser wrapper;
4. return `ParsedProgram` with English diagnostics;
5. implement or enrich `build_index`;
6. generate stage-level `CandidateGroup` values where language-specific syntax
   handling is useful;
7. register the adapter in `lang::adapter_for`;
8. add parser, candidate, group, and integration tests;
9. update both English and Chinese documentation.

Adapters should prefer syntax-preserving replacements. They may still generate
candidates that fail type checking or runtime checks; those failures are
ordinary oracle rejections.
