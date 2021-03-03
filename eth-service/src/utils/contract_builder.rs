use ethabi::{Function, Param, ParamType};
pub struct ContractBuilder {}

impl ContractBuilder {
    pub fn create_send_fn() -> Function {
        let interface = Function {
            name: "send".to_owned(),
            inputs: vec![
                Param {
                    name: "to".to_owned(),
                    kind: ParamType::FixedBytes(32),
                },
                Param {
                    name: "amount".to_owned(),
                    kind: ParamType::Uint(256),
                },
            ],
            outputs: vec![],
            constant: false,
        };

        Function::from(interface)
    }

    pub fn create_request_fn() -> Function {
        let interface = Function {
            name: "request".to_owned(),
            inputs: vec![
                Param {
                    name: "from".to_owned(),
                    kind: ParamType::FixedBytes(32),
                },
                Param {
                    name: "amount".to_owned(),
                    kind: ParamType::Uint(256),
                },
            ],
            outputs: vec![],
            constant: false,
        };

        Function::from(interface)
    }
}
