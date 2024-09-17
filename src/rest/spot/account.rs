use crate::{
    define_request,
    models::{
        spot::{AccountInformation, OrderInfo},
        OrderType, Product, Side, TimeInForce,
    },
};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::Deserialize;

define_request! {
    Name => GetExchangeInfo;
    Product => Product::Spot;
    Method => Method::GET;
    Endpoint => "/api/v3/exchangeInfo";
    Signed => false;
    Request => {};
    Response => {
        pub timezone: String,
        pub server_time: u64,
        pub rate_limits: Vec<serde_json::Value>,
        pub exchange_filters: Vec<serde_json::Value>,
        pub symbols: Vec<Symbol>,
    };
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub quote_precision: u64,
    pub quote_asset_precision: u64,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub allow_trailing_stop: bool,
    pub cancel_replace_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<serde_json::Value>,
    pub permissions: Vec<String>,
    pub permission_sets: Vec<Vec<String>>,
    pub default_self_trade_prevention_mode: String,
    pub allowed_self_trade_prevention_modes: Vec<String>,
}

define_request! {
    Name => GetAccount;
    Product => Product::Spot;
    Method => Method::GET;
    Endpoint => "/api/v3/account";
    Signed => true;
    Request => {};
    Response => AccountInformation;
}

define_request! {
    Name => Order;
    Product => Product::Spot;
    Method => Method::GET;
    Endpoint => "/api/v3/order";
    Signed => true;
    Request => {
        pub symbol: String,
        pub qty: Decimal,
        pub price: Option<Decimal>,
        pub stop_price: Option<Decimal>,
        pub order_side: Side,
        pub order_type: OrderType,
        pub time_in_force: TimeInForce,
        pub new_client_order_id: Option<String>,
    };
    Response => OrderInfo;
}
