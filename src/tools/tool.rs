use crate::{tool_result::ToolResult, tools::schema::ToolDefinition};

pub trait Tool {
    // 返回工具的名称、说明和参数格式，供模型了解如何调用它。
    fn definition(&self) -> ToolDefinition;
    // 接收模型传入的 JSON 参数，并把执行结果转换成 Observation 文本。
    fn execute(&self, args: serde_json::Value) -> ToolResult;
}
