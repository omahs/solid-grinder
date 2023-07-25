use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FunctionArgObject {
    pub function_name: String,
    pub name: String,
    pub memory_type: bool,
    pub r#type: String,
    pub custom_type: bool,
    pub packed_bit_size: i8,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FunctionObject {
    pub args: Vec<FunctionArgObject>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ContractObject {
    pub solidity_filepath: String,
    pub contract_name: String,
    pub function_name: String,
    pub function: FunctionObject,
}
