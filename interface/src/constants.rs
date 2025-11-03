use crate::state::Fee;

/// Seed for deposit authority seed
pub const AUTHORITY_DEPOSIT: &[u8] = b"deposit";

/// Seed for withdraw authority seed
pub const AUTHORITY_WITHDRAW: &[u8] = b"withdraw";

/// Seed for transient stake account
pub const TRANSIENT_STAKE_SEED_PREFIX: &[u8] = b"transient";

/// Seed for ephemeral stake account
pub const EPHEMERAL_STAKE_SEED_PREFIX: &[u8] = b"ephemeral";

/// Minimum amount of staked lamports required in a validator stake account to
/// allow for merges without a mismatch on credits observed
pub const MINIMUM_ACTIVE_STAKE: u64 = 1_000_000;

/// Minimum amount of lamports in the reserve
pub const MINIMUM_RESERVE_LAMPORTS: u64 = 0;

/// Maximum amount of validator stake accounts to update per
/// `UpdateValidatorListBalance` instruction, based on compute limits
pub const MAX_VALIDATORS_TO_UPDATE: usize = 4;

/// Maximum factor by which a withdrawal fee can be increased per epoch
/// protecting stakers from malicious users.
/// If current fee is 0, `WITHDRAWAL_BASELINE_FEE` is used as the baseline
pub const MAX_WITHDRAWAL_FEE_INCREASE: Fee = Fee {
    numerator: 3,
    denominator: 2,
};
/// Drop-in baseline fee when evaluating withdrawal fee increases when fee is 0
pub const WITHDRAWAL_BASELINE_FEE: Fee = Fee {
    numerator: 1,
    denominator: 1000,
};

/// The maximum number of transient stake accounts respecting
/// transaction account limits.
pub const MAX_TRANSIENT_STAKE_ACCOUNTS: usize = 10;
