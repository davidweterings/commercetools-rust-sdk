//Generated file, please do not change

use super::common::LocalizedString;
use super::cartdiscount::CartDiscountReference;
use super::common::Reference;
use super::cttype::CustomFields;
use super::common::Resource;
use super::cttype::CustomFieldsDraft;
use super::cttype::TypeReference;
use super::cttype::FieldContainer;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCode {
   pub cart_discounts: Vec<CartDiscountReference>,
   pub groups: Vec<String>,
   pub references: Vec<Reference>,
   pub is_active: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub code: String,
   pub id: String,
   pub custom: Option<CustomFields>,
   pub description: Option<LocalizedString>,
   pub name: Option<LocalizedString>,
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
   pub max_applications: Option<u64>,
   pub max_applications_per_customer: Option<u64>,
   pub cart_predicate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeDraft {
   pub cart_discounts: Vec<CartDiscountReference>,
   pub code: String,
   pub custom: Option<CustomFieldsDraft>,
   pub description: Option<LocalizedString>,
   pub name: Option<LocalizedString>,
   pub groups: Option<Vec<String>>,
   pub is_active: Option<bool>,
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
   pub max_applications: Option<u64>,
   pub max_applications_per_customer: Option<u64>,
   pub cart_predicate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodePagedQueryResponse {
   pub results: Vec<DiscountCode>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeReference {
   pub obj: Option<DiscountCode>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeUpdate {
   pub actions: Vec<DiscountCodeUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeChangeCartDiscountsAction {
   pub cart_discounts: Vec<CartDiscountReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeChangeGroupsAction {
   pub groups: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeChangeIsActiveAction {
   pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeSetCartPredicateAction {
   pub cart_predicate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeSetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeSetDescriptionAction {
   pub description: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeSetMaxApplicationsAction {
   pub max_applications: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeSetMaxApplicationsPerCustomerAction {
   pub max_applications_per_customer: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeSetNameAction {
   pub name: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeSetValidFromAction {
   pub valid_from: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeSetValidFromAndUntilAction {
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeSetValidUntilAction {
   pub valid_until: Option<DateTime<Utc>>,
}