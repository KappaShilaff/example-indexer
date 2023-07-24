use once_cell::sync::OnceCell;
use ton_abi::Contract;
use transaction_buffer::models::AnyExtractable;

pub fn all_for_parse() -> Vec<AnyExtractable> {
    vec![
        AnyExtractable::Event(example_abi().events.get("exampleEvent").unwrap().clone()),
        AnyExtractable::Function(
            example_abi()
                .functions
                .get("exampleFunction")
                .unwrap()
                .clone(),
        ),
    ]
}

macro_rules! declare_abi {
    ($($contract:ident => $source:literal),*$(,)?) => {$(
    #[allow(non_snake_case)]
        pub fn $contract() -> &'static Contract {
            static ABI: OnceCell<Contract> = OnceCell::new();
            ABI.load(include_str!($source))
        }
    )*};
}

trait OnceCellExt {
    fn load(&self, data: &str) -> &Contract;
}

impl OnceCellExt for OnceCell<Contract> {
    fn load(&self, data: &str) -> &Contract {
        self.get_or_init(|| Contract::load(data).expect("Trust me"))
    }
}

declare_abi! {
    example_abi => "ExampleAbi.abi.json",
}
