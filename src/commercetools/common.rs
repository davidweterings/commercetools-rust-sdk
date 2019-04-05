//Generated file, please do not change

use super::cttype::CustomFields;
use super::cttype::CustomFieldsDraft;
use super::productdiscount::ProductDiscountReference;
use super::customergroup::CustomerGroupReference;
use super::channel::ChannelReference;
use super::category::CategoryReference;
use super::shippingmethod::ShippingMethodReference;
use super::zone::ZoneReference;
use super::discountcode::DiscountCodeReference;
use super::review::ReviewReference;
use super::payment::PaymentReference;
use super::order::OrderReference;
use super::cttype::TypeReference;
use super::state::StateReference;
use super::inventory::InventoryEntryReference;
use super::product::ProductReference;
use super::producttype::ProductTypeReference;
use super::customer::CustomerReference;
use super::shoppinglist::ShoppingListReference;
use super::orderedit::OrderEditReference;
use super::customobject::CustomObjectReference;
use super::cartdiscount::CartDiscountReference;
use super::taxcategory::TaxCategoryReference;
use super::cart::CartReference;
use super::customobject::CustomObject;
use super::customergroup::CustomerGroup;
use super::customer::Customer;
use super::discountcode::DiscountCode;
use super::extension::Extension;
use super::inventory::InventoryEntry;
use super::message::Message;
use super::orderedit::OrderEdit;
use super::order::Order;
use super::payment::Payment;
use super::productdiscount::ProductDiscount;
use super::producttype::ProductType;
use super::product::Product;
use super::product::ProductProjection;
use super::review::Review;
use super::shippingmethod::ShippingMethod;
use super::shoppinglist::ShoppingList;
use super::state::State;
use super::subscription::Subscription;
use super::taxcategory::TaxCategory;
use super::cttype::Type;
use super::zone::Zone;
use super::channel::Channel;
use super::category::Category;
use super::cartdiscount::CartDiscount;
use super::cart::Cart;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Address {
   /**
   	<p>A two-digit country code as per <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a>.</p>
   */
   pub country: String,
   pub additional_address_info: Option<String>,
   pub additional_street_info: Option<String>,
   pub apartment: Option<String>,
   pub building: Option<String>,
   pub city: Option<String>,
   pub company: Option<String>,
   pub department: Option<String>,
   pub email: Option<String>,
   pub external_id: Option<String>,
   pub fax: Option<String>,
   pub first_name: Option<String>,
   pub id: Option<String>,
   pub key: Option<String>,
   pub last_name: Option<String>,
   pub mobile: Option<String>,
   pub p_o_box: Option<String>,
   pub phone: Option<String>,
   pub postal_code: Option<String>,
   pub region: Option<String>,
   pub salutation: Option<String>,
   pub state: Option<String>,
   pub street_name: Option<String>,
   pub street_number: Option<String>,
   pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
   pub sources: Vec<AssetSource>,
   pub name: LocalizedString,
   pub id: String,
   pub tags: Option<Vec<String>>,
   pub custom: Option<CustomFields>,
   pub description: Option<LocalizedString>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetDimensions {
   pub h: u32,
   pub w: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetDraft {
   pub sources: Vec<AssetSource>,
   pub name: LocalizedString,
   pub tags: Option<Vec<String>>,
   pub custom: Option<CustomFieldsDraft>,
   pub description: Option<LocalizedString>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetSource {
   pub uri: String,
   pub dimensions: Option<AssetDimensions>,
   pub content_type: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CentPrecisionMoney {
   /**
   	<p>The currency code compliant to <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>.</p>
   */
   pub currency_code: String,
   pub cent_amount: u64,
   pub fraction_digits: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountedPrice {
   pub value: Money,
   pub discount: ProductDiscountReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeoJson {
   pub coordinates: Vec<serde_json::Value>,
   pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeoJsonPoint {
   pub coordinates: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HighPrecisionMoney {
   /**
   	<p>The currency code compliant to <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>.</p>
   */
   pub currency_code: String,
   pub cent_amount: u64,
   pub fraction_digits: u32,
   pub precise_amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
   pub dimensions: ImageDimensions,
   pub url: String,
   pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageDimensions {
   pub h: u32,
   pub w: u32,
}

pub type LocalizedString = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Money {
   /**
   	<p>The currency code compliant to <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>.</p>
   */
   pub currency_code: String,
   pub cent_amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MoneyType {
   CentPrecision(String),
   HighPrecision(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Price {
   pub value: Money,
   pub tiers: Option<Vec<PriceTier>>,
   pub channel: Option<ChannelReference>,
   /**
   	<p>A two-digit country code as per <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a>.</p>
   */
   pub country: Option<String>,
   pub custom: Option<CustomFields>,
   pub customer_group: Option<CustomerGroupReference>,
   pub discounted: Option<DiscountedPrice>,
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
   pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceDraft {
   pub value: Money,
   pub tiers: Option<Vec<PriceTier>>,
   pub channel: Option<ChannelReference>,
   /**
   	<p>A two-digit country code as per <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a>.</p>
   */
   pub country: Option<String>,
   pub custom: Option<CustomFieldsDraft>,
   pub customer_group: Option<CustomerGroupReference>,
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceTier {
   pub value: Money,
   pub minimum_quantity: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
   pub type_id: Option<ReferenceTypeId>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReferenceTypeId {
   Cart,
   CartDiscount,
   Category,
   Channel,
   Customer,
   CustomerGroup,
   DiscountCode,
   KeyValueDocument,
   Payment,
   Product,
   ProductType,
   ProductDiscount,
   Order,
   Review,
   ShoppingList,
   ShippingMethod,
   State,
   TaxCategory,
   Type,
   Zone,
   InventoryEntry,
   OrderEdit
}

impl ReferenceTypeId {
    pub fn from_str(s: &str) -> Option<ReferenceTypeId> {
        match s {
            "cart" => Some(ReferenceTypeId::Cart),
            "cart-discount" => Some(ReferenceTypeId::CartDiscount),
            "category" => Some(ReferenceTypeId::Category),
            "channel" => Some(ReferenceTypeId::Channel),
            "customer" => Some(ReferenceTypeId::Customer),
            "customer-group" => Some(ReferenceTypeId::CustomerGroup),
            "discount-code" => Some(ReferenceTypeId::DiscountCode),
            "key-value-document" => Some(ReferenceTypeId::KeyValueDocument),
            "payment" => Some(ReferenceTypeId::Payment),
            "product" => Some(ReferenceTypeId::Product),
            "product-type" => Some(ReferenceTypeId::ProductType),
            "product-discount" => Some(ReferenceTypeId::ProductDiscount),
            "order" => Some(ReferenceTypeId::Order),
            "review" => Some(ReferenceTypeId::Review),
            "shopping-list" => Some(ReferenceTypeId::ShoppingList),
            "shipping-method" => Some(ReferenceTypeId::ShippingMethod),
            "state" => Some(ReferenceTypeId::State),
            "tax-category" => Some(ReferenceTypeId::TaxCategory),
            "type" => Some(ReferenceTypeId::Type),
            "zone" => Some(ReferenceTypeId::Zone),
            "inventory-entry" => Some(ReferenceTypeId::InventoryEntry),
            "order-edit" => Some(ReferenceTypeId::OrderEdit),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ReferenceTypeId::Cart => "cart",
           ReferenceTypeId::CartDiscount => "cart-discount",
           ReferenceTypeId::Category => "category",
           ReferenceTypeId::Channel => "channel",
           ReferenceTypeId::Customer => "customer",
           ReferenceTypeId::CustomerGroup => "customer-group",
           ReferenceTypeId::DiscountCode => "discount-code",
           ReferenceTypeId::KeyValueDocument => "key-value-document",
           ReferenceTypeId::Payment => "payment",
           ReferenceTypeId::Product => "product",
           ReferenceTypeId::ProductType => "product-type",
           ReferenceTypeId::ProductDiscount => "product-discount",
           ReferenceTypeId::Order => "order",
           ReferenceTypeId::Review => "review",
           ReferenceTypeId::ShoppingList => "shopping-list",
           ReferenceTypeId::ShippingMethod => "shipping-method",
           ReferenceTypeId::State => "state",
           ReferenceTypeId::TaxCategory => "tax-category",
           ReferenceTypeId::Type => "type",
           ReferenceTypeId::Zone => "zone",
           ReferenceTypeId::InventoryEntry => "inventory-entry",
           ReferenceTypeId::OrderEdit => "order-edit",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceIdentifier {
   pub type_id: Option<ReferenceTypeId>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScopedPrice {
   pub current_value: TypedMoney,
   pub value: TypedMoney,
   pub id: String,
   pub channel: Option<ChannelReference>,
   /**
   	<p>A two-digit country code as per <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a>.</p>
   */
   pub country: Option<String>,
   pub custom: Option<CustomFields>,
   pub customer_group: Option<CustomerGroupReference>,
   pub discounted: Option<DiscountedPrice>,
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypedMoney {
   /**
   	<p>The currency code compliant to <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>.</p>
   */
   pub currency_code: String,
   pub r#type: MoneyType,
   pub cent_amount: u64,
   pub fraction_digits: u32,
}