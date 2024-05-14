use anchor_lang::prelude::*;

#[error_code]
pub enum TreasuryBondsError {
    // treasury bonds
    #[msg("Invalid issuer length")]
    InvalidIssuerLength,
    #[msg("Invalid issuer no length")]
    InvalidIssuerNoLength,
    #[msg("Invalid type of bond")]
    InvalidTypeOfBond,
    #[msg("Invalid bond tenor")]
    InvalidBondTenor,
    #[msg("Invalid bond coupon rate")]
    InvalidBondCouponRate,
    #[msg("Invalid value date length")]
    InvalidValueDateLength,
    #[msg("Invalid redemption date length")]
    InvalidValueRedemptionLength,
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Invalid numeric value.")]
    InvalidNumeric,
    #[msg("Invalid minimum bid amount.")]
    InvalidMinimumBidAmount,

    //
    #[msg("Invalid country length")]
    InvalidCountryLength,

    // Arithmetic
    #[msg("Arithmetic operation failed.")]
    InvalidArithmeticOperation,

    // investor
    #[msg("Invalid full names length")]
    InvalidFullNamesLength,
    #[msg("Investor has no active status.")]
    InvalidInvestorStatus,
    #[msg("Insufficient funds.")]
    InsufficientFunds,

    // account
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
