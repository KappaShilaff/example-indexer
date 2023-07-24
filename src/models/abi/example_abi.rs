use nekoton_abi::{PackAbi, PackAbiPlain, UnpackAbiPlain};

#[derive(Debug, Clone, PackAbi, UnpackAbiPlain)]
pub struct ExampleEvent {
    #[abi(address)]
    pub value0: ton_block::MsgAddressInt,
}

#[derive(Debug, Clone, PackAbiPlain, UnpackAbiPlain)]
pub struct ExampleFunctionInput {
    #[abi(uint32)]
    pub value1: u32,
    #[abi(address)]
    pub value2: ton_block::MsgAddressInt,
}

#[derive(Debug, Clone, PackAbi, UnpackAbiPlain)]
pub struct ExampleFunctionOutput {
    #[abi(address)]
    pub value0: ton_block::MsgAddressInt,
}