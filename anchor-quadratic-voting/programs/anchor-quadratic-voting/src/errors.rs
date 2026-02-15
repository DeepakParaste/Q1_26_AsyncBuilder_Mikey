use anchor_lang::prelude::*;

/// Custom error codes for the quadratic voting program
/// 
/// These errors provide specific feedback when validation checks fail
/// during poll creation, candidate registration, or voting operations.
#[error_code]
pub enum VotingError {
    /// Attempted to vote before the poll's start time
    #[msg("The poll has not started yet")]
    PollNotStarted,
    
    /// Attempted to vote after the poll's end time
    #[msg("The poll has ended")]
    PollEnded,
    
    /// Arithmetic operation would overflow (vote count too large)
    #[msg("Arithmetic overflow")]
    Overflow,
    
    /// Attempted to cast zero votes (must cast at least 1)
    #[msg("Number of votes must be greater than zero")]
    ZeroVotes,
    
    /// Poll start time is not before end time
    #[msg("Poll start must be before poll end")]
    InvalidPollDuration,
}
