// Declare the modules for your files
mod hedera;
mod ballot;
mod vote_casting;

use hedera::HederaClient; // Import the HederaClient struct
use ballot::Ballot; // Import the Ballot struct
use vote_casting::VoteCasting; // Import the VoteCasting struct
use std::env;

#[tokio::main] // Use the Tokio runtime for asynchronous programming
async fn main() {
    // Load environment variables from the .env file
    dotenv::dotenv().ok();

    // Initialize the Hedera client
    let hedera_client = HederaClient::new().await.expect("Failed to create Hedera client");

    // Example usage of the HederaClient
    println!("Hedera client initialized.");

    // Here you can set up your voting logic
    // For example, you might want to create a ballot and validate it
    let voter_id = "voter123".to_string();
    let candidate_id = 1;
    let timestamp = 1622547800; // Example timestamp
    let election_id = "election456".to_string();

    let ballot = Ballot::new(voter_id, candidate_id, timestamp, election_id);
    
    // Validate the ballot
    if let Err(err) = ballot.validate() {
        eprintln!("Ballot validation failed: {}", err);
        return;
    }
    
    // Proceed to cast the vote using the VoteCasting struct
    let vote_casting = VoteCasting::new(hedera_client, "your_contract_id_here".to_string());
    match vote_casting.cast_vote(candidate_id).await {
        Ok(message) => println!("{}", message),
        Err(err) => eprintln!("Error casting vote: {}", err),
    }
}
