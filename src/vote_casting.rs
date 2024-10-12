use crate::hedera::HederaClient; // Import your HederaClient struct
use std::error::Error;

pub struct VoteCasting {
    hedera_client: HederaClient,
    contract_id: String, // Store the smart contract ID
}

impl VoteCasting {
    /// Create a new VoteCasting instance
    pub fn new(hedera_client: HederaClient, contract_id: String) -> Self {
        VoteCasting { hedera_client, contract_id }
    }

    /// Cast a vote for a specific candidate
    pub async fn cast_vote(&self, candidate_id: u32) -> Result<String, Box<dyn Error>> {
        // Call the cast_vote method from HederaClient
        let response = self.hedera_client.cast_vote(&self.contract_id, candidate_id).await?;

        // Return the response or confirmation
        Ok(response)
    }
}
