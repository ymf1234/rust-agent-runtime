use crate::{action::Action, state::AgentState};

pub trait Llm {
    fn think(&self, state: &AgentState) -> Action;
}