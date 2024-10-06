use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::env;

pub fn get_client() -> Result<Client, bitcoincore_rpc::Error> {
    let rpc_url = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set in .env");
    let rpc_user = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set in .env");
    let rpc_password = env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set in .env");

    println!("Attempting to connect to Bitcoin node at URL: {}", rpc_url);
    
    Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).map_err(|e| {
        eprintln!("Failed to create Bitcoin RPC client: {:?}", e);
        e
    })
}

pub fn fetch_block_height(client: &Client) -> Result<i64, bitcoincore_rpc::Error> {
    println!("Attempting to fetch blockchain info...");
    match client.get_blockchain_info() {
        Ok(info) => {
            println!("Successfully fetched blockchain info.");
            println!("Current height: {}", info.blocks);
            println!("Sync progress: {:.2}%", info.verification_progress * 100.0);
            
            if info.initial_block_download {
                println!("Note: Bitcoin Core is still syncing the blockchain.");
            }
            
            Ok(info.blocks as i64)
        },
        Err(e) => {
            eprintln!("Error fetching blockchain info: {:?}", e);
            Err(e)
        }
    }
}