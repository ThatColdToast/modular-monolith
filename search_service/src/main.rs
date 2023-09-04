use product_finder::ProductFinder;
use types::*;

// For now I will pretend that this is listening for requests
fn main() {
    println!("Totally Listening for connections on 127.0.0.1:8080");

    let result = search(
        "USERID_HERE".to_string(),
        SearchFilter {
            page: 0,
            page_size: 50,
            keyword: "QUERY_STRING".to_string(),
        },
    );

    println!("Result: {:?}", result);
}

#[derive(Debug)]
struct SearchResult {
    pub product: ProductRecord,
    pub price: PriceRecord,
    pub history: Option<OrderRecord>,
}

fn search(customerId: String, filter: SearchFilter) -> Vec<SearchResult> {
    let products = ProductFinder::FindProducts(filter);
    let mut product_price_history = Vec::new();

    // Better if done with bulk operations for performance
    // but this gets the point across
    for p in products {
        product_price_history.push(SearchResult {
            product: p.clone(),
            price: pricer::Pricer::PriceProduct(&p.id),
            history: order_history::OrderHistory::FindOrder(&customerId, &p.id),
        });
    }

    return product_price_history;
}
