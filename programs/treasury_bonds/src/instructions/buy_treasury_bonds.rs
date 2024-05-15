//! BuyTreasuryBonds instruction handler

use {
    crate::{
        error::TreasuryBondsError,
        state::{investor::Investor, treasury_bonds::TreasuryBonds},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(params: BuyTreasuryBondsParams)]
pub struct BuyTreasuryBonds<'info> {
    #[account(mut,
        constraint = treasury_bonds.is_initialized @ TreasuryBondsError::AccountNotInitialized
    )]
    pub treasury_bonds: Account<'info, TreasuryBonds>,
    #[account(mut,has_one = owner,
        constraint = investor.active @ TreasuryBondsError::InvalidInvestorStatus
    )]
    pub investor: Account<'info, Investor>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BuyTreasuryBondsParams {
    pub amount: u32,
}

pub fn buy_treasury_bonds(
    ctx: Context<BuyTreasuryBonds>,
    params: &BuyTreasuryBondsParams,
) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(TreasuryBondsError::InvalidAmount.into());
    }

    let sender = &ctx.accounts.owner;
    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let token_program = &ctx.accounts.token_program;
    let treasury_bonds = &mut ctx.accounts.treasury_bonds;
    let investor = &mut ctx.accounts.investor;
    let unit_cost_of_treasury_bonds: u32 = treasury_bonds.unit_cost_of_treasury_bonds;
    let total_amounts_accepted = treasury_bonds.total_amounts_accepted;
    let total_available_funds = treasury_bonds.total_available_funds;
    let minimum_bid_amount = treasury_bonds.minimum_bid_amount;
    let total_units_treasury_bonds: u32 = investor.total_units_treasury_bonds;
    let available_funds: u32 = investor.available_funds;
    let decimals = treasury_bonds.decimals as u64;
    let _amount = params.amount;

    if _amount < minimum_bid_amount {
        return Err(TreasuryBondsError::InvalidMinimumBidAmount.into());
    }

    // Get unit_treasury_bonds from the product of unit_cost_of_treasury_bonds and _amount
    let unit_treasury_bonds = unit_cost_of_treasury_bonds
        .checked_mul(_amount)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Increment total_units_treasury_bonds with new unit_treasury_bonds
    investor.total_units_treasury_bonds = total_units_treasury_bonds
        .checked_add(unit_treasury_bonds)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Increment available_funds with new _amount
    investor.available_funds = available_funds
        .checked_add(_amount)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Increment total_amounts_accepted with new _amount
    treasury_bonds.total_amounts_accepted = total_amounts_accepted
        .checked_add(_amount)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Increment total_available_funds with new _amount
    treasury_bonds.total_available_funds = total_available_funds
        .checked_add(_amount)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    let base: u32 = 10;
    let exponent = treasury_bonds.decimals as u32;

    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    let result = (base).pow(exponent);
    let _amount = (_amount as u64)
        .checked_mul(result as u64)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    treasury_bonds.investors.push(*sender.key);

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: sender_tokens.to_account_info(),
                to: recipient_tokens.to_account_info(),
                authority: sender.to_account_info(),
            },
        ),
        _amount,
    )?;

    Ok(())
}
