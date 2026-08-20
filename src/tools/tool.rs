use crate::tools::schema::ToolDefinition;

pub trait Tool {
    // 工具介绍
    fn definition(&self) -> ToolDefinition;
    // 工具执行
    fn execute(&self, args: serde_json::Value) -> String;
}
