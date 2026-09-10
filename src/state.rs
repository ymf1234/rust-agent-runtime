#[derive(Debug)]
pub struct AgentState {
    // 用户希望 Agent 完成的事情。
    pub goal: String,
    // 已经执行过多少轮，用来让模型了解当前进度。
    pub step: u32,

    // 变为 true 后，Agent::run 中的 while 循环结束。
    pub finished: bool,

    pub observation: Option<String>,
}
