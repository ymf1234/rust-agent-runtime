use crate::agent::Agent;

// 二进制程序的模块入口。这里声明的模块才会参与编译；src/mod.rs 不会被使用。
mod action;
mod agent;
mod loops;
mod state;
mod tools;
mod llm;

fn main() {
    println!("Agent Runtime Started");

    // 真实项目中 goal 可以来自命令行或用户输入；当前先用固定目标演示完整流程。
    Agent::run("统计 src 目录中的 Rust 文件".to_string());
}
