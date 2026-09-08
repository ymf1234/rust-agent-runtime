use crate::tool_call::ToolCall;

pub fn parse_tool_call(response: &str) -> Result<ToolCall, serde_json::Error> {
    serde_json::from_str(response)
}
