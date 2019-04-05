//Generated file, please do not change

use super::common::TypedMoney;
use super::common::Address;
use super::customergroup::CustomerGroupReference;
use super::cttype::CustomFields;
use super::order::PaymentInfo;
use super::cartdiscount::CartDiscountReference;
use super::common::Resource;
use super::shippingmethod::ShippingMethodReference;
use super::cttype::CustomFieldsDraft;
use super::common::Reference;
use super::common::LocalizedString;
use super::order::ItemState;
use super::taxcategory::TaxCategoryReference;
use super::taxcategory::TaxRate;
use super::common::Money;
use super::discountcode::DiscountCodeReference;
use super::taxcategory::SubRate;
use super::producttype::ProductTypeReference;
use super::product::ProductVariant;
use super::common::Price;
use super::channel::ChannelReference;
use super::shippingmethod::ShippingRate;
use super::order::Delivery;
use super::payment::PaymentReference;
use super::shoppinglist::ShoppingListReference;
use super::cttype::TypeReference;
use super::cttype::FieldContainer;
use super::shippingmethod::ShippingRateDraft;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cart {
   pub custom_line_items: Vec<CustomLineItem>,
   pub line_items: Vec<LineItem>,
   pub refused_gifts: Vec<CartDiscountReference>,
   pub origin: CartOrigin,
   pub cart_state: CartState,
   pub tax_rounding_mode: RoundingMode,
   pub tax_calculation_mode: TaxCalculationMode,
   pub tax_mode: TaxMode,
   pub total_price: TypedMoney,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub discount_codes: Option<Vec<DiscountCodeInfo>>,
   pub item_shipping_addresses: Option<Vec<Address>>,
   pub billing_address: Option<Address>,
   pub shipping_address: Option<Address>,
   /**
   	<p>A two-digit country code as per <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a>.</p>
   */
   pub country: Option<String>,
   pub custom: Option<CustomFields>,
   pub customer_group: Option<CustomerGroupReference>,
   pub inventory_mode: Option<InventoryMode>,
   pub payment_info: Option<PaymentInfo>,
   pub shipping_info: Option<ShippingInfo>,
   pub shipping_rate_input: Option<ShippingRateInput>,
   pub taxed_price: Option<TaxedPrice>,
   pub delete_days_after_last_modification: Option<u32>,
   pub anonymous_id: Option<String>,
   pub customer_email: Option<String>,
   pub customer_id: Option<String>,
   pub locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDraft {
   /**
   	<p>The currency code compliant to <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>.</p>
   */
   pub currency: String,
   pub custom_line_items: Option<Vec<CustomLineItemDraft>>,
   pub item_shipping_addresses: Option<Vec<Address>>,
   pub line_items: Option<Vec<LineItemDraft>>,
   pub billing_address: Option<Address>,
   pub shipping_address: Option<Address>,
   pub origin: Option<CartOrigin>,
   pub custom: Option<CustomFieldsDraft>,
   pub customer_group: Option<CustomerGroupReference>,
   pub external_tax_rate_for_shipping_method: Option<ExternalTaxRateDraft>,
   pub inventory_mode: Option<InventoryMode>,
   pub tax_rounding_mode: Option<RoundingMode>,
   pub shipping_method: Option<ShippingMethodReference>,
   pub shipping_rate_input: Option<ShippingRateInputDraft>,
   pub tax_calculation_mode: Option<TaxCalculationMode>,
   pub tax_mode: Option<TaxMode>,
   pub delete_days_after_last_modification: Option<u64>,
   pub anonymous_id: Option<String>,
   pub country: Option<String>,
   pub customer_email: Option<String>,
   pub customer_id: Option<String>,
   pub locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CartOrigin {
   Customer,
   Merchant
}

impl CartOrigin {
    pub fn from_str(s: &str) -> Option<CartOrigin> {
        match s {
            "Customer" => Some(CartOrigin::Customer),
            "Merchant" => Some(CartOrigin::Merchant),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           CartOrigin::Customer => "Customer",
           CartOrigin::Merchant => "Merchant",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartPagedQueryResponse {
   pub results: Vec<Cart>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartReference {
   pub obj: Option<Cart>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CartState {
   Active,
   Merged,
   Ordered
}

impl CartState {
    pub fn from_str(s: &str) -> Option<CartState> {
        match s {
            "Active" => Some(CartState::Active),
            "Merged" => Some(CartState::Merged),
            "Ordered" => Some(CartState::Ordered),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           CartState::Active => "Active",
           CartState::Merged => "Merged",
           CartState::Ordered => "Ordered",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartUpdate {
   pub actions: Vec<CartUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum CartUpdateAction {
   #[serde(rename = "addCustomLineItem")]
   ECartAddCustomLineItemAction(CartAddCustomLineItemAction),
   #[serde(rename = "addDiscountCode")]
   ECartAddDiscountCodeAction(CartAddDiscountCodeAction),
   #[serde(rename = "addItemShippingAddress")]
   ECartAddItemShippingAddressAction(CartAddItemShippingAddressAction),
   #[serde(rename = "addLineItem")]
   ECartAddLineItemAction(CartAddLineItemAction),
   #[serde(rename = "addPayment")]
   ECartAddPaymentAction(CartAddPaymentAction),
   #[serde(rename = "addShoppingList")]
   ECartAddShoppingListAction(CartAddShoppingListAction),
   #[serde(rename = "applyDeltaToCustomLineItemShippingDetailsTargets")]
   ECartApplyDeltaToCustomLineItemShippingDetailsTargetsAction(CartApplyDeltaToCustomLineItemShippingDetailsTargetsAction),
   #[serde(rename = "applyDeltaToLineItemShippingDetailsTargets")]
   ECartApplyDeltaToLineItemShippingDetailsTargetsAction(CartApplyDeltaToLineItemShippingDetailsTargetsAction),
   #[serde(rename = "changeCustomLineItemMoney")]
   ECartChangeCustomLineItemMoneyAction(CartChangeCustomLineItemMoneyAction),
   #[serde(rename = "changeCustomLineItemQuantity")]
   ECartChangeCustomLineItemQuantityAction(CartChangeCustomLineItemQuantityAction),
   #[serde(rename = "changeLineItemQuantity")]
   ECartChangeLineItemQuantityAction(CartChangeLineItemQuantityAction),
   #[serde(rename = "changeTaxCalculationMode")]
   ECartChangeTaxCalculationModeAction(CartChangeTaxCalculationModeAction),
   #[serde(rename = "changeTaxMode")]
   ECartChangeTaxModeAction(CartChangeTaxModeAction),
   #[serde(rename = "changeTaxRoundingMode")]
   ECartChangeTaxRoundingModeAction(CartChangeTaxRoundingModeAction),
   #[serde(rename = "recalculate")]
   ECartRecalculateAction(CartRecalculateAction),
   #[serde(rename = "removeCustomLineItem")]
   ECartRemoveCustomLineItemAction(CartRemoveCustomLineItemAction),
   #[serde(rename = "removeDiscountCode")]
   ECartRemoveDiscountCodeAction(CartRemoveDiscountCodeAction),
   #[serde(rename = "removeItemShippingAddress")]
   ECartRemoveItemShippingAddressAction(CartRemoveItemShippingAddressAction),
   #[serde(rename = "removeLineItem")]
   ECartRemoveLineItemAction(CartRemoveLineItemAction),
   #[serde(rename = "removePayment")]
   ECartRemovePaymentAction(CartRemovePaymentAction),
   #[serde(rename = "setAnonymousId")]
   ECartSetAnonymousIdAction(CartSetAnonymousIdAction),
   #[serde(rename = "setBillingAddress")]
   ECartSetBillingAddressAction(CartSetBillingAddressAction),
   #[serde(rename = "setCartTotalTax")]
   ECartSetCartTotalTaxAction(CartSetCartTotalTaxAction),
   #[serde(rename = "setCountry")]
   ECartSetCountryAction(CartSetCountryAction),
   #[serde(rename = "setCustomField")]
   ECartSetCustomFieldAction(CartSetCustomFieldAction),
   #[serde(rename = "setCustomLineItemCustomField")]
   ECartSetCustomLineItemCustomFieldAction(CartSetCustomLineItemCustomFieldAction),
   #[serde(rename = "setCustomLineItemCustomType")]
   ECartSetCustomLineItemCustomTypeAction(CartSetCustomLineItemCustomTypeAction),
   #[serde(rename = "setCustomLineItemShippingDetails")]
   ECartSetCustomLineItemShippingDetailsAction(CartSetCustomLineItemShippingDetailsAction),
   #[serde(rename = "setCustomLineItemTaxAmount")]
   ECartSetCustomLineItemTaxAmountAction(CartSetCustomLineItemTaxAmountAction),
   #[serde(rename = "setCustomLineItemTaxRate")]
   ECartSetCustomLineItemTaxRateAction(CartSetCustomLineItemTaxRateAction),
   #[serde(rename = "setCustomShippingMethod")]
   ECartSetCustomShippingMethodAction(CartSetCustomShippingMethodAction),
   #[serde(rename = "setCustomType")]
   ECartSetCustomTypeAction(CartSetCustomTypeAction),
   #[serde(rename = "setCustomerEmail")]
   ECartSetCustomerEmailAction(CartSetCustomerEmailAction),
   #[serde(rename = "setCustomerGroup")]
   ECartSetCustomerGroupAction(CartSetCustomerGroupAction),
   #[serde(rename = "setCustomerId")]
   ECartSetCustomerIdAction(CartSetCustomerIdAction),
   #[serde(rename = "setDeleteDaysAfterLastModification")]
   ECartSetDeleteDaysAfterLastModificationAction(CartSetDeleteDaysAfterLastModificationAction),
   #[serde(rename = "setLineItemCustomField")]
   ECartSetLineItemCustomFieldAction(CartSetLineItemCustomFieldAction),
   #[serde(rename = "setLineItemCustomType")]
   ECartSetLineItemCustomTypeAction(CartSetLineItemCustomTypeAction),
   #[serde(rename = "setLineItemPrice")]
   ECartSetLineItemPriceAction(CartSetLineItemPriceAction),
   #[serde(rename = "setLineItemShippingDetails")]
   ECartSetLineItemShippingDetailsAction(CartSetLineItemShippingDetailsAction),
   #[serde(rename = "setLineItemTaxAmount")]
   ECartSetLineItemTaxAmountAction(CartSetLineItemTaxAmountAction),
   #[serde(rename = "setLineItemTaxRate")]
   ECartSetLineItemTaxRateAction(CartSetLineItemTaxRateAction),
   #[serde(rename = "setLineItemTotalPrice")]
   ECartSetLineItemTotalPriceAction(CartSetLineItemTotalPriceAction),
   #[serde(rename = "setLocale")]
   ECartSetLocaleAction(CartSetLocaleAction),
   #[serde(rename = "setShippingAddress")]
   ECartSetShippingAddressAction(CartSetShippingAddressAction),
   #[serde(rename = "setShippingMethod")]
   ECartSetShippingMethodAction(CartSetShippingMethodAction),
   #[serde(rename = "setShippingMethodTaxAmount")]
   ECartSetShippingMethodTaxAmountAction(CartSetShippingMethodTaxAmountAction),
   #[serde(rename = "setShippingMethodTaxRate")]
   ECartSetShippingMethodTaxRateAction(CartSetShippingMethodTaxRateAction),
   #[serde(rename = "setShippingRateInput")]
   ECartSetShippingRateInputAction(CartSetShippingRateInputAction),
   #[serde(rename = "updateItemShippingAddress")]
   ECartUpdateItemShippingAddressAction(CartUpdateItemShippingAddressAction),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationShippingRateInput {
   pub label: LocalizedString,
   pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationShippingRateInputDraft {
   pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomLineItem {
   pub discounted_price_per_quantity: Vec<DiscountedLineItemPriceForQuantity>,
   pub state: Vec<ItemState>,
   pub name: LocalizedString,
   pub money: TypedMoney,
   pub total_price: TypedMoney,
   pub quantity: u32,
   pub id: String,
   pub slug: String,
   pub custom: Option<CustomFields>,
   pub shipping_details: Option<ItemShippingDetails>,
   pub tax_category: Option<TaxCategoryReference>,
   pub tax_rate: Option<TaxRate>,
   pub taxed_price: Option<TaxedItemPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomLineItemDraft {
   pub name: LocalizedString,
   pub money: Money,
   pub quantity: u32,
   pub slug: String,
   pub custom: Option<CustomFields>,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
   pub tax_category: Option<TaxCategoryReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeInfo {
   pub discount_code: DiscountCodeReference,
   pub state: DiscountCodeState,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DiscountCodeState {
   NotActive,
   DoesNotMatchCart,
   MatchesCart,
   MaxApplicationReached
}

impl DiscountCodeState {
    pub fn from_str(s: &str) -> Option<DiscountCodeState> {
        match s {
            "NotActive" => Some(DiscountCodeState::NotActive),
            "DoesNotMatchCart" => Some(DiscountCodeState::DoesNotMatchCart),
            "MatchesCart" => Some(DiscountCodeState::MatchesCart),
            "MaxApplicationReached" => Some(DiscountCodeState::MaxApplicationReached),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           DiscountCodeState::NotActive => "NotActive",
           DiscountCodeState::DoesNotMatchCart => "DoesNotMatchCart",
           DiscountCodeState::MatchesCart => "MatchesCart",
           DiscountCodeState::MaxApplicationReached => "MaxApplicationReached",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountedLineItemPortion {
   pub discount: CartDiscountReference,
   pub discounted_amount: Money,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountedLineItemPrice {
   pub included_discounts: Vec<DiscountedLineItemPortion>,
   pub value: TypedMoney,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountedLineItemPriceForQuantity {
   pub discounted_price: DiscountedLineItemPrice,
   pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExternalLineItemTotalPrice {
   pub price: Money,
   pub total_price: Money,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExternalTaxAmountDraft {
   pub tax_rate: ExternalTaxRateDraft,
   pub total_gross: Money,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExternalTaxRateDraft {
   pub country: String,
   pub name: String,
   pub sub_rates: Option<Vec<SubRate>>,
   pub included_in_price: Option<bool>,
   pub amount: Option<u32>,
   pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InventoryMode {
   TrackOnly,
   ReserveOnOrder,
   None
}

impl InventoryMode {
    pub fn from_str(s: &str) -> Option<InventoryMode> {
        match s {
            "TrackOnly" => Some(InventoryMode::TrackOnly),
            "ReserveOnOrder" => Some(InventoryMode::ReserveOnOrder),
            "None" => Some(InventoryMode::None),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           InventoryMode::TrackOnly => "TrackOnly",
           InventoryMode::ReserveOnOrder => "ReserveOnOrder",
           InventoryMode::None => "None",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemShippingDetails {
   pub targets: Vec<ItemShippingTarget>,
   pub valid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemShippingDetailsDraft {
   pub targets: Vec<ItemShippingTarget>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemShippingTarget {
   pub quantity: u32,
   pub address_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineItem {
   pub discounted_price_per_quantity: Vec<DiscountedLineItemPriceForQuantity>,
   pub state: Vec<ItemState>,
   pub line_item_mode: LineItemMode,
   pub price_mode: LineItemPriceMode,
   pub name: LocalizedString,
   pub total_price: Money,
   pub price: Price,
   pub product_type: ProductTypeReference,
   pub variant: ProductVariant,
   pub quantity: u64,
   pub id: String,
   pub product_id: String,
   pub distribution_channel: Option<ChannelReference>,
   pub supply_channel: Option<ChannelReference>,
   pub custom: Option<CustomFields>,
   pub shipping_details: Option<ItemShippingDetails>,
   pub product_slug: Option<LocalizedString>,
   pub tax_rate: Option<TaxRate>,
   pub taxed_price: Option<TaxedItemPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineItemDraft {
   pub distribution_channel: Option<ChannelReference>,
   pub supply_channel: Option<ChannelReference>,
   pub custom: Option<CustomFieldsDraft>,
   pub external_total_price: Option<ExternalLineItemTotalPrice>,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
   pub external_price: Option<Money>,
   pub quantity: Option<u64>,
   pub variant_id: Option<u64>,
   pub product_id: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LineItemMode {
   Standard,
   GiftLineItem
}

impl LineItemMode {
    pub fn from_str(s: &str) -> Option<LineItemMode> {
        match s {
            "Standard" => Some(LineItemMode::Standard),
            "GiftLineItem" => Some(LineItemMode::GiftLineItem),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           LineItemMode::Standard => "Standard",
           LineItemMode::GiftLineItem => "GiftLineItem",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LineItemPriceMode {
   Platform,
   ExternalTotal,
   ExternalPrice
}

impl LineItemPriceMode {
    pub fn from_str(s: &str) -> Option<LineItemPriceMode> {
        match s {
            "Platform" => Some(LineItemPriceMode::Platform),
            "ExternalTotal" => Some(LineItemPriceMode::ExternalTotal),
            "ExternalPrice" => Some(LineItemPriceMode::ExternalPrice),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           LineItemPriceMode::Platform => "Platform",
           LineItemPriceMode::ExternalTotal => "ExternalTotal",
           LineItemPriceMode::ExternalPrice => "ExternalPrice",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaCartDraft {
   pub reference: Reference,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RoundingMode {
   HalfEven,
   HalfUp,
   HalfDown
}

impl RoundingMode {
    pub fn from_str(s: &str) -> Option<RoundingMode> {
        match s {
            "HalfEven" => Some(RoundingMode::HalfEven),
            "HalfUp" => Some(RoundingMode::HalfUp),
            "HalfDown" => Some(RoundingMode::HalfDown),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           RoundingMode::HalfEven => "HalfEven",
           RoundingMode::HalfUp => "HalfUp",
           RoundingMode::HalfDown => "HalfDown",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScoreShippingRateInput {
   pub score: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScoreShippingRateInputDraft {
   pub score: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingInfo {
   pub shipping_method_state: ShippingMethodState,
   pub shipping_rate: ShippingRate,
   pub price: TypedMoney,
   pub shipping_method_name: String,
   pub deliveries: Option<Vec<Delivery>>,
   pub discounted_price: Option<DiscountedLineItemPrice>,
   pub shipping_method: Option<ShippingMethodReference>,
   pub tax_category: Option<TaxCategoryReference>,
   pub tax_rate: Option<TaxRate>,
   pub taxed_price: Option<TaxedItemPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShippingMethodState {
   DoesNotMatchCart,
   MatchesCart
}

impl ShippingMethodState {
    pub fn from_str(s: &str) -> Option<ShippingMethodState> {
        match s {
            "DoesNotMatchCart" => Some(ShippingMethodState::DoesNotMatchCart),
            "MatchesCart" => Some(ShippingMethodState::MatchesCart),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ShippingMethodState::DoesNotMatchCart => "DoesNotMatchCart",
           ShippingMethodState::MatchesCart => "MatchesCart",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ShippingRateInput {
   #[serde(rename = "Score")]
   EScoreShippingRateInput(ScoreShippingRateInput),
   #[serde(rename = "Classification")]
   EClassificationShippingRateInput(ClassificationShippingRateInput),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ShippingRateInputDraft {
   #[serde(rename = "Classification")]
   EClassificationShippingRateInputDraft(ClassificationShippingRateInputDraft),
   #[serde(rename = "Score")]
   EScoreShippingRateInputDraft(ScoreShippingRateInputDraft),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaxCalculationMode {
   LineItemLevel,
   UnitPriceLevel
}

impl TaxCalculationMode {
    pub fn from_str(s: &str) -> Option<TaxCalculationMode> {
        match s {
            "LineItemLevel" => Some(TaxCalculationMode::LineItemLevel),
            "UnitPriceLevel" => Some(TaxCalculationMode::UnitPriceLevel),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           TaxCalculationMode::LineItemLevel => "LineItemLevel",
           TaxCalculationMode::UnitPriceLevel => "UnitPriceLevel",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaxMode {
   Platform,
   External,
   ExternalAmount,
   Disabled
}

impl TaxMode {
    pub fn from_str(s: &str) -> Option<TaxMode> {
        match s {
            "Platform" => Some(TaxMode::Platform),
            "External" => Some(TaxMode::External),
            "ExternalAmount" => Some(TaxMode::ExternalAmount),
            "Disabled" => Some(TaxMode::Disabled),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           TaxMode::Platform => "Platform",
           TaxMode::External => "External",
           TaxMode::ExternalAmount => "ExternalAmount",
           TaxMode::Disabled => "Disabled",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaxPortion {
   pub amount: Money,
   pub rate: u32,
   pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaxedItemPrice {
   pub total_gross: TypedMoney,
   pub total_net: TypedMoney,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaxedPrice {
   pub tax_portions: Vec<TaxPortion>,
   pub total_gross: Money,
   pub total_net: Money,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartAddCustomLineItemAction {
   pub name: LocalizedString,
   pub money: Money,
   pub quantity: u32,
   pub slug: String,
   pub custom: Option<CustomFieldsDraft>,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub tax_category: Option<TaxCategoryReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartAddDiscountCodeAction {
   pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartAddItemShippingAddressAction {
   pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartAddLineItemAction {
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
pub struct CartAddPaymentAction {
   pub payment: PaymentReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartAddShoppingListAction {
   pub shopping_list: ShoppingListReference,
   pub distribution_channel: Option<ChannelReference>,
   pub supply_channel: Option<ChannelReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartApplyDeltaToCustomLineItemShippingDetailsTargetsAction {
   pub targets_delta: Vec<ItemShippingTarget>,
   pub custom_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartApplyDeltaToLineItemShippingDetailsTargetsAction {
   pub targets_delta: Vec<ItemShippingTarget>,
   pub line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartChangeCustomLineItemMoneyAction {
   pub money: Money,
   pub custom_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartChangeCustomLineItemQuantityAction {
   pub quantity: u32,
   pub custom_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartChangeLineItemQuantityAction {
   pub quantity: u32,
   pub line_item_id: String,
   pub external_total_price: Option<ExternalLineItemTotalPrice>,
   pub external_price: Option<Money>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartChangeTaxCalculationModeAction {
   pub tax_calculation_mode: TaxCalculationMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartChangeTaxModeAction {
   pub tax_mode: TaxMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartChangeTaxRoundingModeAction {
   pub tax_rounding_mode: RoundingMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartRecalculateAction {
   pub update_product_data: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartRemoveCustomLineItemAction {
   pub custom_line_item_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartRemoveDiscountCodeAction {
   pub discount_code: DiscountCodeReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartRemoveItemShippingAddressAction {
   pub address_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartRemoveLineItemAction {
   pub line_item_id: String,
   pub external_total_price: Option<ExternalLineItemTotalPrice>,
   pub shipping_details_to_remove: Option<ItemShippingDetailsDraft>,
   pub external_price: Option<Money>,
   pub quantity: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartRemovePaymentAction {
   pub payment: PaymentReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetAnonymousIdAction {
   pub anonymous_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetBillingAddressAction {
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCartTotalTaxAction {
   pub external_total_gross: Money,
   pub external_tax_portions: Option<Vec<TaxPortion>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCountryAction {
   /**
   	<p>A two-digit country code as per <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a>.</p>
   */
   pub country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomLineItemCustomFieldAction {
   pub custom_line_item_id: String,
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomLineItemCustomTypeAction {
   pub custom_line_item_id: String,
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomLineItemShippingDetailsAction {
   pub custom_line_item_id: String,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomLineItemTaxAmountAction {
   pub custom_line_item_id: String,
   pub external_tax_amount: Option<ExternalTaxAmountDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomLineItemTaxRateAction {
   pub custom_line_item_id: String,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomShippingMethodAction {
   pub shipping_rate: ShippingRateDraft,
   pub shipping_method_name: String,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub tax_category: Option<TaxCategoryReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomTypeAction {
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomerEmailAction {
   pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomerGroupAction {
   pub customer_group: Option<CustomerGroupReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetCustomerIdAction {
   pub customer_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetDeleteDaysAfterLastModificationAction {
   pub delete_days_after_last_modification: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetLineItemCustomFieldAction {
   pub line_item_id: String,
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetLineItemCustomTypeAction {
   pub line_item_id: String,
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetLineItemPriceAction {
   pub line_item_id: String,
   pub external_price: Option<Money>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetLineItemShippingDetailsAction {
   pub line_item_id: String,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetLineItemTaxAmountAction {
   pub line_item_id: String,
   pub external_tax_amount: Option<ExternalTaxAmountDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetLineItemTaxRateAction {
   pub line_item_id: String,
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetLineItemTotalPriceAction {
   pub line_item_id: String,
   pub external_total_price: Option<ExternalLineItemTotalPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetLocaleAction {
   pub locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetShippingAddressAction {
   pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetShippingMethodAction {
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
   pub shipping_method: Option<TypeReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetShippingMethodTaxAmountAction {
   pub external_tax_amount: Option<ExternalTaxAmountDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetShippingMethodTaxRateAction {
   pub external_tax_rate: Option<ExternalTaxRateDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartSetShippingRateInputAction {
   pub shipping_rate_input: Option<ShippingRateInputDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartUpdateItemShippingAddressAction {
   pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProductPublishScope {
   All,
   Prices
}

impl ProductPublishScope {
    pub fn from_str(s: &str) -> Option<ProductPublishScope> {
        match s {
            "All" => Some(ProductPublishScope::All),
            "Prices" => Some(ProductPublishScope::Prices),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ProductPublishScope::All => "All",
           ProductPublishScope::Prices => "Prices",
        }
    }
}