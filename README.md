TODO:

- Implement discriminator values (update actions) in enum type
- Renamed reserved keyword type (serde rename/module name 'type' -> 'cttype') -> Fix duplicate imports in type.rs
- Fix enum serialization for carts (see  paged response for carts).
- Generate client code
- Implement client services for all types (get/create/update/delete/query)

- Fix incorrect imports in orderedit.rs
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

