use types::*;

pub struct HolidayPricer;

impl CanPriceProduct for HolidayPricer {
    fn PriceProduct(id: &String) -> PriceRecord {
        PriceRecord {
            price: 100.0,
            is_sale: true,
        }
    }
}
