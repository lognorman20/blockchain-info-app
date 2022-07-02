#[macro_use]
extern crate serde_derive;

mod blockchain_status;
mod blockchain_info;
mod blockchain_tx;
mod blockchain_address;

use {
    crate::blockchain_status::BlockchainStatus,
    crate::blockchain_address::BlockchainAddress,
    crate::blockchain_tx::BlockchainTx,

    dotenv,
    std::{io, thread, time}
};

fn blockchain_info_app(address: &str) {
    let blockchain_status: BlockchainStatus =  
        blockchain_info::blockchain_status_request();
        
    println!("\n\nQuerying {} - chain: {}", 
        &blockchain_status.blockbook.coin, 
        &blockchain_status.backend.chain);
        
    let sleep_time = time::Duration::from_millis(500);
    thread::sleep(sleep_time);

    let blockchain_address = 
        blockchain_info::blockchain_address_request(address);

    println!("\n\nQuerying the address {}, its current balance is {}",
        &blockchain_address.address,
        &blockchain_address.balance);

    println!("The given address has {} total transactions.",
        &blockchain_address.txids.len()
    );
}

fn main() {
    blockchain_info_app(dotenv::var("WALLET_ADDRESS")
        .expect("Could not find wallet address")
        .as_str()
    );
}
