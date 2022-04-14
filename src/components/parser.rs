pub use crate::components::generated::parser::*;
pub use combined_log_parser::Format;
use vino_provider::info;
use std::str::FromStr;

pub(crate) fn job(input: Inputs, output: OutputPorts) -> JobResult {
    let format_str = input.log_format.as_str();
    let format = Format::from_str(format_str).unwrap();
    let log_entry = input.log_entry.as_str();
    let log_entry_struct = format.parse_to_value(log_entry).unwrap();
    output.log_message.done(&log_entry_struct);
    Ok(())
}
