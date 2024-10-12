use dotenv::dotenv;
use reqwest::Client;
use std::env;
use serde_json::json;

pub struct HederaClient {
    pub client: Client,
    pub account_id: String,
    pub private_key: String,
}

impl HederaClient {
    /// Initializes a new Hedera client.
    pub async fn new() -> Result<Self, reqwest::Error> {
        // Load environment variables from the .env file
        dotenv().ok(); 

        // Retrieve credentials from environment variables
        let account_id = env::var("HEDERA_ACCOUNT_ID").expect("HEDERA_ACCOUNT_ID must be set");
        let private_key = env::var("HEDERA_PRIVATE_KEY").expect("HEDERA_PRIVATE_KEY must be set");

        // Create a new reqwest client
        let client = Client::new();

        // Return an instance of HederaClient
        Ok(HederaClient { client, account_id, private_key })
    }

        /// Example method for making a request to execute a smart contract.
    pub async fn execute_contract(&self, contract_id: String, function: String, params: Vec<String>) -> Result<String, reqwest::Error> {
        // Replace this with the correct API endpoint for executing a smart contract
        let url = format!("https://testnet.mirrornode.hedera.com/api/v1/contracts/{}/execute", contract_id);
        
        // Construct the body of the request, including the function name and parameters
        let body = serde_json::json!({
            "function": function,
            "params": params,
        });

        let response = self.client.post(&url)
            .json(&body)
            .header("Authorization", &self.private_key) // Add authentication if needed
            .send()
            .await?;
        
        let result = response.text().await?;
        Ok(result)
    }
    pub async fn cast_vote(&self, contract_id: &str, candidate_id: u32) -> Result<String, reqwest::Error> {
        // Replace this with the correct API endpoint for casting a vote
        let url = format!("https://testnet.mirrornode.hedera.com/api/v1/contracts/{}/execute", contract_id);
        
        // Construct the body of the request to cast the vote
        let body = json!({
            "function": "castVote", // Function name in your smart contract
            "params": [candidate_id.to_string()],
        });

        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await?;
        
        let result = response.text().await?;
        Ok(result)
    }
}
