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
pub struct EVMNetwork {
    pub http_url: String,
    pub socket_url: String,
    pub chain_id: Option<u64>,
}

/*
BSC
https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/mainnet
wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/mainnet/ws

https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet
wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws

AVAX
https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/avalanche/mainnet
wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/avalanche/mainnet/ws

https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/avalanche/testnet
wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/avalanche/testnet/ws
*/

impl EVMNetwork {
    pub async fn new(name: &str) -> EVMNetwork {
        let mut _http_url =
            "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
        let mut _socket_url =
            "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";

        switch! { name;
            "BSC" => {
                _http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
                _socket_url = "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
            },
            "BSC_TESTNET" => {
                _http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
                _socket_url = "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
            },
            "AVALANCHE" => {
                _http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
                _socket_url = "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
            },
            "AVALANCHE_TESTNET" => {
                _http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
                _socket_url = "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
            },
            _ => {
                _http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
                _socket_url = "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
            },
        }

        let u64chain_id: u64 = 97;

        let chain_id: Option<u64> = Option::Some(u64::try_from(u64chain_id).unwrap());

        EVMNetwork {
            http_url: String::from(_http_url),
            socket_url: String::from(_socket_url),
            chain_id,
        }
    }
}
