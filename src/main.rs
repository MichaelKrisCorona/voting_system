// Declare the modules for your files
mod hedera;
mod ballot;
mod vote_casting;
mod auth;

use hedera::HederaClient; // Import the HederaClient struct
use ballot::Ballot; // Import the Ballot struct
use vote_casting::VoteCasting; // Import the VoteCasting struct
use auth::Voter;
use std::io::{self, Write};

#[tokio::main] // Use the Tokio runtime for asynchronous programming
async fn main() {
    // Load environment variables from the .env file
    dotenv::dotenv().ok();

    // Get voter information 
    let mut voter = get_voter_info();

    // Verify age and authorize the voter
    match voter.verify_age(){
        Ok(_) => println!("Voter Authorized."),
        Err(err) => {
            eprintln!("Authorization failed: {}", err);
            return;
        }
    }

    // Initialize the Hedera client
    let hedera_client = HederaClient::new().await.expect("Failed to create Hedera client");

    // Example usage of the HederaClient
    println!("Hedera client initialized.");

    // Here you can set up your voting logic
    // For example, you might want to create a ballot and validate it
    let _voter_id = "voter123".to_string();
    let candidate_id = 1;
    let timestamp = 1622547800; // Example timestamp
    let election_id = "election456".to_string();

// Create the ballot using the voter's ID
    let ballot = Ballot::new(voter.voter_id.clone(), candidate_id, timestamp, election_id);
    
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
fn get_voter_info() -> Voter {

    let mut age_input = String::new();
    print!("Please enter your age: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid age");

    let mut voter_id = String::new();
    print!("Please enter your voter ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut voter_id).expect("Failed to read input");

    Voter::new(age, voter_id.trim().to_string())

}
