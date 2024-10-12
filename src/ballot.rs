#[derive(Debug, Clone)]
pub struct Ballot {
    pub voter_id: String,      // Unique identifier for the voter
    pub candidate_id: u32,     // ID of the candidate being voted for
    pub timestamp: u64,        // Timestamp of when the vote was cast
    pub election_id: String,    // ID of the election
}

impl Ballot {
    /// Create a new ballot for a specific voter and candidate
    pub fn new(voter_id: String, candidate_id: u32, timestamp: u64, election_id: String) -> Self {
        Ballot { voter_id, candidate_id, timestamp, election_id }
    }

    /// Validate the ballot (e.g., check if the voter is eligible to vote)
    pub fn validate(&self) -> Result<(), &'static str> {
        // Add validation logic here
        if self.voter_id.is_empty() {
            return Err("Voter ID cannot be empty.");
        }
        if self.candidate_id == 0 {
            return Err("Invalid candidate ID.");
        }
        if self.timestamp == 0 {
            return Err("Invalid timestamp.");
        }
        if self.election_id.is_empty() {
            return Err("Election ID cannot be empty.");
        }
        Ok(())
    }
}
