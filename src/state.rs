#[derive(Debug)]
pub struct AgentState {
    // 用户目标
    pub goal: String,
    // 当前执行次数
    pub step: u32,

    // 是否完成
    pub finished: bool,
}
