# 20 小时学会：用 Rust 作为 AI Agent 的 Runtime

> 生成于 2026-08-26 · 锚定：本仓库 `src/` + `AI-Agents-in-Depth-zh-CN.epub` 第 1–2 章 · 工具链 rustc 1.96 / edition 2024
> API 事实核对自 claude-api 技能（模型 ID、tools 结构、tool_use/tool_result 回环、Opus 5 参数限制）

## 一、那关键的 20%

不是"学 Rust"也不是"学 Agent"，而是**这两者交界处的五个点**。吃透这五个，其余 80% 是它们的延伸：

| # | 核心 | 为什么它是 20% | 现实中对应什么 |
|---|---|---|---|
| 1 | **`Action` enum 是模型与运行时唯一的接口** | 模型能做的一切都要穿过这个类型。`match` 的穷尽性检查把"漏处理一种模型行为"从线上事故变成编译错误——这是 Rust 写 Agent 的最大红利 | 加新能力先问"是不是新 Action"，而不是往循环塞 `if` |
| 2 | **`Box<dyn Trait>` 与 dyn 兼容** | 工具要运行时注册、LLM 要能替换，就必须动态分发；而 dyn 兼容的限制会在你加 async 时正面撞上来 | `Vec<Box<dyn Tool>>`、`Box<dyn Llm>`，以及它们为什么不能有泛型方法 |
| 3 | **serde 是静态类型与 LLM 动态 JSON 的边界** | Agent 的一半工作是把 Rust 结构体变成 API 的 JSON、再把模型吐回的 JSON 变回结构体。这条边界没守住，后面全是 `unwrap()` | `ToolDefinition` → `tools` 字段；`tool_use.input` → 你的参数结构体 |
| 4 | **`context = stable_prefix + trajectory`** | 书 §1.4 的公式。落到 Rust 就是所有权难题：轨迹要被反复读、追加、序列化发出去。多轮对话、KV Cache、成本，全挂在这上面 | 每轮把 assistant 的完整 content 和 tool_result 追加回 messages |
| 5 | **错误分类 = 可恢复性的来源** | 网络挂了 / API 报错 / 工具失败 / JSON 非法，只有一部分该重试。用 `String` 装错误，就永远没法自动恢复 | `AgentError` enum + 重试 + 熔断，书 §1.2.2 Harness 的"纠正" |

**这五个点的现实价值**：任何生产 Agent 的可靠性都来自它们。1 和 2 决定架构能不能扩展，3 决定接口层会不会天天崩，4 决定多轮对话对不对且花多少钱，5 决定线上出错时是自愈还是 panic。

---

## 二、10 个 Session（每个 2 小时）

### Session 1 · 让循环转起来（同步 / 假 LLM）

- **目标**：从零手写一个能跑通「想 → 做 → 看」的最小 ReAct 循环，不碰网络、不碰 async。
- **关键概念**：`struct` 表示上下文、`enum` 表示动作、`trait` 表示工具；`&mut` 与循环状态；`match` 穷尽性。
- **练习**：`cargo new my-agent`，手写 `state.rs` / `action.rs` / `loops.rs` / `main.rs` 四个文件，跑出一轮完整循环的打印。**不要复制本仓库代码**——先画调用图，照图写。
- **资源**：《Rust 程序设计语言》中文版 第 5、6、8 章 <https://kaisery.github.io/trpl-zh-cn/>（免费）
- **预期结果**：`cargo run` 打印出 Goal → ToolCall → Observation → Finish；能对着输出逐行指出是哪段代码打的。

**复习题**
1. `execute_action` 为什么拿 `&mut AgentState` 而不是 `AgentState`？换成后者会怎样？
2. `Action` 是 enum 而不是 trait，好处是什么？
3. 如果给 `Action` 加一个新变体但忘了改 `execute_action`，什么时候会发现？
4. `while !state.finished` 的退出条件由谁负责设置？
5. 本仓库的 `src/mod.rs` 为什么改了没效果？

---

### Session 2 · 动作空间设计

- **目标**：把 `Action` 从"够用"改成"经得起加功能"，理解动作空间就是 Agent 能力的边界。
- **关键概念**：书 §1.1 观察空间与动作空间；enum 携带数据（`ToolCall { name, args }`）；为什么 Observation 是 `String` 而不是泛型。
- **练习**：给 `Action` 加两个新变体（例如 `Ask`——向用户提问，`Delegate`——交给子 Agent），让 `match` 编译报错，逐个补齐分支；然后回答"哪些该是 Action、哪些不该"。
- **资源**：epub 第 1 章 §1.1（`unzip -p ...epub EPUB/text/ch002.xhtml`，见 CLAUDE.md 里的提取命令）
- **预期结果**：写得出一段判断依据：什么样的新能力应该是新 Action，什么样的只是工具。

**复习题**
1. "让 Agent 会查天气"该加 Action 还是加工具？为什么？
2. "让 Agent 能中途向用户提问"呢？
3. `Action::Continue` 存在的意义是什么？你的实现里谁产生它？
4. 如果 Observation 改成泛型 `T`，`execute_action` 会遇到什么麻烦？
5. 动作空间过大和过小分别会导致什么问题？

---

### Session 3 · 工具层与 trait object

- **目标**：搞清楚为什么工具集合必须是 `Vec<Box<dyn Tool>>`，以及 dyn 兼容对签名的硬约束。
- **关键概念**：静态分发 vs 动态分发；单态化的代价；dyn 兼容（旧称 object safety）三条主要限制：无泛型方法、不能返回 `impl Trait`、`Self: Sized` 的处理。
- **练习**：给 `Tool` trait 加一个泛型方法 `fn call<T: Serialize>(&self, t: T)`，观察 `Box<dyn Tool>` 报 `E0038`，读懂报错再撤销；然后实现 `read_file` + `write_file` 两个工具并注册。
- **资源**：Rust By Example — Traits / `dyn` 章节 <https://doc.rust-lang.org/rust-by-example/>（免费）
- **预期结果**：能不查资料说出"哪三类方法签名会让 trait 不再 dyn 兼容"，且新增工具不改 `ToolManager` 一行。

**复习题**
1. 为什么不能用泛型 `T: Tool` 代替 `Box<dyn Tool>` 装工具集合？
2. `E0038` 的报错指向哪一行？为什么不是指向你刚改的签名？
3. `ToolManager::execute` 现在按 name 线性遍历，工具到 100 个时该换成什么？
4. `Box<dyn Tool>` 相比泛型分发，运行时多付出了什么？
5. `definition()` 每次调用都构造新的 `String`，这在循环里有什么问题？

---

### Session 4 · serde：结构体与 API JSON 的边界

- **目标**：不联网，先把「Rust 结构体 ↔ Anthropic API 的 JSON」这条边界打通。
- **关键概念**：`#[derive(Serialize, Deserialize)]`；`#[serde(rename)]` / `tag` / `untagged`；用 enum 建模 API 的 content block 联合类型（`text` / `tool_use` / `tool_result`）；`serde_json::Value` 什么时候该用、什么时候是偷懒。
- **练习**：定义 `MessagesRequest` / `ContentBlock` / `MessagesResponse` 三组类型，把 Anthropic 官方文档里的 tool_use 请求/响应示例 JSON **反序列化成你的类型再序列化回去**，用 `assert_eq!` 比对（round-trip 测试）。
- **资源**：serde 官方指南 <https://serde.rs/>（免费）+ Tool Use 文档 <https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview>
- **预期结果**：一组能 round-trip 官方示例 JSON 的类型定义 + 第一批单元测试（本仓库目前 0 个测试，这里补上）。

**复习题**
1. `content` 数组里混着 `text` 和 `tool_use` 两种块，用什么 serde 属性建模最自然？
2. `ToolDefinition.parameters` 用 `serde_json::Value` 而不是强类型，代价和收益各是什么？
3. `tool_use.input` 该反序列化成什么类型？为什么不能是具体结构体？
4. round-trip 测试能发现哪类 bug，不能发现哪类？
5. 模型返回的 JSON 多了一个你没定义的字段，默认行为是报错还是忽略？

---

### Session 5 · 第一次真实 API 调用（curl 先行）

- **目标**：在写任何 Rust 之前，先用 curl 把一次完整的 tool_use 往返跑通，眼见为实。
- **关键概念**：`POST https://api.anthropic.com/v1/messages`；三个必需 header（`x-api-key` / `anthropic-version: 2023-06-01` / `content-type`）；模型 ID **`claude-opus-5`**；`max_tokens` 非流式建议 ~16000；`stop_reason` 的六种取值（`end_turn` / `max_tokens` / `stop_sequence` / `tool_use` / `pause_turn` / `refusal`）。
- **练习**：用 curl 发一个带 `tools` 的请求，拿到 `stop_reason: "tool_use"` 的响应；把响应里的 `tool_use` 块和一个手写的 `tool_result` 拼进第二次请求，拿到最终答案。全程 `jq` 解析，不用 grep。
- **资源**：Messages API 文档 + 本文末尾的 curl 骨架
- **预期结果**：两次 curl 的完整请求/响应存成文件——这是后面 Rust 实现的黄金对照样本。

**复习题**
1. `anthropic-version` 这个 header 少了会怎样？
2. `stop_reason` 是 `tool_use` 时，`content` 数组里一定有什么？
3. `tool_result` 的 `tool_use_id` 必须等于什么？
4. Opus 5 上传 `temperature: 0.7` 会发生什么？
5. `stop_reason` 为 `refusal` 时读 `content[0].text` 会怎样？

---

### Session 6 · 接上真实 LLM，撞 async 的墙

- **目标**：把 `Llm` trait 改成 async 并接上 HTTP——并解决 `async fn` 让 trait 不再 dyn 兼容、`Box<dyn Llm>` 编译失败的问题。
- **关键概念**：`async fn` in trait（Rust 1.75+ AFIT）不是 dyn 兼容的；三条出路：`async-trait` 宏 / 手写 `Pin<Box<dyn Future>>` / 改泛型分发放弃运行时替换；tokio runtime 的启动方式（`#[tokio::main]`）。
- **练习**：用 reqwest + tokio 实现 `ClaudeLlm`，先只做**单轮无工具**的请求，把 Session 5 的 curl 翻译成 Rust；故意先不加 `async-trait` 体验一次 `E0038`。
- **资源**：Tokio 官方教程前 3 节 <https://tokio.rs/tokio/tutorial>（免费）；异步之书 <https://rust-lang.github.io/async-book/>；reqwest 文档 <https://docs.rs/reqwest>
- **预期结果**：`cargo run` 拿到真实模型的一句回答；能说出你选了三条出路中的哪条、为什么。

**复习题**
1. `async fn` 进 trait 后为什么 `Box<dyn Llm>` 编译不过？
2. `async-trait` 宏在背后做了什么？代价是什么？
3. 如果改用泛型分发 `Agent<L: Llm>`，你失去了什么能力？
4. `#[tokio::main]` 展开成了什么？
5. reqwest 的 `.json()` 方法要求响应类型满足什么约束？

---

### Session 7 · tool_use / tool_result 多轮循环（核心）

- **目标**：把整条 ReAct 循环接到真实 API 上——这是全部 20 小时的中心。
- **关键概念**：`stop_reason == "tool_use"` 是循环继续的信号；**必须把 assistant 的完整 `content`（含 tool_use 块）追加回 messages**，只追加 text 会丢工具调用；`tool_result` 放在 `role: "user"` 的消息里；工具失败时返回 `is_error: true` 而不是丢弃。
- **练习**：实现完整循环——发请求 → 判断 `stop_reason` → 从 content 抽出所有 `tool_use` 块 → 逐个执行 → 把**所有** `tool_result` 塞进**一条** user 消息 → 再发。跑通"统计 src 目录 Rust 文件数"这个真实任务，但由模型自己决定调用哪个工具。
- **资源**：Tool Use 文档的 loop 章节 + epub 第 1 章 §1.4 ReAct 轨迹图
- **预期结果**：模型自主完成一个需要 2 次以上工具调用的任务；`MockLlm` 从此只用于测试。

**复习题**
1. 只把 assistant 的 text 追加回 messages，下一轮会发生什么？
2. 一条 assistant 消息里有 3 个 tool_use 块，tool_result 该发 1 条消息还是 3 条？为什么？
3. 工具执行失败，返回 `is_error: true` 和直接不返回，模型的行为差别是什么？
4. `stop_reason` 是 `pause_turn` 时该怎么做？
5. 循环的终止条件现在有几个？分别是什么？

---

### Session 8 · 错误分类与自愈

- **目标**：把 `unwrap()` 全部清掉，建立能支撑自动重试与熔断的错误类型。
- **关键概念**：`Result<T, E>` 贯穿；用 `thiserror` 派生 `AgentError`；四类错误必须可区分——网络失败 / API 状态码错误（429、5xx 可重试，400、401 不可重试）/ 工具执行失败 / 模型返回非法 JSON；书 §1.2.2 Harness 的「纠正」：失败先静默重试，确认不可恢复前不暴露中间态。
- **练习**：定义 `AgentError` enum 覆盖四类；给循环加指数退避重试（只对可重试类）+ 重试上限 + 熔断；写一个**必然失败**的工具验证熔断真的生效；拔网线跑一次，确认报出可区分的错误而不是 panic。
- **资源**：thiserror <https://docs.rs/thiserror>；API 错误码文档 <https://platform.claude.com/docs/en/api/errors>
- **预期结果**：全链路无 `unwrap()`；断网、429、工具失败三种场景各有不同且正确的行为。

**复习题**
1. 429 和 400 都要重试吗？依据是什么？
2. `Box<dyn Error>` 相比自定义 enum，在重试决策上差在哪？
3. 熔断触发后，你的循环下一步做什么？告诉用户什么？
4. "静默重试"和"暴露中间态"的边界在哪？
5. 模型返回的 JSON 缺一个必需字段，落在你 `AgentError` 的哪个变体？

---

### Session 9 · 轨迹、上下文与 KV Cache

- **目标**：把 `context = stable_prefix + trajectory` 真正落成 Rust 类型，并理解它对成本的影响。
- **关键概念**：轨迹的所有权难题（反复读 + 追加 + 序列化）；clone 的代价；**prompt caching 是前缀匹配**——render 顺序是 `tools` → `system` → `messages`，前缀里任何一个字节变了，后面全部失效；`cache_control: {"type": "ephemeral"}`，Opus 5 最小可缓存前缀 512 token，每请求最多 4 个断点；静默失效元凶：把时间戳/UUID 拼进 system prompt。
- **练习**：给 `AgentState` 加 `Vec<Message>` 轨迹；在最后一个 system 块上打 `cache_control`；连跑三轮，用 `usage.cache_read_input_tokens` 验证缓存命中；然后**故意**把当前时间拼进 system prompt，看命中归零。
- **资源**：epub 第 2 章（`ch003.xhtml`）KV Cache 与上下文工程小节
- **预期结果**：能说出你的轨迹每轮 clone 了多少字节；能用 `usage` 字段证明缓存命中与失效。

**复习题**
1. `usage` 里哪个字段证明缓存被读到了？为零说明什么？
2. 为什么工具定义的顺序必须稳定？
3. 把用户名拼进 system prompt，对缓存有什么后果？
4. 轨迹每轮 clone 一遍，编译器会拦你吗？后果是什么？
5. Opus 5 上 800 token 的前缀打了 `cache_control`，会缓存吗？

---

### Session 10 · 并发工具调用与 Harness 收尾

- **目标**：一轮内并发执行多个工具，并对照书 §1.2.2 的五功能给自己的 runtime 打分。
- **关键概念**：书里 "independent calls may run in parallel" 落到 Rust 就是 `futures::join_all`；代价是 `Tool` 必须 `Send + Sync`、`&ToolManager` 跨 await 要换 `Arc`；Harness 五功能——上下文 / 工具 / 约束（默认全关、显式开放）/ 验证（只信结构化数据）/ 纠正。
- **练习**：给 `Tool` 补 `Send + Sync`，用 `join_all` 并发跑工具，实测总耗时接近最慢的那个；再挂 `tracing` 把每轮 decision/observation 输出成结构化日志；最后对照五功能列出自己还缺哪一环。
- **资源**：Tokio 教程并发章节；epub 第 1 章 §1.2.2 Harness 五功能表
- **预期结果**：3 个工具并发耗时 ≈ 最慢的一个；一份写下来的"我的 runtime 缺哪一环"清单。

**复习题**
1. `Box<dyn Tool>` 要跨线程用，trait 声明得补哪两个约束？
2. 为什么 `&ToolManager` 跨 await 点通常要换成 `Arc`？
3. 并发跑工具后，tool_result 的顺序要不要和 tool_use 一致？
4. Harness 五功能里你实现了几个？缺的那些会在什么场景炸？
5. "验证只看结构化数据，不信模型自由文本"——为什么？

---

## 三、最终项目：一个真能干活的 Rust Agent

**做一个「仓库问答 Agent」**：给它一个 Rust 项目路径和一个问题（"这个项目有几个模块？`Tool` trait 在哪定义的？"），它自主调用工具、多轮推理、给出带证据的回答。

**验收标准**（每条都可外部观察）：
1. 至少 4 个工具：`list_dir` / `read_file` / `grep` / `count_lines`，全部由模型自主选择，不是 `step` 分支。
2. 一次任务内完成 ≥ 3 轮工具调用，且第 3 轮的 prompt 里能看到第 1 轮的 Observation。
3. 一轮内并发调用多个工具时，总耗时接近最慢的那个。
4. 注入一个必然失败的工具，Agent 自行重试到上限后熔断退出，全程不 panic、不把半成品打给用户。
5. `usage.cache_read_input_tokens` 在第二轮起大于 0。
6. 断网重跑，报出可区分的错误类型而不是 panic。
7. `cargo clippy` 无 warning，`cargo test` 有 ≥ 5 个测试且全绿。

**做完这七条，你就具备了把 Agent runtime 用在真实项目里的能力**——因为这七条正好覆盖了那 20% 的全部五个点。

---

## 附：Session 5 的 curl 骨架

```bash
curl https://api.anthropic.com/v1/messages \
  -H "content-type: application/json" \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "claude-opus-5",
    "max_tokens": 16000,
    "tools": [{
      "name": "count_rust_files",
      "description": "统计指定目录下的 .rs 文件数量",
      "input_schema": {
        "type": "object",
        "properties": {
          "path": {"type": "string", "description": "要统计的目录路径"}
        },
        "required": ["path"]
      }
    }],
    "messages": [{"role": "user", "content": "src 目录里有几个 Rust 文件？"}]
  }' | jq '{stop_reason, content}'
```

第二轮把 tool_use 与 tool_result 一起发回：

```json
"messages": [
  {"role": "user", "content": "src 目录里有几个 Rust 文件？"},
  {"role": "assistant", "content": [
    {"type": "tool_use", "id": "toolu_abc123", "name": "count_rust_files", "input": {"path": "src"}}
  ]},
  {"role": "user", "content": [
    {"type": "tool_result", "tool_use_id": "toolu_abc123", "content": "7"}
  ]}
]
```
