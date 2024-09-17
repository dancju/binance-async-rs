use anyhow::Error;
use binance_async::{rest::spot, Binance};
use fehler::throws;

#[throws(Error)]
#[tokio::test]
async fn ping() {
    env_logger::init();

    let binance = Binance::new();
    let resp = binance.request(spot::PingRequest {}).await?;
    println!("{resp:?}");
}
