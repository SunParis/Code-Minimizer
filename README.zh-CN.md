# Code Minimizer

Code Minimizer 是一个使用 Rust 编写的单文件程序化简器，面向编译器 /
运行时 fuzzing。它接收一个源码文件、语言类型、两条运行指令 A/B、可选编译指令，并在保持结果可观察
A/B 差异的前提下缩小源码。

第一版通过 tree-sitter 支持 Java 和 JavaScript。

## 项目目标

fuzzing 生成的输入通常是很大的随机程序。Code Minimizer 的目标是在保持 interestingness 条件成立的前提下，
把这样的程序化简成更小的 PoC。当前实现中的 interestingness 是 stdout、stderr、两者同时，或退出状态类别上的
A/B 差异，并可选择保持退出状态策略。

这是结构化 reducer。它会解析源码、构建语言无关 snapshot、生成按 phase 组织的候选组、应用 edit、重新解析候选，
并让 oracle 判断候选是否仍然触发配置的结果差异。它不是只删除空白和注释的工具。

## 非目标

- 只处理单个源码文件。
- 不证明 bug 的语义根因。
- 不保证数学意义上的全局最小。
- 不为用户命令提供沙箱。
- 对非 hang 类 bug，不把实际运行更快作为接受条件。
- 标识符重命名只会由实验性的 `weighted-random` 算法作为保守的后期 cleanup pass 尝试。

## 安装

```bash
cargo build --release
```

编译后的二进制位于 `target/release/code-minimizer`。

## CLI 用法

JavaScript：

```bash
code-minimizer reduce \
  --lang js \
  --input ./case.js \
  --run-a "node {input}" \
  --run-b "node --jitless {input}" \
  --timeout 5s \
  --output ./case.min.js
```

Java，直接运行源码文件：

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

Java，共用编译指令：

```bash
code-minimizer reduce \
  --lang java \
  --input ./Test.java \
  --build "javac {input}" \
  --run-a "java -cp {dir} Test" \
  --run-b "/path/to/other-java -cp {dir} Test" \
  --output ./Test.min.java
```

Java，A/B 使用不同编译指令：

```bash
code-minimizer reduce \
  --lang java \
  --input ./Test.java \
  --build-a "javac-a {input}" \
  --build-b "javac-b {input}" \
  --run-a "java-a -cp {dir} Test" \
  --run-b "java-b -cp {dir} Test"
```

## 命令占位符

命令模板支持：

- `{input}`：当前 side-specific trial 目录中的源码文件路径。
- `{dir}`：`{input}` 所在目录。
- `{stem}`：原始输入文件不含扩展名的文件名。
- `{output}`：为编译产物保留的当前 side 输出目录。

占位符展开后会进行 shell quote。最终命令通过 `sh -c` 执行。

## 重要参数

- `--lang <java|js|javascript>`：选择 parser 和语言适配器。
- `--timeout <duration>`：设置单条命令 timeout，支持 `120s`、`5s`、`250ms` 和 `5`。对当前结果型
  oracle，timeout 表示候选被拒绝。
- `--max-output-bytes <n>`：限制每条命令 stdout/stderr 捕获大小。
- `--confirm-runs <n>`：重复确认 baseline、accepted candidate 和最终结果。每次确认都必须保持配置的差异。
  默认值是 `1`。
- `--preserve-exit <same-class|none|exact>`：控制退出状态保持策略，默认 `same-class`。
- `--diff-mode <any-channel|stdout|stderr|both>`：控制哪个 stdout/stderr stream 必须保持差异，
  默认 `any-channel`。
  stdout/stderr 比较前会归一化 A/B side-specific trial path，避免 compiler diagnostic path 形成假 diff。
- `--max-trials <n>`：限制 baseline 之后的 oracle trial 数。设为 `0` 表示不设置显式 trial 上限。
- `--max-rounds <n>`：限制固定点阶段轮数。设为 `0` 表示不设置显式 round 上限。
- `--algorithm <name>`：选择 reducer 算法。`structured` 是默认的确定性分阶段 reducer。
  `weighted-random` 会按自适应权重随机抽样可删除 statement 点，然后执行 cleanup 和 rename sweep。
  所有算法最后都会执行共享的空白行 cleanup 阶段。
- `--jobs <n>`：预留给后续并行 trial。当前 reducer 按顺序执行。
- `--keep-temp`：保留临时工作区，便于调试。默认情况下，workspace 会创建在输出文件旁边，并在 reduction
  结束后删除。
- `--json-report <path>`：输出 JSON 化简报告。

## 化简模型

每个 accepted source 都表示为一个 `ProgramSnapshot`：

```text
source text
  -> ParsedProgram
  -> ProgramIndex
  -> ProgramAnalysis
  -> ComplexityScore
```

`NodeId`、`SymbolId`、`CandidateId` 和 byte range 只在创建它们的 snapshot 内有效。任何 edit 被接受后，
reducer 都会重新构建 snapshot，然后再生成后续候选。

reducer 的候选调度算法是可插拔的。默认 `structured` 算法会按固定点轮次执行以下阶段：

1. `RuntimeCostReduction`
2. `AggressiveFunctionElimination`
3. `AggressiveBlockElimination`
4. `StatementAndSiblingReduction`
5. `DeadDeclarationAndOutputCleanup`
6. `ExpressionLiteralTypeCleanup`

固定点循环结束后，reducer 会执行 final one-minimal sweep。普通固定点轮次只在“整轮没有接受任何候选”时停止，
不会因为源码字节数刚好不变就提前停止。这个 sweep 会强制对所有启用阶段做单候选尝试。如果接受了任何候选，
就重新解析并重启 sweep。当完整 sweep 没有任何接受时，输出在当前候选生成能力下达到 one-minimal。

所选算法结束后，engine 一定会把 `BlankLineCleanup` 作为最后阶段运行。它会逐个尝试删除空白行和只有空白的行。
每次删除都和普通 candidate 一样经过解析和 oracle 验证；如果配置的差异 `D` 没有保持，当前 accepted source
不会改变，也就是自动回滚。

实验性的 `weighted-random` 算法复用同一套 parser、candidate generator、oracle、cache、workspace 和 report。
它会把可删除 statement candidate 收集成带权重点，按当前权重随机选择一个点并测试删除。invalid edit、parse
failure、compile/build failure、exit-status change 和 timeout 会让该点从当前列表移除。保持程序有效但不满足
oracle 的拒绝会降低该点邻域权重。删除被接受时会保留新源码、重建 snapshot，并按距离衰减提高附近点的权重。
当连续失败次数达到基于剩余点数量的 patience limit 后，随机循环停止；之后会执行 dead declaration/output-only
cleanup 单候选 sweep、保守的长 identifier rename。共享 engine cleanup 会在此之后删除可验证通过的空白行。

## 接受与回退

候选只有在以下条件全部成立时才会被接受：

- edit 能干净地应用到当前 snapshot source；
- candidate 解析成功且没有 diagnostics；
- candidate 满足当前阶段的改进目标，例如 loop bound shrink 降低静态 runtime cost，删除/cleanup 阶段降低结构复杂度；
- 配置的 build 命令成功完成；
- A/B run 命令在 timeout 内完成；
- 输出没有超过捕获上限；
- 配置的结果差异仍然存在；
- 退出状态保持策略仍然成立；
- 每次 `--confirm-runs` 确认都通过。

候选被拒绝时会直接丢弃。回退是隐式的：当前 accepted source 保存在内存中，并同步写入 workspace 的
`accepted/` 目录。完整 candidate source 只保留在固定的 `trials/current/a` 与 `trials/current/b` 输入中，
每个 trial 还会在 `history/` 下保存历史 diff。

reducer 日志行会带本地时间前缀，格式为 `[YYYY-MM-DD HH:MM:SS]`。reducer 会在启动时打印一次固定
current trial directory。拒绝日志会使用可读 phase 名和稳定 enum 名，并包含 group、attempt、candidate
数量和 oracle 原因。

## Java 和 JavaScript 支持

JavaScript 候选包括注释清理、输出调用删除、顶层声明删除、声明删除、block 和 statement 化简、控制流替换、
表达式替换、literal 缩小和 cleanup edit。

Java 候选包括注释清理、import 删除、成员和局部声明删除、输出调用删除、block 和方法体替换、控制流替换、
表达式替换、literal 缩小、空嵌套 block 删除和 cleanup edit。Java adapter 默认保护唯一 public 顶层类。

输出噪音会被优先处理。Java 的 `System.out.print*`、`System.err.print*`、
`java.lang.System.out/err.print*` 和 `printStackTrace(System.out/System.err)` 会作为高价值删除候选。
JavaScript 的 `console.log`、`console.error`、`process.stdout.write` 和 `process.stderr.write`
也会类似处理。

## 报告

`--json-report` 会序列化 session 元数据，包括：

- input/output path 和语言；
- reducer 算法；
- 原始和最终源码大小；
- total/accepted/rejected trial 数；
- trial cache hit 数；
- baseline 和 final stdout/stderr diff 状态，以及用于报告的退出状态差异 metadata；
- 每个阶段的候选数、trial 数、accepted/rejected 数、size 变化和静态 runtime-cost 变化；
- 配置的 `max_rounds`、`max_trials`、`confirm_runs`、`timeout_ms` 和 `max_output_bytes`；
- 是否因为 trial limit 停止；
- 使用 `--keep-temp` 时保留的 workspace 目录。

所有人类可读 report 字符串都使用英文。

## 安全说明

用户提供的 build/run 指令会通过 `sh -c` 在本机执行。Code Minimizer 不是沙箱。对于不可信 fuzzing
输入，请在一次性 VM、容器或其他外部沙箱中运行。

每个 trial 都使用独立的 A/B side 目录，避免编译产物交叉污染。命令应只写入 `{dir}` 或 `{output}`。

收到 Ctrl+C 或 SIGTERM 时，Code Minimizer 会向运行中的命令 process group 发送 SIGTERM，等待 1 秒，
再发送 SIGKILL 后退出。单条命令 timeout 时也使用同样的清理流程。

## 测试

运行 Rust 测试：

```bash
cargo test
```

运行文档测试：

```bash
cargo test --doc
```

`test_cases/java/**` 下的真实 Java fixture 应使用 `--timeout 120s` 和 `--max-trials 0` 做完整化简验证。
完整验证必须确认最终源码仍保持 A/B 结果差异，并确认可删除的输出噪音，例如无关的
`System.out.println(...)`，不会留在最终结果中。

## 扩展新语言

新增语言需要实现 `lang::LanguageAdapter`，并在 `lang::adapter_for` 注册。适配器必须解析源码、构建或增强
`ProgramIndex`、生成按 phase 组织的 `CandidateGroup`，并可选地在 accepted edit 后 normalize 源码。

为了完整支持 reducer，adapter 或共享 index builder 应暴露足够的 function/call-site、loop、block、def-use、output
和 literal analysis 结构。

适配器不应该执行命令、比较 A/B 输出、管理 trial 目录或决定最终 interestingness。这些职责属于共享 reducer
和 oracle。

## 项目约束

源码中的所有运行时输出、日志、CLI 错误、panic 文案、测试失败提示和 JSON report 中的人类可读字段都必须使用英文。

公开模块、公开类型、公开函数、trait、重要 enum variant 和非平凡实现路径必须有英文注释或 doc comment。
注释应解释模块职责、状态流、约束、边界条件和失败处理。

项目文档必须自包含。维护者应能只通过 `README.md`、`README.zh-CN.md`、`docs/API.md`、
`docs/API.zh-CN.md`、`docs/ARCHITECTURE.md` 和 `docs/ARCHITECTURE.zh-CN.md` 理解用法、架构、
公开 API、语言扩展规则、安全边界和测试要求，不依赖任何设计草稿。
