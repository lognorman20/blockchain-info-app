#[macro_use]
extern crate serde_derive;

mod blockchain_status;
mod blockchain_info;
mod blockchain_tx;
mod blockchain_address;
use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_tx::BlockchainTx;

fn main() {
    let blockchain_status: BlockchainStatus =  blockchain_info::blockchain_status_request();
    println!("\n\nQuerying {} - chain: {} \n\n", 
        &blockchain_status.blockbook.coin, 
        &blockchain_status.backend.chain);
}
