use crate::state::AgentState;

// Llm 只负责“根据当前状态想下一步做什么”。
// Agent 不依赖具体模型，因此以后可以把 MockLlm 换成真实模型实现。
pub trait Llm {
    fn think(&self, state: &AgentState) -> String;
}

// 当前用于学习流程的假模型：第一轮调用 filesystem，第二轮结束。
pub struct MockLlm;

impl Llm for MockLlm {
    fn think(&self, state: &AgentState) -> String {
        if state.step == 0 {
            r#"
            {
                "name": "filesystem",
                "arguments": {
                    "path": "src"
                }
            }
            "#
            .to_string()
        } else {
            r#"
            {
                "name": "finish",
                "arguments": {}
            }
            "#
            .to_string()
        }
    }
}
