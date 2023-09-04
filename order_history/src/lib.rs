use types::*;

pub struct OrderHistory;

impl CanFindOrder for OrderHistory {
    fn FindOrder(customerId: &String, productId: &String) -> Option<OrderRecord> {
        Some(OrderRecord {
            date_unix_time: 0,
            price: 99.0,
        })
    }
}
