use rust_agent_runtime::{
    agent::Agent,
    llm::MockLlm,
    tools::{FileSystemTool, ToolManager},
};

fn main() {
    println!("Agent Runtime Started");

    let mut tool_manager = ToolManager::new();

    tool_manager.register(Box::new(FileSystemTool));

    let llm = Box::new(MockLlm);

    // 真实项目中 goal 可以来自命令行或用户输入；当前先用固定目标演示完整流程。
    let agent = Agent::new(llm, tool_manager);

    agent.run("统计 src 目录中的 Rust 文件".to_string());
}
