//Generated file, please do not change

use super::common::LocalizedString;
use super::common::Reference;
use super::common::Resource;
use super::common::Price;
use super::common::Money;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscount {
   pub references: Vec<Reference>,
   pub name: LocalizedString,
   pub value: ProductDiscountValue,
   pub is_active: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub predicate: String,
   pub sort_order: String,
   pub description: Option<LocalizedString>,
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountDraft {
   pub name: LocalizedString,
   pub value: ProductDiscountValue,
   pub is_active: bool,
   pub predicate: String,
   pub sort_order: String,
   pub description: Option<LocalizedString>,
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountMatchQuery {
   pub price: Price,
   pub staged: bool,
   pub variant_id: u32,
   pub product_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountPagedQueryResponse {
   pub results: Vec<ProductDiscount>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountReference {
   pub obj: Option<ProductDiscount>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountUpdate {
   pub actions: Vec<ProductDiscountUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ProductDiscountUpdateAction {
   #[serde(rename = "changeIsActive")]
   EProductDiscountChangeIsActiveAction(ProductDiscountChangeIsActiveAction),
   #[serde(rename = "changeName")]
   EProductDiscountChangeNameAction(ProductDiscountChangeNameAction),
   #[serde(rename = "changePredicate")]
   EProductDiscountChangePredicateAction(ProductDiscountChangePredicateAction),
   #[serde(rename = "changeSortOrder")]
   EProductDiscountChangeSortOrderAction(ProductDiscountChangeSortOrderAction),
   #[serde(rename = "changeValue")]
   EProductDiscountChangeValueAction(ProductDiscountChangeValueAction),
   #[serde(rename = "setDescription")]
   EProductDiscountSetDescriptionAction(ProductDiscountSetDescriptionAction),
   #[serde(rename = "setValidFrom")]
   EProductDiscountSetValidFromAction(ProductDiscountSetValidFromAction),
   #[serde(rename = "setValidFromAndUntil")]
   EProductDiscountSetValidFromAndUntilAction(ProductDiscountSetValidFromAndUntilAction),
   #[serde(rename = "setValidUntil")]
   EProductDiscountSetValidUntilAction(ProductDiscountSetValidUntilAction),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ProductDiscountValue {
   #[serde(rename = "absolute")]
   EProductDiscountValueAbsolute(ProductDiscountValueAbsolute),
   #[serde(rename = "external")]
   EProductDiscountValueExternal(ProductDiscountValueExternal),
   #[serde(rename = "relative")]
   EProductDiscountValueRelative(ProductDiscountValueRelative),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountValueAbsolute {
   pub money: Vec<Money>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountValueExternal {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountValueRelative {
   pub permyriad: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountChangeIsActiveAction {
   pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountChangeNameAction {
   pub name: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountChangePredicateAction {
   pub predicate: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountChangeSortOrderAction {
   pub sort_order: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountChangeValueAction {
   pub value: ProductDiscountValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountSetDescriptionAction {
   pub description: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountSetValidFromAction {
   pub valid_from: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountSetValidFromAndUntilAction {
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDiscountSetValidUntilAction {
   pub valid_until: Option<DateTime<Utc>>,
}