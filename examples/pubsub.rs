use web3_fe::futures::{future, StreamExt};

#[tokio::main]
async fn main() -> web3_fe::Result {
    let ws = web3_fe::transports::WebSocket::new("ws://localhost:8546").await?;
    let web3 = web3_fe::Web3::new(ws.clone());
    let mut sub = web3.eth_subscribe().subscribe_new_heads().await?;

    println!("Got subscription id: {:?}", sub.id());

    (&mut sub)
        .take(5)
        .for_each(|x| {
            println!("Got: {:?}", x);
            future::ready(())
        })
        .await;

    sub.unsubscribe().await?;

    Ok(())
}
