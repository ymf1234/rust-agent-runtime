use crate::action::Action;
use crate::state::AgentState;
use crate::tool_result::ToolResult;
use crate::tools::ToolManager;

pub fn execute_action(action: Action, state: &mut AgentState, tool_manager: &ToolManager) {
    match action {
        Action::Continue => {
            println!("Action: Continue");

            // Continue 本身没有工具结果，但仍然算完成了一轮处理。
            state.step += 1;
        }

        Action::ToolCall { name, args } => {
            println!("Action: ToolCall");

            println!("Tool: {}", name);

            println!("Args: {}", args);

            // ToolManager 负责按工具名称查找并执行工具。
            // 工具返回的字符串就是 Agent 看到的 Observation。
            let result = tool_manager.execute(name, args);

            match result {
                ToolResult::Success(output) => {
                    println!("Observation: {}", output);
                    state.observation = Some(output);
                }
                ToolResult::Error(error) => {
                    println!("Tool Error: {}", error);
                    state.observation = Some(format!("Tool Error: {}", error));
                }
            }

            state.step += 1;
        }

        Action::Finish => {
            println!("Action: Finish");

            // 下一次检查 while 条件时退出 Agent 主循环。
            state.finished = true;
        }
    }
}
