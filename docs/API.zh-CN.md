# API

本文档说明 Code Minimizer 的主要 Rust API 边界。它是自包含的维护和扩展参考，不依赖任何设计草稿。

## 配置

实现位于 `app::config`；crate root 的 `config::*` 仍作为兼容重导出保留。`config::ReduceConfig` 是一次
reduction session 的不可变配置，包含：

- 语言 id；
- input/output path；
- A/B run 命令模板；
- 可选的共用或 side-specific build 命令模板；
- 单命令 timeout；
- 输出捕获上限；
- 确认次数；
- diff mode；
- 退出状态保持策略；
- output-local workspace 和 JSON report 选项；
- reducer 限制；
- reducer 算法。

`ReduceConfig::oracle_fingerprint()` 返回 trial cache key 使用的稳定指纹。它包含所有会影响 oracle 的设置，
包括 `confirm_runs`。

`config::BuildConfig` 支持：

- `None`：没有 build 命令；
- `Shared(String)`：A/B 使用同一个 build 模板；
- `PerSide { a, b }`：A/B 使用不同 build 模板。

`config::DiffMode` 支持 `AnyChannel`、`Stdout`、`Stderr` 和 `Both`。
退出状态不属于 diff mode；需要保持退出状态时使用 `config::PreserveExit`。

`config::PreserveExit` 支持 `None`、`SameClass` 和 `Exact`。

`config::ReducerLimits` 保存 `max_rounds` 和 `max_trials`。CLI 会把用户传入的 `0` 转成 `usize::MAX`，
表示不设置显式上限。

`config::ReductionAlgorithm` 支持 `Structured` 和 `WeightedRandom`。CLI 拼写是 `structured` 和
`weighted-random`。

## 命令模板

实现位于 `execution::command_template`；`command_template::*` 仍作为兼容重导出保留。
`command_template::expand_template()` 展开：

- `{input}`；
- `{dir}`；
- `{stem}`；
- `{output}`。

未知或未闭合占位符会在执行前被拒绝。展开值会进行 shell quote，最终命令通过 `sh -c` 执行。

## Runner

实现位于 `execution::runner`；`runner::*` 仍作为兼容重导出保留。`runner::CommandRunner` 在指定工作目录执行一条
shell 命令，返回 `CommandOutcome`：

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

`InvocationOutcome` 组合可选 build outcome 和必需 run outcome。runner 会用独立 reader thread 持续 drain
stdout/stderr。它只保留配置上限内的字节，但会继续读取 pipe，避免高输出量子进程阻塞。

在 Unix 上，命令 shell 会在独立 process group 中启动。timeout 时会向该 process group 发送 SIGTERM，
等待 1 秒，再发送 SIGKILL。binary 也会安装 Ctrl+C/SIGTERM handler，在退出前对所有运行中的命令
process group 执行同样的清理流程。

## Oracle

`oracle::Oracle` 负责建立 baseline 并评估 candidate source。oracle 实现拆成命令执行、结果检查、输出路径归一化和
decision 类型等聚焦文件，但这些属于内部模块细节。

baseline validation 会运行 A/B `confirm_runs` 次。每次确认都必须在无 timeout、无截断的情况下完成，并满足配置的
diff mode。baseline 不稳定时不会进入 reduction。

candidate evaluation 同样最多运行 `confirm_runs` 次确认。候选只有在每次确认都满足以下条件时才会被接受：

- 配置了 build 时 build 成功；
- run 命令没有 timeout；
- 输出没有被截断；
- 要求的结果差异仍成立；
- 退出状态保持策略仍成立。

比较 stdout/stderr 前，oracle 会把 A/B side-specific 的 `{input}`、`{dir}` 和 `{output}` trial path
归一化成共同占位符，避免 compiler diagnostic 只因为 A 在 `a/` 目录、B 在 `b/` 目录运行而形成假 diff。

任意一次确认失败，候选都会被拒绝；必要时拒绝原因会包含失败的 confirmation 编号。

`OracleDecision` 记录是否接受、拒绝原因、diff 状态以及可用的命令结果。

session workspace 会复用 `trials/current` 来执行 baseline confirmation、candidate trial 和 final
confirmation。完整源码文件只保留在 `accepted/<input>`、`trials/current/a/<input>` 和
`trials/current/b/<input>`。每次尝试会写入 `trials/current/current.diff`，并在
`history/<trial>.diff` 保存历史记录。

reducer 会在输出文件的父目录下创建该 workspace，而不是放到系统临时目录。除非启用 `--keep-temp`，session
成功或失败结束后都会删除该 workspace。

实现位于 `source::output_diff`；`output_diff::*` 仍作为兼容重导出保留。`output_diff::OutputDiff` 是会进入
JSON report 的 diff 状态：

```rust
pub struct OutputDiff {
    pub stdout_differs: bool,
    pub stderr_differs: bool,
    pub exit_differs: bool,
}
```

`exit_differs` 只是 report metadata。`--diff-mode` 只接受 stdout/stderr stream 模式；如果需要保持退出状态，
请使用 `--preserve-exit`。

## Text Edit

实现位于 `source::edit`；`edit::*` 仍作为兼容重导出保留。`edit::TextRange` 是半开 UTF-8 byte range。

`edit::Edit` 支持：

- `Delete(TextRange)`；
- `Replace { range, replacement }`；
- `Multi(Vec<Edit>)`。

`Edit::apply()` 会拒绝非法 range、交叠 range，以及不在 UTF-8 字符边界上的 range。

adapter 必须基于收到的 `ParsedProgram::source` 生成 range。任何 edit 被接受后，旧 range 都失效，必须基于新的
snapshot 重新生成。

## Program Snapshot 与 IR

`ir::ProgramSnapshot` 是 reducer 持久持有的当前程序表示：

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

`SnapshotId` 单调递增。`NodeId`、`ScopeId`、`SymbolId`、candidate id 和 byte range 只在生成它们的 snapshot
内有效。

`ir::ProgramIndex` 包含扁平节点、scope、symbol、reference、call edge、entry point、sibling list 和保守
dependency graph。index 是 advisory 的：它用于改进候选生成和排序，但最终正确性仍由 oracle 决定。

`ir::ProgramAnalysis` 包含 reducer 使用的 advisory summary：function、call site、loop、block、def-use 和静态
`RuntimeCostEstimate`。每次 accepted snapshot 后都会重建。

重要的 analysis record 包括：

- `FunctionSummary`：function node、best-effort name、entry/protected 标记、body bytes、statement/loop/call
  数量和语法 call site。
- `CallSiteSummary`：call node、enclosing function、best-effort callee name，以及 statement、assignment、
  declaration initializer、condition、return、nested expression 等 context。
- `LoopSummary`：loop node/kind、body bytes、nested loop 数、call/output 数、depth、可选 estimated trip count
  和 runtime priority score。
- `BlockSummary`：block node、可选 owner construct、owner/block bytes、statement/nested-block/loop/call 数量
  和 block priority score。
- `DefUseSummary`：declaration node、best-effort name、reference 数、output-only reference 数，以及
  initializer 是否可能有副作用。
- `RuntimeCostEstimate`：静态 loop、call-density、literal、allocation 和 output-noise cost bucket。它只用于调度
  和 oracle 前过滤，不是 oracle decision。

`ir::ComplexityScore` 是字典序结构复杂度评分，并包含静态 runtime-cost total。实测运行时间不作为接受条件。
大多数候选必须先产生严格更小的结构 score，reducer 才会运行 oracle；`RuntimeCostReduction` 候选也可以在静态
runtime-cost total 下降时通过 oracle 前过滤。

高优先级维度包括输出操作、顶层声明、函数、类型、import、statement、控制流节点、loop、嵌套深度、表达式、
literal 大小、dependency edge、AST node 数和源码字节数。

## Language Adapter

新增语言需要实现：

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

`build_index` 有默认实现，会从 parse tree 构建保守的语言无关 index。

`generate_groups` 是阶段级候选接口。默认实现不返回 adapter 专属 group；如果语言无关生成器已经覆盖某个阶段，
这是合适的。具体 adapter 可以为当前 `StageKind` 返回语法敏感的 `CandidateGroup`。engine 还会额外合并
语言无关的 group，包括 loop-bound shrink、call-site neutralization、block/statement chunk、dead declaration
和 literal cleanup。

adapter 不应该执行命令、比较 A/B 输出、管理 workspace 或决定最终 interestingness。

## Parsed Program

`lang::ParsedProgram` 包含：

- 当前源码文本；
- trial 文件名；
- 可选 tree-sitter tree；
- 无生命周期负担的 `NodeSummary` root；
- parse diagnostics。

reducer 使用 `NodeSummary` 和 `ProgramIndex`，而不是直接使用 tree-sitter node 引用。这样 parser lifetime
不会泄漏到 reducer。

`ParseDiagnostic` 消息必须使用英文。

## Candidate 与 EditPlan

`reducer::candidate::Candidate` 包含：

- 稳定 id；
- snapshot id；
- 所属 `StageKind`；
- 可选目标 `NodeId`；
- 英文描述；
- 语义 `EditPlan`；
- 兼容用 `Edit`；
- priority 和估计 byte delta；
- 预期 effect 和 risk；
- dependency/conflict/invalidation metadata。
- 语义 `TransformKind`。

`TransformKind` 表示候选的语义类别：

- `ShrinkLoopBound`；
- `RemoveCallSite`；
- `ReplaceCallWithValue`；
- `DeleteFunctionDecl`；
- `DeleteWholeConstruct`；
- `EmptyBlockBody`；
- `UnwrapBlock`；
- `DeleteStatementChunk`；
- `DeleteDeadDeclaration`；
- `RemoveOutputOnlyVariable`；
- `ShrinkLiteral`；
- `Cleanup`。

`reducer::edit_plan::EditPlan` 记录低层 edit 的语义：

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

`Edit::Multi` 只是低层 edit 容器。`EditPlan` 才是解释 edit 为什么存在、属于哪个 snapshot 的 API。

candidate description 和 attempt description 是用户/report 可见字符串，必须使用英文。

## Candidate Group

`reducer::group::CandidateGroup` 是 reducer 调度的单位：

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

group kind 包括 `Atomic`、`SiblingList`、`DeclarationFamily`、`OutputNoise`、`DependencyClosure`、
`LiteralShrinkSet` 和 `ControlPathSet`。

`ChunkPlan` 会安排 whole-group、chunk、complement 和 single-candidate 尝试。plan 基于当前 snapshot 重建，
因此 accepted edit 后不会复用 stale range。

## Reducer Engine

`reducer::engine::ReducerEngine` 负责端到端流程：

1. 读取 input source；
2. 构建初始 `ProgramSnapshot`；
3. 拒绝有 parse diagnostics 的初始源码；
4. 建立稳定的 baseline oracle observation；
5. 运行选定的 reducer 算法；
6. 写入最小化输出；
7. 运行最终确认；
8. 写入可选 JSON report。

engine 与语言无关。它只处理 `LanguageAdapter`、`ProgramSnapshot`、`CandidateGroup`、`EditPlan`、
`ComplexityScore` 和 oracle decision。实现已经拆分：`engine` 负责 session 生命周期，`context` 负责算法可用 API，
`trial_runner` 负责一次 candidate attempt，`objective` 负责 oracle 前简化目标检查，`grouping` 负责 candidate
retarget 和按结构区域重分组。算法实现位于 `reducer::algorithms`，并通过 `ReductionContext` 访问共享的候选生成、
oracle trial、accepted-source 更新、limit 检查和 stage report 记录。

## 算法

`reducer::algorithms::ReducerAlgorithm` 是调度接口：

```rust
pub trait ReducerAlgorithm {
    fn run(&self, context: &mut ReductionContext<'_>) -> anyhow::Result<()>;
}
```

`structured` 实现确定性 phase 顺序和 final single-candidate sweep。`weighted-random` 拆成 `point_loop`、
`points`、`weights`、`rng`、`cleanup` 和 `rename`；这些模块共同实现按自适应权重抽样可删除 statement 点，
然后执行 cleanup 和保守长名称 rename sweep。空白行删除是共享 engine cleanup，会在所有算法结束后运行。

## Structured Phase 顺序

普通固定点阶段：

1. `RuntimeCostReduction`
2. `AggressiveFunctionElimination`
3. `AggressiveBlockElimination`
4. `StatementAndSiblingReduction`
5. `DeadDeclarationAndOutputCleanup`
6. `ExpressionLiteralTypeCleanup`

final one-minimal sweep 会复用这些阶段，但强制使用 single-candidate attempt。

所有算法专属工作结束后，`BlankLineCleanup` 会逐个尝试删除空白行和只有空白的行。每个 candidate 仍然会经过解析和
oracle 检查；被拒绝时当前 accepted source 不变。

## Report 格式

report 实现位于 `app::report`；`report::*` 仍作为兼容重导出保留。使用 `--json-report` 时会序列化
`report::ReductionReport`。它包含：

- input 和 output path；
- language；
- reducer 算法；
- original/final size；
- total/accepted/rejected trial 数；
- cache hit 数；
- 配置的 `max_rounds`、`max_trials`、`confirm_runs`、`timeout_ms` 和 `max_output_bytes`；
- baseline/final diff 状态；
- per-phase report；
- trial-limit 状态；
- 可用时的 kept workspace directory。

`StageReport` 记录 round、stage、generated candidates、trials、accepted、rejected、size before/after，
以及静态 runtime-cost total before/after。

## 添加新语言

添加语言需要：

1. 创建 `src/lang/<language>.rs`；
2. 实现 `LanguageAdapter`；
3. 使用真实 parser 或结构化 parser wrapper；
4. 返回带英文 diagnostics 的 `ParsedProgram`；
5. 实现或增强 `build_index`；
6. 在语言语法需要专门处理时生成 stage-level `CandidateGroup`；
7. 在 `lang::adapter_for` 注册；
8. 添加 parser、candidate、group 和 integration tests；
9. 同步更新英文和中文文档。

adapter 应优先生成语法保持的替换。它仍然可以生成类型检查或运行时失败的候选；这些失败属于普通 oracle rejection。
