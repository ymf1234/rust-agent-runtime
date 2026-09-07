use crate::llm::{Llm, MockLlm};
use crate::loops::execute_action;
use crate::state::AgentState;
use crate::tools::{FileSystemTool, ToolManager};

pub struct Agent;

impl Agent {
    pub fn run(goal: String) {
        // State 保存一次 Agent 运行期间的上下文和进度。
        let mut state = AgentState {
            goal,
            step: 0,
            finished: false,
        };

        // ToolManager 是运行时的工具注册表。
        let mut tool_manager = ToolManager::new();

        // 新工具需要先注册，模型返回对应名称后运行时才能找到它。
        tool_manager.register(Box::new(FileSystemTool));

        println!("Available Tools:");

        for tool in tool_manager.list_tools() {
            println!("- {}: {}", tool.name, tool.description);
        }

        println!();

        // Box<dyn Llm> 让 Agent 依赖 trait，而不是依赖 MockLlm 的具体实现。
        let llm: Box<dyn Llm> = Box::new(MockLlm);

        // 这就是最小 ReAct 循环：思考（think）→ 执行动作 → 更新状态 → 再思考。
        while !state.finished {
            let action = llm.think(&state);

            execute_action(action, &mut state, &tool_manager);
        }

        // 示例程序最后打印状态，方便观察 goal、step 和 finished 的变化。
        println!("Final State: {:?}", state);
    }
}
