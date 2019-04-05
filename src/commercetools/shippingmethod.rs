//Generated file, please do not change

use super::common::Money;
use super::taxcategory::TaxCategoryReference;
use super::common::Resource;
use super::common::Reference;
use super::common::TypedMoney;
use super::zone::ZoneReference;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartClassificationTier {
   pub price: Money,
   pub value: String,
   pub is_matching: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartScoreTier {
   pub score: u32,
   pub price: Option<Money>,
   pub price_function: Option<PriceFunction>,
   pub is_matching: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartValueTier {
   pub price: Money,
   pub minimum_cent_amount: u64,
   pub is_matching: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceFunction {
   /**
   	<p>The currency code compliant to <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>.</p>
   */
   pub currency_code: String,
   pub function: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethod {
   pub zone_rates: Vec<ZoneRate>,
   pub tax_category: TaxCategoryReference,
   pub is_default: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub name: String,
   pub description: Option<String>,
   pub key: Option<String>,
   pub predicate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodDraft {
   pub zone_rates: Vec<ZoneRateDraft>,
   pub tax_category: TaxCategoryReference,
   pub is_default: bool,
   pub name: String,
   pub description: Option<String>,
   pub key: Option<String>,
   pub predicate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodPagedQueryResponse {
   pub results: Vec<ShippingMethod>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodReference {
   pub obj: Option<ShippingMethod>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodUpdate {
   pub actions: Vec<ShippingMethodUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ShippingMethodUpdateAction {
   #[serde(rename = "addShippingRate")]
   EShippingMethodAddShippingRateAction(ShippingMethodAddShippingRateAction),
   #[serde(rename = "addZone")]
   EShippingMethodAddZoneAction(ShippingMethodAddZoneAction),
   #[serde(rename = "changeIsDefault")]
   EShippingMethodChangeIsDefaultAction(ShippingMethodChangeIsDefaultAction),
   #[serde(rename = "changeName")]
   EShippingMethodChangeNameAction(ShippingMethodChangeNameAction),
   #[serde(rename = "changeTaxCategory")]
   EShippingMethodChangeTaxCategoryAction(ShippingMethodChangeTaxCategoryAction),
   #[serde(rename = "removeShippingRate")]
   EShippingMethodRemoveShippingRateAction(ShippingMethodRemoveShippingRateAction),
   #[serde(rename = "removeZone")]
   EShippingMethodRemoveZoneAction(ShippingMethodRemoveZoneAction),
   #[serde(rename = "setDescription")]
   EShippingMethodSetDescriptionAction(ShippingMethodSetDescriptionAction),
   #[serde(rename = "setKey")]
   EShippingMethodSetKeyAction(ShippingMethodSetKeyAction),
   #[serde(rename = "setPredicate")]
   EShippingMethodSetPredicateAction(ShippingMethodSetPredicateAction),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingRate {
   pub tiers: Vec<ShippingRatePriceTier>,
   pub price: TypedMoney,
   pub free_above: Option<TypedMoney>,
   pub is_matching: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingRateDraft {
   pub price: Money,
   pub tiers: Option<Vec<ShippingRatePriceTier>>,
   pub free_above: Option<Money>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ShippingRatePriceTier {
   #[serde(rename = "CartValue")]
   ECartValueTier(CartValueTier),
   #[serde(rename = "CartClassification")]
   ECartClassificationTier(CartClassificationTier),
   #[serde(rename = "CartScore")]
   ECartScoreTier(CartScoreTier),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShippingRateTierType {
   CartValue,
   CartClassification,
   CartScore
}

impl ShippingRateTierType {
    pub fn from_str(s: &str) -> Option<ShippingRateTierType> {
        match s {
            "CartValue" => Some(ShippingRateTierType::CartValue),
            "CartClassification" => Some(ShippingRateTierType::CartClassification),
            "CartScore" => Some(ShippingRateTierType::CartScore),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ShippingRateTierType::CartValue => "CartValue",
           ShippingRateTierType::CartClassification => "CartClassification",
           ShippingRateTierType::CartScore => "CartScore",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZoneRate {
   pub shipping_rates: Vec<ShippingRate>,
   pub zone: ZoneReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZoneRateDraft {
   pub shipping_rates: Vec<ShippingRateDraft>,
   pub zone: ZoneReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodAddShippingRateAction {
   pub shipping_rate: ShippingRateDraft,
   pub zone: ZoneReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodAddZoneAction {
   pub zone: ZoneReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodChangeIsDefaultAction {
   pub is_default: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodChangeNameAction {
   pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodChangeTaxCategoryAction {
   pub tax_category: TaxCategoryReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodRemoveShippingRateAction {
   pub shipping_rate: ShippingRateDraft,
   pub zone: ZoneReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodRemoveZoneAction {
   pub zone: ZoneReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodSetDescriptionAction {
   pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodSetKeyAction {
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodSetPredicateAction {
   pub predicate: Option<String>,
}