//! Init instruction handler

use {
    crate::{error::TreasuryBondsError, state::configs::TreasuryBondsConfigs},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct Init<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + TreasuryBondsConfigs::INIT_SPACE,
        constraint = !treasury_bonds_configs.is_initialized @ TreasuryBondsError::AccountAlreadyInitialized,
        seeds = [b"treasury-bonds-configs"],
        bump
    )]
    pub treasury_bonds_configs: Account<'info, TreasuryBondsConfigs>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init(ctx: Context<Init>) -> Result<()> {
    msg!("Validate inputs");

    let treasury_bonds_configs = &mut ctx.accounts.treasury_bonds_configs;

    // treasury bonds
    treasury_bonds_configs.is_initialized = true;

    Ok(())
}
