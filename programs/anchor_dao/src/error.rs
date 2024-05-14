use anchor_lang::prelude::*;

// Define the custom errors for the AnchorDAO program
#[error_code]
pub enum DaoError {
    #[msg("The name provided is too long.")]
    NameTooLong,
    
    #[msg("The description provided is too long.")]
    DescriptionTooLong,
    
    #[msg("The proposal description provided is too long.")]
    ProposalDescriptionTooLong,
    
    #[msg("The voting period has ended.")]
    VotingPeriodEnded,
    
    #[msg("The proposal has already been executed.")]
    ProposalAlreadyExecuted,
    
    #[msg("Insufficient funds in the treasury.")]
    InsufficientTreasuryFunds,
    
    #[msg("Unauthorized access to the DAO or proposal.")]
    UnauthorizedAccess,
    
    #[msg("Invalid governance token mint.")]
    InvalidGovernanceTokenMint,
    
    #[msg("Proposal not found.")]
    ProposalNotFound,
    
    #[msg("DAO not found.")]
    DaoNotFound,
    
    #[msg("Treasury not found.")]
    TreasuryNotFound,
    
    #[msg("Vote already cast.")]
    VoteAlreadyCast,
    
    #[msg("Invalid voting weight.")]
    InvalidVotingWeight,
    
    #[msg("The provided authority is not recognized.")]
    InvalidAuthority,
    
    #[msg("The provided timestamp is invalid.")]
    InvalidTimestamp,
}

impl DaoError {
    pub fn from_code(code: u32) -> Option<DaoError> {
        match code {
            0 => Some(DaoError::NameTooLong),
            1 => Some(DaoError::DescriptionTooLong),
            2 => Some(DaoError::ProposalDescriptionTooLong),
            3 => Some(DaoError::VotingPeriodEnded),
            4 => Some(DaoError::ProposalAlreadyExecuted),
            5 => Some(DaoError::InsufficientTreasuryFunds),
            6 => Some(DaoError::UnauthorizedAccess),
            7 => Some(DaoError::InvalidGovernanceTokenMint),
            8 => Some(DaoError::ProposalNotFound),
            9 => Some(DaoError::DaoNotFound),
            10 => Some(DaoError::TreasuryNotFound),
            11 => Some(DaoError::VoteAlreadyCast),
            12 => Some(DaoError::InvalidVotingWeight),
            13 => Some(DaoError::InvalidAuthority),
            14 => Some(DaoError::InvalidTimestamp),
            _ => None,
        }
    }
}

