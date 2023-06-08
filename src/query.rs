use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use crate::typing::{PriceScheme, ValidationResult};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // get all domains owned by a given address
    //AddressAllDomains { address: String },

    // look for addr associated to domain in domain data
    ResolveName { domain: String },

    // look on domains to find those with
    ReverseLookUp { target: String },

    // get the owner of a domain
    //ReverseRecord { domain: String },

    // get domain status for a domain,
    GetDomainStatus { domain: String },
    ExtensionRegistrarAddress { extension: String },

    GetDomainData { domain: String },
    GetPriceScheme {},

    GetNormalizedDomain { domain: String },
    GetNormalizedDomainAndPrice { domain: String },

    GetAllDomainsOwnedBy { owner: String },

    GetAllExtensions {},

    // get associated contracts
    GetStorageContract {},
    GetNormalizerContract {},
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AddressAllDomainResponse {
    pub domains: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ReverseRecordResponse {
    pub owner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CanRegisterResponse {
    pub can_register: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ExtensionRegistrarAddressResponse {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct GetPriceSchemeResponse {
    pub price_scheme: PriceScheme,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct GetAllExtensionsResponse {
    pub extensions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ResolveNameResponse {
    pub address: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ReverseLookUpResponse {
    pub address: Addr,
    pub domains: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct GetNormalizedDomainAndPriceResponseOld {
    pub is_valid_domain: bool,
    pub normalized_domain: String,
    pub price: Option<Coin>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct GetNormalizedDomainAndPriceResponse {
    pub is_valid_domain: bool,
    pub result: ValidationResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct GetStorageContractResponse {
    pub address: Addr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct GetNormalizerContractResponse {
    pub address: Addr,
}