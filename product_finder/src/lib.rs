use types::*;

pub struct ProductFinder;

impl CanFindProduct for ProductFinder {
    fn FindProducts(filter: SearchFilter) -> Vec<ProductRecord> {
        return vec![ProductRecord {
            name: "PRODUCT_NAME".to_string(),
            id: "PRODUCT_ID".to_string(),
        }];
    }
}
