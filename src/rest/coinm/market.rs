use crate::{
    define_request,
    models::{Filter, Product, RateLimit},
    parser::string_or_decimal,
};
use chrono::{
    serde::{ts_milliseconds, ts_milliseconds_option},
    DateTime, Utc,
};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

define_request! {
    Name => ExchangeInformation;
    Product => Product::CoinMFutures;
    Method => Method::GET;
    Endpoint => "/dapi/v1/exchangeInfo";
    Signed => false;
    Request => {};
    Response => {
        pub timezone: String,
        pub rate_limits: Vec<RateLimit>,
        pub server_time: u64,
        pub symbols: Vec<Symbol>
    };
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub pair: String,
    pub contract_type: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub margin_asset: String,
    pub contract_size: u64,
    pub delivery_date: u64,
    pub filters: Vec<Filter>,
}

define_request! {
    Name => FundingRate;
    Product => Product::CoinMFutures;
    Method => Method::GET;
    Endpoint => "/dapi/v1/fundingRate";
    Signed => false;
    Request => {
        pub symbol: String,
        #[serde(with = "ts_milliseconds_option")]
        pub start_time: Option<DateTime<Utc>>,
        #[serde(with = "ts_milliseconds_option")]
        pub end_time: Option<DateTime<Utc>>,
        pub limit: Option<u64>,
    };
    Response => Vec<FundingRate>;
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    #[serde(with = "string_or_decimal")]
    pub funding_rate: Decimal,
    #[serde(with = "ts_milliseconds")]
    pub funding_time: DateTime<Utc>,
}

define_request! {
    Name => GetDepths;
    Product => Product::CoinMFutures;
    Method => Method::GET;
    Endpoint => "/dapi/v1/depth";
    Signed => false;
    Request => {
        pub symbol: String,
        pub limit: Option<u64>,
    };
    Response => Depths;
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Depths {
    pub last_update_id: u64,
    pub symbol: String,
    pub pair: String,
    #[serde(rename = "E")]
    pub message_output_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    pub bids: Vec<[Decimal; 2]>,
    pub asks: Vec<[Decimal; 2]>,
}
