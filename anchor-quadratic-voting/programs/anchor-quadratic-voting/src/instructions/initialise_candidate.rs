use anchor_lang::prelude::*;

use crate::{Candidate, Poll};

/// Account structure for candidate registration
/// 
/// Registers a new candidate for an existing poll. The candidate account
/// is derived from the poll_id and candidate name, ensuring uniqueness.
#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct InitialiseCandidate<'info> {
    /// Account that pays for candidate creation
    #[account(mut)]
    pub signer: Signer<'info>,
    
    /// Poll account - mutable to increment candidate counter
    #[account(
        mut,
        seeds = [b"poll", poll_id.to_le_bytes().as_ref()],
        bump = poll.bump,
    )]
    pub poll: Account<'info, Poll>,
    
    /// Candidate account to be created
    #[account(
        init,
        payer = signer,
        seeds = [b"candidate", poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
        space = Candidate::DISCRIMINATOR.len() + Candidate::INIT_SPACE,
        bump
    )]
    pub candidate: Account<'info, Candidate>,
    
    /// System program for account creation
    pub system_program: Program<'info, System>,
}

impl<'info> InitialiseCandidate<'info> {
    /// Register a new candidate for the poll
    /// 
    /// Increments the poll's candidate count and creates the candidate account
    /// with zero initial votes.
    pub fn initialise_candidate(
        &mut self,
        candidate_name: String,
        _poll_id: u64,
        bumps: &InitialiseCandidateBumps,
    ) -> Result<()> {
        // Increment total candidate count for this poll (with overflow protection)
        self.poll.candidates = self.poll.candidates
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        
        // Create candidate account with initial state
        self.candidate.set_inner(Candidate {
            candidate_name,
            candidate_votes: 0, // Start with zero votes
            bump: bumps.candidate,
        });
        
        Ok(())
    }
}
