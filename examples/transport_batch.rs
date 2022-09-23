#[tokio::main]
async fn main() -> fathom_web3::Result {
    let _ = env_logger::try_init();
    let http = fathom_web3::transports::Http::new("http://localhost:8545")?;
    let web3 = fathom_web3::Web3::new(fathom_web3::transports::Batch::new(http));

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
