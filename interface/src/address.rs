use crate::constants::{
    AUTHORITY_DEPOSIT, AUTHORITY_WITHDRAW, EPHEMERAL_STAKE_SEED_PREFIX, TRANSIENT_STAKE_SEED_PREFIX,
};
use solana_pubkey::Pubkey;
use std::num::NonZeroU32;

/// Generates the deposit authority program address for the stake pool
pub fn find_deposit_authority_program_address(
    program_id: &Pubkey,
    stake_pool_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[stake_pool_address.as_ref(), AUTHORITY_DEPOSIT],
        program_id,
    )
}

/// Generates the withdraw authority program address for the stake pool
pub fn find_withdraw_authority_program_address(
    program_id: &Pubkey,
    stake_pool_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[stake_pool_address.as_ref(), AUTHORITY_WITHDRAW],
        program_id,
    )
}

/// Generates the stake program address for a validator's vote account
pub fn find_stake_program_address(
    program_id: &Pubkey,
    vote_account_address: &Pubkey,
    stake_pool_address: &Pubkey,
    seed: Option<NonZeroU32>,
) -> (Pubkey, u8) {
    let seed = seed.map(|s| s.get().to_le_bytes());
    Pubkey::find_program_address(
        &[
            vote_account_address.as_ref(),
            stake_pool_address.as_ref(),
            seed.as_ref().map(|s| s.as_slice()).unwrap_or(&[]),
        ],
        program_id,
    )
}

/// Generates the stake program address for a validator's vote account
pub fn find_transient_stake_program_address(
    program_id: &Pubkey,
    vote_account_address: &Pubkey,
    stake_pool_address: &Pubkey,
    seed: u64,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            TRANSIENT_STAKE_SEED_PREFIX,
            vote_account_address.as_ref(),
            stake_pool_address.as_ref(),
            &seed.to_le_bytes(),
        ],
        program_id,
    )
}

/// Generates the ephemeral program address for stake pool redelegation
pub fn find_ephemeral_stake_program_address(
    program_id: &Pubkey,
    stake_pool_address: &Pubkey,
    seed: u64,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            EPHEMERAL_STAKE_SEED_PREFIX,
            stake_pool_address.as_ref(),
            &seed.to_le_bytes(),
        ],
        program_id,
    )
}
