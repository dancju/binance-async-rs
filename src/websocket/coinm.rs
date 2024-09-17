use crate::{
    error::BinanceError,
    models::Product,
    parser::{string_or_decimal, string_or_decimal_opt},
    websocket::ParseMessage,
};
use fehler::{throw, throws};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum WebsocketMessage {
    Ping,
    AggregateTrade(super::models::AggregateTrade),
    IndexPrice(IndexPrice),
    MarkPrice(MarkPrice),
}

impl ParseMessage for WebsocketMessage {
    const PRODUCT: Product = Product::CoinMFutures;

    #[throws(BinanceError)]
    fn parse(stream: &str, data: &str) -> Self {
        if stream.ends_with("@aggTrade") {
            Self::AggregateTrade(from_str(data)?)
        } else if stream.contains("@indexPrice") {
            Self::IndexPrice(from_str(data)?)
        } else if stream.contains("@markPrice") {
            Self::MarkPrice(from_str(data)?)
        } else if stream.starts_with("!markPrice@arr") {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream.contains("@kline_") {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream.contains("@continuousKline_") {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@miniTicker") {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream == "!miniTicker@arr" {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@ticker") {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream == "!ticker@arr" {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@bookTicker") {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream == "!bookTicker" {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@forceOrder") {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream == "!forceOrder@arr" {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@depth") || stream.contains("@depth@") {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream.contains("@depth") {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@compositeIndex") {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else if stream == "!contractInfo" {
            throw!(BinanceError::StreamNotImplemented(stream.into()))
        } else {
            throw!(BinanceError::UnknownStream(stream.into()))
        }
    }

    fn ping() -> Self {
        Self::Ping
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct IndexPrice {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "i")]
    pub symbol: String,
    #[serde(rename = "p", with = "string_or_decimal")]
    pub index_price: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarkPrice {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p", with = "string_or_decimal")]
    pub mark_price: Decimal,
    #[serde(rename = "P", with = "string_or_decimal")]
    pub estimated_settle_price: Decimal,
    #[serde(rename = "i", with = "string_or_decimal")]
    pub index_price: Decimal,
    #[serde(rename = "r", with = "string_or_decimal_opt")]
    pub funding_rate: Option<Decimal>,
    #[serde(rename = "T")]
    pub next_funding_time: u64,
}
