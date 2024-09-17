use crate::{define_request, models::Product, parser::string_or_decimal};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::Deserialize;

define_request! {
    Name => GetBalance;
    Product => Product::PortfolioMargin;
    Method => Method::GET;
    Endpoint => "/papi/v1/balance";
    Signed => true;
    Request => {};
    Response => Vec<GetBalanceResponseItem>;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponseItem {
    pub asset: String,
    #[serde(with = "string_or_decimal")]
    pub total_wallet_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cross_margin_asset: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cross_margin_borrowed: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cross_margin_free: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cross_margin_interest: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cross_margin_locked: Decimal,
    #[serde(with = "string_or_decimal")]
    pub um_wallet_balance: Decimal,
    #[serde(rename = "umUnrealizedPNL", with = "string_or_decimal")]
    pub um_unrealized_pnl: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cm_wallet_balance: Decimal,
    #[serde(rename = "cmUnrealizedPNL", with = "string_or_decimal")]
    pub cm_unrealized_pnl: Decimal,
    pub update_time: u64,
    #[serde(with = "string_or_decimal")]
    pub negative_balance: Decimal,
}

define_request! {
    Name => GetUMPositionRisk;
    Product => Product::PortfolioMargin;
    Method => Method::GET;
    Endpoint => "/papi/v1/um/positionRisk";
    Signed => true;
    Request => {};
    Response => Vec<GetUMPositionRiskResponseItem>;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUMPositionRiskResponseItem {
    pub symbol: String,
    #[serde(with = "string_or_decimal")]
    pub position_amt: Decimal,
    #[serde(with = "string_or_decimal")]
    pub entry_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub mark_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub un_realized_profit: Decimal,
    #[serde(with = "string_or_decimal")]
    pub liquidation_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub leverage: Decimal,
    #[serde(with = "string_or_decimal")]
    pub max_notional_value: Decimal,
    pub position_side: String,
    #[serde(with = "string_or_decimal")]
    pub notional: Decimal,
    pub update_time: u64,
}

define_request! {
    Name => GetCMPositionRisk;
    Product => Product::PortfolioMargin;
    Method => Method::GET;
    Endpoint => "/papi/v1/cm/positionRisk";
    Signed => true;
    Request => {};
    Response => Vec<GetCMPositionRiskResponseItem>;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCMPositionRiskResponseItem {
    pub symbol: String,
    #[serde(with = "string_or_decimal")]
    pub position_amt: Decimal,
    #[serde(with = "string_or_decimal")]
    pub entry_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub mark_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub liquidation_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub un_realized_profit: Decimal,
    #[serde(with = "string_or_decimal")]
    pub leverage: Decimal,
    pub position_side: String,
    pub update_time: u64,
    #[serde(with = "string_or_decimal")]
    pub max_qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub notional_value: Decimal,
    #[serde(with = "string_or_decimal")]
    pub break_even_price: Decimal,
}
