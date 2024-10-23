use crate::{
    define_request,
    models::{spot::OrderBook, Product},
};
use reqwest::Method;
use rust_decimal::Decimal;

define_request! {
    Name => Ping;
    Product => Product::Spot;
    Method => Method::GET;
    Endpoint => "/api/v3/ping";
    Signed => false;
    Request => {};
    Response => {};
}

define_request! {
    Name => GetSymbolPrice;
    Product => Product::Spot;
    Method => Method::GET;
    Endpoint => "/api/v3/ticker/price";
    Signed => false;
    Request => {
        pub symbol: String,
    };
    Response => {
        pub symbol: String,
        pub price: Decimal,
    };
}

define_request! {
    Name => GetDepths;
    Product => Product::Spot;
    Method => Method::GET;
    Endpoint => "/api/v3/depth";
    Signed => false;
    Request => {
        pub symbol: String,
        pub limit: u64,
    };
    Response => OrderBook;
}
