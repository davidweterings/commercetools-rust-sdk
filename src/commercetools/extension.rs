//Generated file, please do not change

use super::common::Resource;
use super::common::Reference;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
   pub triggers: Vec<ExtensionTrigger>,
   pub destination: ExtensionDestination,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionAWSLambdaDestination {
   pub access_key: String,
   pub access_secret: String,
   pub arn: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExtensionAction {
   Create,
   Update
}

impl ExtensionAction {
    pub fn from_str(s: &str) -> Option<ExtensionAction> {
        match s {
            "Create" => Some(ExtensionAction::Create),
            "Update" => Some(ExtensionAction::Update),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ExtensionAction::Create => "Create",
           ExtensionAction::Update => "Update",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionAuthorizationHeaderAuthentication {
   pub header_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionAzureFunctionsAuthentication {
   pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionDestination {
   pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionDraft {
   pub triggers: Vec<ExtensionTrigger>,
   pub destination: ExtensionDestination,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionHttpDestination {
   pub url: String,
   pub authentication: Option<ExtensionHttpDestinationAuthentication>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionHttpDestinationAuthentication {
   pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionInput {
   pub action: ExtensionAction,
   pub resource: Reference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionPagedQueryResponse {
   pub results: Vec<Extension>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExtensionResourceTypeId {
   Cart,
   Order,
   Payment,
   Customer
}

impl ExtensionResourceTypeId {
    pub fn from_str(s: &str) -> Option<ExtensionResourceTypeId> {
        match s {
            "cart" => Some(ExtensionResourceTypeId::Cart),
            "order" => Some(ExtensionResourceTypeId::Order),
            "payment" => Some(ExtensionResourceTypeId::Payment),
            "customer" => Some(ExtensionResourceTypeId::Customer),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ExtensionResourceTypeId::Cart => "cart",
           ExtensionResourceTypeId::Order => "order",
           ExtensionResourceTypeId::Payment => "payment",
           ExtensionResourceTypeId::Customer => "customer",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionTrigger {
   pub actions: Vec<ExtensionAction>,
   pub resource_type_id: ExtensionResourceTypeId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionUpdate {
   pub actions: Vec<ExtensionUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionChangeDestinationAction {
   pub destination: ExtensionDestination,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionChangeTriggersAction {
   pub triggers: Vec<ExtensionTrigger>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionSetKeyAction {
   pub key: Option<String>,
}