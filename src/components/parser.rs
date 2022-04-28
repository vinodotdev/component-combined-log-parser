pub use crate::components::generated::parser::*;
use serde_json::{Map, Value};
pub use text_log_parser::{parse, LogFormat};

pub(crate) fn job(input: Inputs, output: OutputPorts) -> JobResult {
    let format = input.log_format.as_str();
    let seperator = input.field_seperator.as_str();
    let log_format = LogFormat::new(format, seperator);

    let log_string = input.log_entry.as_str();
    let log_message_hash = parse(log_format, log_string);
    let mut log_message_map = Map::new();

    for (key, value) in log_message_hash {
        log_message_map.insert(key, Value::String(value));
    }

    let log_entry_struct = Value::Object(log_message_map);

    output.log_message.done(&log_entry_struct);

    Ok(())
}