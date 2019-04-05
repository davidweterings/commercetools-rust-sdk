//Generated file, please do not change

use super::common::LocalizedString;
use super::common::Address;
use super::review::ReviewRatingStatistics;
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
pub struct Channel {
   pub roles: Vec<ChannelRoleEnum>,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub key: String,
   pub geo_location: Option<serde_json::Value>,
   pub address: Option<Address>,
   pub custom: Option<CustomFields>,
   pub description: Option<LocalizedString>,
   pub name: Option<LocalizedString>,
   pub review_rating_statistics: Option<ReviewRatingStatistics>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelDraft {
   pub key: String,
   pub geo_location: Option<serde_json::Value>,
   pub roles: Option<Vec<ChannelRoleEnum>>,
   pub address: Option<Address>,
   pub custom: Option<CustomFieldsDraft>,
   pub description: Option<LocalizedString>,
   pub name: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelPagedQueryResponse {
   pub results: Vec<Channel>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelReference {
   pub obj: Option<Channel>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChannelRoleEnum {
   InventorySupply,
   ProductDistribution,
   OrderExport,
   OrderImport,
   Primary
}

impl ChannelRoleEnum {
    pub fn from_str(s: &str) -> Option<ChannelRoleEnum> {
        match s {
            "InventorySupply" => Some(ChannelRoleEnum::InventorySupply),
            "ProductDistribution" => Some(ChannelRoleEnum::ProductDistribution),
            "OrderExport" => Some(ChannelRoleEnum::OrderExport),
            "OrderImport" => Some(ChannelRoleEnum::OrderImport),
            "Primary" => Some(ChannelRoleEnum::Primary),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ChannelRoleEnum::InventorySupply => "InventorySupply",
           ChannelRoleEnum::ProductDistribution => "ProductDistribution",
           ChannelRoleEnum::OrderExport => "OrderExport",
           ChannelRoleEnum::OrderImport => "OrderImport",
           ChannelRoleEnum::Primary => "Primary",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelUpdate {
   pub actions: Vec<ChannelUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelAddRolesAction {
   pub roles: Vec<ChannelRoleEnum>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelChangeDescriptionAction {
   pub description: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelChangeKeyAction {
   pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelChangeNameAction {
   pub name: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelRemoveRolesAction {
   pub roles: Vec<ChannelRoleEnum>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSetAddressAction {
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSetGeoLocationAction {
   pub geo_location: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSetRolesAction {
   pub roles: Vec<ChannelRoleEnum>,
}