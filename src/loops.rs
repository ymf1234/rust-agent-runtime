use crate::action::Action;
use crate::state::AgentState;
use crate::tools::ToolManager;

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
