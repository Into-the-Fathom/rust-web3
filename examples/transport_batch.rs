#[tokio::main]
async fn main() -> web3_fe::Result {
    let _ = env_logger::try_init();
    let http = web3_fe::transports::Http::new("http://localhost:8545")?;
    let web3 = web3_fe::Web3::new(web3_fe::transports::Batch::new(http));

    let accounts = web3.eth().accounts();
    let block = web3.eth().block_number();

    let result = web3.transport().submit_batch().await?;
    println!("Result: {:?}", result);

    let accounts = accounts.await?;
    println!("Accounts: {:?}", accounts);

    let block = block.await?;
    println!("Block: {:?}", block);

    Ok(())
}
