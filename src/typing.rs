use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Coin;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NftAvatarRecord {
    pub contract: String,
    pub token_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceRange {
    pub min: u32,
    pub max: u32,
    pub price: Coin,
}

impl PriceRange {
    pub fn contains(&self, val: u32) -> bool {
        return val >= self.min && val <= self.max;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceScheme {
    pub price_ranges: Vec<PriceRange>,
}

impl PriceScheme {
    pub fn get_price(&self, len_label: usize) -> Option<Coin> {
        for price_range in &self.price_ranges {
            if price_range.contains(len_label as u32) {
                return Some(price_range.price.to_owned());
            }
        }

        return None;
    }

    pub fn get_min_size(&self) -> u32 {
        return self.price_ranges.iter().map(|elem| elem.min).min().unwrap();
    }

    pub fn get_max_size(&self) -> u32 {
        return self.price_ranges.iter().map(|elem| elem.max).max().unwrap();
    }

    pub fn get_min_max_size(&self) -> (u32, u32) {
        return (
            self.price_ranges.iter().map(|elem| elem.min).min().unwrap(),
            self.price_ranges.iter().map(|elem| elem.max).max().unwrap(),
        );
    }
}

// errors for validation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ValidationError {
    DomainTooShort,
    DomainTooLong,
    NoSubdomainsAllowed,
    NormalizationFailed,
    ExtensionDoesNotExist,
    InvalidInput,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ValidationSuccess {
    pub normalized_domain: String,
    pub pricing: Coin,
    pub can_register: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ValidationResult {
    Error(ValidationError),
    Success(ValidationSuccess),
}