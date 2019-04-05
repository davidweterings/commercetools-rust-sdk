//Generated file, please do not change

use super::orderedit::StagedOrderAddCustomLineItemAction;
use super::orderedit::StagedOrderAddDeliveryAction;
use super::orderedit::StagedOrderAddDiscountCodeAction;
use super::orderedit::StagedOrderAddItemShippingAddressAction;
use super::orderedit::StagedOrderAddLineItemAction;
use super::orderedit::StagedOrderAddParcelToDeliveryAction;
use super::orderedit::StagedOrderAddPaymentAction;
use super::orderedit::StagedOrderAddReturnInfoAction;
use super::orderedit::StagedOrderAddShoppingListAction;
use super::orderedit::StagedOrderChangeCustomLineItemMoneyAction;
use super::orderedit::StagedOrderChangeCustomLineItemQuantityAction;
use super::orderedit::StagedOrderChangeLineItemQuantityAction;
use super::orderedit::StagedOrderChangeOrderStateAction;
use super::orderedit::StagedOrderChangePaymentStateAction;
use super::orderedit::StagedOrderChangeShipmentStateAction;
use super::orderedit::StagedOrderChangeTaxCalculationModeAction;
use super::orderedit::StagedOrderChangeTaxModeAction;
use super::orderedit::StagedOrderChangeTaxRoundingModeAction;
use super::orderedit::StagedOrderImportCustomLineItemStateAction;
use super::orderedit::StagedOrderImportLineItemStateAction;
use super::orderedit::StagedOrderRemoveCustomLineItemAction;
use super::orderedit::StagedOrderRemoveDeliveryAction;
use super::orderedit::StagedOrderRemoveDiscountCodeAction;
use super::orderedit::StagedOrderRemoveItemShippingAddressAction;
use super::orderedit::StagedOrderRemoveLineItemAction;
use super::orderedit::StagedOrderRemoveParcelFromDeliveryAction;
use super::orderedit::StagedOrderRemovePaymentAction;
use super::orderedit::StagedOrderSetBillingAddressAction;
use super::orderedit::StagedOrderSetCountryAction;
use super::orderedit::StagedOrderSetCustomFieldAction;
use super::orderedit::StagedOrderSetCustomLineItemCustomFieldAction;
use super::orderedit::StagedOrderSetCustomLineItemCustomTypeAction;
use super::orderedit::StagedOrderSetCustomLineItemShippingDetailsAction;
use super::orderedit::StagedOrderSetCustomLineItemTaxAmountAction;
use super::orderedit::StagedOrderSetCustomLineItemTaxRateAction;
use super::orderedit::StagedOrderSetCustomShippingMethodAction;
use super::orderedit::StagedOrderSetCustomTypeAction;
use super::orderedit::StagedOrderSetCustomerEmailAction;
use super::orderedit::StagedOrderSetCustomerGroupAction;
use super::orderedit::StagedOrderSetCustomerIdAction;
use super::orderedit::StagedOrderSetDeliveryAddressAction;
use super::orderedit::StagedOrderSetDeliveryItemsAction;
use super::orderedit::StagedOrderSetLineItemCustomFieldAction;
use super::orderedit::StagedOrderSetLineItemCustomTypeAction;
use super::orderedit::StagedOrderSetLineItemPriceAction;
use super::orderedit::StagedOrderSetLineItemShippingDetailsAction;
use super::orderedit::StagedOrderSetLineItemTaxAmountAction;
use super::orderedit::StagedOrderSetLineItemTaxRateAction;
use super::orderedit::StagedOrderSetLineItemTotalPriceAction;
use super::orderedit::StagedOrderSetLocaleAction;
use super::orderedit::StagedOrderSetOrderNumberAction;
use super::orderedit::StagedOrderSetOrderTotalTaxAction;
use super::orderedit::StagedOrderSetParcelItemsAction;
use super::orderedit::StagedOrderSetParcelMeasurementsAction;
use super::orderedit::StagedOrderSetParcelTrackingDataAction;
use super::orderedit::StagedOrderSetReturnPaymentStateAction;
use super::orderedit::StagedOrderSetReturnShipmentStateAction;
use super::orderedit::StagedOrderSetShippingAddressAction;
use super::orderedit::StagedOrderSetShippingAddressAndCustomShippingMethodAction;
use super::orderedit::StagedOrderSetShippingAddressAndShippingMethodAction;
use super::orderedit::StagedOrderSetShippingMethodAction;
use super::orderedit::StagedOrderSetShippingMethodTaxAmountAction;
use super::orderedit::StagedOrderSetShippingMethodTaxRateAction;
use super::orderedit::StagedOrderSetShippingRateInputAction;
use super::orderedit::StagedOrderTransitionCustomLineItemStateAction;
use super::orderedit::StagedOrderTransitionLineItemStateAction;
use super::orderedit::StagedOrderTransitionStateAction;
use super::orderedit::StagedOrderUpdateItemShippingAddressAction;
use super::orderedit::StagedOrderUpdateSyncInfoAction;
use super::common::Address;
use super::common::Money;
use super::cart::DiscountedLineItemPortion;
use super::state::StateReference;
use super::common::LocalizedString;
use super::common::PriceDraft;
use super::channel::ChannelReference;
use super::taxcategory::TaxRate;
use super::cttype::CustomFieldsDraft;
use super::cart::ItemShippingDetailsDraft;
use super::cart::LineItem;
use super::cart::CustomLineItem;
use super::cart::TaxedPrice;
use super::cart::TaxMode;
use super::cart::RoundingMode;
use super::customergroup::CustomerGroupReference;
use super::cart::ShippingInfo;
use super::cart::DiscountCodeInfo;
use super::cart::CartReference;
use super::cttype::CustomFields;
use super::cart::InventoryMode;
use super::cart::CartOrigin;
use super::cart::TaxCalculationMode;
use super::cart::ShippingRateInput;
use super::orderedit::StagedOrder;
use super::common::Resource;
use super::cart::CustomLineItemDraft;
use super::common::Reference;
use super::payment::PaymentReference;
use super::common::Price;
use super::product::Attribute;
use super::common::Image;
use super::shippingmethod::ShippingRateDraft;
use super::taxcategory::TaxCategoryReference;
use super::shippingmethod::ShippingMethodReference;
use super::cart::ShippingMethodState;
use super::cttype::TypeReference;
use super::cttype::FieldContainer;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum StagedOrderUpdateAction {
   #[serde(rename = "addCustomLineItem")]
   EStagedOrderAddCustomLineItemAction(StagedOrderAddCustomLineItemAction),
   #[serde(rename = "addDelivery")]
   EStagedOrderAddDeliveryAction(StagedOrderAddDeliveryAction),
   #[serde(rename = "addDiscountCode")]
   EStagedOrderAddDiscountCodeAction(StagedOrderAddDiscountCodeAction),
   #[serde(rename = "addItemShippingAddress")]
   EStagedOrderAddItemShippingAddressAction(StagedOrderAddItemShippingAddressAction),
   #[serde(rename = "addLineItem")]
   EStagedOrderAddLineItemAction(StagedOrderAddLineItemAction),
   #[serde(rename = "addParcelToDelivery")]
   EStagedOrderAddParcelToDeliveryAction(StagedOrderAddParcelToDeliveryAction),
   #[serde(rename = "addPayment")]
   EStagedOrderAddPaymentAction(StagedOrderAddPaymentAction),
   #[serde(rename = "addReturnInfo")]
   EStagedOrderAddReturnInfoAction(StagedOrderAddReturnInfoAction),
   #[serde(rename = "addShoppingList")]
   EStagedOrderAddShoppingListAction(StagedOrderAddShoppingListAction),
   #[serde(rename = "changeCustomLineItemMoney")]
   EStagedOrderChangeCustomLineItemMoneyAction(StagedOrderChangeCustomLineItemMoneyAction),
   #[serde(rename = "changeCustomLineItemQuantity")]
   EStagedOrderChangeCustomLineItemQuantityAction(StagedOrderChangeCustomLineItemQuantityAction),
   #[serde(rename = "changeLineItemQuantity")]
   EStagedOrderChangeLineItemQuantityAction(StagedOrderChangeLineItemQuantityAction),
   #[serde(rename = "changeOrderState")]
   EStagedOrderChangeOrderStateAction(StagedOrderChangeOrderStateAction),
   #[serde(rename = "changePaymentState")]
   EStagedOrderChangePaymentStateAction(StagedOrderChangePaymentStateAction),
   #[serde(rename = "changeShipmentState")]
   EStagedOrderChangeShipmentStateAction(StagedOrderChangeShipmentStateAction),
   #[serde(rename = "changeTaxCalculationMode")]
   EStagedOrderChangeTaxCalculationModeAction(StagedOrderChangeTaxCalculationModeAction),
   #[serde(rename = "changeTaxMode")]
   EStagedOrderChangeTaxModeAction(StagedOrderChangeTaxModeAction),
   #[serde(rename = "changeTaxRoundingMode")]
   EStagedOrderChangeTaxRoundingModeAction(StagedOrderChangeTaxRoundingModeAction),
   #[serde(rename = "importCustomLineItemState")]
   EStagedOrderImportCustomLineItemStateAction(StagedOrderImportCustomLineItemStateAction),
   #[serde(rename = "importLineItemState")]
   EStagedOrderImportLineItemStateAction(StagedOrderImportLineItemStateAction),
   #[serde(rename = "removeCustomLineItem")]
   EStagedOrderRemoveCustomLineItemAction(StagedOrderRemoveCustomLineItemAction),
   #[serde(rename = "removeDelivery")]
   EStagedOrderRemoveDeliveryAction(StagedOrderRemoveDeliveryAction),
   #[serde(rename = "removeDiscountCode")]
   EStagedOrderRemoveDiscountCodeAction(StagedOrderRemoveDiscountCodeAction),
   #[serde(rename = "removeItemShippingAddress")]
   EStagedOrderRemoveItemShippingAddressAction(StagedOrderRemoveItemShippingAddressAction),
   #[serde(rename = "removeLineItem")]
   EStagedOrderRemoveLineItemAction(StagedOrderRemoveLineItemAction),
   #[serde(rename = "removeParcelFromDelivery")]
   EStagedOrderRemoveParcelFromDeliveryAction(StagedOrderRemoveParcelFromDeliveryAction),
   #[serde(rename = "removePayment")]
   EStagedOrderRemovePaymentAction(StagedOrderRemovePaymentAction),
   #[serde(rename = "setBillingAddress")]
   EStagedOrderSetBillingAddressAction(StagedOrderSetBillingAddressAction),
   #[serde(rename = "setCountry")]
   EStagedOrderSetCountryAction(StagedOrderSetCountryAction),
   #[serde(rename = "setCustomField")]
   EStagedOrderSetCustomFieldAction(StagedOrderSetCustomFieldAction),
   #[serde(rename = "setCustomLineItemCustomField")]
   EStagedOrderSetCustomLineItemCustomFieldAction(StagedOrderSetCustomLineItemCustomFieldAction),
   #[serde(rename = "setCustomLineItemCustomType")]
   EStagedOrderSetCustomLineItemCustomTypeAction(StagedOrderSetCustomLineItemCustomTypeAction),
   #[serde(rename = "setCustomLineItemShippingDetails")]
   EStagedOrderSetCustomLineItemShippingDetailsAction(StagedOrderSetCustomLineItemShippingDetailsAction),
   #[serde(rename = "setCustomLineItemTaxAmount")]
   EStagedOrderSetCustomLineItemTaxAmountAction(StagedOrderSetCustomLineItemTaxAmountAction),
   #[serde(rename = "setCustomLineItemTaxRate")]
   EStagedOrderSetCustomLineItemTaxRateAction(StagedOrderSetCustomLineItemTaxRateAction),
   #[serde(rename = "setCustomShippingMethod")]
   EStagedOrderSetCustomShippingMethodAction(StagedOrderSetCustomShippingMethodAction),
   #[serde(rename = "setCustomType")]
   EStagedOrderSetCustomTypeAction(StagedOrderSetCustomTypeAction),
   #[serde(rename = "setCustomerEmail")]
   EStagedOrderSetCustomerEmailAction(StagedOrderSetCustomerEmailAction),
   #[serde(rename = "setCustomerGroup")]
   EStagedOrderSetCustomerGroupAction(StagedOrderSetCustomerGroupAction),
   #[serde(rename = "setCustomerId")]
   EStagedOrderSetCustomerIdAction(StagedOrderSetCustomerIdAction),
   #[serde(rename = "setDeliveryAddress")]
   EStagedOrderSetDeliveryAddressAction(StagedOrderSetDeliveryAddressAction),
   #[serde(rename = "setDeliveryItems")]
   EStagedOrderSetDeliveryItemsAction(StagedOrderSetDeliveryItemsAction),
   #[serde(rename = "setLineItemCustomField")]
   EStagedOrderSetLineItemCustomFieldAction(StagedOrderSetLineItemCustomFieldAction),
   #[serde(rename = "setLineItemCustomType")]
   EStagedOrderSetLineItemCustomTypeAction(StagedOrderSetLineItemCustomTypeAction),
   #[serde(rename = "setLineItemPrice")]
   EStagedOrderSetLineItemPriceAction(StagedOrderSetLineItemPriceAction),
   #[serde(rename = "setLineItemShippingDetails")]
   EStagedOrderSetLineItemShippingDetailsAction(StagedOrderSetLineItemShippingDetailsAction),
   #[serde(rename = "setLineItemTaxAmount")]
   EStagedOrderSetLineItemTaxAmountAction(StagedOrderSetLineItemTaxAmountAction),
   #[serde(rename = "setLineItemTaxRate")]
   EStagedOrderSetLineItemTaxRateAction(StagedOrderSetLineItemTaxRateAction),
   #[serde(rename = "setLineItemTotalPrice")]
   EStagedOrderSetLineItemTotalPriceAction(StagedOrderSetLineItemTotalPriceAction),
   #[serde(rename = "setLocale")]
   EStagedOrderSetLocaleAction(StagedOrderSetLocaleAction),
   #[serde(rename = "setOrderNumber")]
   EStagedOrderSetOrderNumberAction(StagedOrderSetOrderNumberAction),
   #[serde(rename = "setOrderTotalTax")]
   EStagedOrderSetOrderTotalTaxAction(StagedOrderSetOrderTotalTaxAction),
   #[serde(rename = "setParcelItems")]
   EStagedOrderSetParcelItemsAction(StagedOrderSetParcelItemsAction),
   #[serde(rename = "setParcelMeasurements")]
   EStagedOrderSetParcelMeasurementsAction(StagedOrderSetParcelMeasurementsAction),
   #[serde(rename = "setParcelTrackingData")]
   EStagedOrderSetParcelTrackingDataAction(StagedOrderSetParcelTrackingDataAction),
   #[serde(rename = "setReturnPaymentState")]
   EStagedOrderSetReturnPaymentStateAction(StagedOrderSetReturnPaymentStateAction),
   #[serde(rename = "setReturnShipmentState")]
   EStagedOrderSetReturnShipmentStateAction(StagedOrderSetReturnShipmentStateAction),
   #[serde(rename = "setShippingAddress")]
   EStagedOrderSetShippingAddressAction(StagedOrderSetShippingAddressAction),
   #[serde(rename = "setShippingAddressAndCustomShippingMethod")]
   EStagedOrderSetShippingAddressAndCustomShippingMethodAction(StagedOrderSetShippingAddressAndCustomShippingMethodAction),
   #[serde(rename = "setShippingAddressAndShippingMethod")]
   EStagedOrderSetShippingAddressAndShippingMethodAction(StagedOrderSetShippingAddressAndShippingMethodAction),
   #[serde(rename = "setShippingMethod")]
   EStagedOrderSetShippingMethodAction(StagedOrderSetShippingMethodAction),
   #[serde(rename = "setShippingMethodTaxAmount")]
   EStagedOrderSetShippingMethodTaxAmountAction(StagedOrderSetShippingMethodTaxAmountAction),
   #[serde(rename = "setShippingMethodTaxRate")]
   EStagedOrderSetShippingMethodTaxRateAction(StagedOrderSetShippingMethodTaxRateAction),
   #[serde(rename = "setShippingRateInput")]
   EStagedOrderSetShippingRateInputAction(StagedOrderSetShippingRateInputAction),
   #[serde(rename = "transitionCustomLineItemState")]
   EStagedOrderTransitionCustomLineItemStateAction(StagedOrderTransitionCustomLineItemStateAction),
   #[serde(rename = "transitionLineItemState")]
   EStagedOrderTransitionLineItemStateAction(StagedOrderTransitionLineItemStateAction),
   #[serde(rename = "transitionState")]
   EStagedOrderTransitionStateAction(StagedOrderTransitionStateAction),
   #[serde(rename = "updateItemShippingAddress")]
   EStagedOrderUpdateItemShippingAddressAction(StagedOrderUpdateItemShippingAddressAction),
   #[serde(rename = "updateSyncInfo")]
   EStagedOrderUpdateSyncInfoAction(StagedOrderUpdateSyncInfoAction),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomLineItemReturnItem {
   pub payment_state: ReturnPaymentState,
   pub shipment_state: ReturnShipmentState,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub quantity: u64,
   pub custom_line_item_id: String,
   pub id: String,
   pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Delivery {
   pub items: Vec<DeliveryItem>,
   pub parcels: Vec<Parcel>,
   pub created_at: DateTime<Utc>,
   pub id: String,
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryItem {
   pub quantity: u32,
   pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountedLineItemPriceDraft {
   pub included_discounts: Vec<DiscountedLineItemPortion>,
   pub value: Money,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemState {
   pub state: StateReference,
   pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineItemImportDraft {
   pub name: LocalizedString,
   pub price: PriceDraft,
   pub variant: ProductVariantImportDraft,
   pub quantity: u32,
   pub state: Option<Vec<ItemState>>,
   pub distribution_channel: Option<ChannelReference>,
   pub supply_channel: Option<ChannelReference>,
   pub custom: Option<CustomFieldsDraft>,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
   pub tax_rate: Option<TaxRate>,
   pub product_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineItemReturnItem {
   pub payment_state: ReturnPaymentState,
   pub shipment_state: ReturnShipmentState,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub quantity: u64,
   pub id: String,
   pub line_item_id: String,
   pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
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
pub struct OrderFromCartDraft {
   pub version: u64,
   pub id: String,
   pub payment_state: Option<PaymentState>,
   pub order_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderImportDraft {
   pub total_price: Money,
   pub custom_line_items: Option<Vec<CustomLineItemDraft>>,
   pub item_shipping_addresses: Option<Vec<Address>>,
   pub line_items: Option<Vec<LineItemImportDraft>>,
   pub billing_address: Option<Address>,
   pub shipping_address: Option<Address>,
   pub custom: Option<CustomFieldsDraft>,
   pub customer_group: Option<CustomerGroupReference>,
   pub inventory_mode: Option<InventoryMode>,
   pub order_state: Option<OrderState>,
   pub payment_state: Option<PaymentState>,
   pub tax_rounding_mode: Option<RoundingMode>,
   pub shipment_state: Option<ShipmentState>,
   pub shipping_info: Option<ShippingInfoImportDraft>,
   pub taxed_price: Option<TaxedPrice>,
   pub completed_at: Option<DateTime<Utc>>,
   pub country: Option<String>,
   pub customer_email: Option<String>,
   pub customer_id: Option<String>,
   pub order_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderPagedQueryResponse {
   pub results: Vec<Order>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderReference {
   pub obj: Option<Order>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderState {
   Open,
   Confirmed,
   Complete,
   Cancelled
}

impl OrderState {
    pub fn from_str(s: &str) -> Option<OrderState> {
        match s {
            "Open" => Some(OrderState::Open),
            "Confirmed" => Some(OrderState::Confirmed),
            "Complete" => Some(OrderState::Complete),
            "Cancelled" => Some(OrderState::Cancelled),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           OrderState::Open => "Open",
           OrderState::Confirmed => "Confirmed",
           OrderState::Complete => "Complete",
           OrderState::Cancelled => "Cancelled",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdate {
   pub actions: Vec<OrderUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum OrderUpdateAction {
   #[serde(rename = "addDelivery")]
   EOrderAddDeliveryAction(OrderAddDeliveryAction),
   #[serde(rename = "addItemShippingAddress")]
   EOrderAddItemShippingAddressAction(OrderAddItemShippingAddressAction),
   #[serde(rename = "addParcelToDelivery")]
   EOrderAddParcelToDeliveryAction(OrderAddParcelToDeliveryAction),
   #[serde(rename = "addPayment")]
   EOrderAddPaymentAction(OrderAddPaymentAction),
   #[serde(rename = "addReturnInfo")]
   EOrderAddReturnInfoAction(OrderAddReturnInfoAction),
   #[serde(rename = "changeOrderState")]
   EOrderChangeOrderStateAction(OrderChangeOrderStateAction),
   #[serde(rename = "changePaymentState")]
   EOrderChangePaymentStateAction(OrderChangePaymentStateAction),
   #[serde(rename = "changeShipmentState")]
   EOrderChangeShipmentStateAction(OrderChangeShipmentStateAction),
   #[serde(rename = "importCustomLineItemState")]
   EOrderImportCustomLineItemStateAction(OrderImportCustomLineItemStateAction),
   #[serde(rename = "importLineItemState")]
   EOrderImportLineItemStateAction(OrderImportLineItemStateAction),
   #[serde(rename = "removeDelivery")]
   EOrderRemoveDeliveryAction(OrderRemoveDeliveryAction),
   #[serde(rename = "removeItemShippingAddress")]
   EOrderRemoveItemShippingAddressAction(OrderRemoveItemShippingAddressAction),
   #[serde(rename = "removeParcelFromDelivery")]
   EOrderRemoveParcelFromDeliveryAction(OrderRemoveParcelFromDeliveryAction),
   #[serde(rename = "removePayment")]
   EOrderRemovePaymentAction(OrderRemovePaymentAction),
   #[serde(rename = "setBillingAddress")]
   EOrderSetBillingAddressAction(OrderSetBillingAddressAction),
   #[serde(rename = "setCustomField")]
   EOrderSetCustomFieldAction(OrderSetCustomFieldAction),
   #[serde(rename = "setCustomLineItemCustomField")]
   EOrderSetCustomLineItemCustomFieldAction(OrderSetCustomLineItemCustomFieldAction),
   #[serde(rename = "setCustomLineItemCustomType")]
   EOrderSetCustomLineItemCustomTypeAction(OrderSetCustomLineItemCustomTypeAction),
   #[serde(rename = "setCustomLineItemShippingDetails")]
   EOrderSetCustomLineItemShippingDetailsAction(OrderSetCustomLineItemShippingDetailsAction),
   #[serde(rename = "setCustomType")]
   EOrderSetCustomTypeAction(OrderSetCustomTypeAction),
   #[serde(rename = "setCustomerEmail")]
   EOrderSetCustomerEmailAction(OrderSetCustomerEmailAction),
   #[serde(rename = "setCustomerId")]
   EOrderSetCustomerIdAction(OrderSetCustomerIdAction),
   #[serde(rename = "setDeliveryAddress")]
   EOrderSetDeliveryAddressAction(OrderSetDeliveryAddressAction),
   #[serde(rename = "setDeliveryItems")]
   EOrderSetDeliveryItemsAction(OrderSetDeliveryItemsAction),
   #[serde(rename = "setLineItemCustomField")]
   EOrderSetLineItemCustomFieldAction(OrderSetLineItemCustomFieldAction),
   #[serde(rename = "setLineItemCustomType")]
   EOrderSetLineItemCustomTypeAction(OrderSetLineItemCustomTypeAction),
   #[serde(rename = "setLineItemShippingDetails")]
   EOrderSetLineItemShippingDetailsAction(OrderSetLineItemShippingDetailsAction),
   #[serde(rename = "setLocale")]
   EOrderSetLocaleAction(OrderSetLocaleAction),
   #[serde(rename = "setOrderNumber")]
   EOrderSetOrderNumberAction(OrderSetOrderNumberAction),
   #[serde(rename = "setParcelItems")]
   EOrderSetParcelItemsAction(OrderSetParcelItemsAction),
   #[serde(rename = "setParcelMeasurements")]
   EOrderSetParcelMeasurementsAction(OrderSetParcelMeasurementsAction),
   #[serde(rename = "setParcelTrackingData")]
   EOrderSetParcelTrackingDataAction(OrderSetParcelTrackingDataAction),
   #[serde(rename = "setReturnPaymentState")]
   EOrderSetReturnPaymentStateAction(OrderSetReturnPaymentStateAction),
   #[serde(rename = "setReturnShipmentState")]
   EOrderSetReturnShipmentStateAction(OrderSetReturnShipmentStateAction),
   #[serde(rename = "setShippingAddress")]
   EOrderSetShippingAddressAction(OrderSetShippingAddressAction),
   #[serde(rename = "transitionCustomLineItemState")]
   EOrderTransitionCustomLineItemStateAction(OrderTransitionCustomLineItemStateAction),
   #[serde(rename = "transitionLineItemState")]
   EOrderTransitionLineItemStateAction(OrderTransitionLineItemStateAction),
   #[serde(rename = "transitionState")]
   EOrderTransitionStateAction(OrderTransitionStateAction),
   #[serde(rename = "updateItemShippingAddress")]
   EOrderUpdateItemShippingAddressAction(OrderUpdateItemShippingAddressAction),
   #[serde(rename = "updateSyncInfo")]
   EOrderUpdateSyncInfoAction(OrderUpdateSyncInfoAction),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Parcel {
   pub created_at: DateTime<Utc>,
   pub id: String,
   pub items: Option<Vec<DeliveryItem>>,
   pub measurements: Option<ParcelMeasurements>,
   pub tracking_data: Option<TrackingData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelDraft {
   pub items: Option<Vec<DeliveryItem>>,
   pub measurements: Option<ParcelMeasurements>,
   pub tracking_data: Option<TrackingData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParcelMeasurements {
   pub height_in_millimeter: Option<u32>,
   pub length_in_millimeter: Option<u32>,
   pub weight_in_gram: Option<u32>,
   pub width_in_millimeter: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInfo {
   pub payments: Vec<PaymentReference>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PaymentState {
   BalanceDue,
   Failed,
   Pending,
   CreditOwed,
   Paid
}

impl PaymentState {
    pub fn from_str(s: &str) -> Option<PaymentState> {
        match s {
            "BalanceDue" => Some(PaymentState::BalanceDue),
            "Failed" => Some(PaymentState::Failed),
            "Pending" => Some(PaymentState::Pending),
            "CreditOwed" => Some(PaymentState::CreditOwed),
            "Paid" => Some(PaymentState::Paid),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           PaymentState::BalanceDue => "BalanceDue",
           PaymentState::Failed => "Failed",
           PaymentState::Pending => "Pending",
           PaymentState::CreditOwed => "CreditOwed",
           PaymentState::Paid => "Paid",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductVariantImportDraft {
   pub attributes: Option<Vec<Attribute>>,
   pub images: Option<Vec<Image>>,
   pub prices: Option<Vec<Price>>,
   pub id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReturnInfo {
   pub items: Vec<ReturnItem>,
   pub return_date: Option<DateTime<Utc>>,
   pub return_tracking_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ReturnItem {
   #[serde(rename = "LineItemReturnItem")]
   ELineItemReturnItem(LineItemReturnItem),
   #[serde(rename = "CustomLineItemReturnItem")]
   ECustomLineItemReturnItem(CustomLineItemReturnItem),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReturnItemDraft {
   pub shipment_state: ReturnShipmentState,
   pub quantity: u64,
   pub comment: Option<String>,
   pub custom_line_item_id: Option<String>,
   pub line_item_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReturnPaymentState {
   NonRefundable,
   Initial,
   Refunded,
   NotRefunded
}

impl ReturnPaymentState {
    pub fn from_str(s: &str) -> Option<ReturnPaymentState> {
        match s {
            "NonRefundable" => Some(ReturnPaymentState::NonRefundable),
            "Initial" => Some(ReturnPaymentState::Initial),
            "Refunded" => Some(ReturnPaymentState::Refunded),
            "NotRefunded" => Some(ReturnPaymentState::NotRefunded),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ReturnPaymentState::NonRefundable => "NonRefundable",
           ReturnPaymentState::Initial => "Initial",
           ReturnPaymentState::Refunded => "Refunded",
           ReturnPaymentState::NotRefunded => "NotRefunded",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReturnShipmentState {
   Advised,
   Returned,
   BackInStock,
   Unusable
}

impl ReturnShipmentState {
    pub fn from_str(s: &str) -> Option<ReturnShipmentState> {
        match s {
            "Advised" => Some(ReturnShipmentState::Advised),
            "Returned" => Some(ReturnShipmentState::Returned),
            "BackInStock" => Some(ReturnShipmentState::BackInStock),
            "Unusable" => Some(ReturnShipmentState::Unusable),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ReturnShipmentState::Advised => "Advised",
           ReturnShipmentState::Returned => "Returned",
           ReturnShipmentState::BackInStock => "BackInStock",
           ReturnShipmentState::Unusable => "Unusable",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShipmentState {
   Shipped,
   Ready,
   Pending,
   Delayed,
   Partial,
   Backorder
}

impl ShipmentState {
    pub fn from_str(s: &str) -> Option<ShipmentState> {
        match s {
            "Shipped" => Some(ShipmentState::Shipped),
            "Ready" => Some(ShipmentState::Ready),
            "Pending" => Some(ShipmentState::Pending),
            "Delayed" => Some(ShipmentState::Delayed),
            "Partial" => Some(ShipmentState::Partial),
            "Backorder" => Some(ShipmentState::Backorder),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ShipmentState::Shipped => "Shipped",
           ShipmentState::Ready => "Ready",
           ShipmentState::Pending => "Pending",
           ShipmentState::Delayed => "Delayed",
           ShipmentState::Partial => "Partial",
           ShipmentState::Backorder => "Backorder",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingInfoImportDraft {
   pub price: Money,
   pub shipping_rate: ShippingRateDraft,
   pub shipping_method_name: String,
   pub deliveries: Option<Vec<Delivery>>,
   pub discounted_price: Option<DiscountedLineItemPriceDraft>,
   pub shipping_method: Option<ShippingMethodReference>,
   pub shipping_method_state: Option<ShippingMethodState>,
   pub tax_category: Option<TaxCategoryReference>,
   pub tax_rate: Option<TaxRate>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyncInfo {
   pub channel: ChannelReference,
   pub synced_at: DateTime<Utc>,
   pub external_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaxedItemPriceDraft {
   pub total_gross: Money,
   pub total_net: Money,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrackingData {
   pub is_return: Option<bool>,
   pub carrier: Option<String>,
   pub provider: Option<String>,
   pub provider_transaction: Option<String>,
   pub tracking_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderAddDeliveryAction {
   pub items: Option<Vec<DeliveryItem>>,
   pub parcels: Option<Vec<ParcelDraft>>,
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderAddItemShippingAddressAction {
   pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderAddParcelToDeliveryAction {
   pub delivery_id: String,
   pub items: Option<Vec<DeliveryItem>>,
   pub measurements: Option<ParcelMeasurements>,
   pub tracking_data: Option<TrackingData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderAddPaymentAction {
   pub payment: PaymentReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderAddReturnInfoAction {
   pub items: Vec<ReturnItemDraft>,
   pub return_date: Option<DateTime<Utc>>,
   pub return_tracking_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderChangeOrderStateAction {
   pub order_state: OrderState,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderChangePaymentStateAction {
   pub payment_state: Option<PaymentState>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderChangeShipmentStateAction {
   pub shipment_state: Option<ShipmentState>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderImportCustomLineItemStateAction {
   pub state: Vec<ItemState>,
   pub custom_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderImportLineItemStateAction {
   pub state: Vec<ItemState>,
   pub line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderRemoveDeliveryAction {
   pub delivery_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderRemoveItemShippingAddressAction {
   pub address_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderRemoveParcelFromDeliveryAction {
   pub parcel_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderRemovePaymentAction {
   pub payment: PaymentReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetBillingAddressAction {
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetCustomLineItemCustomFieldAction {
   pub custom_line_item_id: String,
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetCustomLineItemCustomTypeAction {
   pub custom_line_item_id: String,
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetCustomLineItemShippingDetailsAction {
   pub custom_line_item_id: String,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetCustomerEmailAction {
   pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetCustomerIdAction {
   pub customer_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetDeliveryAddressAction {
   pub delivery_id: String,
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetDeliveryItemsAction {
   pub items: Vec<DeliveryItem>,
   pub delivery_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetLineItemCustomFieldAction {
   pub line_item_id: String,
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetLineItemCustomTypeAction {
   pub line_item_id: String,
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetLineItemShippingDetailsAction {
   pub line_item_id: String,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetLocaleAction {
   pub locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetOrderNumberAction {
   pub order_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetParcelItemsAction {
   pub items: Vec<DeliveryItem>,
   pub parcel_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetParcelMeasurementsAction {
   pub parcel_id: String,
   pub measurements: Option<ParcelMeasurements>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetParcelTrackingDataAction {
   pub parcel_id: String,
   pub tracking_data: Option<TrackingData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetReturnPaymentStateAction {
   pub payment_state: ReturnPaymentState,
   pub return_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetReturnShipmentStateAction {
   pub shipment_state: ReturnShipmentState,
   pub return_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSetShippingAddressAction {
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderTransitionCustomLineItemStateAction {
   pub from_state: StateReference,
   pub to_state: StateReference,
   pub quantity: u64,
   pub custom_line_item_id: String,
   pub actual_transition_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderTransitionLineItemStateAction {
   pub from_state: StateReference,
   pub to_state: StateReference,
   pub quantity: u64,
   pub line_item_id: String,
   pub actual_transition_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderTransitionStateAction {
   pub state: StateReference,
   pub force: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdateItemShippingAddressAction {
   pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdateSyncInfoAction {
   pub channel: ChannelReference,
   pub synced_at: Option<DateTime<Utc>>,
   pub external_id: Option<String>,
}