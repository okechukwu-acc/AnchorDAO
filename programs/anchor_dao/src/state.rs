use anchor_lang::prelude::*;

// Define the program's ID
declare_id!("YourProgramIDHere");

// The maximum length for strings to prevent excessive on-chain storage usage
const MAX_NAME_LENGTH: usize = 32;
const MAX_DESCRIPTION_LENGTH: usize = 256;

// Define the state structure for the DAO
#[account]
pub struct Dao {
    pub name: String, // The name of the DAO
    pub description: String, // A brief description of the DAO
    pub governance_token_mint: Pubkey, // The mint address of the governance token
    pub treasury: Pubkey, // The treasury account address
    pub authority: Pubkey, // The authority managing the DAO
    pub proposal_count: u64, // A counter for tracking the number of proposals
    pub created_at: i64, // The timestamp when the DAO was created
}

// Implement the default values for the DAO state
impl Default for Dao {
    fn default() -> Self {
        Self {
            name: String::from("Unnamed DAO"),
            description: String::from(""),
            governance_token_mint: Pubkey::default(),
            treasury: Pubkey::default(),
            authority: Pubkey::default(),
            proposal_count: 0,
            created_at: 0,
        }
    }
}

// Define the state structure for a Proposal
#[account]
pub struct Proposal {
    pub dao: Pubkey, // The DAO this proposal belongs to
    pub proposer: Pubkey, // The address of the proposer
    pub description: String, // A detailed description of the proposal
    pub vote_count: u64, // The number of votes the proposal has received
    pub executed: bool, // Whether the proposal has been executed
    pub created_at: i64, // The timestamp when the proposal was created
    pub voting_deadline: i64, // The deadline for voting on the proposal
}

// Implement the default values for the Proposal state
impl Default for Proposal {
    fn default() -> Self {
        Self {
            dao: Pubkey::default(),
            proposer: Pubkey::default(),
            description: String::from(""),
            vote_count: 0,
            executed: false,
            created_at: 0,
            voting_deadline: 0,
        }
    }
}

// Define the state structure for Treasury management
#[account]
pub struct Treasury {
    pub dao: Pubkey, // The DAO this treasury belongs to
    pub balance: u64, // The balance of the treasury in lamports
    pub authority: Pubkey, // The authority managing the treasury
}

// Implement the default values for the Treasury state
impl Default for Treasury {
    fn default() -> Self {
        Self {
            dao: Pubkey::default(),
            balance: 0,
            authority: Pubkey::default(),
        }
    }
}

// Define the state structure for Voting
#[account]
pub struct Vote {
    pub proposal: Pubkey, // The proposal this vote belongs to
    pub voter: Pubkey, // The address of the voter
    pub weight: u64, // The weight of the vote based on the governance tokens held
    pub cast_at: i64, // The timestamp when the vote was cast
}

// Implement the default values for the Vote state
impl Default for Vote {
    fn default() -> Self {
        Self {
            proposal: Pubkey::default(),
            voter: Pubkey::default(),
            weight: 0,
            cast_at: 0,
        }
    }
}

impl Dao {
    pub const MAX_SIZE: usize = 8 +  // Discriminator
        4 + MAX_NAME_LENGTH +  // Name
        4 + MAX_DESCRIPTION_LENGTH +  // Description
        32 +  // Governance token mint
        32 +  // Treasury
        32 +  // Authority
        8 +  // Proposal count
        8;  // Created at
}

impl Proposal {
    pub const MAX_SIZE: usize = 8 +  // Discriminator
        32 +  // DAO
        32 +  // Proposer
        4 + MAX_DESCRIPTION_LENGTH +  // Description
        8 +  // Vote count
        1 +  // Executed
        8 +  // Created at
        8;  // Voting deadline
}

impl Treasury {
    pub const MAX_SIZE: usize = 8 +  // Discriminator
        32 +  // DAO
        8 +  // Balance
        32;  // Authority
}

impl Vote {
    pub const MAX_SIZE: usize = 8 +  // Discriminator
        32 +  // Proposal
        32 +  // Voter
        8 +  // Weight
        8;  // Cast at
}

