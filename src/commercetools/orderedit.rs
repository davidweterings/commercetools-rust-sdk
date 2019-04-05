//Generated file, please do not change

use super::order::OrderReference;
use super::order::StagedOrderUpdateAction;
use super::cttype::CustomFields;
use super::common::Resource;
use super::cttype::CustomFieldsDraft;
use super::error::ErrorObject;
use super::message::MessagePayload;
use super::common::Reference;
use super::common::Money;
use super::cart::TaxedPrice;
use super::order::Order;
use super::cttype::TypeReference;
use super::common::LocalizedString;
use super::taxcategory::TaxCategoryReference;
use super::cart::ExternalTaxRateDraft;
use super::order::DeliveryItem;
use super::common::Address;
use super::order::ParcelDraft;
use super::channel::ChannelReference;
use super::cart::ExternalLineItemTotalPrice;
use super::cart::ItemShippingDetailsDraft;
use super::order::ParcelMeasurements;
use super::order::TrackingData;
use super::payment::PaymentReference;
use super::order::ReturnItemDraft;
use super::shoppinglist::ShoppingListReference;
use super::order::OrderState;
use super::order::PaymentState;
use super::order::ShipmentState;
use super::cart::TaxCalculationMode;
use super::cart::TaxMode;
use super::cart::RoundingMode;
use super::order::ItemState;
use super::discountcode::DiscountCodeReference;
use super::cttype::FieldContainer;
use super::cart::ExternalTaxAmountDraft;
use super::shippingmethod::ShippingRateDraft;
use super::customergroup::CustomerGroupReference;
use super::cart::TaxPortion;
use super::order::ReturnPaymentState;
use super::order::ReturnShipmentState;
use super::shippingmethod::ShippingMethodReference;
use super::cart::ShippingRateInputDraft;
use super::state::StateReference;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::cart::CustomLineItem;
use super::cart::LineItem;
use super::order::SyncInfo;
use super::cart::CartOrigin;
use super::cart::DiscountCodeInfo;
use super::order::ReturnInfo;
use super::cart::CartReference;
use super::cart::InventoryMode;
use super::order::PaymentInfo;
use super::cart::ShippingInfo;
use super::cart::ShippingRateInput;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEdit {
   pub staged_actions: Vec<StagedOrderUpdateAction>,
   pub result: OrderEditResult,
   pub resource: OrderReference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub custom: Option<CustomFields>,
   pub comment: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditApplied {
   pub excerpt_after_edit: OrderExcerpt,
   pub excerpt_before_edit: OrderExcerpt,
   pub applied_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditApply {
   pub edit_version: u32,
   pub resource_version: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditDraft {
   pub resource: OrderReference,
   pub staged_actions: Option<Vec<StagedOrderUpdateAction>>,
   pub custom: Option<CustomFieldsDraft>,
   pub dry_run: Option<bool>,
   pub comment: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditNotProcessed {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditPagedQueryResponse {
   pub results: Vec<OrderEdit>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditPreviewFailure {
   pub errors: Vec<ErrorObject>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditPreviewSuccess {
   pub message_payloads: Vec<MessagePayload>,
   pub preview: StagedOrder,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditReference {
   pub obj: Option<OrderEdit>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum OrderEditResult {
   #[serde(rename = "Applied")]
   EOrderEditApplied(OrderEditApplied),
   #[serde(rename = "NotProcessed")]
   EOrderEditNotProcessed(OrderEditNotProcessed),
   #[serde(rename = "PreviewSuccess")]
   EOrderEditPreviewSuccess(OrderEditPreviewSuccess),
   #[serde(rename = "PreviewFailure")]
   EOrderEditPreviewFailure(OrderEditPreviewFailure),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditUpdate {
   pub actions: Vec<OrderEditUpdateAction>,
   pub dry_run: bool,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum OrderEditUpdateAction {
   #[serde(rename = "addStagedAction")]
   EOrderEditAddStagedActionAction(OrderEditAddStagedActionAction),
   #[serde(rename = "setComment")]
   EOrderEditSetCommentAction(OrderEditSetCommentAction),
   #[serde(rename = "setCustomField")]
   EOrderEditSetCustomFieldAction(OrderEditSetCustomFieldAction),
   #[serde(rename = "setCustomType")]
   EOrderEditSetCustomTypeAction(OrderEditSetCustomTypeAction),
   #[serde(rename = "setKey")]
   EOrderEditSetKeyAction(OrderEditSetKeyAction),
   #[serde(rename = "setStagedActions")]
   EOrderEditSetStagedActionsAction(OrderEditSetStagedActionsAction),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderExcerpt {
   pub total_price: Money,
   pub version: u32,
   pub taxed_price: Option<TaxedPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrder {
   pub custom_line_items: Vec<CustomLineItem>,
   pub line_items: Vec<LineItem>,
   pub sync_info: Vec<SyncInfo>,
   pub origin: CartOrigin,
   pub total_price: Money,
   pub order_state: OrderState,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub last_message_sequence_number: u64,
   pub version: u64,
   pub id: String,
   pub discount_codes: Option<Vec<DiscountCodeInfo>>,
   pub item_shipping_addresses: Option<Vec<Address>>,
   pub return_info: Option<Vec<ReturnInfo>>,
   pub billing_address: Option<Address>,
   pub shipping_address: Option<Address>,
   pub cart: Option<CartReference>,
   pub custom: Option<CustomFields>,
   pub customer_group: Option<CustomerGroupReference>,
   pub inventory_mode: Option<InventoryMode>,
   pub payment_info: Option<PaymentInfo>,
   pub payment_state: Option<PaymentState>,
   pub tax_rounding_mode: Option<RoundingMode>,
   pub shipment_state: Option<ShipmentState>,
   pub shipping_info: Option<ShippingInfo>,
   pub shipping_rate_input: Option<ShippingRateInput>,
   pub state: Option<StateReference>,
   pub tax_calculation_mode: Option<TaxCalculationMode>,
   pub tax_mode: Option<TaxMode>,
   pub taxed_price: Option<TaxedPrice>,
   pub completed_at: Option<DateTime<Utc>>,
   pub anonymous_id: Option<String>,
   pub country: Option<String>,
   pub customer_email: Option<String>,
   pub customer_id: Option<String>,
   pub locale: Option<String>,
   pub order_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditAddStagedActionAction {
   pub staged_action: StagedOrderUpdateAction,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditSetCommentAction {
   pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditSetCustomTypeAction {
   pub r#type: Option<TypeReference>,
   pub fields: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditSetKeyAction {
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEditSetStagedActionsAction {
   pub staged_actions: Vec<StagedOrderUpdateAction>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderAddCustomLineItemAction {
   pub name: LocalizedString,
   pub money: Money,
   pub slug: String,
   pub custom: Option<CustomFieldsDraft>,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub tax_category: Option<TaxCategoryReference>,
   pub quantity: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderAddDeliveryAction {
   pub items: Option<Vec<DeliveryItem>>,
   pub parcels: Option<Vec<ParcelDraft>>,
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderAddDiscountCodeAction {
   pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderAddItemShippingAddressAction {
   pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderAddLineItemAction {
   pub distribution_channel: Option<ChannelReference>,
   pub supply_channel: Option<ChannelReference>,
   pub custom: Option<CustomFieldsDraft>,
   pub external_total_price: Option<ExternalLineItemTotalPrice>,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
   pub external_price: Option<Money>,
   pub quantity: Option<u32>,
   pub variant_id: Option<u64>,
   pub product_id: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderAddParcelToDeliveryAction {
   pub delivery_id: String,
   pub items: Option<Vec<DeliveryItem>>,
   pub measurements: Option<ParcelMeasurements>,
   pub tracking_data: Option<TrackingData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderAddPaymentAction {
   pub payment: PaymentReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderAddReturnInfoAction {
   pub items: Vec<ReturnItemDraft>,
   pub return_date: Option<DateTime<Utc>>,
   pub return_tracking_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderAddShoppingListAction {
   pub shopping_list: ShoppingListReference,
   pub distribution_channel: Option<ChannelReference>,
   pub supply_channel: Option<ChannelReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderChangeCustomLineItemMoneyAction {
   pub money: Money,
   pub custom_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderChangeCustomLineItemQuantityAction {
   pub quantity: u32,
   pub custom_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderChangeLineItemQuantityAction {
   pub quantity: u32,
   pub line_item_id: String,
   pub external_total_price: Option<ExternalLineItemTotalPrice>,
   pub external_price: Option<Money>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderChangeOrderStateAction {
   pub order_state: OrderState,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderChangePaymentStateAction {
   pub payment_state: Option<PaymentState>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderChangeShipmentStateAction {
   pub shipment_state: Option<ShipmentState>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderChangeTaxCalculationModeAction {
   pub tax_calculation_mode: TaxCalculationMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderChangeTaxModeAction {
   pub tax_mode: TaxMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderChangeTaxRoundingModeAction {
   pub tax_rounding_mode: RoundingMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderImportCustomLineItemStateAction {
   pub state: Vec<ItemState>,
   pub custom_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderImportLineItemStateAction {
   pub state: Vec<ItemState>,
   pub line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderRemoveCustomLineItemAction {
   pub custom_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderRemoveDeliveryAction {
   pub delivery_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderRemoveDiscountCodeAction {
   pub discount_code: DiscountCodeReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderRemoveItemShippingAddressAction {
   pub address_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderRemoveLineItemAction {
   pub line_item_id: String,
   pub external_total_price: Option<ExternalLineItemTotalPrice>,
   pub shipping_details_to_remove: Option<ItemShippingDetailsDraft>,
   pub external_price: Option<Money>,
   pub quantity: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderRemoveParcelFromDeliveryAction {
   pub parcel_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderRemovePaymentAction {
   pub payment: PaymentReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetBillingAddressAction {
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCountryAction {
   pub country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomLineItemCustomFieldAction {
   pub custom_line_item_id: String,
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomLineItemCustomTypeAction {
   pub custom_line_item_id: String,
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomLineItemShippingDetailsAction {
   pub custom_line_item_id: String,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomLineItemTaxAmountAction {
   pub custom_line_item_id: String,
   pub external_tax_amount: Option<ExternalTaxAmountDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomLineItemTaxRateAction {
   pub custom_line_item_id: String,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomShippingMethodAction {
   pub shipping_rate: ShippingRateDraft,
   pub shipping_method_name: String,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub tax_category: Option<TaxCategoryReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomerEmailAction {
   pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomerGroupAction {
   pub customer_group: Option<CustomerGroupReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetCustomerIdAction {
   pub customer_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetDeliveryAddressAction {
   pub delivery_id: String,
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetDeliveryItemsAction {
   pub items: Vec<DeliveryItem>,
   pub delivery_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetLineItemCustomFieldAction {
   pub line_item_id: String,
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetLineItemCustomTypeAction {
   pub line_item_id: String,
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetLineItemPriceAction {
   pub line_item_id: String,
   pub external_price: Option<Money>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetLineItemShippingDetailsAction {
   pub line_item_id: String,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetLineItemTaxAmountAction {
   pub line_item_id: String,
   pub external_tax_amount: Option<ExternalTaxAmountDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetLineItemTaxRateAction {
   pub line_item_id: String,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetLineItemTotalPriceAction {
   pub line_item_id: String,
   pub external_total_price: Option<ExternalLineItemTotalPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetLocaleAction {
   pub locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetOrderNumberAction {
   pub order_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetOrderTotalTaxAction {
   pub external_total_gross: Money,
   pub external_tax_portions: Option<Vec<TaxPortion>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetParcelItemsAction {
   pub items: Vec<DeliveryItem>,
   pub parcel_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetParcelMeasurementsAction {
   pub parcel_id: String,
   pub measurements: Option<ParcelMeasurements>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetParcelTrackingDataAction {
   pub parcel_id: String,
   pub tracking_data: Option<TrackingData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetReturnPaymentStateAction {
   pub payment_state: ReturnPaymentState,
   pub return_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetReturnShipmentStateAction {
   pub shipment_state: ReturnShipmentState,
   pub return_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetShippingAddressAction {
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetShippingAddressAndCustomShippingMethodAction {
   pub address: Address,
   pub shipping_rate: ShippingRateDraft,
   pub shipping_method_name: String,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub tax_category: Option<TaxCategoryReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetShippingAddressAndShippingMethodAction {
   pub address: Address,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub shipping_method: Option<ShippingMethodReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetShippingMethodAction {
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub shipping_method: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetShippingMethodTaxAmountAction {
   pub external_tax_amount: Option<ExternalTaxAmountDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetShippingMethodTaxRateAction {
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderSetShippingRateInputAction {
   pub shipping_rate_input: Option<ShippingRateInputDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderTransitionCustomLineItemStateAction {
   pub from_state: StateReference,
   pub to_state: StateReference,
   pub quantity: u64,
   pub custom_line_item_id: String,
   pub actual_transition_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderTransitionLineItemStateAction {
   pub from_state: StateReference,
   pub to_state: StateReference,
   pub quantity: u64,
   pub line_item_id: String,
   pub actual_transition_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderTransitionStateAction {
   pub state: StateReference,
   pub force: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderUpdateItemShippingAddressAction {
   pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StagedOrderUpdateSyncInfoAction {
   pub channel: ChannelReference,
   pub synced_at: Option<DateTime<Utc>>,
   pub external_id: Option<String>,
}