use crate::tool_result::ToolResult;
use crate::tools::schema::ToolDefinition;
use crate::tools::tool::Tool;

pub struct ToolManager {
    // 使用 trait object 保存不同类型的工具，让管理器不需要知道每个工具的具体类型。
    tools: Vec<Box<dyn Tool>>,
}

impl ToolManager {
    pub fn new() -> Self {
        // Agent 创建管理器后，再逐个注册可用工具。
        Self { tools: Vec::new() }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        // Box<dyn Tool> 让 filesystem 等不同工具可以放进同一个 Vec。
        self.tools.push(tool);
    }

    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        // 启动时用于展示当前 Runtime 注册了哪些工具。
        self.tools.iter().map(|tool| tool.definition()).collect()
    }

    pub fn execute(&self, name: String, args: serde_json::Value) -> ToolResult {
        // 当前实现按名称线性查找；找不到时也返回字符串，作为 Observation 交给模型处理。
        for tool in &self.tools {
            if tool.definition().name == name {
                return tool.execute(args);
            }
        }

        ToolResult::Error(format!("Tool not found: {}", name))
    }
}
