//Generated file, please do not change

use super::common::Resource;
use super::common::Reference;
use super::message::UserProvidedIdentifiers;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AzureEventGridDestination {
   pub access_key: String,
   pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AzureServiceBusDestination {
   pub connection_string: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSubscription {
   pub resource_type_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryCloudEventsFormat {
   pub cloud_events_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryFormat {
   pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPlatformFormat {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Destination {
   pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudPubSubDestination {
   pub project_id: String,
   pub topic: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IronMqDestination {
   pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageDelivery {
   pub payload_not_included: PayloadNotIncluded,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub project_key: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageSubscription {
   pub resource_type_id: String,
   pub types: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PayloadNotIncluded {
   pub payload_type: String,
   pub reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCreatedDelivery {
   pub resource: Reference,
   pub modified_at: DateTime<Utc>,
   pub version: u64,
   pub project_key: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDeletedDelivery {
   pub resource: Reference,
   pub modified_at: DateTime<Utc>,
   pub version: u64,
   pub project_key: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUpdatedDelivery {
   pub resource: Reference,
   pub modified_at: DateTime<Utc>,
   pub old_version: u64,
   pub version: u64,
   pub project_key: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SnsDestination {
   pub access_key: String,
   pub access_secret: String,
   pub topic_arn: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SqsDestination {
   pub access_key: String,
   pub access_secret: String,
   pub queue_url: String,
   pub region: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
   pub changes: Vec<ChangeSubscription>,
   pub messages: Vec<MessageSubscription>,
   pub format: DeliveryFormat,
   pub destination: Destination,
   pub status: SubscriptionHealthStatus,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionDelivery {
   pub resource: Reference,
   pub notification_type: String,
   pub project_key: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionDraft {
   pub destination: Destination,
   pub changes: Option<Vec<ChangeSubscription>>,
   pub messages: Option<Vec<MessageSubscription>>,
   pub format: Option<DeliveryFormat>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscriptionHealthStatus {
   Healthy,
   ConfigurationError,
   ConfigurationErrorDeliveryStopped,
   TemporaryError
}

impl SubscriptionHealthStatus {
    pub fn from_str(s: &str) -> Option<SubscriptionHealthStatus> {
        match s {
            "Healthy" => Some(SubscriptionHealthStatus::Healthy),
            "ConfigurationError" => Some(SubscriptionHealthStatus::ConfigurationError),
            "ConfigurationErrorDeliveryStopped" => Some(SubscriptionHealthStatus::ConfigurationErrorDeliveryStopped),
            "TemporaryError" => Some(SubscriptionHealthStatus::TemporaryError),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           SubscriptionHealthStatus::Healthy => "Healthy",
           SubscriptionHealthStatus::ConfigurationError => "ConfigurationError",
           SubscriptionHealthStatus::ConfigurationErrorDeliveryStopped => "ConfigurationErrorDeliveryStopped",
           SubscriptionHealthStatus::TemporaryError => "TemporaryError",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPagedQueryResponse {
   pub results: Vec<Subscription>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionUpdate {
   pub actions: Vec<SubscriptionUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionChangeDestinationAction {
   pub destination: Destination,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionSetChangesAction {
   pub changes: Option<Vec<ChangeSubscription>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionSetKeyAction {
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionSetMessagesAction {
   pub messages: Option<Vec<MessageSubscription>>,
}