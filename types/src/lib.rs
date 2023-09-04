#[derive(Clone, Debug)]
pub struct ProductRecord {
    pub name: String,
    pub id: String,
}

pub struct SearchFilter {
    pub page: u32,
    pub page_size: u32,
    pub keyword: String,
}

#[derive(Clone, Debug)]
pub struct PriceRecord {
    pub price: f32,
    pub is_sale: bool,
}

#[derive(Clone, Debug)]
pub struct OrderRecord {
    pub date_unix_time: i64,
    pub price: f32,
}

pub trait CanFindProduct {
    fn FindProducts(filter: SearchFilter) -> Vec<ProductRecord>;
}

pub trait CanPriceProduct {
    fn PriceProduct(id: &String) -> PriceRecord;
}

pub trait CanFindOrder {
    fn FindOrder(customerId: &String, productId: &String) -> Option<OrderRecord>;
    // fn FindOrders(customerId: String, productId: String) -> Option<Vec<OrderRecord>>;
}
