use crate::agent::Agent;

mod action;
mod agent;
mod loops;
mod state;
mod tools;
mod llm;
fn main() {
    println!("Agent Runtime Started");

    Agent::run("统计 src 目录中的 Rust 文件".to_string());
}
