use crate::decision::tool_call_to_action;
use crate::llm::Llm;
use crate::loops::execute_action;
use crate::parser::parse_tool_call;
use crate::state::AgentState;
use crate::tools::ToolManager;

pub struct Agent {
    llm: Box<dyn Llm>,
    tool_manager: ToolManager,
}

impl Agent {
    pub fn new(llm: Box<dyn Llm>, tool_manager: ToolManager) -> Self {
        Self { llm, tool_manager }
    }

    pub fn run(&self, goal: String) {
        // State 保存一次 Agent 运行期间的上下文和进度。
        let mut state = AgentState {
            goal: goal,
            step: 0,
            finished: false,
            observation: None,
        };

        println!("Available Tools:");

        for tool in self.tool_manager.list_tools() {
            println!("- {}: {}", tool.name, tool.description);
        }

        println!();

        // Box<dyn Llm> 让 Agent 依赖 trait，而不是依赖 MockLlm 的具体实现。
        // let llm: Box<dyn Llm> = Box::new(MockLlm);

        // 这就是最小 ReAct 循环：思考（think）→ 执行动作 → 更新状态 → 再思考。
        while !state.finished {
            let response = self.llm.think(&state);

            println!("LLM Response: {}", response.trim());

            let tool_call = parse_tool_call(&response).expect("解析 ToolCall 失败");

            // 将模型协议中的 "finish" 转换为结束动作，而不是当成工具名执行。
            let action = tool_call_to_action(tool_call);

            execute_action(action, &mut state, &self.tool_manager);
        }

        // 示例程序最后打印状态，方便观察 goal、step 和 finished 的变化。
        println!("Final State: {:?}", state);
    }
}
