use crate::tools::schema::ToolDefinition;
use crate::tools::tool::Tool;

pub struct ToolManager {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolManager {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.push(tool);
    }

    pub fn execute(&self, name: String, args: serde_json::Value) -> String {
        for tool in &self.tools {
            if tool.definition().name == name {
                return tool.execute(args);
            }
        }

        format!("Tool not found: {}", name)
    }

    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        self.tools.iter().map(|tool| tool.definition()).collect()
    }
}
