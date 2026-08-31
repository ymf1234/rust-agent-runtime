use crate::{action::Action, state::AgentState};

pub trait Llm {
    fn think(&self, state: &AgentState) -> Action;
}

pub struct MockLlm;

impl Llm for MockLlm {
    fn think(&self, state: &AgentState) -> Action {
        if state.step == 0 {
            Action::ToolCall {
                name: "filesystem".to_string(),
                args: serde_json::json!({
                    "path": "src"
                }),
            }
        } else {
            Action::Finish
        }
    }
}
