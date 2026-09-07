# Rust Agent Runtime

一个用 Rust 从零实现的最小 Agent Runtime，配合《深入理解 AI Agent：设计原理与工程实践》学习 Agent 的核心设计。项目当前优先保持简单，代码结构直接对应书中的基本公式：

```text
Agent = LLM + 上下文 + 工具
```

## 当前能力

- 使用 `MockLlm` 模拟模型决策。
- 通过 `AgentState` 保存目标、执行轮次和完成状态。
- 通过 `Action` 表示继续、调用工具或结束任务。
- 实现最小 ReAct 循环：思考 → 执行动作 → 获取 Observation。
- 提供 `filesystem` 工具，读取目录并统计其中的 Rust 文件。

## 快速开始

需要安装 Rust 工具链，然后在仓库根目录运行：

```bash
cargo run
```

程序会执行一个固定目标：统计 `src` 目录中的 Rust 文件，并打印工具调用、Observation 和最终状态。

常用开发命令：

```bash
cargo build          # 编译项目
cargo test           # 运行测试
cargo fmt --check    # 检查代码格式
cargo clippy         # 执行静态检查
```

## 代码结构

```text
src/
├── main.rs           # 程序入口和模块声明
├── agent.rs          # 创建状态、注册工具并驱动主循环
├── llm.rs            # Llm trait 和 MockLlm
├── action.rs         # LLM 与运行时之间的动作协议
├── state.rs          # Agent 运行状态
├── loops.rs          # 执行 Action 并更新状态
└── tools/
    ├── tool.rs       # Tool trait
    ├── manager.rs    # 工具注册、查找和执行
    ├── schema.rs     # 工具定义和参数 Schema
    └── filesystem.rs # 文件系统统计工具
```

一次运行的主要数据流如下：

```text
main.rs
  ↓
Agent::run
  ↓
Llm::think → Action
  ↓
execute_action → ToolManager → Tool
  ↓
Observation → 下一轮状态
```

## 学习路线

建议先阅读 `src/agent.rs`，再沿着上面的数据流依次阅读 `llm.rs`、`action.rs`、`loops.rs` 和 `tools/`。项目中的中文注释会解释每个模块为什么存在。

- 学习计划：[`docs/20h-plan-rust-agent-runtime.md`](docs/20h-plan-rust-agent-runtime.md)
- 学习阶梯：[`docs/learning-ladder/rust-as-agent-runtime.md`](docs/learning-ladder/rust-as-agent-runtime.md)
- 贡献与开发约定：[`AGENTS.md`](AGENTS.md)

## 当前限制

这是学习用骨架，不是生产级 Agent 框架：模型仍由 `MockLlm` 模拟，Observation 尚未保存为 trajectory，测试尚未建立完整骨架，`src/mod.rs` 也不参与二进制编译。新增抽象或依赖前，请先确认对应学习章节确实需要。
