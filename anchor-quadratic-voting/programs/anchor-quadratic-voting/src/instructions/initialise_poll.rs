use anchor_lang::prelude::*;

use crate::{errors::VotingError, Poll};

/// Account structure for poll initialization
/// 
/// Creates a new poll account using a PDA derived from the poll_id.
/// The poll creator pays for the account creation.
#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitialisePoll<'info> {
    /// Poll creator who pays for account creation
    #[account(mut)]
    pub signer: Signer<'info>,
    
    /// Poll account to be created
    #[account(
        init,
        payer = signer,
        seeds = [b"poll", poll_id.to_le_bytes().as_ref()],
        space = Poll::DISCRIMINATOR.len() + Poll::INIT_SPACE,
        bump
    )]
    pub poll: Account<'info, Poll>,
    
    /// System program required for account creation
    pub system_program: Program<'info, System>,
}

impl<'info> InitialisePoll<'info> {
    /// Initialize a new poll with time boundaries
    /// 
    /// Validates that the poll end time is after the start time,
    /// then creates the poll account with initial state.
    pub fn initialise_poll(
        &mut self,
        poll_id: u64,
        poll_start: u64,
        poll_end: u64,
        bumps: &InitialisePollBumps,
    ) -> Result<()> {
        // Ensure poll has valid time range (start must be before end)
        require!(poll_start < poll_end, VotingError::InvalidPollDuration);

        // Initialize poll account with provided parameters
        self.poll.set_inner(Poll {
            poll_id,
            poll_start,
            poll_end,
            candidates: 0, // No candidates registered yet
            bump: bumps.poll,
        });
        
        Ok(())
    }
}
