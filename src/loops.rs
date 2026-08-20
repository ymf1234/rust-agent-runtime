use crate::action::Action;
use crate::state::AgentState;
use crate::tools::{ToolManager};

pub fn think(state: &AgentState) -> Action {
    println!("Goal:{}", state.goal);

    if state.step == 0 {
        Action::ToolCall {
            name: "filesystem".to_string(),
            args: serde_json::json!({"path": "src"}),
        }
    } else if state.step >= 1 {
        Action::Finish
    } else {
        Action::Continue
    }
}


pub fn execute_action(action: Action, state: &mut AgentState, tool_manager: &ToolManager) {
    match action {
        Action::Continue => {
            println!("Action: Continue");

            state.step += 1;
        }

        Action::ToolCall { name, args } => {
            println!("Action: ToolCall");

            println!("Tool: {}", name);

            println!("Args: {}", args);

            let result = tool_manager.execute(name, args);

            println!("Observation: {}", result);

            state.step += 1;
        }

        Action::Finish => {
            println!("Action: Finish");

            state.finished = true;
        }
    }
}
