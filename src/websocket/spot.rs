use crate::{
    error::BinanceError, models::Product, parser::string_or_decimal, websocket::ParseMessage,
};
use fehler::throws;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "e")]
pub enum WebsocketMessage {
    Ping,
    #[serde(rename = "aggTrade")]
    AggregateTrade(AggregateTrade),
    #[serde(rename = "trade")]
    Trade(Trade),
    #[serde(rename = "kline")]
    Kline(Kline),
    #[serde(rename = "24hrMiniTicker")]
    MiniTicker(MiniTicker),
    #[serde(rename = "24hrTicker")]
    Ticker24hr(Ticker24hr),
    #[serde(rename = "1hTicker")]
    Ticker1hr(Ticker1hr),
    #[serde(rename = "avgPrice")]
    AveragePrice(AveragePrice),
    #[serde(rename = "depthUpdate")]
    DepthUpdate(DepthUpdate),
    #[serde(rename = "outboundAccountPosition")]
    OutboundAccountPosition(OutboundAccountPosition),
    #[serde(rename = "balanceUpdate")]
    BalanceUpdate(BalanceUpdate),
    #[serde(rename = "executionReport")]
    ExecutionReport(ExecutionReport),
    #[serde(rename = "listStatus")]
    ListStatus(ListStatus),
    #[serde(rename = "listenKeyExpired")]
    ListenKeyExpired(ListenKeyExpired),
}

impl ParseMessage for WebsocketMessage {
    const PRODUCT: Product = Product::Spot;

    #[throws(BinanceError)]
    fn parse(_: &str, data: &str) -> Self {
        serde_json::from_str(data)?
    }

    fn ping() -> Self {
        Self::Ping
    }
}

/// The Aggregate Trade Streams push trade information that is aggregated for a single taker order.
///
/// <https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#aggregate-trade-streams>
#[derive(Debug, Deserialize, Clone)]
pub struct AggregateTrade {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub aggregated_trade_id: u64,
    #[serde(rename = "p", with = "string_or_decimal")]
    pub price: Decimal,
    #[serde(rename = "q", with = "string_or_decimal")]
    pub qty: Decimal,
    #[serde(rename = "f")]
    pub first_break_trade_id: u64,
    #[serde(rename = "l")]
    pub last_break_trade_id: u64,
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Trade {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "p", with = "string_or_decimal")]
    pub price: Decimal,
    #[serde(rename = "q", with = "string_or_decimal")]
    pub qty: Decimal,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Kline {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline_data: KlineData,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KlineData {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    pub close_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "o", with = "string_or_decimal")]
    pub open_price: Decimal,
    #[serde(rename = "c", with = "string_or_decimal")]
    pub close_price: Decimal,
    #[serde(rename = "h", with = "string_or_decimal")]
    pub high_price: Decimal,
    #[serde(rename = "l", with = "string_or_decimal")]
    pub low_price: Decimal,
    #[serde(rename = "v", with = "string_or_decimal")]
    pub base_asset_volume: Decimal,
    #[serde(rename = "n")]
    pub number_of_trades: u64,
    pub is_closed: bool,
    #[serde(rename = "q", with = "string_or_decimal")]
    pub quote_asset_volume: Decimal,
    #[serde(rename = "V", with = "string_or_decimal")]
    pub taker_buy_base_asset_volume: Decimal,
    #[serde(rename = "Q", with = "string_or_decimal")]
    pub taker_buy_quote_asset_volume: Decimal,
    #[serde(skip, rename = "B")]
    pub ignore: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MiniTicker {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c", with = "string_or_decimal")]
    pub close_price: Decimal,
    #[serde(rename = "o", with = "string_or_decimal")]
    pub open_price: Decimal,
    #[serde(rename = "h", with = "string_or_decimal")]
    pub high_price: Decimal,
    #[serde(rename = "l", with = "string_or_decimal")]
    pub low_price: Decimal,
    #[serde(rename = "v", with = "string_or_decimal")]
    pub total_traded_base_asset_volume: Decimal,
    #[serde(rename = "q", with = "string_or_decimal")]
    pub total_traded_quote_asset_volume: Decimal,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ticker24hr {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p", with = "string_or_decimal")]
    pub price_change: Decimal,
    #[serde(rename = "P", with = "string_or_decimal")]
    pub price_change_percent: Decimal,
    #[serde(rename = "w", with = "string_or_decimal")]
    pub weighted_average_price: Decimal,
    #[serde(rename = "x", with = "string_or_decimal")]
    pub first_trade_price: Decimal,
    #[serde(rename = "c", with = "string_or_decimal")]
    pub last_price: Decimal,
    #[serde(rename = "Q", with = "string_or_decimal")]
    pub last_quantity: Decimal,
    #[serde(rename = "b", with = "string_or_decimal")]
    pub best_bid_price: Decimal,
    #[serde(rename = "B", with = "string_or_decimal")]
    pub best_bid_quantity: Decimal,
    #[serde(rename = "a", with = "string_or_decimal")]
    pub best_ask_price: Decimal,
    #[serde(rename = "A", with = "string_or_decimal")]
    pub best_ask_quantity: Decimal,
    #[serde(rename = "o", with = "string_or_decimal")]
    pub open_price: Decimal,
    #[serde(rename = "h", with = "string_or_decimal")]
    pub high_price: Decimal,
    #[serde(rename = "l", with = "string_or_decimal")]
    pub low_price: Decimal,
    #[serde(rename = "v", with = "string_or_decimal")]
    pub total_traded_base_asset_volume: Decimal,
    #[serde(rename = "q", with = "string_or_decimal")]
    pub total_traded_quote_asset_volume: Decimal,
    #[serde(rename = "O")]
    pub statistics_open_time: u64,
    #[serde(rename = "C")]
    pub statistics_close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub total_number_of_trades: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ticker1hr {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p", with = "string_or_decimal")]
    pub price_change: Decimal,
    #[serde(rename = "P", with = "string_or_decimal")]
    pub price_change_percent: Decimal,
    #[serde(rename = "o", with = "string_or_decimal")]
    pub open_price: Decimal,
    #[serde(rename = "h", with = "string_or_decimal")]
    pub high_price: Decimal,
    #[serde(rename = "l", with = "string_or_decimal")]
    pub low_price: Decimal,
    #[serde(rename = "c", with = "string_or_decimal")]
    pub last_price: Decimal,
    #[serde(rename = "w", with = "string_or_decimal")]
    pub weighted_average_price: Decimal,
    #[serde(rename = "v", with = "string_or_decimal")]
    pub total_traded_base_asset_volume: Decimal,
    #[serde(rename = "q", with = "string_or_decimal")]
    pub total_traded_quote_asset_volume: Decimal,
    #[serde(rename = "O")]
    pub statistics_open_time: u64,
    #[serde(rename = "C")]
    pub statistics_close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub total_number_of_trades: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AveragePrice {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "w", with = "string_or_decimal")]
    pub weighted_average_price: Decimal,
    #[serde(rename = "T")]
    pub last_trade_time: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DepthUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<(String, String)>,
    #[serde(rename = "a")]
    pub asks: Vec<(String, String)>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OutboundAccountPosition {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "u")]
    pub last_account_update_time: u64,
    #[serde(rename = "B")]
    pub balances: Vec<Balance>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Balance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f", with = "string_or_decimal")]
    pub free: Decimal,
    #[serde(rename = "l", with = "string_or_decimal")]
    pub locked: Decimal,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BalanceUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "d", with = "string_or_decimal")]
    pub balance_delta: Decimal,
    #[serde(rename = "T")]
    pub clear_time: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExecutionReport {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "o")]
    pub order_type: String,
    #[serde(rename = "f")]
    pub time_in_force: String,
    #[serde(rename = "q", with = "string_or_decimal")]
    pub order_quantity: Decimal,
    #[serde(rename = "p", with = "string_or_decimal")]
    pub order_price: Decimal,
    #[serde(rename = "P", with = "string_or_decimal")]
    pub stop_price: Decimal,
    #[serde(rename = "F", with = "string_or_decimal")]
    pub iceberg_quantity: Decimal,
    #[serde(rename = "g")]
    pub order_list_id: i64,
    #[serde(rename = "C")]
    pub original_client_order_id: String,
    #[serde(rename = "x")]
    pub current_execution_type: String,
    #[serde(rename = "X")]
    pub current_order_status: String,
    #[serde(rename = "r")]
    pub order_reject_reason: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l", with = "string_or_decimal")]
    pub last_executed_quantity: Decimal,
    #[serde(rename = "z", with = "string_or_decimal")]
    pub cumulative_filled_quantity: Decimal,
    #[serde(rename = "L", with = "string_or_decimal")]
    pub last_executed_price: Decimal,
    #[serde(rename = "n", with = "string_or_decimal")]
    pub commission_amount: Decimal,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(rename = "w")]
    pub is_working: bool,
    #[serde(rename = "m")]
    pub is_maker: bool,
    #[serde(rename = "O")]
    pub order_creation_time: u64,
    #[serde(rename = "Z", with = "string_or_decimal")]
    pub cumulative_quote_asset_transacted_quantity: Decimal,
    #[serde(rename = "Y", with = "string_or_decimal")]
    pub last_quote_asset_transacted_quantity: Decimal,
    #[serde(rename = "Q", with = "string_or_decimal")]
    pub quote_order_quantity: Decimal,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ListStatus {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "g")]
    pub order_list_id: u64,
    #[serde(rename = "c")]
    pub contingency_type: String,
    #[serde(rename = "l")]
    pub list_status_type: String,
    #[serde(rename = "L")]
    pub list_order_status: String,
    #[serde(rename = "r")]
    pub list_reject_reason: String,
    #[serde(rename = "C")]
    pub list_client_order_id: String,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "O")]
    pub orders: Vec<ListStatusOrder>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ListStatusOrder {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "c")]
    pub client_order_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ListenKeyExpired {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "listenKey")]
    pub listen_key: String,
}
