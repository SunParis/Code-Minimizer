# Architecture

Code Minimizer is split into four main layers:

1. Application-facing configuration, CLI parsing, logging, and reports.
2. Execution, workspace, and A/B oracle services.
3. Language-independent snapshot reducer.
4. Tree-sitter-backed language adapters.

The reducer and oracle do not depend on Java or JavaScript syntax. Language
specifics stop at `LanguageAdapter`.

## Data Flow

```text
CLI
  -> ReduceConfig
  -> SessionWorkspace
  -> LanguageAdapter::parse
  -> LanguageAdapter::build_index
  -> ProgramSnapshot
  -> Oracle baseline confirmations
  -> selected reducer algorithm
       -> LanguageAdapter::generate_groups
       -> generate language-neutral groups
       -> normalize/order CandidateGroup values
       -> apply EditPlan or chunk Edit::Multi
       -> rebuild ProgramSnapshot
       -> reject if the active stage objective did not improve
       -> Oracle candidate confirmations
       -> accept snapshot or discard candidate
  -> final oracle confirmation
  -> output file and optional JSON report
```

## Module Boundaries

- `app` owns user-facing application concerns: `cli`, immutable `config`,
  `logging`, JSON `report` types, and crate errors.
- `execution` owns shell-command concerns. `command_template` expands command
  placeholders. `runner` is split into outcome types, pipe output draining,
  process spawning, and Unix process-group/signal cleanup.
- `workspace` creates isolated trial directories and mirrors the accepted
  source.
- `oracle` runs A/B commands, repeats confirmations, and decides whether the
  configured result difference still holds. Internally it is split into public
  oracle flow, command execution, result checks/reason formatting, path
  normalization, and decision types.
- `source` owns language-agnostic source utilities: UTF-8-safe `edit`
  application and `output_diff` comparison/normalization.
- `ir` stores snapshot-local nodes, indexes, dependency summaries, sibling
  lists, and complexity scores.
- `reducer` owns pluggable algorithms, candidate grouping, chunk schedules,
  acceptance, rollback, cache use, and final sweeps.
- `lang` owns parsing, tree-sitter conversion, index construction, candidate
  generation, and language-local normalization.

The crate root keeps compatibility re-exports such as `config`, `runner`,
`oracle`, `edit`, `output_diff`, and `report`, but implementation files live in
the responsibility-oriented directories above.

Within `reducer`, shared mechanics are also separated by responsibility:

- `engine` owns the end-to-end session lifecycle.
- `context` is the algorithm-facing service API.
- `trial_runner` owns one candidate attempt path.
- `objective` owns pre-oracle simplification checks.
- `grouping` owns candidate retargeting, edit extraction, and regional
  regrouping.
- `state`, `attempt`, `cache`, `reporting`, and `validation` hold focused
  session state and helper types.
- `algorithms/structured` and `algorithms/weighted_random/*` contain only
  scheduling policy.

## Core Invariants

- The original input file is read-only.
- The current accepted source is the only source that can advance reduction.
- Each accepted source is represented by a fresh `ProgramSnapshot`.
- Snapshot-local ids and byte ranges are never reused after acceptance.
- Candidate source must parse without diagnostics before oracle execution.
- Candidates must satisfy the active stage objective before oracle execution:
  structural phases require lower complexity, while `RuntimeCostReduction` can
  use lower static runtime cost.
- Static analysis only proposes candidates; it never replaces oracle checks.
- Language adapters must not execute commands or decide interestingness.
- The oracle must not generate edits or inspect language syntax.
- Runtime output, CLI errors, report strings, and panic text in source code must
  be English.

## Program Snapshot

A `ProgramSnapshot` contains source text, `ParsedProgram`, `ProgramIndex`,
`ProgramAnalysis`, `ComplexityScore`, source hash, and structure hash.

`ProgramIndex` flattens the parse tree into snapshot-local `NodeId` values and
records parent/child relationships, sibling lists, scopes, symbols, references,
call edges, entry points, and dependency edges when available.

The index can be conservative. It is used for grouping, ordering, dependency
closures, and cleanup opportunities. The oracle is still the only authority on
whether a candidate remains interesting.

## Trial Workspace

Each session creates a temporary workspace under the directory that will contain
the output file:

```text
code-minimizer-*/
  accepted/
    <input file>
  history/
    trial-1.diff
    trial-2.diff
  trials/
    current/
      current.diff
      a/
        <input file>
        out/
      b/
        <input file>
        out/
```

`trials/current` is reused for baseline confirmations, candidate trials, and
final confirmation. It is printed once at session startup. Full candidate source
files are kept only for the current A/B inputs and the last accepted source.
Each attempt also writes `trials/current/current.diff` plus a historical
`history/<trial>.diff` record against the last accepted source.

The workspace is removed when reduction finishes unless `--keep-temp` is used.
Placing it next to the output file avoids large active trial trees in the system
temporary directory.

A and B always run in separate side directories. `{input}`, `{dir}`, and
`{output}` are side-specific. After a candidate is evaluated, build/run outputs
inside the side directories are removed while preserving the current source
files and diff.

## Oracle Semantics

The oracle handles result-only interestingness:

- stdout and stderr stream differences under `--diff-mode`;
- exit-status preservation under `--preserve-exit`;
- build/run completion, timeout, and output truncation.

Timeout is not considered a successful target condition in the current design.
For crash, wrong-result, compiler-diagnostic, and output-diff bugs, timeout
rejects the candidate.

Baseline, accepted candidates, and final output are repeated according to
`--confirm-runs`. Every confirmation must pass.

Before stdout/stderr comparison, side-specific `{input}`, `{dir}`, and
`{output}` paths are normalized to common placeholders. This keeps A/B trial
directory names out of the interestingness decision.

Candidate trials execute side A first, then side B. Progress logs show when A
starts, when A finishes with elapsed time, when B starts, and when B finishes
with elapsed time.

## Reducer Algorithms

Reducer algorithms live under `reducer::algorithms` and implement one trait.
They own scheduling policy only. Shared mechanics such as parsing, candidate
generation, oracle execution, cache use, accepted-source writes, and report
updates are exposed through `reducer::context::ReductionContext` and re-exported
through `reducer::engine::ReductionContext` for compatibility.

The `structured` algorithm is the default deterministic reducer. It is the
original staged flow and is described below.

The `weighted-random` algorithm is experimental and split by behavior:
`point_loop` owns adaptive random sampling, `points` converts generated
candidates into weighted points, `weights` owns patience/sampling/update math,
`rng` owns deterministic pseudo-random scheduling, and `cleanup` and `rename`
own algorithm-specific late deterministic sweeps. It collects simple deletable
statement candidates as weighted points, randomly samples one point by weight,
and validates the deletion through the same oracle path as every other
candidate. Syntax-like failures, build/compile failures, exit-status changes,
and timeouts remove a point from the current list. Valid oracle rejections lower
neighbor weights. Accepted deletions rebuild the snapshot and increase nearby
weights with distance decay. The loop stops after a patience limit based on the
remaining point count, then runs single-candidate cleanup, conservative
long-name rename passes.

## Structured Stages

Normal fixed-point phases run in this order:

1. `RuntimeCostReduction`
2. `AggressiveFunctionElimination`
3. `AggressiveBlockElimination`
4. `StatementAndSiblingReduction`
5. `DeadDeclarationAndOutputCleanup`
6. `ExpressionLiteralTypeCleanup`

`BaselineAndIndex` is represented in the enum for reporting/design clarity, but
normal deletion work starts after baseline validation. `FinalOneMinimalSweep`
is implemented as a separate engine phase that reuses the normal phase
candidates as single attempts.

`BlankLineCleanup` is the shared final stage for every algorithm. It is not fed
by language adapters. The engine creates byte-range candidates for blank and
whitespace-only lines, tries them one at a time, and accepts a deletion only
after parse validation and the oracle confirm that the configured difference is
still present. A rejected blank-line deletion leaves the previous accepted
snapshot current.

A normal fixed-point round stops only when no candidate is accepted in that
round. The engine does not use source byte length as the fixed-point signal,
because some structural simplifications keep the same byte size while enabling
follow-up candidates.

Phase responsibilities:

- `RuntimeCostReduction` ranks high-cost loops by static trip-count hints, body
  size, nested loops, calls, outputs, and depth. It generates deterministic
  loop-bound shrink candidates before more expensive structural phases.
- `AggressiveFunctionElimination` works on function summaries and call sites. It
  can replace calls with deterministic default values, remove standalone call
  statements, and delete function declarations when the oracle allows it.
- `AggressiveBlockElimination` tries whole owner-construct deletion, minimal or
  empty block bodies, and block unwrapping.
- `StatementAndSiblingReduction` works on statement sibling lists. It schedules
  whole-list/chunk/complement attempts and sliding windows for long generated
  blocks before falling back to single statements.
- `DeadDeclarationAndOutputCleanup` deletes dead declarations, output
  operations, and output-only data flow.
- `ExpressionLiteralTypeCleanup` handles literals, trivia, low-risk expression
  replacements, and late cleanup leftovers.

## Candidate Lifecycle

```text
generate CandidateGroup values
  -> normalize duplicate edits
  -> order groups by priority and size
  -> build chunk schedule
  -> apply one chunk or single EditPlan
  -> reject invalid edit/range
  -> parse and index candidate source
  -> reject parse diagnostics
  -> reject if the active stage objective did not improve
  -> check trial cache
  -> run oracle confirmations on cache miss
  -> accept new snapshot or discard candidate
```

After any acceptance, the engine writes the new accepted source, advances the
snapshot id, and regenerates groups for the current stage.

## Candidate Groups and Chunking

Adapters generate stage-level `CandidateGroup` values directly when
language-specific syntax handling is useful. The language-neutral generator also
contributes groups for stages that can be derived from `ProgramAnalysis`.

Group kinds include output noise, sibling lists, declaration families,
dependency closures, literal shrink sets, and control path sets.

Chunk plans try larger edits first where appropriate:

- whole group;
- ddmin-style chunks;
- chunk complements;
- sliding windows for long statement lists;
- single candidates.

The final sweep overrides chunk plans and tries single candidates only. If any
single candidate is accepted, the sweep reparses the source and starts over.

## Complexity Score and Runtime Cost

Most candidates must make structural complexity strictly lower before running
the oracle. `RuntimeCostReduction` additionally accepts loop-bound candidates
when the static runtime-cost total decreases, even if source byte size is
unchanged. Runtime measurements are reported for diagnostics but are not the
sole acceptance signal. The score is lexicographic and includes dimensions such
as:

- output operations;
- top-level declarations;
- functions and types;
- imports;
- statements;
- control-flow nodes and loops;
- nesting depth and block length;
- expressions and calls;
- literal bytes and numeric magnitude;
- dependency edges;
- AST node count;
- source bytes.

Runtime duration is intentionally excluded. Loop or literal simplification can
be accepted because structure or literal magnitude decreases, not because a run
was faster.

## Cache Strategy

The trial cache key is:

```text
sha256(oracle_fingerprint || "\0" || source)
```

The oracle fingerprint includes language, run commands, build commands,
timeout, output cap, confirmation count, diff mode, and exit preservation.

Accepted and rejected oracle outcomes are cached within one session. This keeps
different edit paths from repeatedly executing the same source.

## Progress Logs

Reducer logs are prefixed with local time in `[YYYY-MM-DD HH:MM:SS]` format.
They use human-readable phase names plus the stable enum name, for example
`Runtime cost reduction (RuntimeCostReduction)`. A rejection line includes the
phase, group id, group description, attempt description, candidate count, and
oracle reason. Group skip lines explain the phase purpose so a log entry such
as runtime-cost reduction can be understood without reading the source.

Oracle rejection reasons include the relevant preservation policy and observed
state where useful, such as baseline and candidate exit status for
`--preserve-exit` failures or the configured diff mode and observed stream
diffs when the A/B output difference is lost.

## Process Cleanup

Build and run commands are spawned in their own Unix process group. On Ctrl+C or
SIGTERM, the main process sends SIGTERM to every active command process group,
waits one second, sends SIGKILL to anything still present in those groups, and
then exits with the conventional signal-based status code. Command timeouts use
the same SIGTERM, one-second wait, then SIGKILL sequence for the timed-out
process group.

## Rollback Strategy

Rollback is implicit. The accepted source is held in memory and mirrored under
`accepted/`. Candidate full source exists only in `trials/current/a` and
`trials/current/b`; historical trial records are diffs. If edit application,
parsing, scoring, build, run, diff preservation, exit preservation, or
confirmation fails, the candidate source is discarded.

## Failure Categories

Expected candidate rejections include:

- invalid edit or range;
- parse failure or parse diagnostics;
- candidate was not structurally simpler;
- build failure;
- run timeout;
- output truncation;
- A/B diff lost;
- exit-status policy mismatch;
- command startup or template failure;
- cache-hit rejection.

Unexpected I/O failures, report write failures, unsupported languages, and
invalid initial input are returned as session errors.

## Java and JavaScript Adapters

Both initial adapters use tree-sitter and return lifetime-free `NodeSummary`
trees. They share the default index builder and generate stage-level syntax
candidates where the language-neutral generator needs help.

The Java adapter adds Java-specific safety rules, including protection for the
only public top-level class and method-body replacements with legal default
returns. It prioritizes output statements such as `System.out.print*`,
`System.err.print*`, and `printStackTrace(System.out/System.err)`.

The JavaScript adapter handles console/process output calls, top-level
declarations, block and statement edits, control-flow edits, expression
replacements, literal shrinking, and cleanup candidates.

## Parallelism

The first implementation is sequential. `--jobs` is accepted for API stability
and future work, but candidate verification currently runs one attempt at a
time.

Future parallel trials must only run candidates based on the same current
snapshot. Once any candidate is accepted, all other results from the old
snapshot must be discarded.

## Documentation and Tests

Documentation is part of the architecture. English and Chinese README, API, and
architecture documents must remain complete and consistent when CLI options,
report fields, language extension rules, or public APIs change.

Tests should cover local invariants and end-to-end behavior: edit application,
command templates, runner timeout/truncation, oracle confirmations, cache keys,
complexity scoring, candidate grouping, final sweep, JavaScript reduction, Java
reduction, and real Java fixture validation with `--timeout 120s --max-trials
0`.

Stage-specific tests should also cover loop-bound recognition and shrinking,
static runtime-cost objective checks, call-site context/default-value
generation, block owner/body/unwrap transforms, switch/case grouping, sibling
list ddmin and sliding-window schedules, def-use unused/output-only
classification, stale snapshot-id rejection, and print-noise cleanup.
