// Action 是 LLM 和 Agent Runtime 之间的“协议”。
// 模型不直接操作工具，而是先返回一个动作，再由运行时统一执行。
#[derive(Debug)]
pub enum Action {
    // 暂时不调用工具，继续下一轮思考。
    Continue,
    // 请求运行时调用指定工具，并传入 JSON 参数。
    ToolCall {
        name: String,
        args: serde_json::Value,
    },
    // 告诉运行时任务已经完成，可以退出循环。
    Finish,
}
