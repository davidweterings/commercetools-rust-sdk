//Generated file, please do not change

use super::common::Resource;
use super::common::Reference;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomObject {
   pub value: serde_json::Value,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub container: String,
   pub id: String,
   pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomObjectDraft {
   pub value: serde_json::Value,
   pub container: String,
   pub key: String,
   pub version: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomObjectPagedQueryResponse {
   pub results: Vec<CustomObject>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomObjectReference {
   pub obj: Option<CustomObject>,
   pub id: Option<String>,
   pub key: Option<String>,
}