//Generated file, please do not change


use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiClient {
   pub id: String,
   pub name: String,
   pub scope: String,
   pub last_used_at: Option<NaiveDate>,
   pub created_at: Option<DateTime<Utc>>,
   pub delete_at: Option<DateTime<Utc>>,
   pub secret: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiClientDraft {
   pub name: String,
   pub scope: String,
   pub delete_days_after_creation: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiClientPagedQueryResponse {
   pub results: Vec<ApiClient>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}