use std::fs;

use crate::tool_result::ToolResult;
use crate::tools::Tool;
use crate::tools::schema::ToolDefinition;

pub struct FileSystemTool;

impl Tool for FileSystemTool {
    fn definition(&self) -> ToolDefinition {
        // 这份定义相当于给 LLM 的工具说明书；parameters 使用 JSON Schema。
        ToolDefinition {
            name: "filesystem".to_string(),
            description: "读取目录并统计 Rust 文件".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "要读取的目录路径"
                    }
                },
                "required": ["path"]
            }),
        }
    }

    fn execute(&self, args: serde_json::Value) -> ToolResult {
        // 如果参数缺少 path，默认读取当前目录，避免示例程序直接崩溃。
        let path = args["path"].as_str().unwrap_or(".");

        // read_dir 失败时，把错误转成字符串返回，而不是让 Agent 进程退出。
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(error) => {
                return ToolResult::Error(format!("读取目录失败:{}", error));
            }
        };

        let mut rust_file_count = 0;

        for entry in entries {
            // 单个目录项读取失败时跳过它，继续统计其他目录项。
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let path = entry.path();

            // 当前只统计目录的直接子项，不递归进入子目录。
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "rs" {
                        rust_file_count += 1;
                    }
                }
            }
        }

        // 返回给 execute_action 的文本会被打印为 Observation。
        ToolResult::Success(format!(
            "目录 {} 中有 {} 个 Rust 文件",
            path, rust_file_count
        ))
    }
}
