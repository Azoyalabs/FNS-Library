use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::dependencies::DomainData;


use cosmwasm_std::Coin;

use crate::typing::PriceScheme;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AdminExecuteMsg {
    WithdrawFunds { beneficiary: String, amount: Coin },
    AddExtension { extension: String },

    UpdatePriceScheme { new_price_scheme: PriceScheme },
}



#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Admin(AdminExecuteMsg),

    Register {
        domain: String,
    },
    UpdateRecord {
        domain: String,
        data: DomainData,
    },

    // hooks
    OnNftOwnerChanged {
        domain: String,
        previous_owner: String,
        new_owner: String,
    },

    OnNftBurned {
        domain: String,
        previous_owner: String,
        caller: String,
    },
}