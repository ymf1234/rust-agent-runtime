# 学习阶梯：用 Rust 作为 AI Agent 的 Runtime

> 生成于 2026-08-26 · 锚定材料：本仓库 `src/`（含 `Box<dyn Llm>` / `Box<dyn Tool>` 现状）+ `AI-Agents-in-Depth-zh-CN.epub` 第 1 章 · 工具链 rustc 1.96 / edition 2024 · lint：✓ 硬门全过

**这条阶梯的主线**：不是"学 Rust"，也不是"学 Agent"，而是**当 Agent 的那条循环撞上 Rust 的所有权、trait object 和 async 时，会撞出哪些墙、按什么顺序拆**。每一级的墙都是真的，级别 3 那堵我已经在本机编译验证过。

## 级别 1：完全初学者 — 让循环先转起来
- **理解**：`Agent = LLM + 上下文 + 工具` 在 Rust 里落成什么类型——上下文是一个 struct，动作是一个 enum，工具是一个 trait；以及 `match` 的穷尽性检查为什么让 enum 天然适合表达"模型能做的所有动作"
- **掌握**：对着 `cargo run` 的输出，逐行指出是哪个文件哪一段打印的
- **重点**：`Action` enum 是模型与运行时之间**唯一**的接口——想让 Agent 多一种行为，先问"这是不是一个新 Action"，而不是往循环里塞 `if`
- **里程碑**：不复制本仓库代码，从零 `cargo new` 一个 crate，手写出 state / action / loop 三个文件，`cargo run` 打印出一轮完整的「想 → 做 → 看」
- **练习**：先画出 `main → Agent::run → think → execute_action` 的调用图，再照着图重写一遍最小循环，最后与 `src/` 对照差异
- **常犯错误**：一上来就想接真实 API 和 async，结果整晚卡在 tokio 报错上，而不是卡在 Agent 本身——这一级请全程同步、单线程、假 LLM
- **自检**：`execute_action` 为什么拿 `&mut AgentState` 而不是 `AgentState`？

## 级别 2：基本理解 — 工具层与 trait object
- **理解**：为什么工具集合是 `Vec<Box<dyn Tool>>` 而不是泛型——运行时才知道注册了哪些工具，就必须放弃单态化；以及 dyn 兼容（旧称 object safety）对 trait 方法签名的限制
- **掌握**：新增一个工具时不改 `ToolManager` 一行，只写实现 + 注册
- **重点**：`ToolDefinition.parameters` 那段 JSON Schema 不是给人看的，是**直接喂给 LLM API 的 tools 字段**；工具描述写得含糊，模型就会系统性地误用（书 §1.2.3 ACI：从 Agent 视角而非程序员视角设计接口）
- **里程碑**：加两个新工具，`list_tools()` 输出的 JSON 能被 `serde_json` 解析且 schema 字段完整
- **练习**：实现 `read_file` 与 `write_file` 两个工具并注册，同步改 `MockLlm` 让它们真被调到（不改 `MockLlm` 的话工具永远调不到）
- **常犯错误**：给 `Tool` trait 加泛型方法或让它返回 `impl Trait`，trait 立刻不再 dyn 兼容，`Box<dyn Tool>` 编译不过——而报错信息指向的是装箱那一行，不是你刚改的签名
- **自检**：往 `Tool` trait 里加一个 `fn call<T: Serialize>(&self, t: T)`，会发生什么？

## 级别 3：实际使用者 — 接真实 LLM，撞上 async 的墙
- **理解**：接真实 API 后循环必须变 async，而 **`async fn` 写进 trait 会让这个 trait 不再 dyn 兼容**，你现有的 `Box<dyn Llm>` 会当场 `E0038` 编译失败（rustc 1.96 实测）；出路是 `async-trait` 宏、或手写 `Pin<Box<dyn Future>>`、或改用泛型分发放弃运行时替换
- **掌握**：把 `Llm` trait 改成 async 并接上真实 HTTP，而 `Agent::run` 的循环结构不塌
- **重点**：错误分类——网络失败、API 报错、工具执行失败、模型返回非法 JSON，这四种在 Rust 里应该是同一个 error enum 的不同变体，而不是全塞进 `String`；因为其中只有一部分该重试
- **里程碑**：跑一次程序，调用哪个工具由真实模型决定而不是 `state.step` 分支；拔网线重跑，报出可区分的错误变体而不是 panic
- **练习**：用 reqwest + tokio 实现 `ClaudeLlm`，把 `ToolDefinition` 序列化进请求的 tools 字段；用 thiserror 定义 `AgentError` enum 覆盖上述四类错误
- **常犯错误**：整条链路 `unwrap()` 或 `Box<dyn Error>` 一把梭——出错时分不清是网络挂了还是模型返回了非法 JSON，于是没法决定该不该重试
- **自检**：模型返回的 JSON 少了一个必需字段，这个错误落在你的 `AgentError` 哪个变体上？

## 级别 4：问题解决者 — 轨迹的所有权与并发工具调用
- **理解**：书 §1.4 的 `context = stable_prefix + trajectory` 落成 Rust 类型时的真正难题是所有权——轨迹要被反复读、被追加、被序列化发出去；而书里那句 "independent calls may run in parallel" 在 Rust 里就是 `join_all`，代价是 `Tool` 必须 `Send + Sync`
- **掌握**：说得出你的轨迹结构每轮 clone 了多少字节、为什么，以及不 clone 该怎么写
- **重点**：跨 `await` 点持有引用的限制——`&ToolManager` 想活过 await 通常得换 `Arc<ToolManager>`；这是 Rust 相对 Python Agent 框架**最贵的那道税**
- **里程碑**：一轮内并发执行 3 个工具调用，实测总耗时接近最慢的那个，而不是三者之和
- **练习**：改造 `AgentState` 加 `Vec<Message>` 轨迹，让 `execute_action` 把 decision 与 observation 都追加进去；给 `Tool` 补 `Send + Sync` 约束并用 `futures::join_all` 并发跑工具
- **常犯错误**：为了绕开借用检查，每轮把整条轨迹 `clone()` 一遍再传下去——循环一长，内存和延迟一起爆，而编译器完全不会拦你
- **自检**：`Box<dyn Tool>` 要能跨线程用，trait 声明得补哪两个约束？

## 级别 5：自信的实践者 — 补齐 Harness 五功能
- **理解**：书 §1.2.2 的 Harness 五功能里，上下文与工具只是前两个；**约束**（能力默认全关、必须显式开放）、**验证**（只看结构化数据，不信模型自由生成的文本）、**纠正**（失败先静默重试、确认无法恢复前不暴露中间态）才是可靠性的来源；同时 KV Cache 要求静态前缀逐字节稳定，这对你拼 prompt 的顺序是硬约束
- **掌握**：拿到任意一个 Agent 项目，指得出它缺哪一环、会在什么场景炸
- **重点**：纠正的边界——哪些错误静默重试、哪些直接熔断转人工；以及重试时**不能顺手重排系统提示词**，否则缓存前缀失效、成本翻倍
- **里程碑**：注入一个必然失败的工具，程序自行重试到上限后熔断退出，全程不 panic，也不把半成品结果打给用户
- **练习**：给循环加错误分类、重试上限与熔断；再挂一层 `tracing`，把每轮的 decision 与 observation 输出成结构化日志
- **常犯错误**：用一个 catch-all 吞掉所有错误当作"纠正"，实际掩盖根因——书里叫这个"治标不治本"，你的循环会看起来很稳定但一直在做错事
- **自检**：工具连续失败 3 次后你的循环下一步做什么？重试时发出去的 prompt 前缀和上一次是否逐字节相同？
