/**********************************************
***** This file is generated, do not edit *****
***********************************************/

pub use vino_provider::prelude::*;

pub mod __multi__;
pub mod parser; // parser

type Result<T> = std::result::Result<T, WasmError>;

#[no_mangle]
pub(crate) extern "C" fn __guest_call(op_len: i32, req_len: i32) -> i32 {
    use std::slice;

    let buf: Vec<u8> = Vec::with_capacity(req_len as _);
    let req_ptr = buf.as_ptr();

    let opbuf: Vec<u8> = Vec::with_capacity(op_len as _);
    let op_ptr = opbuf.as_ptr();

    let (slice, op) = unsafe {
        wapc::__guest_request(op_ptr, req_ptr);
        (
            slice::from_raw_parts(req_ptr, req_len as _),
            slice::from_raw_parts(op_ptr, op_len as _),
        )
    };

    let op_str = ::std::str::from_utf8(op).unwrap();

    match Dispatcher::dispatch(op_str, slice) {
        Ok(response) => {
            unsafe { wapc::__guest_response(response.as_ptr(), response.len()) }
            1
        }
        Err(e) => {
            let errmsg = e.to_string();
            unsafe {
                wapc::__guest_error(errmsg.as_ptr(), errmsg.len() as _);
            }
            0
        }
    }
}

static ALL_COMPONENTS: &[&str] = &["parser"];

pub struct Dispatcher {}
impl Dispatch for Dispatcher {
    fn dispatch(op: &str, payload: &[u8]) -> CallResult {
        let payload = IncomingPayload::from_buffer(payload)?;
        let result = match op {
            "parser" => {
                crate::components::generated::parser::Component::default().execute(&payload)
            }
            _ => Err(WasmError::ComponentNotFound(
                op.to_owned(),
                ALL_COMPONENTS.join(", "),
            )),
        }?;
        Ok(serialize(&result)?)
    }
}

pub mod types {
    // no additional types
}

pub mod generated {
    use super::*;

    // start namespace
    pub mod parser {
        use crate::components::parser as implementation;

        pub use vino_provider::prelude::*;

        use super::*;

        #[derive(Default)]
        pub struct Component {}

        impl WapcComponent for Component {
            fn execute(&self, payload: &IncomingPayload) -> JobResult {
                let outputs = get_outputs(payload.id());
                let inputs = populate_inputs(payload)?;
                implementation::job(inputs, outputs)
            }
        }

        fn populate_inputs(payload: &IncomingPayload) -> Result<Inputs> {
            Ok(Inputs {
                log_entry: deserialize(payload.get("log_entry")?)?,
                log_format: deserialize(payload.get("log_format")?)?,
                field_seperator: deserialize(payload.get("field_seperator")?)?,
            })
        }

        impl From<Inputs> for TransportMap {
            fn from(inputs: Inputs) -> TransportMap {
                let mut map = TransportMap::new();
                map.insert("log_entry", MessageTransport::success(&inputs.log_entry));
                map.insert("log_format", MessageTransport::success(&inputs.log_format));
                map.insert(
                    "field_seperator",
                    MessageTransport::success(&inputs.field_seperator),
                );
                map
            }
        }

        #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
        pub struct Inputs {
            #[serde(rename = "log_entry")]
            pub log_entry: String,
            #[serde(rename = "log_format")]
            pub log_format: String,
            #[serde(rename = "field_seperator")]
            pub field_seperator: String,
        }

        #[derive(Debug)]
        pub struct OutputPorts {
            pub log_message: LogMessageSender,
        }

        #[derive(Debug, PartialEq, Clone)]
        pub struct LogMessageSender {
            id: u32,
        }

        impl PortSender for LogMessageSender {
            type PayloadType = serde_json::Value;
            fn get_name(&self) -> String {
                "log_message".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }

        fn get_outputs(id: u32) -> OutputPorts {
            OutputPorts {
                log_message: LogMessageSender { id },
            }
        }

        #[derive(Debug)]
        pub struct Outputs {
            packets: ProviderOutput,
        }

        impl Outputs {
            pub fn log_message(&mut self) -> Result<PortOutput> {
                let packets = self.packets.take("log_message").ok_or_else(|| {
                    ComponentError::new("No packets for port 'log_message' found")
                })?;
                Ok(PortOutput::new("log_message".to_owned(), packets))
            }
        }

        impl From<ProviderOutput> for Outputs {
            fn from(packets: ProviderOutput) -> Self {
                Self { packets }
            }
        }
    }

    pub mod __multi__ {
        use super::Result;
        use crate::components::__multi__ as implementation;

        #[cfg(any(feature = "native"))]
        pub use vino_provider::native::prelude::*;
        #[cfg(any(feature = "wasm"))]
        pub use vino_provider::wasm::prelude::*;

        pub use vino_provider::prelude::*;
        #[derive(Default)]
        pub struct Component {}

        impl WapcComponent for Component {
            fn execute(&self, payload: &IncomingPayload) -> JobResult {
                let outputs = get_outputs(payload.id());
                let inputs = populate_inputs(payload)?;
                implementation::job(inputs, outputs)
            }
        }

        fn populate_inputs(payload: &IncomingPayload) -> Result<Vec<ComponentInputs>> {
            Ok(deserialize(payload.get("inputs")?)?)
        }

        #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
        pub enum ComponentInputs {
            Parser(super::parser::Inputs),
        }

        #[cfg(all(feature = "guest"))]
        #[allow(missing_debug_implementations)]
        pub enum ComponentOutputs {
            Parser(super::parser::Outputs),
        }

        #[derive(Debug)]
        pub struct OutputPorts {
            pub result: ResultSender,
        }

        #[derive(Debug, PartialEq, Clone)]
        pub struct ResultSender {
            id: u32,
        }

        impl PortSender for ResultSender {
            type PayloadType = bool;
            fn get_name(&self) -> String {
                "result".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }

        fn get_outputs(id: u32) -> OutputPorts {
            OutputPorts {
                result: ResultSender { id },
            }
        }

        #[derive(Debug)]
        pub struct Outputs {
            packets: ProviderOutput,
        }

        impl Outputs {
            pub fn result(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("result")
                    .ok_or_else(|| ComponentError::new("No packets for port 'result' found"))?;
                Ok(PortOutput::new("result".to_owned(), packets))
            }
        }

        impl From<ProviderOutput> for Outputs {
            fn from(packets: ProviderOutput) -> Self {
                Self { packets }
            }
        }
    }
}
