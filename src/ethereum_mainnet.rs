use alloc::string::String;
use crate::traits::GetAddress;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref ETHEREUM_MAIN_NET: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("1INCH / ETH", "0x72AFAECF99C9d9C8215fF44C77B94B99C28741e8");
        map.insert("1INCH / USD", "0xc929ad75B72593967DE83E7F7Cda0493458261D9");
        map.insert("AAPL / USD", "0x139C8512Cde1778e9b9a8e721ce1aEbd4dD43587");
        map.insert("AAVE / ETH", "0x6Df09E975c830ECae5bd4eD9d90f3A95a4f88012");
        map.insert("AAVE / USD", "0x547a514d5e3769680Ce22B2361c10Ea13619e8a9");
        map.insert("ACH / USD", "0xfDFa69A65826e86BD25478ACE08294DC49c02237");
        map.insert("ADA / USD", "0xAE48c91dF1fE419994FFDa27da09D5aC69c30f55");
        map.insert("ADX / USD", "0x231e764B44b2C1b7Ca171fa8021A24ed520Cde10");
        map.insert("AKRO / USD", "0xB23D105dF4958B4b81757e12f2151B5b5183520B");
        map.insert("ALBT / USD", "0x057e52Fb830318E096CD96F369f0DB4B196fBfa7");
        map.insert("ALCX / ETH", "0x194a9AaF2e0b67c35915cD01101585A33Fe25CAa");
        map.insert("ALPHA / ETH", "0x89c7926c7c15fD5BFDB1edcFf7E7fC8283B578F6");
        map.insert("AMP / USD", "0x8797ABc4641dE76342b8acE9C63e3301DC35e3d8");
        map.insert("AMPL / ETH", "0x492575FDD11a0fCf2C6C719867890a7648d526eB");
        map.insert("AMPL / USD", "0xe20CA8D7546932360e37E9D72c1a47334af57706");
        map.insert("AMZN / USD", "0x8994115d287207144236c13Be5E2bDbf6357D9Fd");
        map.insert("ANKR / ETH", "0x2f2ea25382A236FD115Dff160d258351B8b32D63");
        map.insert("ANKR / USD", "0x7eed379bf00005CfeD29feD4009669dE9Bcc21ce");
        map.insert("ANT / ETH", "0x8f83670260F8f7708143b836a2a6F11eF0aBac01");
        map.insert("ARPA / USD", "0xc40ec815A2f8eb9912BD688d3bdE6B6D50A37ff2");
        map.insert("ATOM / ETH", "0x15c8eA24Ba2d36671Fa22aD4Cff0a8eafe144352");
        map.insert("ATOM / USD", "0xDC4BDB458C6361093069Ca2aD30D74cc152EdC75");
        map.insert(
            "AUCTION / USD",
            "0xA6BCac72431A4178f07d016E1D912F56E6D989Ec",
        );
        map.insert("AUD / USD", "0x77F9710E7d0A19669A13c055F62cd80d313dF022");
        map.insert("AUDIO / USD", "0xBf739E677Edf6cF3408857404746cAcfd7120EB2");
        map.insert("AUTO / USD", "0x21c44778293E43afC0F318aC051Ef867c3bDB5ee");
        map.insert("AVAX / USD", "0xFF3EEb22B5E3dE6e705b44749C2559d704923FD7");
        map.insert("AXS / ETH", "0x8B4fC5b68cD50eAc1dD33f695901624a4a1A0A8b");
        map.insert("BADGER / ETH", "0x58921Ac140522867bf50b9E009599Da0CA4A2379");
        map.insert("BADGER / USD", "0x66a47b7206130e6FF64854EF0E1EDfa237E65339");
        map.insert("BAL / ETH", "0xC1438AA3823A6Ba0C159CfA8D98dF5A994bA120b");
        map.insert("BAL / USD", "0xdF2917806E30300537aEB49A7663062F4d1F2b5F");
        map.insert("BAND / ETH", "0x0BDb051e10c9718d1C29efbad442E88D38958274");
        map.insert("BAND / USD", "0x919C77ACc7373D000b329c1276C76586ed2Dd19F");
        map.insert("BAT / ETH", "0x0d16d4528239e9ee52fa531af613AcdB23D88c94");
        map.insert("BAT / USD", "0x9441D7556e7820B5ca42082cfa99487D56AcA958");
        map.insert("BCH / USD", "0x9F0F69428F923D6c95B781F89E165C9b2df9789D");
        map.insert("BETA / ETH", "0x8eb7bAe1eCd3dcf87159Eb5BACe78209722F795B");
        map.insert("BICO / USD", "0x3aFFc457372D7b64F5F4BdC46D0989baA96DC74A");
        map.insert("BIT / USD", "0x7b33EbfA52F215a30FaD5a71b3FeE57a4831f1F0");
        map.insert("BNB / ETH", "0xc546d2d06144F9DD42815b8bA46Ee7B8FcAFa4a2");
        map.insert("BNB / USD", "0x14e613AC84a31f709eadbdF89C6CC390fDc9540A");
        map.insert("BNT / ETH", "0xCf61d1841B178fe82C8895fe60c2EDDa08314416");
        map.insert("BNT / USD", "0x1E6cF0D433de4FE882A437ABC654F58E1e78548c");
        map.insert("BOND / ETH", "0xdd22A54e05410D8d1007c38b5c7A3eD74b855281");
        map.insert("BRL / USD", "0x971E8F1B779A5F1C36e1cd7ef44Ba1Cc2F5EeE0f");
        map.insert("BTC / ETH", "0xdeb288F737066589598e9214E782fa5A8eD689e8");
        map.insert("BTC / USD", "0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c");
        map.insert("BTC / height", "0x4D2574c790d836b8C886615d927e9BA585B10EbA");
        map.insert(
            "BTC Difficulty",
            "0xA792Ebd0E4465DB2657c7971519Cfa0f0275F428",
        );
        map.insert(
            "BTC-USD Total Marketcap",
            "0x47E1e89570689c13E723819bf633548d611D630C",
        );
        map.insert("BTM / USD", "0x9fCCF42D21AB278e205e7Bb310D8979F8f4B5751");
        map.insert("BUSD / ETH", "0x614715d2Af89E6EC99A233818275142cE88d1Cfd");
        map.insert("BUSD / USD", "0x833D8Eb16D306ed1FbB5D7A2E019e106B960965A");
        map.insert("C98 / USD", "0xE95CDc33E1F5BfE7eB26f45E29C6C9032B97db7F");
        map.insert("CAD / USD", "0xa34317DB73e77d453b1B8d04550c44D10e981C8e");
        map.insert("CEL / ETH", "0x75FbD83b4bd51dEe765b2a01e8D3aa1B020F9d33");
        map.insert("CELO / USD", "0x10D35eFa5C26C3d994C511576641248405465AeF");
        map.insert("CELR / USD", "0xb19Ca014fC913508225d2B7228F909b299d05a41");
        map.insert("CHF / USD", "0x449d117117838fFA61263B61dA6301AA2a88B13A");
        map.insert("CNY / USD", "0xeF8A4aF35cd47424672E3C590aBD37FBB7A7759a");
        map.insert("COIN / USD", "0xb10a047f8db80d781D006F1401BEB7d70Eb4da1a");
        map.insert("COMP / ETH", "0x1B39Ee86Ec5979ba5C322b826B3ECb8C79991699");
        map.insert("COMP / USD", "0xdbd020CAeF83eFd542f4De03e3cF0C28A4428bd5");
        map.insert("CREAM / ETH", "0x82597CFE6af8baad7c0d441AA82cbC3b51759607");
        map.insert("CRO / ETH", "0xcA696a9Eb93b81ADFE6435759A29aB4cf2991A96");
        map.insert("CRO / USD", "0x00Cb80Cf097D9aA9A3779ad8EE7cF98437eaE050");
        map.insert("CRV / ETH", "0x8a12Be339B0cD1829b91Adc01977caa5E9ac121e");
        map.insert("CRV / USD", "0xCd627aA160A6fA45Eb793D19Ef54f5062F20f33f");
        map.insert("CSPR / USD", "0x9e37a8Ee3bFa8eD6783Db031Dc458d200b226074");
        map.insert("CTSI / ETH", "0x0a1d1b9847d602e789be38B802246161FFA24930");
        map.insert("CTX / USD", "0x441c93944F428B0404Ff30BbB1DC2BaB35A6F04F");
        map.insert("CV / Index", "0x1B58B67B2b2Df71b4b0fb6691271E83A0fa36aC5");
        map.insert("CVX / USD", "0xd962fC30A72A84cE50161031391756Bf2876Af5D");
        map.insert(
            "CacheGold PoR USD",
            "0x5586bF404C7A22A4a4077401272cE5945f80189C",
        );
        map.insert(
            "Calculated XSUSHI / ETH",
            "0xF05D9B6C08757EAcb1fbec18e36A1B7566a13DEB",
        );
        map.insert(
            "Calculated XSUSHI / USD",
            "0xCC1f5d9e6956447630d703C8e93b2345c2DE3D13",
        );
        map.insert(
            "CelsiusX Cardano->Ethereum ADA PoR",
            "0xB95c17882EA3d06f7091D12ce32E7eEBC8D8a6a6",
        );
        map.insert(
            "CelsiusX Dogecoin->Ethereum DOGE PoR",
            "0xe6D28A56E6bD1C123c8210f9A9c95bb6e107A1ef",
        );
        map.insert("DAI / ETH", "0x773616E4d11A78F511299002da57A0a94577F1f4");
        map.insert("DAI / USD", "0xAed0c38402a5d19df6E4c03F4E2DceD6e29c1ee9");
        map.insert("DASH / USD", "0xFb0cADFEa136E9E343cfb55B863a6Df8348ab912");
        map.insert("DATA / ETH", "0xD48B96131F3de05B7C3500891C8c4c1E2dbc6E3d");
        map.insert("DIA / USD", "0xeE636E1f7A0A846EEc2385E729CeA7D1b339D40D");
        map.insert("DNT / ETH", "0x1F9eB026e549a5f47A6aa834689053117239334A");
        map.insert("DODO / USD", "0x9613A51Ad59EE375e6D8fa12eeef0281f1448739");
        map.insert("DOGE / USD", "0x2465CefD3b488BE410b941b1d4b2767088e2A028");
        map.insert("DOT / USD", "0x1C07AFb8E2B827c5A4739C6d59Ae3A5035f28734");
        map.insert(
            "DPI / ETH Index",
            "0x029849bbc0b1d93b85a8b6190e979fd38F5760E2",
        );
        map.insert(
            "DPI / USD Index",
            "0xD2A593BF7594aCE1faD597adb697b5645d5edDB2",
        );
        map.insert("DYDX / USD", "0x478909D4D798f3a1F11fFB25E4920C959B4aDe0b");
        map.insert("ENJ / ETH", "0x24D9aB51950F3d62E9144fdC2f3135DAA6Ce8D1B");
        map.insert("ENJ / USD", "0x23905C55dC11D609D5d11Dc604905779545De9a7");
        map.insert("ENS / USD", "0x5C00128d4d1c2F4f652C267d7bcdD7aC99C16E16");
        map.insert("EOS / USD", "0x10a43289895eAff840E8d45995BBa89f9115ECEe");
        map.insert("EPS / USD", "0x9831e1fC56f473B42f5CE2a856D5c8706ee3949f");
        map.insert("ERN / USD", "0x0a87e12689374A4EF49729582B474a1013cceBf8");
        map.insert("ETC / USD", "0xaEA2808407B7319A31A383B6F8B60f04BCa23cE2");
        map.insert("ETH / BTC", "0xAc559F25B1619171CbC396a50854A3240b6A4e99");
        map.insert("ETH / USD", "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419");
        map.insert("ETH / XDR", "0xb022E2970b3501d8d83eD07912330d178543C1eB");
        map.insert(
            "ETH-USD Total Marketcap",
            "0xAA2FE1324b84981832AafCf7Dc6E6Fe6cF124283",
        );
        map.insert("EUR / USD", "0xb49f677943BC038e9857d61E7d053CaA2C1734C1");
        map.insert(
            "EURS RESERVES",
            "0xbcD05A3E0c11f340cCcD9a4Efe05eEB2b33AB67A",
        );
        map.insert("EURT / USD", "0x01D391A48f4F7339aC64CA2c83a07C22F95F587a");
        map.insert("FARM / ETH", "0x611E0d2709416E002A3f38085e4e1cf77c015921");
        map.insert("FB / USD", "0xCe1051646393087e706288C1B57Fd26446657A7f");
        map.insert("FEI / ETH", "0x7F0D2c2838c6AC24443d13e23d99490017bDe370");
        map.insert("FEI / USD", "0x31e0a88fecB6eC0a411DBe0e9E76391498296EE9");
        map.insert("FET / USD", "0xf0ddE55cA308eaa95eF3EB433dfE7200CEc09ffe");
        map.insert("FIL / ETH", "0x0606Be69451B1C9861Ac6b3626b99093b713E801");
        map.insert("FIL / USD", "0x1A31D42149e82Eb99777f903C08A2E41A00085d3");
        map.insert("FLOW / USD", "0xD9BdD9f5ffa7d89c846A5E3231a093AE4b3469D2");
        map.insert("FOR / USD", "0x456834f736094Fb0AAD40a9BBc9D4a0f37818A54");
        map.insert("FOX / USD", "0xccA02FFEFAcE21325befD6616cB4Ba5fCB047480");
        map.insert("FRONT / USD", "0xbf86e7B2565eAc3bFD80634176F31bd186566b06");
        map.insert("FTM / ETH", "0x2DE7E4a9488488e0058B95854CC2f7955B35dC9b");
        map.insert("FTT / ETH", "0xF0985f7E2CaBFf22CecC5a71282a89582c382EFE");
        map.insert("FTT / USD", "0x84e3946C6df27b453315a1B38e4dECEF23d9F16F");
        map.insert("FXS / USD", "0x6Ebc52C8C1089be9eB3945C4350B68B8E4C2233f");
        map.insert(
            "Fast Gas / Gwei",
            "0x169E633A2D1E6c10dD91238Ba11c4A708dfEF37C",
        );
        map.insert("GBP / USD", "0x5c0Ab2d9b5a7ed9f470386e82BB36A3613cDd4b5");
        map.insert("GHST / ETH", "0x5877385f9F51B46Bbd93F24AD278D681E1Fd2A93");
        map.insert("GLM / USD", "0x83441C3A10F4D05de6e0f2E849A850Ccf27E6fa7");
        map.insert("GNO / ETH", "0xA614953dF476577E90dcf4e3428960e221EA4727");
        map.insert("GOOGL / USD", "0x36D39936BeA501755921beB5A382a88179070219");
        map.insert("GRT / ETH", "0x17D054eCac33D91F7340645341eFB5DE9009F1C1");
        map.insert("GRT / USD", "0x86cF33a451dE9dc61a2862FD94FF4ad4Bd65A5d2");
        map.insert("GTC / ETH", "0x0e773A17a01E2c92F5d4c53435397E2bd48e215F");
        map.insert("GUSD / ETH", "0x96d15851CBac05aEe4EFD9eA3a3DD9BDEeC9fC28");
        map.insert("GUSD / USD", "0xa89f5d2365ce98B3cD68012b6f503ab1416245Fc");
        map.insert("HBAR / USD", "0x38C5ae3ee324ee027D88c5117ee58d07c9b4699b");
        map.insert("HEGIC / ETH", "0xAf5E8D9Cd9fC85725A83BF23C52f1C39A71588a6");
        map.insert("HEGIC / USD", "0xBFC189aC214E6A4a35EBC281ad15669619b75534");
        map.insert("HOOD-USD", "0x3528B448a62189eb6Bf5633851b2f33147642a2a");
        map.insert("HT / USD", "0xE1329B3f6513912CAf589659777b66011AEE5880");
        map.insert("HUSD / ETH", "0x1B61BAD1495161bCb6C03DDB0E41622c0270bB1A");
        map.insert("IDR / USD", "0x91b99C9b75aF469a71eE1AB528e8da994A5D7030");
        map.insert("ILV / ETH", "0xf600984CCa37cd562E74E3EE514289e3613ce8E4");
        map.insert("IMX / USD", "0xBAEbEFc1D023c0feCcc047Bff42E75F15Ff213E6");
        map.insert("INJ / USD", "0xaE2EbE3c4D20cE13cE47cbb49b6d7ee631Cd816e");
        map.insert("INR / USD", "0x605D5c2fBCeDb217D7987FC0951B5753069bC360");
        map.insert("IOST / USD", "0xd0935838935349401c73a06FCde9d63f719e84E5");
        map.insert("IOTX / USD", "0x96c45535d235148Dc3ABA1E48A6E3cFB3510f4E2");
        map.insert("IWM / USD", "0xd6Cc0819228622CcbDb5852EDbc060367E91C7a1");
        map.insert("JPY / USD", "0xBcE206caE7f0ec07b545EddE332A47C2F75bbeb3");
        map.insert("KNC / ETH", "0x656c0544eF4C98A6a98491833A89204Abb045d6b");
        map.insert("KNC / USD", "0xf8fF43E991A81e6eC886a3D281A2C6cC19aE70Fc");
        map.insert("KP3R / ETH", "0xe7015CCb7E5F788B8c1010FC22343473EaaC3741");
        map.insert("KRW / USD", "0x01435677FB11763550905594A16B645847C1d0F3");
        map.insert("KSM / USD", "0x06E4164E24E72B879D93360D1B9fA05838A62EB5");
        map.insert("LDO / ETH", "0x4e844125952D32AcdF339BE976c98E22F6F318dB");
        map.insert("LINK / ETH", "0xDC530D9457755926550b59e8ECcdaE7624181557");
        map.insert("LINK / USD", "0x2c1d072e956AFFC0D435Cb7AC38EF18d24d9127c");
        map.insert("LON / ETH", "0x13A8F2cC27ccC2761ca1b21d2F3E762445f201CE");
        map.insert("LRC / ETH", "0x160AC928A16C93eD4895C2De6f81ECcE9a7eB7b4");
        map.insert("LRC / USD", "0xFd33ec6ABAa1Bdc3D9C6C85f1D6299e5a1a5511F");
        map.insert("LTC / USD", "0x6AF09DF7563C363B5763b9102712EbeD3b9e859B");
        map.insert("LUNA / ETH", "0x91E9331556ED76C9393055719986409e11b56f73");
        map.insert("LUSD / USD", "0x3D7aE7E594f2f2091Ad8798313450130d0Aba3a0");
        map.insert("MANA / ETH", "0x82A44D92D6c329826dc557c5E1Be6ebeC5D5FeB9");
        map.insert("MANA / USD", "0x56a4857acbcfe3a66965c251628B1c9f1c408C19");
        map.insert("MASK / USD", "0xE66acA0CBAb601ca933acCe6BA3Eb8D9c0A13bd7");
        map.insert("MATIC / USD", "0x7bAC85A8a13A4BcD8abb3eB7d6b4d632c5a57676");
        map.insert("MIM / USD", "0x7A364e8770418566e3eb2001A96116E6138Eb32F");
        map.insert("MIR / USD", "0x97e4f2Bc7231f2AFA05c51F524A80E1c8bF944e5");
        map.insert("MKR / ETH", "0x24551a8Fb2A7211A25a17B1481f043A8a8adC7f2");
        map.insert("MKR / USD", "0xec1D1B3b0443256cc3860e24a46F108e699484Aa");
        map.insert("MLN / ETH", "0xDaeA8386611A157B08829ED4997A8A62B557014C");
        map.insert("MSFT / USD", "0x021Fb44bfeafA0999C7b07C4791cf4B859C3b431");
        map.insert("NEAR / USD", "0xC12A6d1D827e23318266Ef16Ba6F397F2F91dA9b");
        map.insert("NFLX / USD", "0x67C2e69c5272B94AF3C90683a9947C39Dc605ddE");
        map.insert("NGN / USD", "0x3e59bc23ea3f39e69b5e662B6fC5e7e6D22B6914");
        map.insert("NMR / ETH", "0x9cB2A01A7E64992d32A34db7cEea4c919C391f6A");
        map.insert("NMR / USD", "0xcC445B35b3636bC7cC7051f4769D8982ED0d449A");
        map.insert("NU / ETH", "0xFd93C391f3a81565DaE1f6A66115C26f36A92d6D");
        map.insert("NZD / USD", "0x3977CFc9e4f29C184D4675f4EB8e0013236e5f3e");
        map.insert("OCEAN / ETH", "0x9b0FC4bb9981e5333689d69BdBF66351B9861E62");
        map.insert("OCEAN / USD", "0x7ece4e4E206eD913D991a074A19C192142726797");
        map.insert("OGN / ETH", "0x2c881B6f3f6B5ff6C975813F87A4dad0b241C15b");
        map.insert("OHMv1 / ETH", "0x90c2098473852E2F07678Fe1B6d595b1bd9b16Ed");
        map.insert("OHMv2 / ETH", "0x9a72298ae3886221820B1c878d12D872087D3a23");
        map.insert("OKB / USD", "0x22134617Ae0f6CA8D89451e5Ae091c94f7D743DC");
        map.insert("OM / USD", "0xb9583cfBdEeacd2705546F392E43F8E03eB92216");
        map.insert("OMG / ETH", "0x57C9aB3e56EE4a83752c181f241120a3DBba06a1");
        map.insert("OMG / USD", "0x7D476f061F8212A8C9317D5784e72B4212436E93");
        map.insert("ONT / USD", "0xcDa3708C5c2907FCca52BB3f9d3e4c2028b89319");
        map.insert("ORN / ETH", "0xbA9B2a360eb8aBdb677d6d7f27E12De11AA052ef");
        map.insert("OXT / USD", "0xd75AAaE4AF0c398ca13e2667Be57AF2ccA8B5de6");
        map.insert("Orchid", "0xa175FA75795c6Fb2aFA48B72d22054ee0DeDa4aC");
        map.insert("PAX / ETH", "0x3a08ebBaB125224b7b6474384Ee39fBb247D2200");
        map.insert(
            "PAX / RESERVES",
            "0xf482Ed35406933F321f293aC0e4c6c8f59a22fA5",
        );
        map.insert("PAXG / ETH", "0x9B97304EA12EFed0FAd976FBeCAad46016bf269e");
        map.insert(
            "PAXG / RESERVES",
            "0x716BB8c60D409e54b8Fb5C4f6aBC50E794DA048a",
        );
        map.insert("PERP / ETH", "0x3b41D5571468904D4e53b6a8d93A6BaC43f02dC9");
        map.insert("PERP / USD", "0x01cE1210Fe8153500F60f7131d63239373D7E26C");
        map.insert("PHA / USD", "0x2B1248028fe48864c4f1c305E524e2e6702eAFDF");
        map.insert("PHP / USD", "0x9481e7ad8BE6BbB22A8B9F7B9fB7588d1df65DF6");
        map.insert("PLA / USD", "0xbc535B134DdF81fc83254a3D0Ed2C0C60144405E");
        map.insert("PUNDIX / USD", "0x552dDBEf6f5a1316aec3E30Db6afCD433548dbF3");
        map.insert("QQQ / USD", "0x6b54e83f44047d2168a195ABA5e9b768762167b5");
        map.insert("RAI / ETH", "0x4ad7B025127e89263242aB68F0f9c4E5C033B489");
        map.insert("RAI / USD", "0x483d36F6a1d063d580c7a24F9A42B346f3a69fbb");
        map.insert("RAMP / USD", "0x4EA6Ec4C1691C62623122B213572b2be5A618C0d");
        map.insert("RARI / ETH", "0x2a784368b1D492f458Bf919389F42c18315765F5");
        map.insert("REN / ETH", "0x3147D7203354Dc06D9fd350c7a2437bcA92387a4");
        map.insert("REN / USD", "0x0f59666EDE214281e956cb3b2D0d69415AfF4A01");
        map.insert("REP / ETH", "0xD4CE430C3b67b3E2F7026D86E7128588629e2455");
        map.insert("REP / USD", "0xF9FCC6E1186Acf6529B1c1949453f51B4B6eEE67");
        map.insert("REQ / USD", "0x2F05888D185970f178f40610306a0Cc305e52bBF");
        map.insert("RLC / ETH", "0x4cba1e1fdc738D0fe8DB3ee07728E2Bc4DA676c6");
        map.insert("RSR / USD", "0x759bBC1be8F90eE6457C44abc7d443842a976d02");
        map.insert("RUB / USD", "0x73A11E47325e3C9b6a48B8ed48Ee0ba89109FB75");
        map.insert("RUNE / ETH", "0x875D60C44cfbC38BaA4Eb2dDB76A767dEB91b97e");
        map.insert("RUNE / USD", "0x48731cF7e84dc94C5f84577882c14Be11a5B7456");
        map.insert("SAND / USD", "0x35E3f7E558C04cE7eEE1629258EcbbA03B36Ec56");
        map.insert("SGD / USD", "0xe25277fF4bbF9081C75Ab0EB13B4A13a721f3E13");
        map.insert("SHIB / ETH", "0x8dD1CD88F43aF196ae478e91b9F5E4Ac69A97C61");
        map.insert("SLP / ETH", "0x5EfE7b8304FA77Bb72031203189C0Be0F5596801");
        map.insert("SNX / ETH", "0x79291A9d692Df95334B1a0B3B4AE6bC606782f8c");
        map.insert("SNX / USD", "0xDC3EA94CD0AC27d9A86C180091e7f78C683d3699");
        map.insert("SOL / USD", "0x4ffC43a60e009B551865A93d232E33Fce9f01507");
        map.insert("SPELL / USD", "0x8c110B94C5f1d347fAcF5E1E938AB2db60E3c9a8");
        map.insert("SPY / USD", "0x065B8808087C2d7A3C104E276C80Fe6Fc1B47f1c");
        map.insert("SRM / ETH", "0x050c048c9a0CD0e76f166E2539F87ef2acCEC58f");
        map.insert("STAKE / ETH", "0xa1FFC11Eaa62d34C3B3272270AEcF9D879773B32");
        map.insert("STMX / USD", "0x00a773bD2cE922F866BB43ab876009fb959d7C29");
        map.insert("SUSD / ETH", "0x8e0b7e6062272B5eF4524250bFFF8e5Bd3497757");
        map.insert("SUSHI / ETH", "0xe572CeF69f43c2E488b33924AF04BDacE19079cf");
        map.insert("SUSHI / USD", "0xCc70F09A6CC17553b2E31954cD36E4A2d89501f7");
        map.insert("SXP / USD", "0xFb0CfD6c19e25DB4a08D8a204a387cEa48Cc138f");
        map.insert(
            "Synthetix Aggregator Debt Ratio",
            "0x0981af0C002345c9C5AD5efd26242D0cBe5aCA99",
        );
        map.insert(
            "Synthetix Aggregator Issued Synths",
            "0xbCF5792575bA3A875D8C406F4E7270f51a902539",
        );
        map.insert("TOKE / USD", "0x104cD02b2f22972E8d8542867a36bDeDA4f104d8");
        map.insert("TOMO / USD", "0x3d44925a8E9F9DFd90390E58e92Ec16c996A331b");
        map.insert("TRIBE / ETH", "0x84a24deCA415Acc0c395872a9e6a63E27D6225c8");
        map.insert("TRU / USD", "0x26929b85fE284EeAB939831002e1928183a10fb1");
        map.insert("TRX / USD", "0xacD0D1A29759CC01E8D925371B72cb2b5610EA25");
        map.insert("TRY / USD", "0xB09fC5fD3f11Cf9eb5E1C5Dba43114e3C9f477b5");
        map.insert("TSLA / USD", "0x1ceDaaB50936881B3e449e47e40A2cDAF5576A4a");
        map.insert("TUSD / ETH", "0x3886BA987236181D98F2401c507Fb8BeA7871dF2");
        map.insert("TUSD / USD", "0xec746eCF986E2927Abd291a2A1716c940100f8Ba");
        map.insert(
            "TUSD Reserves",
            "0x478f4c42b877c697C4b19E396865D4D533EcB6ea",
        );
        map.insert("TUSD Supply", "0x807b029DD462D5d9B9DB45dff90D3414013B969e");
        map.insert(
            "Total Marketcap / USD",
            "0xEC8761a0A73c34329CA5B1D3Dc7eD07F30e836e2",
        );
        map.insert("UFT / USD", "0x2788330dC1eE04cffAb7804A151ef4807880E143");
        map.insert("UMA / ETH", "0xf817B69EA583CAFF291E287CaE00Ea329d22765C");
        map.insert("UNI / ETH", "0xD6aA3D25116d8dA79Ea0246c4826EB951872e02e");
        map.insert("UNI / USD", "0x553303d460EE0afB37EdFf9bE42922D8FF63220e");
        map.insert("USDC / ETH", "0x986b5E1e1755e3C2440e960477f25201B0a8bbD4");
        map.insert("USDC / USD", "0x8fFfFfd4AfB6115b954Bd326cbe7B4BA576818f6");
        map.insert("USDK / USD", "0xfAC81Ea9Dd29D8E9b212acd6edBEb6dE38Cb43Af");
        map.insert("USDN / USD", "0x7a8544894F7FD0C69cFcBE2b4b2E277B0b9a4355");
        map.insert("USDP / USD", "0x09023c0DA49Aaf8fc3fA3ADF34C6A7016D38D5e3");
        map.insert("USDT / ETH", "0xEe9F2375b4bdF6387aa8265dD4FB8F16512A1d46");
        map.insert("USDT / USD", "0x3E7d1eAB13ad0104d2750B8863b489D65364e32D");
        map.insert("UST / ETH", "0xa20623070413d42a5C01Db2c8111640DD7A5A03a");
        map.insert("UST / USD", "0x8b6d9085f310396C6E4f0012783E9f850eaa8a82");
        map.insert("VGX / ETH", "0x5dd1c4a1aCfC3f83446cFe79cB0660Bb07f76513");
        map.insert("VXX / USD", "0xC18F2a0C166A091fcD5E2051EFEFD63c4f4A27E9");
        map.insert(
            "Vesper Finance TVL",
            "0x13e9cF2Cc0577b0D831878055dA0629F98D194c2",
        );
        map.insert("WAVES / USD", "0x9a79fdCd0E326dF6Fa34EA13c05d3106610798E9");
        map.insert("WBTC PoR", "0xa81FE04086865e63E12dD3776978E49DEEa2ea4e");
        map.insert("WING / USD", "0x134fE0a225Fb8e6683617C13cEB6B3319fB4fb82");
        map.insert("WNXM / ETH", "0xe5Dc0A609Ab8bCF15d3f35cFaa1Ff40f521173Ea");
        map.insert("WOO / ETH", "0x926a93B44a887076eDd00257E5D42fafea313363");
        map.insert("WTI / USD", "0xf3584F4dd3b467e73C2339EfD008665a70A4185c");
        map.insert("XAG / USD", "0x379589227b15F1a12195D3f2d90bBc9F31f95235");
        map.insert("XAU / USD", "0x214eD9Da11D2fbe465a6fc601a91E62EbEc1a0D6");
        map.insert("XLM / USD", "0x64168007BAcbB5fF3f52639db22C6300827f5036");
        map.insert("XMR / USD", "0xFA66458Cce7Dd15D8650015c4fce4D278271618F");
        map.insert("XRP / USD", "0xCed2660c6Dd1Ffd856A5A82C67f3482d88C50b12");
        map.insert("XSUSHI / ETH", "0x7f59A29507282703B4A796D02cAcf23388FfF00D");
        map.insert("XTZ / USD", "0x5239a625dEb44bF3EeAc2CD5366ba24b8e9DB63F");
        map.insert("XVS / USD", "0x558E45a0cb2F376F771b6Dcb3caC5C3f42dd74f9");
        map.insert("YFI / ETH", "0x7c5d4F8345e66f68099581Db340cd65B078C41f4");
        map.insert("YFI / USD", "0xA027702dbb89fbd58938e4324ac03B58d812b0E1");
        map.insert("YFII / ETH", "0xaaB2f6b45B28E962B3aCd1ee4fC88aEdDf557756");
        map.insert("YGG / ETH", "0xeb9De2f84F318e8c3081ccb485A6399A82344A00");
        map.insert("ZAR / USD", "0x438F81D95761d7036cd2617295827D9d01Cf593f");
        map.insert("ZEC / USD", "0xd54B033D48d0475f19c5fccf7484E8A981848501");
        map.insert("ZRX / ETH", "0x2Da4983a622a8498bb1a21FaE9D8F6C664939962");
        map.insert("ZRX / USD", "0x2885d15b8Af22648b98B122b22FDF4D2a56c6023");
        map.insert("aUST / UST", "0x73bB8A4220E5C7Db3E73e4Fcb8d7DCf2efe04805");
        map.insert("eFIL PoR", "0x8917800a6BDd8fA8b7c94E25aE2219Db28050622");
        map.insert("sCEX / USD", "0x283D433435cFCAbf00263beEF6A362b7cc5ed9f2");
        map.insert("sDEFI / USD", "0xa8E875F94138B0C5b51d1e1d5dE35bbDdd28EA87");
        map.insert("sUSD / USD", "0xad35Bd71b9aFE6e4bDc266B345c198eaDEf9Ad94");
        map
    };
}

pub struct EthereumMainNet;

impl GetAddress for EthereumMainNet {
    fn get_address(&self, feed_name: &str) -> Option<String> {
        Some(ETHEREUM_MAIN_NET.get(feed_name)?.to_string())
    }
}
