//Generated file, please do not change

use super::common::LocalizedString;
use super::cttype::CustomFields;
use super::common::Asset;
use super::common::Resource;
use super::cttype::CustomFieldsDraft;
use super::common::AssetDraft;
use super::common::Reference;
use super::cttype::TypeReference;
use super::common::AssetSource;
use super::cttype::FieldContainer;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Category {
   pub ancestors: Vec<CategoryReference>,
   pub name: LocalizedString,
   pub slug: LocalizedString,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub order_hint: String,
   pub assets: Option<Vec<Asset>>,
   pub parent: Option<CategoryReference>,
   pub custom: Option<CustomFields>,
   pub description: Option<LocalizedString>,
   pub meta_description: Option<LocalizedString>,
   pub meta_keywords: Option<LocalizedString>,
   pub meta_title: Option<LocalizedString>,
   pub external_id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDraft {
   pub name: LocalizedString,
   pub slug: LocalizedString,
   pub assets: Option<Vec<AssetDraft>>,
   pub parent: Option<CategoryReference>,
   pub custom: Option<CustomFieldsDraft>,
   pub description: Option<LocalizedString>,
   pub meta_description: Option<LocalizedString>,
   pub meta_keywords: Option<LocalizedString>,
   pub meta_title: Option<LocalizedString>,
   pub external_id: Option<String>,
   pub key: Option<String>,
   pub order_hint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryPagedQueryResponse {
   pub results: Vec<Category>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryReference {
   pub obj: Option<Box<Category>>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryUpdate {
   pub actions: Vec<CategoryUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryAddAssetAction {
   pub asset: AssetDraft,
   pub position: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryChangeAssetNameAction {
   pub name: LocalizedString,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryChangeAssetOrderAction {
   pub asset_order: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryChangeNameAction {
   pub name: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryChangeOrderHintAction {
   pub order_hint: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryChangeParentAction {
   pub parent: CategoryReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryChangeSlugAction {
   pub slug: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryRemoveAssetAction {
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetAssetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetAssetCustomTypeAction {
   pub r#type: Option<TypeReference>,
   pub fields: Option<serde_json::Value>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetAssetDescriptionAction {
   pub description: Option<LocalizedString>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetAssetKeyAction {
   pub asset_id: String,
   pub asset_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetAssetSourcesAction {
   pub sources: Vec<AssetSource>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetAssetTagsAction {
   pub tags: Option<Vec<String>>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetDescriptionAction {
   pub description: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetExternalIdAction {
   pub external_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetKeyAction {
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetMetaDescriptionAction {
   pub meta_description: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetMetaKeywordsAction {
   pub meta_keywords: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySetMetaTitleAction {
   pub meta_title: Option<LocalizedString>,
}