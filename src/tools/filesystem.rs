use std::fs;

use crate::tools::Tool;
use crate::tools::schema::ToolDefinition;

pub struct FileSystemTool;

impl Tool for FileSystemTool {
    fn definition(&self) -> ToolDefinition {
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

    fn execute(&self, args: serde_json::Value) -> String {
        let path = args["path"].as_str().unwrap_or(".");

        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(error) => {
                return format!("读取目录失败:{}", error);
            }
        };

        let mut rust_file_count = 0;

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "rs" {
                        rust_file_count += 1;
                    }
                }
            }
        }

        format!("目录 {} 中有 {} 个 Rust 文件", path, rust_file_count)
    }
}
