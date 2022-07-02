use dotenv;
use reqwest;
use tokio;
use serde_json::Result;
use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_tx::BlockchainTx;

const ROOT_URL: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    client
        .get(url)
        .header("api-key", dotenv::var("API_KEY").expect("Could not find API key :("))
        .send()
        .await
        .expect("Failed to get data from API :( ")
        .text()
        .await
        .expect("Failed to convert to text :(")
}

pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&ROOT_URL);
    serde_json::from_str(&response).expect("Could not parse json :(")
}

pub fn blockchain_address_request(address: &str) -> BlockchainAddress{
    let response = send_request(&[ROOT_URL, "v2/address/", &address].join(""));
    serde_json::from_str(&response).expect("Could not parse address Json :('")
}