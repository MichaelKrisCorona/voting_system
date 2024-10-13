pub struct Voter {
    pub age: u32,          // Voter's age
    pub voter_id: String,  // Unique identifier for the voter
    pub authorized: bool,  // Authorization status
}

impl Voter {
    /// Create a new Voter instance
    pub fn new(age: u32, voter_id: String) -> Self {
        Voter {
            age,
            voter_id,
            authorized: false, // Default to not authorized
        }
    }

    /// Verify the age of the voter
    pub fn verify_age(&mut self) -> Result<(), &'static str> {
        if self.age < 18 {
            return Err("Voter must be at least 18 years old to vote.");
        }
        self.authorized = true; // Authorize the voter if age verification passes
        Ok(())
    }
}
