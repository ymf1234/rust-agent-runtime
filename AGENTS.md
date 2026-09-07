# Repository Guidelines

## 项目结构与模块组织

本仓库是配合《深入理解 AI Agent：设计原理与工程实践》的 Rust 学习项目；优先复现书中的概念切分，而非提前抽象成生产级框架。`src/main.rs` 是二进制入口，也是唯一有效的模块声明处；不要修改不参与编译的 `src/mod.rs`。核心模块包括：`agent.rs`（运行循环）、`action.rs`（动作空间）、`state.rs`（上下文）、`llm.rs`（模型 trait）与 `loops.rs`（动作执行）。工具位于 `src/tools/`：`tool.rs` 定义 trait，`manager.rs` 负责注册和分发，`schema.rs` 定义工具 JSON Schema。

## 构建、测试与本地开发

- `cargo run`：运行当前硬编码目标的完整 ReAct 循环；改动后应确认工具调用和最终状态正常输出。
- `cargo build`：编译项目，适合作为快速类型检查。
- `cargo test`：运行单元与集成测试；当前尚无测试骨架，新增行为时应同时补充测试。
- `cargo fmt`：格式化 Rust 代码；提交前运行。使用 `cargo fmt --check` 检查格式差异。
- `cargo clippy`：执行静态检查；请勿为了消除既有 warning 而顺带重构无关代码。

## 代码风格与命名

遵循 Rust 2024 edition 与 `rustfmt` 默认风格：4 空格缩进、`snake_case` 用于函数和模块、`PascalCase` 用于类型与 trait。代码注释使用中文，并尽量沿用书中术语。保持依赖最小化；引入 crate、async 运行时或 Agent 框架前，先确认学习章节确有需要。

`Action` 是 LLM 与运行时的唯一接口。新增能力时，先判断是否需要新 `Action`；不要直接向 `loops.rs` 堆叠特例。新增工具应实现 `Tool` trait，并在 `Agent::run` 注册；若仍使用 `MockLlm`，同步更新其决策逻辑以覆盖新工具。

## 测试指南

将模块测试放在对应源文件的 `#[cfg(test)] mod tests` 中，测试名描述行为，例如 `unknown_tool_returns_observation`。优先覆盖 `Action` 分发、工具参数与错误 Observation；避免依赖机器上的目录内容。提交前至少运行 `cargo fmt --check`、`cargo clippy` 和 `cargo test`。

## 提交与 Pull Request

提交历史使用 Conventional Commits，例如 `feat: add filesystem tool`、`refactor: extract Llm trait`。每个提交聚焦单一学习步骤。PR 应说明对应章节、行为变化、已执行命令及结果；若改变 `cargo run` 输出，请附示例输出。不要使用 `git add .`：根目录的 epub 参考书可能未跟踪，不应误提交。
