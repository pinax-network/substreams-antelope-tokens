#[allow(dead_code)]
pub const ACCOUNT: Option<&'static str> = None;
pub mod types {
    use substreams_antelope::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Account {
        pub balance: Asset,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct CurrencyStats {
        pub supply: Asset,
        pub max_supply: Asset,
        pub issuer: Name,
    }
}
pub mod actions {
    use substreams_antelope::types::*;
    use substreams_antelope::decoder::decode;
    #[allow(unused_imports)]
    use super::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Close {
        pub owner: Name,
        pub symbol: Symbol,
    }
    impl substreams_antelope::Action for Close {
        const NAME: &'static str = "close";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Create {
        pub issuer: Name,
        pub maximum_supply: Asset,
    }
    impl substreams_antelope::Action for Create {
        const NAME: &'static str = "create";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Issue {
        pub to: Name,
        pub quantity: Asset,
        pub memo: String,
    }
    impl substreams_antelope::Action for Issue {
        const NAME: &'static str = "issue";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Open {
        pub owner: Name,
        pub symbol: Symbol,
        pub ram_payer: Name,
    }
    impl substreams_antelope::Action for Open {
        const NAME: &'static str = "open";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Retire {
        pub quantity: Asset,
        pub memo: String,
    }
    impl substreams_antelope::Action for Retire {
        const NAME: &'static str = "retire";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Transfer {
        pub from: Name,
        pub to: Name,
        pub quantity: Asset,
        pub memo: String,
    }
    impl substreams_antelope::Action for Transfer {
        const NAME: &'static str = "transfer";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
}