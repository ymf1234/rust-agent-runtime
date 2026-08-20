use crate::loops::{execute_action, think};
use crate::state::AgentState;
use crate::tools::{FileSystemTool, ToolManager};

pub struct Agent;

impl Agent {
    pub fn run(goal: String) {
        let mut state = AgentState {
            goal,
            step: 0,
            finished: false,
        };

        let mut tool_manager = ToolManager::new();

        tool_manager.register(Box::new(FileSystemTool));

        println!("Available Tools:");

        for tool in tool_manager.list_tools() {
            println!("- {}: {}", tool.name, tool.description);
        }

        println!();

        while !state.finished {
            let action = think(&state);

            execute_action(action, &mut state, &tool_manager);
        }
        println!("Final State: {:?}", state);
    }
}
