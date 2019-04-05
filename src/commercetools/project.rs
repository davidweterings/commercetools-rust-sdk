//Generated file, please do not change

use super::cttype::CustomFieldLocalizedEnumValue;
use super::message::MessageConfiguration;
use super::shippingmethod::ShippingRateTierType;
use super::message::MessageConfigurationDraft;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartClassificationType {
   pub values: Vec<CustomFieldLocalizedEnumValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartScoreType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartValueType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
   pub countries: Vec<String>,
   pub currencies: Vec<String>,
   pub languages: Vec<String>,
   pub messages: MessageConfiguration,
   pub created_at: DateTime<Utc>,
   pub version: u64,
   pub key: String,
   pub name: String,
   pub shipping_rate_input_type: Option<ShippingRateInputType>,
   pub trial_until: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUpdate {
   pub actions: Vec<ProjectUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingRateInputType {
   pub r#type: ShippingRateTierType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectChangeCountriesAction {
   pub countries: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectChangeCurrenciesAction {
   pub currencies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectChangeLanguagesAction {
   pub languages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectChangeMessagesConfigurationAction {
   pub messages_configuration: MessageConfigurationDraft,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectChangeMessagesEnabledAction {
   pub messages_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectChangeNameAction {
   pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSetShippingRateInputTypeAction {
   pub shipping_rate_input_type: Option<ShippingRateInputType>,
}