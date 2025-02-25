use crate::types::ChatCompletionFunctions;

use super::{AssistantTools, AssistantToolsCode, AssistantToolsFunction, AssistantToolsRetrieval};

impl From<AssistantToolsCode> for AssistantTools {
    fn from(value: AssistantToolsCode) -> Self {
        Self::Code(value)
    }
}

impl From<AssistantToolsRetrieval> for AssistantTools {
    fn from(value: AssistantToolsRetrieval) -> Self {
        Self::Retrieval(value)
    }
}

impl From<AssistantToolsFunction> for AssistantTools {
    fn from(value: AssistantToolsFunction) -> Self {
        Self::Function(value)
    }
}

impl Default for AssistantToolsCode {
    fn default() -> Self {
        Self {
            r#type: "code_interpreter".into(),
        }
    }
}

impl Default for AssistantToolsRetrieval {
    fn default() -> Self {
        Self {
            r#type: "retrieval".into(),
        }
    }
}

impl Default for AssistantToolsFunction {
    fn default() -> Self {
        Self {
            r#type: "function".into(),
            function: Default::default(),
        }
    }
}

impl From<ChatCompletionFunctions> for AssistantToolsFunction {
    fn from(value: ChatCompletionFunctions) -> Self {
        Self {
            r#type: "function".into(),
            function: value,
        }
    }
}

impl From<ChatCompletionFunctions> for AssistantTools {
    fn from(value: ChatCompletionFunctions) -> Self {
        Self::Function(value.into())
    }
}
