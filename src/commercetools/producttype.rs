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
pub struct AttributeBooleanType {
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AttributeConstraintEnum {
   None,
   Unique,
   CombinationUnique,
   SameForAll
}

impl AttributeConstraintEnum {
    pub fn from_str(s: &str) -> Option<AttributeConstraintEnum> {
        match s {
            "None" => Some(AttributeConstraintEnum::None),
            "Unique" => Some(AttributeConstraintEnum::Unique),
            "CombinationUnique" => Some(AttributeConstraintEnum::CombinationUnique),
            "SameForAll" => Some(AttributeConstraintEnum::SameForAll),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           AttributeConstraintEnum::None => "None",
           AttributeConstraintEnum::Unique => "Unique",
           AttributeConstraintEnum::CombinationUnique => "CombinationUnique",
           AttributeConstraintEnum::SameForAll => "SameForAll",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AttributeConstraintEnumDraft {
   None
}

impl AttributeConstraintEnumDraft {
    pub fn from_str(s: &str) -> Option<AttributeConstraintEnumDraft> {
        match s {
            "None" => Some(AttributeConstraintEnumDraft::None),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           AttributeConstraintEnumDraft::None => "None",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDateTimeType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDateType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDefinition {
   pub r#type: serde_json::Value,
   pub attribute_constraint: AttributeConstraintEnum,
   pub label: LocalizedString,
   pub input_hint: TextInputHint,
   pub is_required: bool,
   pub is_searchable: bool,
   pub name: String,
   pub input_tip: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDefinitionDraft {
   pub r#type: serde_json::Value,
   pub label: LocalizedString,
   pub is_required: bool,
   pub name: String,
   pub attribute_constraint: Option<AttributeConstraintEnum>,
   pub input_tip: Option<LocalizedString>,
   pub input_hint: Option<TextInputHint>,
   pub is_searchable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeEnumType {
   pub values: Vec<AttributePlainEnumValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeLocalizableTextType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeLocalizedEnumType {
   pub values: Vec<AttributeLocalizedEnumValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeLocalizedEnumValue {
   pub label: LocalizedString,
   pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeMoneyType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeNestedType {
   pub type_reference: ProductTypeReference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeNumberType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributePlainEnumValue {
   pub key: String,
   pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeReferenceType {
   pub reference_type_id: ReferenceTypeId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeSetType {
   pub element_type: Box<AttributeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeTextType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeTimeType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name")]
pub enum AttributeType {
   #[serde(rename = "enum")]
   EAttributeEnumType(AttributeEnumType),
   #[serde(rename = "text")]
   EAttributeTextType(AttributeTextType),
   #[serde(rename = "ltext")]
   EAttributeLocalizableTextType(AttributeLocalizableTextType),
   #[serde(rename = "date")]
   EAttributeDateType(AttributeDateType),
   #[serde(rename = "datetime")]
   EAttributeDateTimeType(AttributeDateTimeType),
   #[serde(rename = "number")]
   EAttributeNumberType(AttributeNumberType),
   #[serde(rename = "money")]
   EAttributeMoneyType(AttributeMoneyType),
   #[serde(rename = "time")]
   EAttributeTimeType(AttributeTimeType),
   #[serde(rename = "lenum")]
   EAttributeLocalizedEnumType(AttributeLocalizedEnumType),
   #[serde(rename = "reference")]
   EAttributeReferenceType(AttributeReferenceType),
   #[serde(rename = "nested")]
   EAttributeNestedType(AttributeNestedType),
   #[serde(rename = "set")]
   EAttributeSetType(AttributeSetType),
   #[serde(rename = "boolean")]
   EAttributeBooleanType(AttributeBooleanType),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductType {
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub description: String,
   pub id: String,
   pub name: String,
   pub attributes: Option<Vec<AttributeDefinition>>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeDraft {
   pub description: String,
   pub name: String,
   pub attributes: Option<Vec<AttributeDefinitionDraft>>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypePagedQueryResponse {
   pub results: Vec<ProductType>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeReference {
   pub obj: Option<ProductType>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeUpdate {
   pub actions: Vec<ProductTypeUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ProductTypeUpdateAction {
   #[serde(rename = "addAttributeDefinition")]
   EProductTypeAddAttributeDefinitionAction(ProductTypeAddAttributeDefinitionAction),
   #[serde(rename = "addLocalizedEnumValue")]
   EProductTypeAddLocalizedEnumValueAction(ProductTypeAddLocalizedEnumValueAction),
   #[serde(rename = "addPlainEnumValue")]
   EProductTypeAddPlainEnumValueAction(ProductTypeAddPlainEnumValueAction),
   #[serde(rename = "changeAttributeConstraint")]
   EProductTypeChangeAttributeConstraintAction(ProductTypeChangeAttributeConstraintAction),
   #[serde(rename = "changeAttributeName")]
   EProductTypeChangeAttributeNameAction(ProductTypeChangeAttributeNameAction),
   #[serde(rename = "changeAttributeOrder")]
   EProductTypeChangeAttributeOrderAction(ProductTypeChangeAttributeOrderAction),
   #[serde(rename = "changeAttributeOrderByName")]
   EProductTypeChangeAttributeOrderByNameAction(ProductTypeChangeAttributeOrderByNameAction),
   #[serde(rename = "changeDescription")]
   EProductTypeChangeDescriptionAction(ProductTypeChangeDescriptionAction),
   #[serde(rename = "changeEnumKey")]
   EProductTypeChangeEnumKeyAction(ProductTypeChangeEnumKeyAction),
   #[serde(rename = "changeInputHint")]
   EProductTypeChangeInputHintAction(ProductTypeChangeInputHintAction),
   #[serde(rename = "changeIsSearchable")]
   EProductTypeChangeIsSearchableAction(ProductTypeChangeIsSearchableAction),
   #[serde(rename = "changeLabel")]
   EProductTypeChangeLabelAction(ProductTypeChangeLabelAction),
   #[serde(rename = "changeLocalizedEnumValueLabel")]
   EProductTypeChangeLocalizedEnumValueLabelAction(ProductTypeChangeLocalizedEnumValueLabelAction),
   #[serde(rename = "changeLocalizedEnumValueOrder")]
   EProductTypeChangeLocalizedEnumValueOrderAction(ProductTypeChangeLocalizedEnumValueOrderAction),
   #[serde(rename = "changeName")]
   EProductTypeChangeNameAction(ProductTypeChangeNameAction),
   #[serde(rename = "changePlainEnumValueLabel")]
   EProductTypeChangePlainEnumValueLabelAction(ProductTypeChangePlainEnumValueLabelAction),
   #[serde(rename = "changePlainEnumValueOrder")]
   EProductTypeChangePlainEnumValueOrderAction(ProductTypeChangePlainEnumValueOrderAction),
   #[serde(rename = "removeAttributeDefinition")]
   EProductTypeRemoveAttributeDefinitionAction(ProductTypeRemoveAttributeDefinitionAction),
   #[serde(rename = "removeEnumValues")]
   EProductTypeRemoveEnumValuesAction(ProductTypeRemoveEnumValuesAction),
   #[serde(rename = "setInputTip")]
   EProductTypeSetInputTipAction(ProductTypeSetInputTipAction),
   #[serde(rename = "setKey")]
   EProductTypeSetKeyAction(ProductTypeSetKeyAction),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TextInputHint {
   SingleLine,
   MultiLine
}

impl TextInputHint {
    pub fn from_str(s: &str) -> Option<TextInputHint> {
        match s {
            "SingleLine" => Some(TextInputHint::SingleLine),
            "MultiLine" => Some(TextInputHint::MultiLine),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           TextInputHint::SingleLine => "SingleLine",
           TextInputHint::MultiLine => "MultiLine",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeAddAttributeDefinitionAction {
   pub attribute: AttributeDefinitionDraft,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeAddLocalizedEnumValueAction {
   pub value: AttributeLocalizedEnumValue,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeAddPlainEnumValueAction {
   pub value: AttributePlainEnumValue,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeAttributeConstraintAction {
   pub new_value: AttributeConstraintEnumDraft,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeAttributeNameAction {
   pub attribute_name: String,
   pub new_attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeAttributeOrderAction {
   pub attributes: Vec<AttributeDefinition>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeAttributeOrderByNameAction {
   pub attribute_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeDescriptionAction {
   pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeEnumKeyAction {
   pub attribute_name: String,
   pub key: String,
   pub new_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeInputHintAction {
   pub new_value: TextInputHint,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeIsSearchableAction {
   pub is_searchable: bool,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeLabelAction {
   pub label: LocalizedString,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeLocalizedEnumValueLabelAction {
   pub new_value: AttributeLocalizedEnumValue,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeLocalizedEnumValueOrderAction {
   pub values: Vec<AttributeLocalizedEnumValue>,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangeNameAction {
   pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangePlainEnumValueLabelAction {
   pub new_value: AttributePlainEnumValue,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeChangePlainEnumValueOrderAction {
   pub values: Vec<AttributePlainEnumValue>,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeRemoveAttributeDefinitionAction {
   pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeRemoveEnumValuesAction {
   pub keys: Vec<String>,
   pub attribute_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeSetInputTipAction {
   pub attribute_name: String,
   pub input_tip: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeSetKeyAction {
   pub key: Option<String>,
}