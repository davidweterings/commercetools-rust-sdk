//Generated file, please do not change

use super::channel::ChannelReference;
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
pub struct InventoryEntry {
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub available_quantity: u64,
   pub quantity_on_stock: u64,
   pub version: u64,
   pub id: String,
   pub sku: String,
   pub supply_channel: Option<ChannelReference>,
   pub custom: Option<CustomFields>,
   pub expected_delivery: Option<DateTime<Utc>>,
   pub restockable_in_days: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryEntryDraft {
   pub quantity_on_stock: u64,
   pub sku: String,
   pub supply_channel: Option<ChannelReference>,
   pub custom: Option<CustomFieldsDraft>,
   pub expected_delivery: Option<DateTime<Utc>>,
   pub restockable_in_days: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryEntryReference {
   pub obj: Option<InventoryEntry>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryPagedQueryResponse {
   pub results: Vec<InventoryEntry>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryUpdate {
   pub actions: Vec<InventoryUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum InventoryUpdateAction {
   #[serde(rename = "addQuantity")]
   EInventoryAddQuantityAction(InventoryAddQuantityAction),
   #[serde(rename = "changeQuantity")]
   EInventoryChangeQuantityAction(InventoryChangeQuantityAction),
   #[serde(rename = "removeQuantity")]
   EInventoryRemoveQuantityAction(InventoryRemoveQuantityAction),
   #[serde(rename = "setCustomField")]
   EInventorySetCustomFieldAction(InventorySetCustomFieldAction),
   #[serde(rename = "setCustomType")]
   EInventorySetCustomTypeAction(InventorySetCustomTypeAction),
   #[serde(rename = "setExpectedDelivery")]
   EInventorySetExpectedDeliveryAction(InventorySetExpectedDeliveryAction),
   #[serde(rename = "setRestockableInDays")]
   EInventorySetRestockableInDaysAction(InventorySetRestockableInDaysAction),
   #[serde(rename = "setSupplyChannel")]
   EInventorySetSupplyChannelAction(InventorySetSupplyChannelAction),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryAddQuantityAction {
   pub quantity: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryChangeQuantityAction {
   pub quantity: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryRemoveQuantityAction {
   pub quantity: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventorySetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventorySetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventorySetExpectedDeliveryAction {
   pub expected_delivery: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventorySetRestockableInDaysAction {
   pub restockable_in_days: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventorySetSupplyChannelAction {
   pub supply_channel: Option<ChannelReference>,
}