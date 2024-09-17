use anyhow::Error;
use binance_async::{rest::usdm, Binance};
use fehler::throws;
use std::env::var;

#[throws(Error)]
#[tokio::test]
async fn cancel_all_open_orders() {
    env_logger::init();

    let binance = Binance::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let resp = binance
        .request(usdm::CancelAllOpenOrdersRequest {
            symbol: "BTCUSDT".into(),
        })
        .await?;
    println!("{resp:?}");
}

#[throws(Error)]
#[tokio::test]
async fn auto_cancel_all_open_orders() {
    env_logger::init();

    let binance = Binance::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let resp = binance
        .request(usdm::AutoCancelAllOpenOrdersRequest {
            symbol: "BTCUSDT".into(),
            countdown_time: 1000000,
        })
        .await?;
    println!("{resp:?}");
}
