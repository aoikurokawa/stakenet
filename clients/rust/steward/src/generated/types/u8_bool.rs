//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// A boolean type stored as a u8.
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct U8Bool {
    pub value: u8,
}