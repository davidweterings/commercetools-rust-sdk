//Generated file, please do not change

use super::common::LocalizedString;
use super::common::Reference;
use super::cttype::CustomFields;
use super::common::Resource;
use super::common::Money;
use super::product::ProductReference;
use super::channel::ChannelReference;
use super::cttype::TypeReference;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscount {
   pub references: Vec<Reference>,
   pub value: CartDiscountValue,
   pub name: LocalizedString,
   /**
   	<p>Specifies whether the application of this discount causes the following discounts to be ignored.
   	Defaults to Stacking.</p>
   */
   pub stacking_mode: StackingMode,
   pub is_active: bool,
   pub requires_discount_code: bool,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub cart_predicate: String,
   pub id: String,
   pub sort_order: String,
   pub target: Option<CartDiscountTarget>,
   pub custom: Option<CustomFields>,
   pub description: Option<LocalizedString>,
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountCustomLineItemsTarget {
   pub predicate: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountDraft {
   pub value: CartDiscountValue,
   pub name: LocalizedString,
   pub requires_discount_code: bool,
   pub cart_predicate: String,
   pub sort_order: String,
   pub target: Option<CartDiscountTarget>,
   pub custom: Option<CustomFields>,
   pub description: Option<LocalizedString>,
   /**
   	<p>Specifies whether the application of this discount causes the following discounts to be ignored.
   	Defaults to Stacking.</p>
   */
   pub stacking_mode: Option<StackingMode>,
   pub is_active: Option<bool>,
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountLineItemsTarget {
   pub predicate: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountPagedQueryResponse {
   pub results: Vec<CartDiscount>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountReference {
   pub obj: Option<CartDiscount>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountShippingCostTarget {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountTarget {
   pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountUpdate {
   pub actions: Vec<CartDiscountUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountValue {
   pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountValueAbsolute {
   pub money: Vec<Money>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountValueGiftLineItem {
   pub product: ProductReference,
   pub variant_id: u64,
   pub distribution_channel: Option<ChannelReference>,
   pub supply_channel: Option<ChannelReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountValueRelative {
   pub permyriad: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultiBuyCustomLineItemsTarget {
   pub selection_mode: SelectionMode,
   /**
   	<p>Quantity of line items that are discounted per application of this discount.</p>
   */
   pub discounted_quantity: u32,
   /**
   	<p>Quantity of line items that need to be present in order to trigger an application of this discount.</p>
   */
   pub trigger_quantity: u32,
   /**
   	<p>A valid custom line item target predicate. The discount will be applied to custom line items that are
   	matched by the predicate.</p>
   */
   pub predicate: String,
   /**
   	<p>Maximum number of applications of this discount.</p>
   */
   pub max_occurrence: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultiBuyLineItemsTarget {
   pub selection_mode: SelectionMode,
   /**
   	<p>Quantity of line items that are discounted per application of this discount.</p>
   */
   pub discounted_quantity: u32,
   /**
   	<p>Quantity of line items that need to be present in order to trigger an application of this discount.</p>
   */
   pub trigger_quantity: u32,
   /**
   	<p>A valid line item target predicate. The discount will be applied to line items that are matched by the predicate.</p>
   */
   pub predicate: String,
   /**
   	<p>Maximum number of applications of this discount.</p>
   */
   pub max_occurrence: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SelectionMode {
   Cheapest,
   MostExpensive
}

impl SelectionMode {
    pub fn from_str(s: &str) -> Option<SelectionMode> {
        match s {
            "Cheapest" => Some(SelectionMode::Cheapest),
            "MostExpensive" => Some(SelectionMode::MostExpensive),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           SelectionMode::Cheapest => "Cheapest",
           SelectionMode::MostExpensive => "MostExpensive",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StackingMode {
   Stacking,
   StopAfterThisDiscount
}

impl StackingMode {
    pub fn from_str(s: &str) -> Option<StackingMode> {
        match s {
            "Stacking" => Some(StackingMode::Stacking),
            "StopAfterThisDiscount" => Some(StackingMode::StopAfterThisDiscount),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           StackingMode::Stacking => "Stacking",
           StackingMode::StopAfterThisDiscount => "StopAfterThisDiscount",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountChangeCartPredicateAction {
   pub cart_predicate: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountChangeIsActiveAction {
   pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountChangeNameAction {
   pub name: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountChangeRequiresDiscountCodeAction {
   pub requires_discount_code: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountChangeSortOrderAction {
   pub sort_order: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountChangeStackingModeAction {
   pub stacking_mode: StackingMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountChangeTargetAction {
   pub target: CartDiscountTarget,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountChangeValueAction {
   pub value: CartDiscountValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountSetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountSetCustomTypeAction {
   pub r#type: Option<TypeReference>,
   pub fields: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountSetDescriptionAction {
   pub description: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountSetValidFromAction {
   pub valid_from: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountSetValidFromAndUntilAction {
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CartDiscountSetValidUntilAction {
   pub valid_until: Option<DateTime<Utc>>,
}