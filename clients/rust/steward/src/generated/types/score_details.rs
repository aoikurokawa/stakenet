//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScoreDetails {
    /// Max MEV commission observed
    pub max_mev_commission: u16,
    /// Epoch of max MEV commission
    pub max_mev_commission_epoch: u16,
    /// Epoch when superminority was detected
    pub superminority_epoch: u16,
    /// Ratio that failed delinquency check
    pub delinquency_ratio: f64,
    /// Epoch when delinquency was detected
    pub delinquency_epoch: u16,
    /// Max commission observed
    pub max_commission: u8,
    /// Epoch of max commission
    pub max_commission_epoch: u16,
    /// Max historical commission observed
    pub max_historical_commission: u8,
    /// Epoch of max historical commission
    pub max_historical_commission_epoch: u16,
}