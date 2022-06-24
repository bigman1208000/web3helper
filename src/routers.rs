use std::convert::{From, TryFrom};

#[macro_export]
macro_rules! switch {
    ($v:expr; $($a:expr => $b:expr,)* _ => $e:expr $(,)?) => {
        match $v {
            $(v if v == $a => $b,)*
            _ => $e,
        }
    };
}

#[derive(Clone, Debug)]
pub struct DexRouter {
    pub address:String,
    pub factory: String,
    pub WETH: String,
}

impl DexRouter {
    pub async fn new(name: &str) -> DexRouter {
        let mut address ="";
        let mut factory ="";
        let mut WETH ="";

        DexRouter {
            address: address,
            factory: factory,
            WETH,
        }
    }
}
