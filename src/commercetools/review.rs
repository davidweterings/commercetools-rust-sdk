//Generated file, please do not change

use super::state::StateReference;
use super::customer::CustomerReference;
use super::cttype::CustomFields;
use super::common::Resource;
use super::cttype::CustomFieldsDraft;
use super::common::Reference;
use super::cttype::TypeReference;
use super::cttype::FieldContainer;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Review {
   pub included_in_statistics: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub target: Option<serde_json::Value>,
   pub custom: Option<CustomFields>,
   pub customer: Option<CustomerReference>,
   pub state: Option<StateReference>,
   pub rating: Option<u32>,
   pub author_name: Option<String>,
   pub key: Option<String>,
   pub locale: Option<String>,
   pub text: Option<String>,
   pub title: Option<String>,
   pub uniqueness_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewDraft {
   pub state: Option<serde_json::Value>,
   pub target: Option<serde_json::Value>,
   pub custom: Option<CustomFieldsDraft>,
   pub customer: Option<CustomerReference>,
   pub rating: Option<u32>,
   pub author_name: Option<String>,
   pub key: Option<String>,
   pub locale: Option<String>,
   pub text: Option<String>,
   pub title: Option<String>,
   pub uniqueness_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewPagedQueryResponse {
   pub results: Vec<Review>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewRatingStatistics {
   pub count: u32,
   pub average_rating: u32,
   pub highest_rating: u32,
   pub lowest_rating: u32,
   pub ratings_distribution: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewReference {
   pub obj: Option<Review>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewUpdate {
   pub actions: Vec<ReviewUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSetAuthorNameAction {
   pub author_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSetCustomerAction {
   pub customer: Option<CustomerReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSetKeyAction {
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSetLocaleAction {
   pub locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSetRatingAction {
   pub rating: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSetTargetAction {
   pub target: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSetTextAction {
   pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSetTitleAction {
   pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewTransitionStateAction {
   pub state: StateReference,
   pub force: Option<bool>,
}