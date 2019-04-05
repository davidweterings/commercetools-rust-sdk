//Generated file, please do not change

use super::producttype::ProductTypeReference;
use super::taxcategory::TaxCategoryReference;
use super::state::StateReference;
use super::review::ReviewRatingStatistics;
use super::common::Resource;
use super::common::LocalizedString;
use super::category::CategoryReference;
use super::common::Reference;
use super::common::Price;
use super::common::Image;
use super::common::Asset;
use super::common::ScopedPrice;
use super::common::PriceDraft;
use super::common::AssetDraft;
use super::cart::ProductPublishScope;
use super::cttype::TypeReference;
use super::common::AssetSource;
use super::common::DiscountedPrice;
use super::cttype::FieldContainer;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
   pub value: serde_json::Value,
   pub name: String,
}

pub type CategoryOrderHints = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomTokenizer {
   pub inputs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FacetResult {
   pub r#type: FacetTypes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FacetResultRange {
   pub count: u64,
   pub from: u32,
   pub max: u32,
   pub mean: u32,
   pub min: u32,
   pub to: u32,
   pub total: u64,
   pub from_str: String,
   pub to_str: String,
   pub product_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FacetResultTerm {
   pub term: serde_json::Value,
   pub count: u64,
   pub product_count: Option<u64>,
}

pub type FacetResults = HashMap<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Debug)]
pub enum FacetTypes {
   Terms,
   Range,
   Filter
}

impl FacetTypes {
    pub fn from_str(s: &str) -> Option<FacetTypes> {
        match s {
            "terms" => Some(FacetTypes::Terms),
            "range" => Some(FacetTypes::Range),
            "filter" => Some(FacetTypes::Filter),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           FacetTypes::Terms => "terms",
           FacetTypes::Range => "range",
           FacetTypes::Filter => "filter",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FilteredFacetResult {
   pub count: u64,
   pub product_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Product {
   pub master_data: ProductCatalogData,
   pub product_type: ProductTypeReference,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub review_rating_statistics: Option<ReviewRatingStatistics>,
   pub state: Option<StateReference>,
   pub tax_category: Option<TaxCategoryReference>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductCatalogData {
   pub current: ProductData,
   pub staged: ProductData,
   pub has_staged_changes: bool,
   pub published: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductData {
   pub categories: Vec<CategoryReference>,
   pub variants: Vec<ProductVariant>,
   pub name: LocalizedString,
   pub slug: LocalizedString,
   pub master_variant: ProductVariant,
   pub search_keywords: SearchKeywords,
   pub category_order_hints: Option<CategoryOrderHints>,
   pub description: Option<LocalizedString>,
   pub meta_description: Option<LocalizedString>,
   pub meta_keywords: Option<LocalizedString>,
   pub meta_title: Option<LocalizedString>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDraft {
   pub name: LocalizedString,
   pub slug: LocalizedString,
   pub categories: Option<Vec<CategoryReference>>,
   pub variants: Option<Vec<ProductVariantDraft>>,
   pub category_order_hints: Option<CategoryOrderHints>,
   pub description: Option<LocalizedString>,
   pub meta_description: Option<LocalizedString>,
   pub meta_keywords: Option<LocalizedString>,
   pub meta_title: Option<LocalizedString>,
   pub product_type: Option<ProductTypeReference>,
   pub master_variant: Option<ProductVariantDraft>,
   pub search_keywords: Option<SearchKeywords>,
   pub state: Option<StateReference>,
   pub tax_category: Option<TaxCategoryReference>,
   pub publish: Option<bool>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductPagedQueryResponse {
   pub results: Vec<Product>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductProjection {
   pub categories: Vec<CategoryReference>,
   pub variants: Vec<ProductVariant>,
   pub name: LocalizedString,
   pub slug: LocalizedString,
   pub product_type: ProductTypeReference,
   pub master_variant: ProductVariant,
   pub created_at: DateTime<Utc>,
   pub last_modified_at: DateTime<Utc>,
   pub version: u64,
   pub id: String,
   pub category_order_hints: Option<CategoryOrderHints>,
   pub description: Option<LocalizedString>,
   pub meta_description: Option<LocalizedString>,
   pub meta_keywords: Option<LocalizedString>,
   pub meta_title: Option<LocalizedString>,
   pub review_rating_statistics: Option<ReviewRatingStatistics>,
   pub search_keywords: Option<SearchKeywords>,
   pub state: Option<StateReference>,
   pub tax_category: Option<TaxCategoryReference>,
   pub has_staged_changes: Option<bool>,
   pub published: Option<bool>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductProjectionPagedQueryResponse {
   pub results: Vec<ProductProjection>,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductProjectionPagedSearchResponse {
   pub results: Vec<ProductProjection>,
   pub facets: FacetResults,
   pub count: u64,
   pub offset: u64,
   pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductReference {
   pub obj: Option<Product>,
   pub id: Option<String>,
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductUpdate {
   pub actions: Vec<ProductUpdateAction>,
   pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductUpdateAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductVariant {
   pub id: u64,
   pub assets: Option<Vec<Asset>>,
   pub attributes: Option<Vec<Attribute>>,
   pub images: Option<Vec<Image>>,
   pub prices: Option<Vec<Price>>,
   pub price: Option<Price>,
   pub availability: Option<ProductVariantAvailability>,
   pub scoped_price: Option<ScopedPrice>,
   pub is_matching_variant: Option<bool>,
   pub scoped_price_discounted: Option<bool>,
   pub key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductVariantAvailability {
   pub channels: Option<ProductVariantChannelAvailabilityMap>,
   pub is_on_stock: Option<bool>,
   pub available_quantity: Option<u64>,
   pub restockable_in_days: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductVariantChannelAvailability {
   pub is_on_stock: Option<bool>,
   pub available_quantity: Option<u64>,
   pub restockable_in_days: Option<u64>,
}

pub type ProductVariantChannelAvailabilityMap = HashMap<String, ProductVariantChannelAvailability>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductVariantDraft {
   pub assets: Option<Vec<AssetDraft>>,
   pub attributes: Option<Vec<Attribute>>,
   pub images: Option<Vec<Image>>,
   pub prices: Option<Vec<PriceDraft>>,
   pub key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RangeFacetResult {
   pub ranges: Vec<FacetResultRange>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchKeyword {
   pub text: String,
   pub suggest_tokenizer: Option<serde_json::Value>,
}

pub type SearchKeywords = HashMap<String, Vec<SearchKeyword>>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuggestTokenizer {
   pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Suggestion {
   pub text: String,
}

pub type SuggestionResult = HashMap<String, Vec<Suggestion>>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TermFacetResult {
   pub terms: Vec<FacetResultTerm>,
   pub data_type: TermFacetResultType,
   pub missing: u64,
   pub other: u64,
   pub total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TermFacetResultType {
   Text,
   Date,
   Time,
   Datetime,
   Boolean,
   Number
}

impl TermFacetResultType {
    pub fn from_str(s: &str) -> Option<TermFacetResultType> {
        match s {
            "text" => Some(TermFacetResultType::Text),
            "date" => Some(TermFacetResultType::Date),
            "time" => Some(TermFacetResultType::Time),
            "datetime" => Some(TermFacetResultType::Datetime),
            "boolean" => Some(TermFacetResultType::Boolean),
            "number" => Some(TermFacetResultType::Number),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
           TermFacetResultType::Text => "text",
           TermFacetResultType::Date => "date",
           TermFacetResultType::Time => "time",
           TermFacetResultType::Datetime => "datetime",
           TermFacetResultType::Boolean => "boolean",
           TermFacetResultType::Number => "number",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WhitespaceTokenizer {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductAddAssetAction {
   pub asset: AssetDraft,
   pub staged: Option<bool>,
   pub position: Option<u32>,
   pub variant_id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductAddExternalImageAction {
   pub image: Image,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductAddPriceAction {
   pub price: PriceDraft,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductAddToCategoryAction {
   pub category: CategoryReference,
   pub staged: Option<bool>,
   pub order_hint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductAddVariantAction {
   pub assets: Option<Vec<Asset>>,
   pub attributes: Option<Vec<Attribute>>,
   pub images: Option<Vec<Image>>,
   pub prices: Option<Vec<PriceDraft>>,
   pub staged: Option<bool>,
   pub key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductChangeAssetNameAction {
   pub name: LocalizedString,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductChangeAssetOrderAction {
   pub asset_order: Vec<String>,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductChangeMasterVariantAction {
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductChangeNameAction {
   pub name: LocalizedString,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductChangePriceAction {
   pub price: PriceDraft,
   pub price_id: String,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductChangeSlugAction {
   pub slug: LocalizedString,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductLegacySetSkuAction {
   pub variant_id: u32,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductMoveImageToPositionAction {
   pub position: u64,
   pub image_url: String,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductPublishAction {
   pub scope: Option<ProductPublishScope>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductRemoveAssetAction {
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductRemoveFromCategoryAction {
   pub category: CategoryReference,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductRemoveImageAction {
   pub image_url: String,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductRemovePriceAction {
   pub price_id: String,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductRemoveVariantAction {
   pub staged: Option<bool>,
   pub id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductRevertStagedChangesAction {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductRevertStagedVariantChangesAction {
   pub variant_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetAssetCustomFieldAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetAssetCustomTypeAction {
   pub r#type: Option<TypeReference>,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub fields: Option<serde_json::Value>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetAssetDescriptionAction {
   pub description: Option<LocalizedString>,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetAssetKeyAction {
   pub asset_id: String,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub asset_key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetAssetSourcesAction {
   pub sources: Vec<AssetSource>,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetAssetTagsAction {
   pub tags: Option<Vec<String>>,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub asset_id: Option<String>,
   pub asset_key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetAttributeAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetAttributeInAllVariantsAction {
   pub name: String,
   pub value: Option<serde_json::Value>,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetCategoryOrderHintAction {
   pub category_id: String,
   pub staged: Option<bool>,
   pub order_hint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetDescriptionAction {
   pub description: Option<LocalizedString>,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetDiscountedPriceAction {
   pub price_id: String,
   pub discounted: Option<DiscountedPrice>,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetImageLabelAction {
   pub image_url: String,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub label: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetKeyAction {
   pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetMetaDescriptionAction {
   pub meta_description: Option<LocalizedString>,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetMetaKeywordsAction {
   pub meta_keywords: Option<LocalizedString>,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetMetaTitleAction {
   pub meta_title: Option<LocalizedString>,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetPricesAction {
   pub prices: Vec<PriceDraft>,
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetProductPriceCustomFieldAction {
   pub name: String,
   pub price_id: String,
   pub value: Option<serde_json::Value>,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetProductPriceCustomTypeAction {
   pub price_id: String,
   pub fields: Option<FieldContainer>,
   pub r#type: Option<TypeReference>,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetProductVariantKeyAction {
   pub staged: Option<bool>,
   pub variant_id: Option<u64>,
   pub key: Option<String>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetSearchKeywordsAction {
   pub search_keywords: SearchKeywords,
   pub staged: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetSkuAction {
   pub variant_id: u64,
   pub staged: Option<bool>,
   pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductSetTaxCategoryAction {
   pub tax_category: Option<TaxCategoryReference>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductTransitionStateAction {
   pub state: Option<StateReference>,
   pub force: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductUnpublishAction {
}