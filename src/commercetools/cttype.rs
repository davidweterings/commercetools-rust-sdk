//Generated file, please do not change

use super::common::LocalizedString;
use super::common::ReferenceTypeId;
use super::common::Resource;
use super::common::Reference;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldBooleanType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldDateTimeType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldDateType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldEnumType {
   pub values: Vec<CustomFieldEnumValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldEnumValue {
   pub key: String,
   pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldLocalizedEnumType {
   pub values: Vec<CustomFieldLocalizedEnumValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldLocalizedEnumValue {
   pub label: LocalizedString,
   pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldLocalizedStringType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldMoneyType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldNumberType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldReferenceType {
   pub reference_type_id: ReferenceTypeId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldSetType {
   pub element_type: Box<FieldType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldStringType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldTimeType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFields {
   pub fields: FieldContainer,
   pub r#type: TypeReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldsDraft {
   pub r#type: serde_json::Value,
   pub fields: Option<FieldContainer>,
}

pub type FieldContainer = HashMap<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition {
   pub label: LocalizedString,
   pub required: bool,
   pub r#type: serde_json::Value,
   pub name: String,
   pub input_hint: Option<TypeTextInputHint>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name")]
pub enum FieldType {
   #[serde(rename = "Reference")]
   ECustomFieldReferenceType(CustomFieldReferenceType),
   #[serde(rename = "Boolean")]
   ECustomFieldBooleanType(CustomFieldBooleanType),
   #[serde(rename = "Number")]
   ECustomFieldNumberType(CustomFieldNumberType),
   #[serde(rename = "LocalizedEnum")]
   ECustomFieldLocalizedEnumType(CustomFieldLocalizedEnumType),
   #[serde(rename = "String")]
   ECustomFieldStringType(CustomFieldStringType),
   #[serde(rename = "Set")]
   ECustomFieldSetType(CustomFieldSetType),
   #[serde(rename = "Time")]
   ECustomFieldTimeType(CustomFieldTimeType),
   #[serde(rename = "DateTime")]
   ECustomFieldDateTimeType(CustomFieldDateTimeType),
   #[serde(rename = "Date")]
   ECustomFieldDateType(CustomFieldDateType),
   #[serde(rename = "Enum")]
   ECustomFieldEnumType(CustomFieldEnumType),
   #[serde(rename = "Money")]
   ECustomFieldMoneyType(CustomFieldMoneyType),
   #[serde(rename = "LocalizedString")]
   ECustomFieldLocalizedStringType(CustomFieldLocalizedStringType),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceTypeId {
   Asset,
   Category,
   Channel,
   Customer,
   Order,
   OrderEdit,
   InventoryEntry,
   LineItem,
   CustomLineItem,
   ProductPrice,
   Payment,
   PaymentInterfaceInteraction,
   Review,
   ShoppingList,
   ShoppingListTextLineItem,
   DiscountCode,
   CartDiscount,
   CustomerGroup
}

impl ResourceTypeId {
    pub fn from_str(s: &str) -> Option<ResourceTypeId> {
        match s {
            "asset" => Some(ResourceTypeId::Asset),
            "category" => Some(ResourceTypeId::Category),
            "channel" => Some(ResourceTypeId::Channel),
            "customer" => Some(ResourceTypeId::Customer),
            "order" => Some(ResourceTypeId::Order),
            "order-edit" => Some(ResourceTypeId::OrderEdit),
            "inventory-entry" => Some(ResourceTypeId::InventoryEntry),
            "line-item" => Some(ResourceTypeId::LineItem),
            "custom-line-item" => Some(ResourceTypeId::CustomLineItem),
            "product-price" => Some(ResourceTypeId::ProductPrice),
            "payment" => Some(ResourceTypeId::Payment),
            "payment-interface-interaction" => Some(ResourceTypeId::PaymentInterfaceInteraction),
            "review" => Some(ResourceTypeId::Review),
            "shopping-list" => Some(ResourceTypeId::ShoppingList),
            "shopping-list-text-line-item" => Some(ResourceTypeId::ShoppingListTextLineItem),
            "discount-code" => Some(ResourceTypeId::DiscountCode),
            "cart-discount" => Some(ResourceTypeId::CartDiscount),
            "customer-group" => Some(ResourceTypeId::CustomerGroup),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           ResourceTypeId::Asset => "asset",
           ResourceTypeId::Category => "category",
           ResourceTypeId::Channel => "channel",
           ResourceTypeId::Customer => "customer",
           ResourceTypeId::Order => "order",
           ResourceTypeId::OrderEdit => "order-edit",
           ResourceTypeId::InventoryEntry => "inventory-entry",
           ResourceTypeId::LineItem => "line-item",
           ResourceTypeId::CustomLineItem => "custom-line-item",
           ResourceTypeId::ProductPrice => "product-price",
           ResourceTypeId::Payment => "payment",
           ResourceTypeId::PaymentInterfaceInteraction => "payment-interface-interaction",
           ResourceTypeId::Review => "review",
           ResourceTypeId::ShoppingList => "shopping-list",
           ResourceTypeId::ShoppingListTextLineItem => "shopping-list-text-line-item",
           ResourceTypeId::DiscountCode => "discount-code",
           ResourceTypeId::CartDiscount => "cart-discount",
           ResourceTypeId::CustomerGroup => "customer-group",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Type {
   pub field_definitions: Vec<FieldDefinition>,
   pub resource_type_ids: Vec<ResourceTypeId>,
   pub name: LocalizedString,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub key: String,
   pub description: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeDraft {
   pub resource_type_ids: Vec<ResourceTypeId>,
   pub name: LocalizedString,
   pub key: String,
   pub field_definitions: Option<Vec<FieldDefinition>>,
   pub description: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypePagedQueryResponse {
   pub results: Vec<Type>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeReference {
   pub obj: Option<Type>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TypeTextInputHint {
   SingleLine,
   MultiLine
}

impl TypeTextInputHint {
    pub fn from_str(s: &str) -> Option<TypeTextInputHint> {
        match s {
            "SingleLine" => Some(TypeTextInputHint::SingleLine),
            "MultiLine" => Some(TypeTextInputHint::MultiLine),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           TypeTextInputHint::SingleLine => "SingleLine",
           TypeTextInputHint::MultiLine => "MultiLine",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeUpdate {
   pub actions: Vec<TypeUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum TypeUpdateAction {
   #[serde(rename = "addEnumValue")]
   ETypeAddEnumValueAction(TypeAddEnumValueAction),
   #[serde(rename = "addFieldDefinition")]
   ETypeAddFieldDefinitionAction(TypeAddFieldDefinitionAction),
   #[serde(rename = "addLocalizedEnumValue")]
   ETypeAddLocalizedEnumValueAction(TypeAddLocalizedEnumValueAction),
   #[serde(rename = "changeEnumValueOrder")]
   ETypeChangeEnumValueOrderAction(TypeChangeEnumValueOrderAction),
   #[serde(rename = "changeFieldDefinitionLabel")]
   ETypeChangeFieldDefinitionLabelAction(TypeChangeFieldDefinitionLabelAction),
   #[serde(rename = "changeFieldDefinitionOrder")]
   ETypeChangeFieldDefinitionOrderAction(TypeChangeFieldDefinitionOrderAction),
   #[serde(rename = "changeKey")]
   ETypeChangeKeyAction(TypeChangeKeyAction),
   #[serde(rename = "changeLabel")]
   ETypeChangeLabelAction(TypeChangeLabelAction),
   #[serde(rename = "changeLocalizedEnumValueOrder")]
   ETypeChangeLocalizedEnumValueOrderAction(TypeChangeLocalizedEnumValueOrderAction),
   #[serde(rename = "changeName")]
   ETypeChangeNameAction(TypeChangeNameAction),
   #[serde(rename = "removeFieldDefinition")]
   ETypeRemoveFieldDefinitionAction(TypeRemoveFieldDefinitionAction),
   #[serde(rename = "setDescription")]
   ETypeSetDescriptionAction(TypeSetDescriptionAction),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeAddEnumValueAction {
   pub value: CustomFieldEnumValue,
   pub field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeAddFieldDefinitionAction {
   pub field_definition: FieldDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeAddLocalizedEnumValueAction {
   pub value: CustomFieldLocalizedEnumValue,
   pub field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeChangeEnumValueOrderAction {
   pub keys: Vec<String>,
   pub field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeChangeFieldDefinitionLabelAction {
   pub label: LocalizedString,
   pub field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeChangeFieldDefinitionOrderAction {
   pub field_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeChangeKeyAction {
   pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeChangeLabelAction {
   pub label: LocalizedString,
   pub field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeChangeLocalizedEnumValueOrderAction {
   pub keys: Vec<String>,
   pub field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeChangeNameAction {
   pub name: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeRemoveFieldDefinitionAction {
   pub field_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeSetDescriptionAction {
   pub description: Option<LocalizedString>,
}