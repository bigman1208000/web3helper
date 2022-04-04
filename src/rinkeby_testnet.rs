use crate::traits::GetAddress;
use lazy_static::lazy_static;
use ::std::collections::HashMap;

lazy_static! {
    static ref RINKE_BY_TESTNET: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("ATOM / ETH", "0xc751E86208F0F8aF2d5CD0e29716cA7AD98B5eF5");
        map.insert("ATOM / USD", "0x3539F2E214d8BC7E611056383323aC6D1b01943c");
        map.insert("AUD / USD", "0x21c095d2aDa464A294956eA058077F14F66535af");
        map.insert("Arb Seq Status","0x13E99C19833F557672B67C70508061A2E1e54162");
        map.insert("BAT / USD", "0x031dB56e01f82f20803059331DC6bEe9b17F7fC9");
        map.insert("BNB / USD", "0xcf0f51ca2cDAecb464eeE4227f5295F2384F84ED");
        map.insert("BTC / ETH", "0x2431452A0010a43878bF198e170F6319Af6d27F4");
        map.insert("BTC / USD", "0xECe365B379E1dD183B20fc5f022230C044d51404");
        map.insert("CHF / USD", "0x5e601CF5EF284Bcd12decBDa189479413284E1d2");
        map.insert("CacheGold PoR","0x46F030f9A570aBB4BF21bAc93042d97059bd0350");
        map.insert("DAI / ETH", "0x74825DbC8BF76CC4e9494d0ecB210f676Efa001D");
        map.insert("DAI / USD", "0x2bA49Aaa16E6afD2a993473cfB70Fa8559B523cF");
        map.insert("ETH / USD", "0x8A753747A1Fa494EC906cE90E9f37563A8AF630e");
        map.insert("EUR / USD", "0x78F9e60608bF48a1155b4B2A5e31F32318a1d85F");
        map.insert("Fast Gas", "0xCe3f7378aE409e1CE0dD6fFA70ab683326b73f04");
        map.insert("GBP / USD", "0x7B17A813eEC55515Fb8F49F2ef51502bC54DD40F");
        map.insert("GUSD / ETH", "0xb4c4a493AB6356497713A78FFA6c60FB53517c63");
        map.insert("GUSD / USD", "0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e");
        map.insert("ILV / ETH", "0x48731cF7e84dc94C5f84577882c14Be11a5B7456");
        map.insert("JPY / USD", "0x3Ae2F46a2D84e3D5590ee6Ee5116B80caF77DeCA");
        map.insert("LINK / ETH", "0xFABe80711F3ea886C3AC102c81ffC9825E16162E");
        map.insert("LINK / USD", "0xd8bD0a1cB028a31AA859A21A3758685a95dE4623");
        map.insert("LTC / USD", "0x4d38a35C2D87976F334c2d2379b535F1D461D9B4");
        map.insert("MATIC / USD", "0x7794ee502922e2b723432DDD852B3C30A911F021");
        map.insert("REP / USD", "0x9331b55D9830EF609A2aBCfAc0FBCE050A52fdEa");
        map.insert("SNX / USD", "0xE96C4407597CD507002dF88ff6E0008AB41266Ee");
        map.insert("TRX / USD", "0xb29f616a0d54FF292e997922fFf46012a63E2FAe");
        map.insert("USDC / ETH", "0xdCA36F27cbC4E38aE16C4E9f99D39b42337F6dcf");
        map.insert("USDC / USD", "0xa24de01df22b63d23Ebc1882a5E3d4ec0d907bFB");
        map.insert("XAG / USD", "0x9c1946428f4f159dB4889aA6B218833f467e1BfD");
        map.insert("XAU / USD", "0x81570059A0cb83888f1459Ec66Aad1Ac16730243");
        map.insert("XRP / USD", "0xc3E76f41CAbA4aB38F00c7255d4df663DA02A024");
        map.insert("XTZ / USD", "0xf57FCa8B932c43dFe560d3274262b2597BCD2e5A");
        map.insert("ZRX / USD", "0xF7Bbe4D7d13d600127B6Aa132f1dCea301e9c8Fc");
        map.insert("sCEX / USD", "0x1a602D4928faF0A153A520f58B332f9CAFF320f7");
        map.insert("sDEFI / USD", "0x0630521aC362bc7A19a4eE44b57cE72Ea34AD01c");
        map
    };
}

pub struct RinkeByTestNet;

impl GetAddress for RinkeByTestNet {
    fn get_address(&self, feed_name: &str) -> Option<String> {
        Some(RINKE_BY_TESTNET.get(feed_name)?.to_string())
    }
}
