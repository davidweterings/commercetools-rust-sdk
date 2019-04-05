//Generated file, please do not change

use super::cart::InventoryMode;
use super::common::Address;
use super::shippingmethod::ShippingMethodReference;
use super::cttype::CustomFieldsDraft;
use super::cart::TaxMode;
use super::cttype::CustomFields;
use super::channel::ChannelReference;
use super::cart::ItemShippingDetailsDraft;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MyCartDraft {
   /**
   	<p>The currency code compliant to <a href="https://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>.</p>
   */
   pub currency: String,
   pub item_shipping_addresses: Option<Vec<Address>>,
   pub line_items: Option<Vec<MyLineItemDraft>>,
   pub billing_address: Option<Address>,
   pub shipping_address: Option<Address>,
   pub custom: Option<CustomFieldsDraft>,
   pub inventory_mode: Option<InventoryMode>,
   pub shipping_method: Option<ShippingMethodReference>,
   pub tax_mode: Option<TaxMode>,
   pub delete_days_after_last_modification: Option<u64>,
   pub country: Option<String>,
   pub customer_email: Option<String>,
   pub locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MyCustomerDraft {
   pub email: String,
   pub password: String,
   pub addresses: Option<Vec<Address>>,
   pub custom: Option<CustomFields>,
   pub date_of_birth: Option<NaiveDate>,
   pub default_billing_address: Option<u64>,
   pub default_shipping_address: Option<u64>,
   pub company_name: Option<String>,
   pub first_name: Option<String>,
   pub last_name: Option<String>,
   pub locale: Option<String>,
   pub middle_name: Option<String>,
   pub title: Option<String>,
   pub vat_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MyLineItemDraft {
   pub quantity: u32,
   pub variant_id: u64,
   pub product_id: String,
   pub distribution_channel: Option<ChannelReference>,
   pub supply_channel: Option<ChannelReference>,
   pub custom: Option<CustomFieldsDraft>,
   pub shipping_details: Option<ItemShippingDetailsDraft>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MyOrderFromCartDraft {
   pub version: u64,
   pub id: String,
}