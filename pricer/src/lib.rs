use types::*;

pub struct Pricer;

impl CanPriceProduct for Pricer {
    fn PriceProduct(id: &String) -> PriceRecord {
        let base_price = base_pricer::BasePricer::PriceProduct(&id);
        let holiday_price = holiday_pricer::HolidayPricer::PriceProduct(&id);

        let best_price = if base_price.price < holiday_price.price {
            base_price
        } else {
            holiday_price
        };

        return best_price;
    }
}
