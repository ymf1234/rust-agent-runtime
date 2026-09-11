#[derive(Debug)]
pub enum Message {
    User(String),
    Assistant(String),
    Tool(String),
}