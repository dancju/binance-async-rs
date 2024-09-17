use anyhow::Error;
use binance_async::{
    rest::spot::StartUserDataStreamRequest,
    websocket::{spot::WebsocketMessage, BinanceWebsocket},
    Binance,
};
use fehler::throws;
use futures::StreamExt;
use std::{env::var, time::Duration};
use tokio::time::timeout;

#[throws(Error)]
#[tokio::test]
async fn ws_spot_userstream() {
    env_logger::init();

    let binance = Binance::with_key(&var("BINANCE_KEY")?);
    let listen_key = binance.request(StartUserDataStreamRequest {}).await?;
    let mut ws: BinanceWebsocket<WebsocketMessage> =
        BinanceWebsocket::new(&[listen_key.listen_key.as_str()]).await?;

    let fut = timeout(Duration::from_secs(5), ws.next());
    let msg = fut.await?.expect("ws exited")?;
    match msg {
        WebsocketMessage::Ping => todo!(),
        WebsocketMessage::AggregateTrade(agg) => log::info!("{agg:?}"),
        WebsocketMessage::Trade(trade) => log::info!("{trade:?}"),
        WebsocketMessage::Kline(kline) => log::info!("{kline:?}"),
        WebsocketMessage::MiniTicker(ticker) => log::info!("{ticker:?}"),
        WebsocketMessage::Ticker24hr(ticker) => log::info!("{ticker:?}"),
        WebsocketMessage::Ticker1hr(ticker) => log::info!("{ticker:?}"),
        WebsocketMessage::AveragePrice(avg) => log::info!("{avg:?}"),
        WebsocketMessage::DepthUpdate(depth) => log::info!("{depth:?}"),
        WebsocketMessage::OutboundAccountPosition(acc) => log::info!("{acc:?}"),
        WebsocketMessage::BalanceUpdate(bal) => log::info!("{bal:?}"),
        WebsocketMessage::ExecutionReport(report) => log::info!("{report:?}"),
        WebsocketMessage::ListStatus(status) => log::info!("{status:?}"),
        WebsocketMessage::ListenKeyExpired(expired) => log::info!("{expired:?}"),
    }
}
