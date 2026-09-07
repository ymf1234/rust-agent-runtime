use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    // 模型在 ToolCall 中使用的唯一名称。
    pub name: String,

    // 给模型看的自然语言说明，应该明确工具能做什么。
    pub description: String,

    // JSON Schema 格式的参数说明，用于约束调用参数。
    pub parameters: serde_json::Value,
}
