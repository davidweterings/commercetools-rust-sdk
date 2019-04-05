//Generated file, please do not change

use super::common::Address;
use super::customergroup::CustomerGroupReference;
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
pub enum AnonymousCartSignInMode {
   MergeWithExistingCustomerCart,
   UseAsNewActiveCustomerCart
}

impl AnonymousCartSignInMode {
    pub fn from_str(s: &str) -> Option<AnonymousCartSignInMode> {
        match s {
            "MergeWithExistingCustomerCart" => Some(AnonymousCartSignInMode::MergeWithExistingCustomerCart),
            "UseAsNewActiveCustomerCart" => Some(AnonymousCartSignInMode::UseAsNewActiveCustomerCart),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           AnonymousCartSignInMode::MergeWithExistingCustomerCart => "MergeWithExistingCustomerCart",
           AnonymousCartSignInMode::UseAsNewActiveCustomerCart => "UseAsNewActiveCustomerCart",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
   pub addresses: Vec<Address>,
   pub is_email_verified: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub email: String,
   pub id: String,
   pub password: String,
   pub billing_address_ids: Option<Vec<String>>,
   pub shipping_address_ids: Option<Vec<String>>,
   pub custom: Option<CustomFields>,
   pub customer_group: Option<CustomerGroupReference>,
   pub date_of_birth: Option<NaiveDate>,
   pub company_name: Option<String>,
   pub customer_number: Option<String>,
   pub default_billing_address_id: Option<String>,
   pub default_shipping_address_id: Option<String>,
   pub external_id: Option<String>,
   pub first_name: Option<String>,
   pub key: Option<String>,
   pub last_name: Option<String>,
   pub locale: Option<String>,
   pub middle_name: Option<String>,
   pub salutation: Option<String>,
   pub title: Option<String>,
   pub vat_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerChangePassword {
   pub version: u64,
   pub current_password: String,
   pub id: String,
   pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerCreateEmailToken {
   pub ttl_minutes: u64,
   pub id: String,
   pub version: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerCreatePasswordResetToken {
   pub email: String,
   pub ttl_minutes: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerDraft {
   pub email: String,
   pub password: String,
   pub addresses: Option<Vec<Address>>,
   pub billing_addresses: Option<Vec<u32>>,
   pub shipping_addresses: Option<Vec<u32>>,
   pub custom: Option<CustomFieldsDraft>,
   pub customer_group: Option<CustomerGroupReference>,
   pub is_email_verified: Option<bool>,
   pub date_of_birth: Option<NaiveDate>,
   pub default_billing_address: Option<u64>,
   pub default_shipping_address: Option<u64>,
   pub anonymous_cart_id: Option<String>,
   pub anonymous_id: Option<String>,
   pub company_name: Option<String>,
   pub customer_number: Option<String>,
   pub external_id: Option<String>,
   pub first_name: Option<String>,
   pub key: Option<String>,
   pub last_name: Option<String>,
   pub locale: Option<String>,
   pub middle_name: Option<String>,
   pub salutation: Option<String>,
   pub title: Option<String>,
   pub vat_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEmailVerify {
   pub token_value: String,
   pub version: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerPagedQueryResponse {
   pub results: Vec<Customer>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerReference {
   pub obj: Option<Customer>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerResetPassword {
   pub new_password: String,
   pub token_value: String,
   pub version: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSignInResult {
   pub customer: Customer,
   pub cart: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSignin {
   pub email: String,
   pub password: String,
   pub anonymous_cart_sign_in_mode: Option<AnonymousCartSignInMode>,
   pub anonymous_cart_id: Option<String>,
   pub anonymous_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerToken {
   pub created_at: DateTime<Utc>,
   pub expires_at: DateTime<Utc>,
   pub customer_id: String,
   pub id: String,
   pub value: String,
   pub last_modified_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerUpdate {
   pub actions: Vec<CustomerUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum CustomerUpdateAction {
   #[serde(rename = "addAddress")]
   ECustomerAddAddressAction(CustomerAddAddressAction),
   #[serde(rename = "addBillingAddressId")]
   ECustomerAddBillingAddressIdAction(CustomerAddBillingAddressIdAction),
   #[serde(rename = "addShippingAddressId")]
   ECustomerAddShippingAddressIdAction(CustomerAddShippingAddressIdAction),
   #[serde(rename = "changeAddress")]
   ECustomerChangeAddressAction(CustomerChangeAddressAction),
   #[serde(rename = "changeEmail")]
   ECustomerChangeEmailAction(CustomerChangeEmailAction),
   #[serde(rename = "removeAddress")]
   ECustomerRemoveAddressAction(CustomerRemoveAddressAction),
   #[serde(rename = "removeBillingAddressId")]
   ECustomerRemoveBillingAddressIdAction(CustomerRemoveBillingAddressIdAction),
   #[serde(rename = "removeShippingAddressId")]
   ECustomerRemoveShippingAddressIdAction(CustomerRemoveShippingAddressIdAction),
   #[serde(rename = "setCompanyName")]
   ECustomerSetCompanyNameAction(CustomerSetCompanyNameAction),
   #[serde(rename = "setCustomField")]
   ECustomerSetCustomFieldAction(CustomerSetCustomFieldAction),
   #[serde(rename = "setCustomType")]
   ECustomerSetCustomTypeAction(CustomerSetCustomTypeAction),
   #[serde(rename = "setCustomerGroup")]
   ECustomerSetCustomerGroupAction(CustomerSetCustomerGroupAction),
   #[serde(rename = "setCustomerNumber")]
   ECustomerSetCustomerNumberAction(CustomerSetCustomerNumberAction),
   #[serde(rename = "setDateOfBirth")]
   ECustomerSetDateOfBirthAction(CustomerSetDateOfBirthAction),
   #[serde(rename = "setDefaultBillingAddress")]
   ECustomerSetDefaultBillingAddressAction(CustomerSetDefaultBillingAddressAction),
   #[serde(rename = "setDefaultShippingAddress")]
   ECustomerSetDefaultShippingAddressAction(CustomerSetDefaultShippingAddressAction),
   #[serde(rename = "setExternalId")]
   ECustomerSetExternalIdAction(CustomerSetExternalIdAction),
   #[serde(rename = "setFirstName")]
   ECustomerSetFirstNameAction(CustomerSetFirstNameAction),
   #[serde(rename = "setKey")]
   ECustomerSetKeyAction(CustomerSetKeyAction),
   #[serde(rename = "setLastName")]
   ECustomerSetLastNameAction(CustomerSetLastNameAction),
   #[serde(rename = "setLocale")]
   ECustomerSetLocaleAction(CustomerSetLocaleAction),
   #[serde(rename = "setMiddleName")]
   ECustomerSetMiddleNameAction(CustomerSetMiddleNameAction),
   #[serde(rename = "setSalutation")]
   ECustomerSetSalutationAction(CustomerSetSalutationAction),
   #[serde(rename = "setTitle")]
   ECustomerSetTitleAction(CustomerSetTitleAction),
   #[serde(rename = "setVatId")]
   ECustomerSetVatIdAction(CustomerSetVatIdAction),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddAddressAction {
   pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddBillingAddressIdAction {
   pub address_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddShippingAddressIdAction {
   pub address_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerChangeAddressAction {
   pub address: Address,
   pub address_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerChangeEmailAction {
   pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerRemoveAddressAction {
   pub address_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerRemoveBillingAddressIdAction {
   pub address_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerRemoveShippingAddressIdAction {
   pub address_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetCompanyNameAction {
   pub company_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetCustomerGroupAction {
   pub customer_group: Option<CustomerGroupReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetCustomerNumberAction {
   pub customer_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetDateOfBirthAction {
   pub date_of_birth: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetDefaultBillingAddressAction {
   pub address_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetDefaultShippingAddressAction {
   pub address_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetExternalIdAction {
   pub external_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetFirstNameAction {
   pub first_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetKeyAction {
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetLastNameAction {
   pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetLocaleAction {
   pub locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetMiddleNameAction {
   pub middle_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetSalutationAction {
   pub salutation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetTitleAction {
   pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSetVatIdAction {
   pub vat_id: Option<String>,
}