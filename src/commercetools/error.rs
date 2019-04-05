//Generated file, please do not change

use super::product::Attribute;
use super::common::Reference;
use super::common::Price;
use super::customergroup::CustomerGroupReference;
use super::channel::ChannelReference;
use super::common::ReferenceTypeId;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccessDeniedError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConcurrentModificationError {
   pub message: String,
   pub current_version: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCodeNonApplicableError {
   pub message: String,
   pub valid_from: Option<DateTime<Utc>>,
   pub valid_until: Option<DateTime<Utc>>,
   pub validity_check_time: Option<DateTime<Utc>>,
   pub dicount_code_id: Option<String>,
   pub discount_code: Option<String>,
   pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateAttributeValueError {
   pub attribute: Attribute,
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateAttributeValuesError {
   pub attributes: Vec<Attribute>,
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateFieldError {
   pub message: String,
   pub duplicate_value: Option<serde_json::Value>,
   pub field: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateFieldWithConflictingResourceError {
   pub conflicting_resource: Reference,
   pub duplicate_value: serde_json::Value,
   pub field: String,
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DuplicatePriceScopeError {
   pub conflicting_prices: Vec<Price>,
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateVariantValuesError {
   pub variant_values: VariantValues,
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValueIsUsedError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "code")]
pub enum ErrorObject {
   #[serde(rename = "ExtensionBadResponse")]
   EExtensionBadResponseError(ExtensionBadResponseError),
   #[serde(rename = "ExtensionNoResponse")]
   EExtensionNoResponseError(ExtensionNoResponseError),
   #[serde(rename = "ExtensionUpdateActionsFailed")]
   EExtensionUpdateActionsFailedError(ExtensionUpdateActionsFailedError),
   #[serde(rename = "insufficient_scope")]
   EInsufficientScopeError(InsufficientScopeError),
   #[serde(rename = "InvalidCredentials")]
   EInvalidCredentialsError(InvalidCredentialsError),
   #[serde(rename = "InvalidCurrentPassword")]
   EInvalidCurrentPasswordError(InvalidCurrentPasswordError),
   #[serde(rename = "InvalidField")]
   EInvalidFieldError(InvalidFieldError),
   #[serde(rename = "InvalidInput")]
   EInvalidInputError(InvalidInputError),
   #[serde(rename = "InvalidItemShippingDetails")]
   EInvalidItemShippingDetailsError(InvalidItemShippingDetailsError),
   #[serde(rename = "InvalidJsonInput")]
   EInvalidJsonInputError(InvalidJsonInputError),
   #[serde(rename = "InvalidOperation")]
   EInvalidOperationError(InvalidOperationError),
   #[serde(rename = "InvalidSubject")]
   EInvalidSubjectError(InvalidSubjectError),
   #[serde(rename = "invalid_token")]
   EInvalidTokenError(InvalidTokenError),
   #[serde(rename = "MatchingPriceNotFound")]
   EMatchingPriceNotFoundError(MatchingPriceNotFoundError),
   #[serde(rename = "MissingTaxRateForCountry")]
   EMissingTaxRateForCountryError(MissingTaxRateForCountryError),
   #[serde(rename = "NoMatchingProductDiscountFound")]
   ENoMatchingProductDiscountFoundError(NoMatchingProductDiscountFoundError),
   #[serde(rename = "OutOfStock")]
   EOutOfStockError(OutOfStockError),
   #[serde(rename = "PriceChanged")]
   EPriceChangedError(PriceChangedError),
   #[serde(rename = "ReferenceExists")]
   EReferenceExistsError(ReferenceExistsError),
   #[serde(rename = "RequiredField")]
   ERequiredFieldError(RequiredFieldError),
   #[serde(rename = "ResourceNotFound")]
   EResourceNotFoundError(ResourceNotFoundError),
   #[serde(rename = "ShippingMethodDoesNotMatchCart")]
   EShippingMethodDoesNotMatchCartError(ShippingMethodDoesNotMatchCartError),
   #[serde(rename = "DuplicateField")]
   EDuplicateFieldError(DuplicateFieldError),
   #[serde(rename = "DuplicateFieldWithConflictingResource")]
   EDuplicateFieldWithConflictingResourceError(DuplicateFieldWithConflictingResourceError),
   #[serde(rename = "DuplicateAttributeValue")]
   EDuplicateAttributeValueError(DuplicateAttributeValueError),
   #[serde(rename = "DuplicatePriceScope")]
   EDuplicatePriceScopeError(DuplicatePriceScopeError),
   #[serde(rename = "access_denied")]
   EAccessDeniedError(AccessDeniedError),
   #[serde(rename = "EnumValueIsUsed")]
   EEnumValueIsUsedError(EnumValueIsUsedError),
   #[serde(rename = "DuplicateAttributeValues")]
   EDuplicateAttributeValuesError(DuplicateAttributeValuesError),
   #[serde(rename = "ConcurrentModification")]
   EConcurrentModificationError(ConcurrentModificationError),
   #[serde(rename = "DiscountCodeNonApplicable")]
   EDiscountCodeNonApplicableError(DiscountCodeNonApplicableError),
   #[serde(rename = "DuplicateVariantValues")]
   EDuplicateVariantValuesError(DuplicateVariantValuesError),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
   pub status_code: u32,
   pub message: String,
   pub errors: Option<Vec<ErrorObject>>,
   pub error: Option<String>,
   pub error_description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionBadResponseError {
   pub extension_id: String,
   pub message: String,
   pub extension_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionNoResponseError {
   pub extension_id: String,
   pub message: String,
   pub extension_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionUpdateActionsFailedError {
   pub extension_id: String,
   pub message: String,
   pub extension_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsufficientScopeError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvalidCredentialsError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvalidCurrentPasswordError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvalidFieldError {
   pub invalid_value: serde_json::Value,
   pub field: String,
   pub message: String,
   pub allowed_values: Option<Vec<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvalidInputError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvalidItemShippingDetailsError {
   pub item_id: String,
   pub message: String,
   pub subject: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvalidJsonInputError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvalidOperationError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvalidSubjectError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvalidTokenError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchingPriceNotFoundError {
   pub variant_id: u32,
   pub message: String,
   pub product_id: String,
   pub channel: Option<ChannelReference>,
   pub customer_group: Option<CustomerGroupReference>,
   pub country: Option<String>,
   pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissingTaxRateForCountryError {
   pub message: String,
   pub tax_category_id: String,
   pub country: Option<String>,
   pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoMatchingProductDiscountFoundError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutOfStockError {
   pub line_items: Vec<String>,
   pub skus: Vec<String>,
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceChangedError {
   pub line_items: Vec<String>,
   pub shipping: bool,
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceExistsError {
   pub message: String,
   pub referenced_by: Option<ReferenceTypeId>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequiredFieldError {
   pub field: String,
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceNotFoundError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingMethodDoesNotMatchCartError {
   pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VariantValues {
   pub attributes: Vec<Attribute>,
   pub prices: Vec<Price>,
   pub sku: Option<String>,
}