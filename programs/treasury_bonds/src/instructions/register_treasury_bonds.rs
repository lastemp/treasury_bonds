//! RegisterTreasuryBonds instruction handler

use {
    crate::{
        error::TreasuryBondsError,
        state::{
            bond_issuer::BondIssuer, configs::TreasuryBondsConfigs, deposit_base::DepositBase,
            treasury_bonds::TreasuryBonds,
        },
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: RegisterTreasuryBondsParams)]
pub struct RegisterTreasuryBonds<'info> {
    #[account(
        mut, constraint = treasury_bonds_configs.is_initialized @ TreasuryBondsError::AccountNotInitialized
    )]
    pub treasury_bonds_configs: Account<'info, TreasuryBondsConfigs>,
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + TreasuryBonds::INIT_SPACE,
        constraint = !treasury_bonds.is_initialized @ TreasuryBondsError::AccountAlreadyInitialized,
        seeds = [b"treasury-bonds", owner.key().as_ref()],
        bump
    )]
    pub treasury_bonds: Account<'info, TreasuryBonds>,
    #[account(init, payer = owner, space = 8 + DepositBase::INIT_SPACE,
        constraint = !deposit_account.is_initialized @ TreasuryBondsError::AccountAlreadyInitialized
    )]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"treasury-vault", pda_auth.key().as_ref()], bump)]
    pub treasury_vault: SystemAccount<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterTreasuryBondsParams {
    issuer: BondIssuer,               // bond issuer details
    country: String,                  // home country where treasury bonds is issued
    issue_no: String,                 // issue no of bond
    type_of_bond: u8, // type of bond i.e Fixed coupon Treasury bonds, Infrastructure bonds
    tenor: u8,        // maturity period i.e between 2-30 years
    coupon_rate: u8,  // coupon rate (%)
    total_amounts_offered: u32, // total amounts offered for the given bond
    minimum_bid_amount: u32, // minimum bid amount
    unit_cost_of_treasury_bonds: u32, // unit cost of treasury bonds
    decimals: u8,     // decimals for the token mint
    value_date: String, // value date of bond
    redemption_date: String, // redemption date of bond
}

// issuer length
const ISSUER_LENGTH: usize = 30;
// issuer NO length
const ISSUER_NO_LENGTH: usize = 20;
// tenor length
const TENOR_LENGTH: u8 = 2;
const TENOR_LENGTH_2: u8 = 30;
// date length
const DATE_LENGTH: usize = 20;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_treasury_bonds(
    ctx: Context<RegisterTreasuryBonds>,
    params: &RegisterTreasuryBondsParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.issuer.issuer.as_bytes().len() > 0
        && params.issuer.issuer.as_bytes().len() <= ISSUER_LENGTH
    {
    } else {
        return Err(TreasuryBondsError::InvalidIssuerLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(TreasuryBondsError::InvalidCountryLength.into());
    }

    if params.issue_no.as_bytes().len() > 0 && params.issue_no.as_bytes().len() <= ISSUER_NO_LENGTH
    {
    } else {
        return Err(TreasuryBondsError::InvalidIssuerNoLength.into());
    }

    // 1 - Fixed coupon Treasury bonds
    // 2 - Infrastructure bonds

    let is_valid_bond_type = {
        match params.type_of_bond {
            1 | 2 => true,
            _ => false,
        }
    };

    if !is_valid_bond_type {
        return Err(TreasuryBondsError::InvalidTypeOfBond.into());
    }

    // T-Bonds with maturities of between 2-30 years
    if params.tenor >= TENOR_LENGTH && params.tenor <= TENOR_LENGTH_2 {
    } else {
        return Err(TreasuryBondsError::InvalidBondTenor.into());
    }

    if params.coupon_rate > 0 {
    } else {
        return Err(TreasuryBondsError::InvalidBondCouponRate.into());
    }

    if params.total_amounts_offered > 0 {
    } else {
        return Err(TreasuryBondsError::InvalidAmount.into());
    }

    if params.minimum_bid_amount > 0 {
    } else {
        return Err(TreasuryBondsError::InvalidAmount.into());
    }

    if params.unit_cost_of_treasury_bonds > 0 {
    } else {
        return Err(TreasuryBondsError::InvalidAmount.into());
    }

    if params.value_date.as_bytes().len() > 0 && params.value_date.as_bytes().len() <= DATE_LENGTH {
    } else {
        return Err(TreasuryBondsError::InvalidValueDateLength.into());
    }

    if params.redemption_date.as_bytes().len() > 0
        && params.redemption_date.as_bytes().len() <= DATE_LENGTH
    {
    } else {
        return Err(TreasuryBondsError::InvalidValueRedemptionLength.into());
    }

    if params.decimals == 0 {
        return Err(TreasuryBondsError::InvalidNumeric.into());
    }

    let deposit_account = &mut ctx.accounts.deposit_account;
    let treasury_bonds = &mut ctx.accounts.treasury_bonds;
    let treasury_bonds_configs = &mut ctx.accounts.treasury_bonds_configs;

    // deposit account
    // * - means dereferencing
    deposit_account.owner = *ctx.accounts.owner.key;
    deposit_account.admin_auth_bump = ctx.bumps.pda_auth;
    deposit_account.admin_treasury_vault_bump = Some(ctx.bumps.treasury_vault);
    deposit_account.is_initialized = true;

    // treasury_bonds
    treasury_bonds.owner = *ctx.accounts.owner.key;
    treasury_bonds.issuer.issuer = params.issuer.issuer.to_string();
    treasury_bonds.country = params.country.to_string();
    treasury_bonds.issue_no = params.issue_no.to_string();
    treasury_bonds.type_of_bond = params.type_of_bond;
    treasury_bonds.tenor = params.tenor;
    treasury_bonds.coupon_rate = params.coupon_rate;
    treasury_bonds.total_amounts_offered = params.total_amounts_offered;
    treasury_bonds.minimum_bid_amount = params.minimum_bid_amount;
    treasury_bonds.is_initialized = true;
    treasury_bonds.unit_cost_of_treasury_bonds = params.unit_cost_of_treasury_bonds;
    treasury_bonds.decimals = params.decimals;
    treasury_bonds.value_date = params.value_date.to_string();
    treasury_bonds.redemption_date = params.redemption_date.to_string();

    let bond_issuer = BondIssuer {
        issuer: params.issuer.issuer.to_string(),
    };

    // treasury_bonds_configs
    treasury_bonds_configs.issuers.push(bond_issuer);

    Ok(())
}
