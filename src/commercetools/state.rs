//Generated file, please do not change

use super::common::LocalizedString;
use super::common::Resource;
use super::common::Reference;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
   pub r#type: StateTypeEnum,
   pub built_in: bool,
   pub initial: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub key: String,
   pub roles: Option<Vec<StateRoleEnum>>,
   pub transitions: Option<Vec<StateReference>>,
   pub description: Option<LocalizedString>,
   pub name: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateDraft {
   pub r#type: StateTypeEnum,
   pub key: String,
   pub roles: Option<Vec<StateRoleEnum>>,
   pub transitions: Option<Vec<StateReference>>,
   pub description: Option<LocalizedString>,
   pub name: Option<LocalizedString>,
   pub initial: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatePagedQueryResponse {
   pub results: Vec<State>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateReference {
   pub obj: Option<State>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StateRoleEnum {
   ReviewIncludedInStatistics
}

impl StateRoleEnum {
    pub fn from_str(s: &str) -> Option<StateRoleEnum> {
        match s {
            "ReviewIncludedInStatistics" => Some(StateRoleEnum::ReviewIncludedInStatistics),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           StateRoleEnum::ReviewIncludedInStatistics => "ReviewIncludedInStatistics",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StateTypeEnum {
   OrderState,
   LineItemState,
   ProductState,
   ReviewState,
   PaymentState
}

impl StateTypeEnum {
    pub fn from_str(s: &str) -> Option<StateTypeEnum> {
        match s {
            "OrderState" => Some(StateTypeEnum::OrderState),
            "LineItemState" => Some(StateTypeEnum::LineItemState),
            "ProductState" => Some(StateTypeEnum::ProductState),
            "ReviewState" => Some(StateTypeEnum::ReviewState),
            "PaymentState" => Some(StateTypeEnum::PaymentState),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           StateTypeEnum::OrderState => "OrderState",
           StateTypeEnum::LineItemState => "LineItemState",
           StateTypeEnum::ProductState => "ProductState",
           StateTypeEnum::ReviewState => "ReviewState",
           StateTypeEnum::PaymentState => "PaymentState",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateUpdate {
   pub actions: Vec<StateUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateAddRolesAction {
   pub roles: Vec<StateRoleEnum>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateChangeInitialAction {
   pub initial: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateChangeKeyAction {
   pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateChangeTypeAction {
   pub r#type: StateTypeEnum,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateRemoveRolesAction {
   pub roles: Vec<StateRoleEnum>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateSetDescriptionAction {
   pub description: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateSetNameAction {
   pub name: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateSetRolesAction {
   pub roles: Vec<StateRoleEnum>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateSetTransitionsAction {
   pub transitions: Option<Vec<StateReference>>,
}