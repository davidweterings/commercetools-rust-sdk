//Generated file, please do not change

use super::customer::CustomerReference;
use super::common::TypedMoney;
use super::cttype::CustomFields;
use super::common::Resource;
use super::common::Money;
use super::cttype::CustomFieldsDraft;
use super::common::LocalizedString;
use super::common::Reference;
use super::state::StateReference;
use super::cttype::TypeReference;
use super::cttype::FieldContainer;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
   pub interface_interactions: Vec<CustomFields>,
   pub transactions: Vec<Transaction>,
   pub payment_method_info: PaymentMethodInfo,
   pub payment_status: PaymentStatus,
   pub amount_planned: TypedMoney,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub custom: Option<CustomFields>,
   pub customer: Option<CustomerReference>,
   pub amount_authorized: Option<TypedMoney>,
   pub amount_paid: Option<TypedMoney>,
   pub amount_refunded: Option<TypedMoney>,
   pub anonymous_id: Option<String>,
   pub authorized_until: Option<String>,
   pub external_id: Option<String>,
   pub interface_id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentDraft {
   pub amount_planned: Money,
   pub interface_interactions: Option<Vec<CustomFieldsDraft>>,
   pub transactions: Option<Vec<TransactionDraft>>,
   pub custom: Option<CustomFieldsDraft>,
   pub customer: Option<CustomerReference>,
   pub amount_authorized: Option<Money>,
   pub amount_paid: Option<Money>,
   pub amount_refunded: Option<Money>,
   pub payment_method_info: Option<PaymentMethodInfo>,
   pub payment_status: Option<PaymentStatus>,
   pub anonymous_id: Option<String>,
   pub authorized_until: Option<String>,
   pub external_id: Option<String>,
   pub interface_id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodInfo {
   pub name: Option<LocalizedString>,
   pub method: Option<String>,
   pub payment_interface: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPagedQueryResponse {
   pub results: Vec<Payment>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReference {
   pub obj: Option<Payment>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentStatus {
   pub state: Option<StateReference>,
   pub interface_code: Option<String>,
   pub interface_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentUpdate {
   pub actions: Vec<PaymentUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
   pub r#type: TransactionType,
   pub amount: TypedMoney,
   pub id: String,
   pub state: Option<TransactionState>,
   pub timestamp: Option<DateTime<Utc>>,
   pub interaction_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDraft {
   pub amount: Money,
   pub r#type: TransactionType,
   pub state: Option<TransactionState>,
   pub timestamp: Option<DateTime<Utc>>,
   pub interaction_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionState {
   Initial,
   Pending,
   Success,
   Failure
}

impl TransactionState {
    pub fn from_str(s: &str) -> Option<TransactionState> {
        match s {
            "Initial" => Some(TransactionState::Initial),
            "Pending" => Some(TransactionState::Pending),
            "Success" => Some(TransactionState::Success),
            "Failure" => Some(TransactionState::Failure),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           TransactionState::Initial => "Initial",
           TransactionState::Pending => "Pending",
           TransactionState::Success => "Success",
           TransactionState::Failure => "Failure",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
   Authorization,
   CancelAuthorization,
   Charge,
   Refund,
   Chargeback
}

impl TransactionType {
    pub fn from_str(s: &str) -> Option<TransactionType> {
        match s {
            "Authorization" => Some(TransactionType::Authorization),
            "CancelAuthorization" => Some(TransactionType::CancelAuthorization),
            "Charge" => Some(TransactionType::Charge),
            "Refund" => Some(TransactionType::Refund),
            "Chargeback" => Some(TransactionType::Chargeback),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           TransactionType::Authorization => "Authorization",
           TransactionType::CancelAuthorization => "CancelAuthorization",
           TransactionType::Charge => "Charge",
           TransactionType::Refund => "Refund",
           TransactionType::Chargeback => "Chargeback",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAddInterfaceInteractionAction {
   pub r#type: TypeReference,
   pub fields: Option<FieldContainer>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAddTransactionAction {
   pub transaction: TransactionDraft,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentChangeAmountPlannedAction {
   pub amount: Money,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentChangeTransactionInteractionIdAction {
   pub interaction_id: String,
   pub transaction_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentChangeTransactionStateAction {
   pub state: TransactionState,
   pub transaction_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentChangeTransactionTimestampAction {
   pub timestamp: DateTime<Utc>,
   pub transaction_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetAmountPaidAction {
   pub amount: Option<Money>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetAmountRefundedAction {
   pub amount: Option<Money>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetAnonymousIdAction {
   pub anonymous_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetAuthorizationAction {
   pub amount: Option<Money>,
   pub until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetCustomerAction {
   pub customer: Option<CustomerReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetExternalIdAction {
   pub external_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetInterfaceIdAction {
   pub interface_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetKeyAction {
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetMethodInfoInterfaceAction {
   pub interface: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetMethodInfoMethodAction {
   pub method: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetMethodInfoNameAction {
   pub name: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetStatusInterfaceCodeAction {
   pub interface_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSetStatusInterfaceTextAction {
   pub interface_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTransitionStateAction {
   pub state: StateReference,
   pub force: Option<bool>,
}