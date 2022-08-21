pub use crate::components::generated::parser::*;
use serde_json::{Map, Value};
use std::collections::HashMap;
pub use text_log_parser::{parse, LogFormat};

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,

        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let format = inputs.log_format.as_str();
        let seperator = inputs.field_seperator.as_str();
        let log_format = LogFormat::new(format, seperator);

        let log_string = inputs.log_entry.as_str();
        let log_message_hash = parse(log_format, log_string);
        let mut log_message_map = HashMap::new();

        for (key, value) in log_message_hash {
            log_message_map.insert(key, value);
        }

        outputs.log_message.done(log_message_map);
        Ok(())
    }
}
