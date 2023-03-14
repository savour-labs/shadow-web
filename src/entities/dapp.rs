use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Type, Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "dapptype", rename_all = "lowercase")]
pub enum DappType {
    Counter,
    DailyDapp,
}

impl Default for DappType {
    fn default() -> Self {
        Self::Counter
    }
}

#[derive(Object, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dapp {
    #[oai(read_only)]
    pub title: String,
    pub description: String,
}
