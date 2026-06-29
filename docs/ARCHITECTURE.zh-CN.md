# 架构

Code Minimizer 分为四个主要层次：

1. 面向应用层的配置、CLI、日志和 report。
2. execution、workspace 和 A/B oracle 服务。
3. 与语言无关的 snapshot reducer。
4. 基于 tree-sitter 的语言适配器。

reducer 和 oracle 不依赖 Java 或 JavaScript 语法。语言相关逻辑止于 `LanguageAdapter`。

## 数据流

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
       -> 当前 stage objective 没有下降则拒绝
       -> Oracle candidate confirmations
       -> accept snapshot 或 discard candidate
  -> final oracle confirmation
  -> output file and optional JSON report
```

## 模块边界

- `app`：负责用户可见的应用层逻辑，包括 `cli`、不可变 `config`、`logging`、JSON `report` 类型和错误类型。
- `execution`：负责 shell 命令相关逻辑。`command_template` 展开命令占位符；`runner` 进一步拆成 outcome
  类型、pipe 输出 drain、process 启动，以及 Unix process group/signal 清理。
- `workspace`：创建隔离 trial 目录，并镜像 accepted source。
- `oracle`：运行 A/B 命令、重复确认，并判断配置的结果差异是否仍成立。内部拆成公开 oracle 流程、命令执行、
  结果检查/原因文案、路径归一化和 decision 类型。
- `source`：负责语言无关 source 工具，包括 UTF-8 安全的 `edit` 应用和 `output_diff` 比较/归一化。
- `ir`：保存 snapshot-local node、index、dependency summary、sibling list 和 complexity score。
- `reducer`：负责可插拔算法、候选分组、chunk schedule、接受、回退、cache 使用和 final sweep。
- `lang`：负责解析、tree-sitter 转换、index 构建、候选生成和语言本地 normalize。

crate root 保留 `config`、`runner`、`oracle`、`edit`、`output_diff`、`report` 等兼容重导出，但实际实现文件位于上面这些
按职责划分的目录中。

`reducer` 内部也按职责拆分：

- `engine`：负责端到端 session 生命周期。
- `context`：算法可使用的共享服务 API。
- `trial_runner`：负责一次 candidate attempt 的完整路径。
- `objective`：负责 oracle 前的简化目标检查。
- `grouping`：负责 candidate retarget、edit 提取和按结构区域重分组。
- `state`、`attempt`、`cache`、`reporting` 和 `validation`：保存聚焦的 session state 和 helper 类型。
- `algorithms/structured` 和 `algorithms/weighted_random/*`：只包含调度策略。

## 核心不变量

- 原始输入文件只读。
- 只有当前 accepted source 可以推进 reduction。
- 每个 accepted source 都表示为新的 `ProgramSnapshot`。
- snapshot-local id 和 byte range 在 accepted edit 后不复用。
- candidate source 必须先成功解析且没有 diagnostics，才会执行 oracle。
- candidate 必须满足当前 stage objective 才会执行 oracle：结构阶段要求复杂度下降，`RuntimeCostReduction`
  可以使用静态 runtime cost 下降。
- 静态分析只提出候选，不替代 oracle。
- 语言适配器不能执行命令，也不能判断 interestingness。
- oracle 不能生成 edit，也不能检查语言语法。
- 源码中的运行时输出、CLI 错误、report 字符串和 panic 文案必须使用英文。

## Program Snapshot

`ProgramSnapshot` 包含 source text、`ParsedProgram`、`ProgramIndex`、`ProgramAnalysis`、`ComplexityScore`、
source hash 和 structure hash。

`ProgramIndex` 会把 parse tree 展平为 snapshot-local `NodeId`，并记录 parent/child 关系、sibling list、
scope、symbol、reference、call edge、entry point 和可用的 dependency edge。

index 可以是保守的。它用于 grouping、ordering、dependency closure 和 cleanup 机会发现。候选是否仍然
interesting 只由 oracle 决定。

## Trial Workspace

每个 session 会在输出文件所在目录下创建临时 workspace：

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

`trials/current` 会被 baseline confirmation、candidate trial 和 final confirmation 复用，并且只在
session 开始时打印一次。完整 candidate source 只保留当前 A/B 输入和上一轮 accepted source。每次尝试还会写入
`trials/current/current.diff`，并在 `history/<trial>.diff` 中保存相对上一轮 accepted source 的历史 diff。

reduction 结束后，除非使用 `--keep-temp`，workspace 会被删除。把 workspace 放在输出文件旁边，可以避免大量运行中
trial tree 堆在系统临时目录。

A 和 B 总是在独立 side 目录运行。`{input}`、`{dir}` 和 `{output}` 都是 side-specific。candidate 评估结束后，
side 目录里的 build/run 输出会被清理，但会保留当前源码文件和 diff。

## Oracle 语义

oracle 处理结果型 interestingness：

- `--diff-mode` 下的 stdout/stderr stream 差异；
- `--preserve-exit` 下的退出状态保持；
- build/run 完成、timeout 和输出截断。

当前设计不把 timeout 视为成功目标条件。对 crash、wrong result、compiler diagnostic 和 output diff 类 bug，
timeout 会拒绝候选。

baseline、accepted candidate 和 final output 会按 `--confirm-runs` 重复确认。每次确认都必须通过。

比较 stdout/stderr 前，side-specific 的 `{input}`、`{dir}` 和 `{output}` path 会被归一化成共同占位符。
这样 A/B trial 目录名不会参与 interestingness 判断。

candidate trial 会先执行 side A，再执行 side B。进度日志会显示 A 开始、A 完成及耗时、B 开始、B 完成及耗时。

## Reducer 算法

Reducer 算法位于 `reducer::algorithms`，并实现同一个 trait。算法只负责调度策略；解析、候选生成、oracle
执行、cache 使用、accepted source 写入和 report 更新都通过共享的 `reducer::context::ReductionContext` 暴露。
为了兼容已有调用路径，`reducer::engine::ReductionContext` 也会重导出同一类型。

`structured` 是默认的确定性 reducer，也就是原来的分阶段流程，下面会描述它的 stage。

`weighted-random` 是实验性算法，并按行为拆分：`point_loop` 负责自适应随机抽样，`points` 把生成的候选转成带权重
点，`weights` 负责 patience、抽样和权重更新数学，`rng` 负责确定性伪随机调度，`cleanup` 和 `rename`
负责该算法自己的后期确定性 sweep。它把简单可删除 statement candidate 收集成带权重点，按权重随机抽取一个点，
并通过同一条 oracle 路径验证删除。语法类失败、build/compile failure、exit-status change 和 timeout 会把点从
当前列表移除。程序仍有效但 oracle 不通过的拒绝会降低邻近点权重。接受删除后会重建 snapshot，并按距离衰减提高
附近点权重。随机循环在达到基于剩余点数量的 patience limit 后停止，随后执行单候选 cleanup、保守长名称 rename
pass。

## Structured Stage

普通固定点阶段按以下顺序执行：

1. `RuntimeCostReduction`
2. `AggressiveFunctionElimination`
3. `AggressiveBlockElimination`
4. `StatementAndSiblingReduction`
5. `DeadDeclarationAndOutputCleanup`
6. `ExpressionLiteralTypeCleanup`

`BaselineAndIndex` 保留在 enum 中用于报告和设计表达，但普通删除工作从 baseline validation 之后开始。
`FinalOneMinimalSweep` 是独立 engine phase，会复用普通 stage 候选并强制单候选尝试。

`BlankLineCleanup` 是所有算法共享的最终阶段，不由语言 adapter 生成。engine 会为每个空白行和只有空白的行创建
byte-range candidate，逐个尝试删除，并且只有在解析验证和 oracle 都确认配置的差异仍然存在时才接受。被拒绝的
空白行删除不会改变当前 accepted snapshot。

普通固定点轮次只在该轮没有接受任何候选时停止。engine 不使用源码字节数作为固定点信号，因为有些结构化简会保持相同
byte size，但会打开后续候选。

各阶段职责：

- `RuntimeCostReduction` 根据静态 trip-count hint、body size、nested loop、call、output 和 depth 对高成本
  loop 排序，并在更昂贵的结构化阶段前生成确定性的 loop-bound shrink 候选。
- `AggressiveFunctionElimination` 基于 function summary 和 call site 工作。它可以把 call 替换为确定性默认值、
  删除独立 call statement，并在 oracle 允许时删除 function declaration。
- `AggressiveBlockElimination` 尝试删除整个 owner construct、替换成 minimal/empty block body，以及 unwrap block。
- `StatementAndSiblingReduction` 处理 statement sibling list。它会在 single statement 前尝试 whole-list、
  chunk、complement 和适用于长生成器 block 的 sliding window。
- `DeadDeclarationAndOutputCleanup` 删除 dead declaration、output operation 和 output-only data flow。
- `ExpressionLiteralTypeCleanup` 处理 literal、trivia、低风险 expression replacement 和后期 cleanup leftovers。

## Candidate 生命周期

```text
generate CandidateGroup values
  -> normalize duplicate edits
  -> order groups by priority and size
  -> build chunk schedule
  -> apply one chunk or single EditPlan
  -> invalid edit/range 则拒绝
  -> parse and index candidate source
  -> parse diagnostics 则拒绝
  -> 当前 stage objective 没有下降则拒绝
  -> check trial cache
  -> cache miss 时运行 oracle confirmations
  -> accept new snapshot 或 discard candidate
```

任何候选被接受后，engine 会写入新的 accepted source、推进 snapshot id，并为当前 stage 重新生成 group。

## Candidate Group 与 Chunk

adapter 可以在语言语法需要专门处理时直接生成 stage-level `CandidateGroup`。语言无关生成器也会根据
`ProgramAnalysis` 为可覆盖的阶段补充 group。

group kind 包括 output noise、sibling list、declaration family、dependency closure、literal shrink set
和 control path set。

chunk plan 会在适合时先尝试更大的 edit：

- whole group；
- ddmin-style chunk；
- chunk complement；
- 长 statement list 的 sliding window；
- single candidate。

final sweep 会覆盖 chunk plan，只尝试 single candidate。如果接受了任何单候选，就重新解析源码并从头开始 sweep。

## Complexity Score 和 Runtime Cost

大多数候选必须让结构复杂度严格降低，才会运行 oracle。`RuntimeCostReduction` 额外允许 loop-bound
候选在静态 runtime-cost total 下降时进入 oracle，即使源码字节数没有变化。动态运行时间只用于报告和排查，
不能作为唯一接受信号。score 是字典序，包含：

- output operation；
- top-level declaration；
- function 和 type；
- import；
- statement；
- control-flow node 和 loop；
- nesting depth 和 block length；
- expression 和 call；
- literal byte 和 numeric magnitude；
- dependency edge；
- AST node count；
- source byte。

运行时间被刻意排除。loop 或 literal 化简可以因为结构或 literal magnitude 下降而被接受，但不能因为一次运行更快而被接受。

## Cache 策略

trial cache key 是：

```text
sha256(oracle_fingerprint || "\0" || source)
```

oracle fingerprint 包含 language、run commands、build commands、timeout、output cap、confirmation count、
diff mode 和 exit preservation。

同一个 session 内会缓存 accepted 和 rejected oracle outcome，避免不同 edit 路径反复执行同一源码。

## 进度日志

reducer 日志会带本地时间前缀，格式为 `[YYYY-MM-DD HH:MM:SS]`。日志会同时使用可读阶段名和稳定 enum 名，
例如 `Runtime cost reduction (RuntimeCostReduction)`。拒绝日志会包含 phase、group id、group 描述、
attempt 描述、candidate 数量和 oracle 原因。跳过 group 的日志会解释该 phase 的目的，因此即使不阅读源码，
也能理解 runtime-cost reduction 这类日志在尝试什么。

oracle 的拒绝原因会在有帮助时包含相关保持策略和观察到的状态，例如 `--preserve-exit` 失败时的 baseline/candidate
退出状态，或 A/B 输出差异丢失时的 diff mode 与实际 stdout/stderr/exit 差异。

## 进程清理

build 和 run 命令会在独立 Unix process group 中启动。收到 Ctrl+C 或 SIGTERM 时，主进程会先向所有运行中的
命令 process group 发送 SIGTERM，等待 1 秒，再向仍在这些 group 中的进程发送 SIGKILL，然后用约定的 signal
退出码退出。命令 timeout 也使用同样的 SIGTERM、等待 1 秒、SIGKILL 流程清理对应 process group。

## 回退策略

回退是隐式的。accepted source 保存在内存中，并镜像到 `accepted/`。candidate 完整源码只存在于
`trials/current/a` 和 `trials/current/b`；历史 trial 记录保存为 diff。如果 edit 应用、解析、评分、build、
run、diff 保持、exit 保持或 confirmation 失败，candidate source 会被丢弃。

## 失败分类

预期的候选拒绝包括：

- invalid edit 或 range；
- parse failure 或 parse diagnostics；
- candidate 不是结构上更简单；
- build failure；
- run timeout；
- output truncation；
- A/B diff lost；
- exit-status policy mismatch；
- command startup 或 template failure；
- cache-hit rejection。

意外 I/O 失败、report 写入失败、不支持的语言和非法初始输入会作为 session error 返回。

## Java 和 JavaScript Adapter

两个初始 adapter 都使用 tree-sitter，并返回无生命周期负担的 `NodeSummary` tree。它们共享默认 index builder，
并在语言无关生成器需要帮助的地方生成 stage-level syntax candidate。

Java adapter 增加 Java 特定安全规则，例如保护唯一 public 顶层类，以及为方法体 replacement 生成合法默认
return。它会优先处理 `System.out.print*`、`System.err.print*` 和
`printStackTrace(System.out/System.err)` 等输出语句。

JavaScript adapter 处理 console/process 输出调用、顶层声明、block 和 statement edit、控制流 edit、
表达式 replacement、literal shrink 和 cleanup candidate。

## 并行能力

第一版是顺序执行。`--jobs` 为 API 稳定性和后续工作保留，但当前 candidate verification 一次只运行一个 attempt。

未来并行 trial 只能验证基于同一个 current snapshot 的候选。一旦任何候选被接受，所有来自旧 snapshot 的其他结果都必须丢弃。

## 文档和测试

文档是架构的一部分。当 CLI 参数、report 字段、语言扩展规则或公开 API 改变时，中英文 README、API 和 architecture
文档必须保持完整且一致。

测试应覆盖局部不变量和端到端行为：edit 应用、命令模板、runner timeout/truncation、oracle confirmation、
cache key、complexity scoring、candidate grouping、final sweep、JavaScript reduction、Java reduction，以及
使用 `--timeout 120s --max-trials 0` 的真实 Java fixture 验证。

阶段专项测试还应覆盖 loop-bound 识别和缩小、静态 runtime-cost objective、call-site context/default-value
生成、block owner/body/unwrap transform、switch/case grouping、sibling list ddmin 和 sliding-window schedule、
def-use unused/output-only 分类、stale snapshot-id 拒绝，以及 print-noise cleanup。
