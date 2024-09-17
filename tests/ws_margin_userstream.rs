use anyhow::Error;
use binance_async::{
    rest::margin::StartUserDataStreamRequest,
    websocket::{margin::WebsocketMessage, BinanceWebsocket},
    Binance,
};
use fehler::throws;
use futures::StreamExt;
use std::{env::var, time::Duration};
use tokio::time::timeout;

#[throws(Error)]
#[tokio::test]
async fn ws_margin_userstream() {
    env_logger::init();
    let binance = Binance::with_key(&var("BINANCE_PK")?);
    let listen_key = binance.request(StartUserDataStreamRequest {}).await?;
    let mut ws: BinanceWebsocket<WebsocketMessage> =
        BinanceWebsocket::new(&[listen_key.listen_key.as_str()]).await?;
    let fut = timeout(Duration::from_secs(50), ws.next());
    let _ = fut.await?.expect("ws exited")?;
}
