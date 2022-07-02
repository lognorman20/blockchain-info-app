#[macro_use]
extern crate serde_derive;

mod blockchain_status;
mod blockchain_info;
mod blockchain_tx;
mod blockchain_address;
use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_tx::BlockchainTx;

fn blockchain_info_app(address: &str) {
    let blockchain_status: BlockchainStatus =  
        blockchain_info::blockchain_status_request();
        
    println!("\n\nQuerying {} - chain: {} \n\n", 
        &blockchain_status.blockbook.coin, 
        &blockchain_status.backend.chain);
        
    let blockchain_address = 
        blockchain_info::blockchain_address_request(address);

    println!("\n\nQuerying the address {}, its current balance is {}",
        &blockchain_address.address,
        &blockchain_address.balance);
}

fn main() {
    blockchain_info_app(dotenv::var("WALLET_ADDRESS")
        .expect("Could not find wallet address")
        .as_str()
    );
}
