#[derive(Debug)]
pub enum Action {
    Continue,
    ToolCall {
        name: String,
        args: serde_json::Value,
    },
    Finish,
}
