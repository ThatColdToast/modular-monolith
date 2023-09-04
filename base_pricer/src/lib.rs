use types::*;

pub struct BasePricer;

impl CanPriceProduct for BasePricer {
    fn PriceProduct(id: &String) -> PriceRecord {
        PriceRecord {
            price: 150.0,
            is_sale: false,
        }
    }
}
