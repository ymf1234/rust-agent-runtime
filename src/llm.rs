use crate::{action::Action, state::AgentState};

// Llm 只负责“根据当前状态想下一步做什么”。
// Agent 不依赖具体模型，因此以后可以把 MockLlm 换成真实模型实现。
pub trait Llm {
    fn think(&self, state: &AgentState) -> Action;
}

// 当前用于学习流程的假模型：第一轮调用 filesystem，第二轮结束。
pub struct MockLlm;

impl Llm for MockLlm {
    fn think(&self, state: &AgentState) -> Action {
        if state.step == 0 {
            // 返回 ToolCall，而不是直接执行工具，体现“模型提出动作、运行时执行动作”的分工。
            Action::ToolCall {
                name: "filesystem".to_string(),
                args: serde_json::json!({
                    "path": "src"
                }),
            }
        } else {
            // 工具执行后进入下一轮；这里的示例模型直接认为任务完成。
            Action::Finish
        }
    }
}
