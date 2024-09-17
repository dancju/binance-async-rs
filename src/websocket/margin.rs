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
    #[serde(rename = "ACCOUNT_CONFIG_UPDATE")]
    AccountConfigUpdate {
        #[serde(rename = "fs")]
        business_unit: String,
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "T")]
        transaction_time: u64,
        #[serde(rename = "ac")]
        account_config: AccountConfig,
    },
    #[serde(rename = "ACCOUNT_UPDATE")]
    AccountUpdate {
        #[serde(rename = "fs")]
        business_unit: String,
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "T")]
        transaction_time: u64,
        #[serde(rename = "i")]
        account_alias: String,
        #[serde(rename = "a")]
        update_data: UpdateData,
    },
    #[serde(rename = "balanceUpdate")]
    BalanceUpdate {
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "a")]
        asset: String,
        #[serde(rename = "d", with = "string_or_decimal")]
        balance_delta: Decimal,
        #[serde(rename = "U")]
        event_update_id: u64,
        #[serde(rename = "T")]
        clear_time: u64,
    },
    #[serde(rename = "CONDITIONAL_ORDER_TRADE_UPDATE")]
    ConditionalOrderTradeUpdate {
        #[serde(rename = "T")]
        transaction_time: u64,
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "fs")]
        business_unit: String,
        #[serde(rename = "so")]
        strategy_order: ConditionalOrderTradeUpdate,
    },
    #[serde(rename = "executionReport")]
    ExecutionReport {
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "c")]
        client_order_id: String,
        #[serde(rename = "S")]
        side: String,
        #[serde(rename = "o")]
        order_type: String,
        #[serde(rename = "f")]
        time_in_force: String,
        #[serde(rename = "q", with = "string_or_decimal")]
        order_quantity: Decimal,
        #[serde(rename = "p", with = "string_or_decimal")]
        order_price: Decimal,
        #[serde(rename = "P", with = "string_or_decimal")]
        stop_price: Decimal,
        #[serde(rename = "d")]
        trailing_delta: Decimal,
        #[serde(rename = "F", with = "string_or_decimal")]
        iceberg_quantity: Decimal,
        #[serde(rename = "g")]
        order_list_id: i64,
        #[serde(rename = "C")]
        original_client_order_id: String,
        #[serde(rename = "x")]
        current_execution_type: String,
        #[serde(rename = "X")]
        current_order_status: String,
        #[serde(rename = "r")]
        order_reject_reason: String,
        #[serde(rename = "i")]
        order_id: u64,
        #[serde(rename = "l", with = "string_or_decimal")]
        last_executed_quantity: Decimal,
        #[serde(rename = "z", with = "string_or_decimal")]
        cumulative_filled_quantity: Decimal,
        #[serde(rename = "L", with = "string_or_decimal")]
        last_executed_price: Decimal,
        #[serde(rename = "n")]
        commission_amount: Option<String>,
        #[serde(rename = "N")]
        commission_asset: Option<String>,
        #[serde(rename = "T")]
        order_trade_time: u64,
        #[serde(rename = "t")]
        trade_id: i64,
        #[serde(rename = "v")]
        prevent_match_id: i64,
        #[serde(rename = "w")]
        is_order_on_book: bool,
        #[serde(rename = "m")]
        is_maker: bool,
        #[serde(rename = "O")]
        order_creation_time: u64,
        #[serde(rename = "Z", with = "string_or_decimal")]
        cumulative_quote_asset_transacted_quantity: Decimal,
        #[serde(rename = "Y", with = "string_or_decimal")]
        last_quote_asset_transacted_quantity: Decimal,
        #[serde(rename = "Q", with = "string_or_decimal")]
        quote_order_quantity: Decimal,
        #[serde(rename = "D")]
        trailing_time: u64,
        #[serde(rename = "j")]
        strategy_id: u64,
        #[serde(rename = "J")]
        strategy_type: u64,
        #[serde(rename = "W")]
        working_time: u64,
        #[serde(rename = "V")]
        self_trade_prevention_mode: String,
        #[serde(rename = "u")]
        trade_group_id: u64,
        #[serde(rename = "U")]
        counter_order_id: u64,
        #[serde(rename = "A")]
        prevented_quantity: Decimal,
        #[serde(rename = "B")]
        last_prevented_quantity: Decimal,
    },
    #[serde(rename = "liabilityChange")]
    LiabilityChange {
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "a")]
        asset: String,
        #[serde(rename = "t")]
        r#type: String,
        #[serde(rename = "tx")]
        transaction_id: u64,
        #[serde(rename = "p", with = "string_or_decimal")]
        principal: Decimal,
        #[serde(rename = "i")]
        interest: Decimal,
        #[serde(rename = "l", with = "string_or_decimal")]
        total_liability: Decimal,
    },
    #[serde(rename = "listenKeyExpired")]
    ListenKeyExpired {
        #[serde(rename = "E")]
        event_time: u64,
    },
    #[serde(rename = "openOrderLoss")]
    OpenOrderLoss {
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "O")]
        update_data: Vec<OpenOrderLossUpdate>,
    },
    #[serde(rename = "ORDER_TRADE_UPDATE")]
    OrderTradeUpdate {
        #[serde(rename = "fs")]
        business_unit: String,
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "T")]
        transaction_time: u64,
        #[serde(rename = "i")]
        account_alias: String,
        #[serde(rename = "o")]
        update: OrderTradeUpdate,
    },
    #[serde(rename = "outboundAccountPosition")]
    OutboundAccountPosition {
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "u")]
        time_of_last_account_update: u64,
        #[serde(rename = "U")]
        event_update_id: u64,
        #[serde(rename = "B")]
        balances: Vec<BalancePosition>,
    },
    #[serde(rename = "riskLevelChange")]
    RiskLevelChange {
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "u", with = "string_or_decimal")]
        maintenance_margin_rate: Decimal,
        #[serde(rename = "s")]
        status: String,
        #[serde(rename = "eq", with = "string_or_decimal")]
        account_equity: Decimal,
        #[serde(rename = "ae", with = "string_or_decimal")]
        actual_equity: Decimal,
    },
}

impl ParseMessage for WebsocketMessage {
    const PRODUCT: Product = Product::PortfolioMargin;

    #[throws(BinanceError)]
    fn parse(_: &str, data: &str) -> Self {
        serde_json::from_str(data)?
    }

    fn ping() -> Self {
        Self::Ping
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountConfig {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l")]
    pub leverage: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateData {
    #[serde(rename = "m")]
    pub event_reason_type: String,
    #[serde(rename = "B")]
    pub balances: Vec<Balance>,
    #[serde(rename = "P")]
    pub positions: Vec<Position>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb", with = "string_or_decimal")]
    pub wallet_balance: Decimal,
    #[serde(rename = "cw", with = "string_or_decimal")]
    pub cross_wallet_balance: Decimal,
    #[serde(rename = "bc", with = "string_or_decimal")]
    pub balance_change: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Position {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa", with = "string_or_decimal")]
    pub position_amount: Decimal,
    #[serde(rename = "ep", with = "string_or_decimal")]
    pub entry_price: Decimal,
    #[serde(rename = "cr", with = "string_or_decimal")]
    pub accumulated_realized: Decimal,
    #[serde(rename = "up", with = "string_or_decimal")]
    pub unrealized_pnl: Decimal,
    #[serde(rename = "ps")]
    pub position_side: String,
    #[serde(rename = "bep", with = "string_or_decimal")]
    pub breakeven_price: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConditionalOrderTradeUpdate {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub strategy_client_order_id: String,
    #[serde(rename = "si")]
    pub strategy_id: u64,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "st")]
    pub strategy_type: String,
    #[serde(rename = "f")]
    pub time_in_force: String,
    #[serde(rename = "q", with = "string_or_decimal")]
    pub quantity: Decimal,
    #[serde(rename = "p", with = "string_or_decimal")]
    pub price: Decimal,
    #[serde(rename = "sp", with = "string_or_decimal")]
    pub stop_price: Decimal,
    #[serde(rename = "os")]
    pub strategy_order_status: String,
    #[serde(rename = "T")]
    pub order_book_time: u64,
    #[serde(rename = "ut")]
    pub order_update_time: u64,
    #[serde(rename = "R")]
    pub is_reduce_only: bool,
    #[serde(rename = "wt")]
    pub stop_price_working_type: String,
    #[serde(rename = "ps")]
    pub position_side: String,
    #[serde(rename = "cp")]
    pub is_close_all: bool,
    #[serde(rename = "AP", with = "string_or_decimal")]
    pub activation_price: Decimal,
    #[serde(rename = "cr")]
    pub callback_rate: Decimal,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "V")]
    pub stp_mode: String,
    #[serde(rename = "gtd")]
    pub gtd_time: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrderLossUpdate {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "o", with = "string_or_decimal")]
    pub amount: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderTradeUpdate {
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
    pub original_quantity: Decimal,
    #[serde(rename = "p")]
    pub original_price: Decimal,
    #[serde(rename = "ap")]
    pub average_price: Decimal,
    #[serde(rename = "x")]
    pub execution_type: String,
    #[serde(rename = "X")]
    pub order_status: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l", with = "string_or_decimal")]
    pub order_last_filled_quantity: Decimal,
    #[serde(rename = "z", with = "string_or_decimal")]
    pub order_filled_accumulated_quantity: Decimal,
    #[serde(rename = "L", with = "string_or_decimal")]
    pub last_filled_price: Decimal,
    #[serde(rename = "N")]
    pub commission_asset: String,
    #[serde(rename = "n", with = "string_or_decimal")]
    pub commission: Decimal,
    #[serde(rename = "T")]
    pub order_trade_time: u64,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "b", with = "string_or_decimal")]
    pub bids_notional: Decimal,
    #[serde(rename = "a", with = "string_or_decimal")]
    pub ask_notional: Decimal,
    #[serde(rename = "m")]
    pub is_maker: bool,
    #[serde(rename = "R")]
    pub is_reduce_only: bool,
    #[serde(rename = "ps")]
    pub position_side: String,
    #[serde(rename = "rp", with = "string_or_decimal")]
    pub realized_profit: Decimal,
    #[serde(rename = "st")]
    pub strategy_type: String,
    #[serde(rename = "si")]
    pub strategy_id: u64,
    #[serde(rename = "V")]
    pub stp_mode: String,
    #[serde(rename = "gtd")]
    pub gtd_time: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalancePosition {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f", with = "string_or_decimal")]
    pub free_amount: Decimal,
    #[serde(rename = "l", with = "string_or_decimal")]
    pub locked_amount: Decimal,
}
