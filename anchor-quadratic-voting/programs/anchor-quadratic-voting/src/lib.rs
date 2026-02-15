use anchor_lang::prelude::*;

// Module declarations for program components
pub mod errors;
pub mod instructions;
pub mod state;

// Re-export modules for easier access
pub use instructions::*;
pub use state::*;

// Program ID - unique identifier for this Solana program
declare_id!("Ew61VRaYCfNu4T2Kuco5uFqs6eUgtMmyrfX92HwDEcad");

/// Quadratic Voting Program
/// 
/// This program implements a quadratic voting system where the cost of votes
/// increases quadratically (cost = votes²), making it expensive to concentrate
/// votes on a single option while encouraging distribution across preferences.
#[program]
pub mod anchor_quadratic_voting {
    use super::*;
    
    /// Creates a new voting poll with specified time boundaries
    /// 
    /// # Arguments
    /// * `poll_id` - Unique identifier for the poll
    /// * `poll_start` - Unix timestamp when voting begins
    /// * `poll_end` - Unix timestamp when voting concludes
    pub fn initialise_poll(
        ctx: Context<InitialisePoll>,
        poll_id: u64,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        ctx.accounts.initialise_poll(poll_id, poll_start, poll_end, &ctx.bumps)
    }
    
    /// Registers a new candidate for an existing poll
    /// 
    /// # Arguments
    /// * `candidate_name` - Name of the candidate (max 64 characters)
    /// * `poll_id` - ID of the poll to add the candidate to
    pub fn initialise_candidate(
        ctx: Context<InitialiseCandidate>,
        candidate_name: String,
        poll_id: u64,
    ) -> Result<()> {
        ctx.accounts.initialise_candidate(candidate_name, poll_id, &ctx.bumps)
    }
    
    /// Cast votes for a candidate using quadratic voting mechanism
    /// 
    /// # Arguments
    /// * `candidate_name` - Name of the candidate to vote for
    /// * `poll_id` - ID of the active poll
    /// * `num_votes` - Number of votes to cast (cost = num_votes²)
    pub fn vote(
        ctx: Context<Vote>,
        candidate_name: String,
        poll_id: u64,
        num_votes: u64,
    ) -> Result<()> {
        ctx.accounts.vote(candidate_name, poll_id, num_votes)
    }
}
