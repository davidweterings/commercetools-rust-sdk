//Generated file, please do not change

use super::cttype::CustomFields;
use super::customer::CustomerReference;
use super::common::LocalizedString;
use super::common::Resource;
use super::cttype::CustomFieldsDraft;
use super::producttype::ProductTypeReference;
use super::product::ProductVariant;
use super::common::Reference;
use super::cttype::TypeReference;
use super::cttype::FieldContainer;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingList {
   pub name: LocalizedString,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub line_items: Option<Vec<ShoppingListLineItem>>,
   pub text_line_items: Option<Vec<TextLineItem>>,
   pub custom: Option<CustomFields>,
   pub customer: Option<CustomerReference>,
   pub description: Option<LocalizedString>,
   pub slug: Option<LocalizedString>,
   pub delete_days_after_last_modification: Option<u64>,
   pub anonymous_id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListDraft {
   pub name: LocalizedString,
   pub line_items: Option<Vec<ShoppingListLineItemDraft>>,
   pub text_line_items: Option<Vec<TextLineItemDraft>>,
   pub custom: Option<CustomFieldsDraft>,
   pub customer: Option<CustomerReference>,
   pub description: Option<LocalizedString>,
   pub slug: Option<LocalizedString>,
   pub delete_days_after_last_modification: Option<u64>,
   pub anonymous_id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListLineItem {
   pub name: LocalizedString,
   pub product_type: ProductTypeReference,
   pub added_at: DateTime<Utc>,
   pub quantity: u32,
   pub id: String,
   pub product_id: String,
   pub custom: Option<CustomFields>,
   pub product_slug: Option<LocalizedString>,
   pub variant: Option<ProductVariant>,
   pub deactivated_at: Option<DateTime<Utc>>,
   pub variant_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListLineItemDraft {
   pub custom: Option<CustomFieldsDraft>,
   pub added_at: Option<DateTime<Utc>>,
   pub quantity: Option<u32>,
   pub variant_id: Option<u64>,
   pub product_id: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListPagedQueryResponse {
   pub results: Vec<ShoppingList>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListReference {
   pub obj: Option<ShoppingList>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListUpdate {
   pub actions: Vec<ShoppingListUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextLineItem {
   pub name: LocalizedString,
   pub added_at: DateTime<Utc>,
   pub quantity: u32,
   pub id: String,
   pub custom: Option<CustomFields>,
   pub description: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextLineItemDraft {
   pub name: LocalizedString,
   pub custom: Option<CustomFieldsDraft>,
   pub description: Option<LocalizedString>,
   pub added_at: Option<DateTime<Utc>>,
   pub quantity: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListAddLineItemAction {
   pub custom: Option<CustomFieldsDraft>,
   pub added_at: Option<DateTime<Utc>>,
   pub quantity: Option<u64>,
   pub variant_id: Option<u64>,
   pub product_id: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListAddTextLineItemAction {
   pub name: LocalizedString,
   pub custom: Option<CustomFieldsDraft>,
   pub description: Option<LocalizedString>,
   pub added_at: Option<DateTime<Utc>>,
   pub quantity: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListChangeLineItemQuantityAction {
   pub quantity: u64,
   pub line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListChangeLineItemsOrderAction {
   pub line_item_order: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListChangeNameAction {
   pub name: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListChangeTextLineItemNameAction {
   pub name: LocalizedString,
   pub text_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListChangeTextLineItemQuantityAction {
   pub quantity: u64,
   pub text_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListChangeTextLineItemsOrderAction {
   pub text_line_item_order: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListRemoveLineItemAction {
   pub line_item_id: String,
   pub quantity: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListRemoveTextLineItemAction {
   pub text_line_item_id: String,
   pub quantity: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetAnonymousIdAction {
   pub anonymous_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetCustomerAction {
   pub customer: Option<CustomerReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetDeleteDaysAfterLastModificationAction {
   pub delete_days_after_last_modification: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetDescriptionAction {
   pub description: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetKeyAction {
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetLineItemCustomFieldAction {
   pub line_item_id: String,
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetLineItemCustomTypeAction {
   pub line_item_id: String,
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetSlugAction {
   pub slug: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetTextLineItemCustomFieldAction {
   pub name: String,
   pub text_line_item_id: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetTextLineItemCustomTypeAction {
   pub text_line_item_id: String,
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListSetTextLineItemDescriptionAction {
   pub text_line_item_id: String,
   pub description: Option<LocalizedString>,
}