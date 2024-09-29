use anchor_lang::prelude::*;

/// Emitted when the state of a ValidatorHistoryAccount changed.
#[event]
pub struct ValidatorHistoryUpdated {
    /// Epoch
    pub epoch: u64,

    /// Vote Account
    pub vote_account: Pubkey,
}
