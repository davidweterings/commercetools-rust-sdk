//Generated file, please do not change

use super::category::Category;
use super::common::LocalizedString;
use super::state::StateReference;
use super::common::Address;
use super::customer::Customer;
use super::customergroup::CustomerGroupReference;
use super::order::Delivery;
use super::order::DeliveryItem;
use super::channel::ChannelReference;
use super::common::Reference;
use super::common::Resource;
use super::order::Order;
use super::cart::DiscountedLineItemPriceForQuantity;
use super::cart::TaxedItemPrice;
use super::customer::CustomerReference;
use super::discountcode::DiscountCodeReference;
use super::cart::DiscountCodeState;
use super::orderedit::OrderEditReference;
use super::orderedit::OrderEditApplied;
use super::common::Money;
use super::order::PaymentState;
use super::order::ReturnInfo;
use super::order::ReturnShipmentState;
use super::order::ShipmentState;
use super::cart::ShippingInfo;
use super::cart::ShippingRateInput;
use super::order::OrderState;
use super::order::Parcel;
use super::order::ParcelMeasurements;
use super::order::TrackingData;
use super::payment::Payment;
use super::cttype::CustomFields;
use super::payment::Transaction;
use super::payment::TransactionState;
use super::product::ProductProjection;
use super::common::Image;
use super::common::DiscountedPrice;
use super::cart::ProductPublishScope;
use super::product::ProductVariant;
use super::review::Review;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCreatedMessage {
   pub category: Category,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySlugChangedMessage {
   pub slug: LocalizedString,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomLineItemStateTransitionMessage {
   pub resource: Reference,
   pub from_state: StateReference,
   pub to_state: StateReference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub transition_date: DateTime<Utc>,
   pub quantity: u64,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub custom_line_item_id: String,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddressAddedMessage {
   pub address: Address,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddressChangedMessage {
   pub address: Address,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddressRemovedMessage {
   pub address: Address,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerCompanyNameSetMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub company_name: String,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerCreatedMessage {
   pub customer: Customer,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerDateOfBirthSetMessage {
   pub resource: Reference,
   pub date_of_birth: NaiveDate,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEmailChangedMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub email: String,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEmailVerifiedMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerGroupSetMessage {
   pub customer_group: CustomerGroupReference,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryAddedMessage {
   pub delivery: Delivery,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryAddressSetMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub delivery_id: String,
   pub id: String,
   pub r#type: String,
   pub address: Option<Address>,
   pub old_address: Option<Address>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryItemsUpdatedMessage {
   pub items: Vec<DeliveryItem>,
   pub old_items: Vec<DeliveryItem>,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub delivery_id: String,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryRemovedMessage {
   pub delivery: Delivery,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryEntryDeletedMessage {
   pub supply_channel: ChannelReference,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub sku: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineItemStateTransitionMessage {
   pub resource: Reference,
   pub from_state: StateReference,
   pub to_state: StateReference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub transition_date: DateTime<Utc>,
   pub quantity: u64,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub line_item_id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageConfiguration {
   pub enabled: bool,
   pub delete_days_after_creation: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageConfigurationDraft {
   pub enabled: bool,
   pub delete_days_after_creation: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagePagedQueryResponse {
   pub results: Vec<Message>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderBillingAddressSetMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub address: Option<Address>,
   pub old_address: Option<Address>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreatedMessage {
   pub order: Order,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCustomLineItemDiscountSetMessage {
   pub discounted_price_per_quantity: Vec<DiscountedLineItemPriceForQuantity>,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub custom_line_item_id: String,
   pub id: String,
   pub r#type: String,
   pub taxed_price: Option<TaxedItemPrice>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCustomerEmailSetMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
   pub email: Option<String>,
   pub old_email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCustomerSetMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub customer_group: Option<CustomerGroupReference>,
   pub old_customer_group: Option<CustomerGroupReference>,
   pub customer: Option<CustomerReference>,
   pub old_customer: Option<CustomerReference>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderDeletedMessage {
   pub order: Order,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderDiscountCodeAddedMessage {
   pub discount_code: DiscountCodeReference,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderDiscountCodeRemovedMessage {
   pub discount_code: DiscountCodeReference,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderDiscountCodeStateSetMessage {
   pub discount_code: DiscountCodeReference,
   pub state: DiscountCodeState,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub old_state: Option<DiscountCodeState>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditAppliedMessage {
   pub result: OrderEditApplied,
   pub edit: OrderEditReference,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderImportedMessage {
   pub order: Order,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderLineItemDiscountSetMessage {
   pub discounted_price_per_quantity: Vec<DiscountedLineItemPriceForQuantity>,
   pub total_price: Money,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub line_item_id: String,
   pub r#type: String,
   pub taxed_price: Option<TaxedItemPrice>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderPaymentStateChangedMessage {
   pub old_payment_state: PaymentState,
   pub payment_state: PaymentState,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderReturnInfoAddedMessage {
   pub resource: Reference,
   pub return_info: ReturnInfo,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderReturnShipmentStateChangedMessage {
   pub resource: Reference,
   pub return_shipment_state: ReturnShipmentState,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub return_item_id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderShipmentStateChangedMessage {
   pub resource: Reference,
   pub old_shipment_state: ShipmentState,
   pub shipment_state: ShipmentState,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderShippingAddressSetMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub address: Option<Address>,
   pub old_address: Option<Address>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderShippingInfoSetMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub old_shipping_info: Option<ShippingInfo>,
   pub shipping_info: Option<ShippingInfo>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderShippingRateInputSetMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub old_shipping_rate_input: Option<ShippingRateInput>,
   pub shipping_rate_input: Option<ShippingRateInput>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStateChangedMessage {
   pub old_order_state: OrderState,
   pub order_state: OrderState,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStateTransitionMessage {
   pub resource: Reference,
   pub state: StateReference,
   pub force: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelAddedToDeliveryMessage {
   pub delivery: Delivery,
   pub parcel: Parcel,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelItemsUpdatedMessage {
   pub items: Vec<DeliveryItem>,
   pub old_items: Vec<DeliveryItem>,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub parcel_id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
   pub delivery_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelMeasurementsUpdatedMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub delivery_id: String,
   pub id: String,
   pub parcel_id: String,
   pub r#type: String,
   pub measurements: Option<ParcelMeasurements>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelRemovedFromDeliveryMessage {
   pub parcel: Parcel,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub delivery_id: String,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelTrackingDataUpdatedMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub delivery_id: String,
   pub id: String,
   pub parcel_id: String,
   pub r#type: String,
   pub tracking_data: Option<TrackingData>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentCreatedMessage {
   pub payment: Payment,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInteractionAddedMessage {
   pub interaction: CustomFields,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentStatusInterfaceCodeSetMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub interface_code: String,
   pub payment_id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentStatusStateTransitionMessage {
   pub resource: Reference,
   pub state: StateReference,
   pub force: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTransactionAddedMessage {
   pub resource: Reference,
   pub transaction: Transaction,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTransactionStateChangedMessage {
   pub resource: Reference,
   pub state: TransactionState,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub transaction_id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductCreatedMessage {
   pub product_projection: ProductProjection,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDeletedMessage {
   pub current_projection: ProductProjection,
   pub resource: Reference,
   pub removed_image_urls: Vec<serde_json::Value>,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductImageAddedMessage {
   pub image: Image,
   pub resource: Reference,
   pub staged: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub variant_id: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceDiscountsSetMessage {
   pub updated_prices: Vec<ProductPriceDiscountsSetUpdatedPrice>,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceDiscountsSetUpdatedPrice {
   pub staged: bool,
   pub variant_id: u32,
   pub price_id: String,
   pub discounted: Option<DiscountedPrice>,
   pub sku: Option<String>,
   pub variant_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceExternalDiscountSetMessage {
   pub resource: Reference,
   pub staged: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub variant_id: u32,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub price_id: String,
   pub r#type: String,
   pub discounted: Option<DiscountedPrice>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
   pub sku: Option<String>,
   pub variant_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductPublishedMessage {
   pub product_projection: ProductProjection,
   pub scope: ProductPublishScope,
   pub resource: Reference,
   pub removed_image_urls: Vec<serde_json::Value>,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductRevertedStagedChangesMessage {
   pub resource: Reference,
   pub removed_image_urls: Vec<serde_json::Value>,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSlugChangedMessage {
   pub slug: LocalizedString,
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductStateTransitionMessage {
   pub resource: Reference,
   pub state: StateReference,
   pub force: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductUnpublishedMessage {
   pub resource: Reference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductVariantDeletedMessage {
   pub variant: ProductVariant,
   pub resource: Reference,
   pub removed_image_urls: Vec<serde_json::Value>,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewCreatedMessage {
   pub resource: Reference,
   pub review: Review,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewRatingSetMessage {
   pub resource: Reference,
   pub included_in_statistics: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub target: Option<Reference>,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
   pub new_rating: Option<u32>,
   pub old_rating: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewStateTransitionMessage {
   pub resource: Reference,
   pub target: Reference,
   pub new_state: StateReference,
   pub old_state: StateReference,
   pub force: bool,
   pub new_included_in_statistics: bool,
   pub old_included_in_statistics: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub resource_version: u64,
   pub sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub r#type: String,
   pub resource_user_provided_identifiers: Option<UserProvidedIdentifiers>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserProvidedIdentifiers {
   pub slug: Option<LocalizedString>,
   pub customer_number: Option<String>,
   pub external_id: Option<String>,
   pub key: Option<String>,
   pub order_number: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCreatedMessagePayload {
   pub category: Category,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySlugChangedMessagePayload {
   pub slug: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomLineItemStateTransitionMessagePayload {
   pub from_state: StateReference,
   pub to_state: StateReference,
   pub transition_date: DateTime<Utc>,
   pub quantity: u64,
   pub custom_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddressAddedMessagePayload {
   pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddressChangedMessagePayload {
   pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddressRemovedMessagePayload {
   pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerCompanyNameSetMessagePayload {
   pub company_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerCreatedMessagePayload {
   pub customer: Customer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerDateOfBirthSetMessagePayload {
   pub date_of_birth: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEmailChangedMessagePayload {
   pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEmailVerifiedMessagePayload {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerGroupSetMessagePayload {
   pub customer_group: CustomerGroupReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryAddedMessagePayload {
   pub delivery: Delivery,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryAddressSetMessagePayload {
   pub delivery_id: String,
   pub address: Option<Address>,
   pub old_address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryItemsUpdatedMessagePayload {
   pub items: Vec<DeliveryItem>,
   pub old_items: Vec<DeliveryItem>,
   pub delivery_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryRemovedMessagePayload {
   pub delivery: Delivery,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryEntryDeletedMessagePayload {
   pub supply_channel: ChannelReference,
   pub sku: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineItemStateTransitionMessagePayload {
   pub from_state: StateReference,
   pub to_state: StateReference,
   pub transition_date: DateTime<Utc>,
   pub quantity: u64,
   pub line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagePayload {
   pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderBillingAddressSetMessagePayload {
   pub address: Option<Address>,
   pub old_address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreatedMessagePayload {
   pub order: Order,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCustomLineItemDiscountSetMessagePayload {
   pub discounted_price_per_quantity: Vec<DiscountedLineItemPriceForQuantity>,
   pub custom_line_item_id: String,
   pub taxed_price: Option<TaxedItemPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCustomerEmailSetMessagePayload {
   pub email: Option<String>,
   pub old_email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCustomerSetMessagePayload {
   pub customer_group: Option<CustomerGroupReference>,
   pub old_customer_group: Option<CustomerGroupReference>,
   pub customer: Option<CustomerReference>,
   pub old_customer: Option<CustomerReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderDeletedMessagePayload {
   pub order: Order,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderDiscountCodeAddedMessagePayload {
   pub discount_code: DiscountCodeReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderDiscountCodeRemovedMessagePayload {
   pub discount_code: DiscountCodeReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderDiscountCodeStateSetMessagePayload {
   pub discount_code: DiscountCodeReference,
   pub state: DiscountCodeState,
   pub old_state: Option<DiscountCodeState>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditAppliedMessagePayload {
   pub result: OrderEditApplied,
   pub edit: OrderEditReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderImportedMessagePayload {
   pub order: Order,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderLineItemDiscountSetMessagePayload {
   pub discounted_price_per_quantity: Vec<DiscountedLineItemPriceForQuantity>,
   pub total_price: Money,
   pub line_item_id: String,
   pub taxed_price: Option<TaxedItemPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderPaymentStateChangedMessagePayload {
   pub old_payment_state: PaymentState,
   pub payment_state: PaymentState,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderReturnInfoAddedMessagePayload {
   pub return_info: ReturnInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderReturnShipmentStateChangedMessagePayload {
   pub return_shipment_state: ReturnShipmentState,
   pub return_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderShipmentStateChangedMessagePayload {
   pub old_shipment_state: ShipmentState,
   pub shipment_state: ShipmentState,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderShippingAddressSetMessagePayload {
   pub address: Option<Address>,
   pub old_address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderShippingInfoSetMessagePayload {
   pub old_shipping_info: Option<ShippingInfo>,
   pub shipping_info: Option<ShippingInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderShippingRateInputSetMessagePayload {
   pub old_shipping_rate_input: Option<ShippingRateInput>,
   pub shipping_rate_input: Option<ShippingRateInput>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStateChangedMessagePayload {
   pub old_order_state: OrderState,
   pub order_state: OrderState,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStateTransitionMessagePayload {
   pub state: StateReference,
   pub force: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelAddedToDeliveryMessagePayload {
   pub delivery: Delivery,
   pub parcel: Parcel,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelItemsUpdatedMessagePayload {
   pub items: Vec<DeliveryItem>,
   pub old_items: Vec<DeliveryItem>,
   pub parcel_id: String,
   pub delivery_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelMeasurementsUpdatedMessagePayload {
   pub delivery_id: String,
   pub parcel_id: String,
   pub measurements: Option<ParcelMeasurements>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelRemovedFromDeliveryMessagePayload {
   pub parcel: Parcel,
   pub delivery_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelTrackingDataUpdatedMessagePayload {
   pub delivery_id: String,
   pub parcel_id: String,
   pub tracking_data: Option<TrackingData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentCreatedMessagePayload {
   pub payment: Payment,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInteractionAddedMessagePayload {
   pub interaction: CustomFields,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentStatusInterfaceCodeSetMessagePayload {
   pub interface_code: String,
   pub payment_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentStatusStateTransitionMessagePayload {
   pub state: StateReference,
   pub force: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTransactionAddedMessagePayload {
   pub transaction: Transaction,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTransactionStateChangedMessagePayload {
   pub state: TransactionState,
   pub transaction_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductCreatedMessagePayload {
   pub product_projection: ProductProjection,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDeletedMessagePayload {
   pub current_projection: ProductProjection,
   pub removed_image_urls: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductImageAddedMessagePayload {
   pub image: Image,
   pub staged: bool,
   pub variant_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceDiscountsSetMessagePayload {
   pub updated_prices: Vec<ProductPriceDiscountsSetUpdatedPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceExternalDiscountSetMessagePayload {
   pub staged: bool,
   pub variant_id: u32,
   pub price_id: String,
   pub discounted: Option<DiscountedPrice>,
   pub sku: Option<String>,
   pub variant_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductPublishedMessagePayload {
   pub product_projection: ProductProjection,
   pub scope: ProductPublishScope,
   pub removed_image_urls: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductRevertedStagedChangesMessagePayload {
   pub removed_image_urls: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSlugChangedMessagePayload {
   pub slug: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductStateTransitionMessagePayload {
   pub state: StateReference,
   pub force: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductUnpublishedMessagePayload {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductVariantDeletedMessagePayload {
   pub variant: ProductVariant,
   pub removed_image_urls: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewCreatedMessagePayload {
   pub review: Review,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewRatingSetMessagePayload {
   pub included_in_statistics: bool,
   pub target: Option<Reference>,
   pub new_rating: Option<u32>,
   pub old_rating: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewStateTransitionMessagePayload {
   pub target: Reference,
   pub new_state: StateReference,
   pub old_state: StateReference,
   pub force: bool,
   pub new_included_in_statistics: bool,
   pub old_included_in_statistics: bool,
}