use crate::{action::Action, tool_call::ToolCall};

pub fn tool_call_to_action(tool_call: ToolCall) -> Action {
    match tool_call.name.as_str() {
        "finish" => Action::Finish,
        _ => Action::ToolCall {
            name: tool_call.name,
            args: tool_call.arguments,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::tool_call_to_action;
    use crate::{action::Action, tool_call::ToolCall};

    #[test]
    fn finish_tool_call_becomes_finish_action() {
        let action = tool_call_to_action(ToolCall {
            name: "finish".to_string(),
            arguments: serde_json::json!({}),
        });

        assert!(matches!(action, Action::Finish));
    }
}
