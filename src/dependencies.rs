use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NftAvatarData {
    pub contract: String,
    pub token_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DomainData {
    pub description: Option<String>,
    pub address: Option<Addr>,
    pub avatar: Option<NftAvatarData>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
}

