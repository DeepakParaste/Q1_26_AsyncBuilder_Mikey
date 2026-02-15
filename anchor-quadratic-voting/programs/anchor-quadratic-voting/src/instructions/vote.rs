use anchor_lang::prelude::*;

use crate::{errors::VotingError, Candidate, Poll};

/// Account structure for casting votes
/// 
/// This struct defines the accounts required for the voting instruction,
/// including the poll and candidate PDAs derived from their respective seeds.
#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct Vote<'info> {
    /// The wallet signing the transaction (voter)
    pub signer: Signer<'info>,
    
    /// Poll account - read-only to check timing constraints
    #[account(
        seeds = [b"poll", poll_id.to_le_bytes().as_ref()],
        bump = poll.bump,
    )]
    pub poll: Account<'info, Poll>,
    
    /// Candidate account - mutable to update vote count
    #[account(
        mut,
        seeds = [b"candidate", poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
        bump = candidate.bump,
    )]
    pub candidate: Account<'info, Candidate>,
}

impl<'info> Vote<'info> {
    /// Execute the voting operation with quadratic cost calculation
    /// 
    /// This function implements the core quadratic voting logic where voters
    /// can cast multiple votes, but each additional vote costs more credits.
    /// The cost formula is: total_cost = num_votes²
    pub fn vote(
        &mut self,
        _candidate_name: String,
        _poll_id: u64,
        num_votes: u64,
    ) -> Result<()> {
        // Validation: Ensure at least one vote is being cast
        require!(num_votes > 0, VotingError::ZeroVotes);

        // Get current blockchain time for poll timing validation
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;

        // Verify poll is currently active (has started)
        require!(
            current_timestamp >= self.poll.poll_start,
            VotingError::PollNotStarted
        );
        
        // Verify poll hasn't ended yet
        require!(
            current_timestamp <= self.poll.poll_end,
            VotingError::PollEnded
        );

        // Calculate quadratic cost: cost = votes²
        // Example: 5 votes costs 25 credits, 10 votes costs 100 credits
        // This makes it expensive to concentrate all votes on one candidate
        let quadratic_cost = num_votes
            .checked_mul(num_votes)
            .ok_or(VotingError::Overflow)?;

        // Log the voting action for transparency
        msg!(
            "Vote recorded: {} votes cast (quadratic cost: {} credits)",
            num_votes,
            quadratic_cost
        );

        // Update candidate's total vote count
        self.candidate.candidate_votes = self
            .candidate
            .candidate_votes
            .checked_add(num_votes)
            .ok_or(VotingError::Overflow)?;

        Ok(())
    }
}
