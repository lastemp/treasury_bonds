//! SellTreasuryBonds instruction handler

use {
    crate::{
        error::TreasuryBondsError,
        state::{deposit_base::DepositBase, investor::Investor, treasury_bonds::TreasuryBonds},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(params: SellTreasuryBondsParams)]
pub struct SellTreasuryBonds<'info> {
    #[account(mut,
        constraint = treasury_bonds.is_initialized @ TreasuryBondsError::AccountNotInitialized,
        constraint = !treasury_bonds.is_matured @ TreasuryBondsError::InvalidBondMaturityStatus
    )]
    pub treasury_bonds: Account<'info, TreasuryBonds>,
    #[account(mut,
        constraint = seller_investor.active @ TreasuryBondsError::InvalidInvestorStatus
    )]
    pub seller_investor: Account<'info, Investor>,
    #[account(mut,has_one = owner,
        constraint = buyer_investor.active @ TreasuryBondsError::InvalidInvestorStatus
    )]
    pub buyer_investor: Account<'info, Investor>,
    #[account(mut)]
    pub from_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_token: Signer<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SellTreasuryBondsParams {
    pub amount: u32,
}

pub fn sell_treasury_bonds(
    ctx: Context<SellTreasuryBonds>,
    params: &SellTreasuryBondsParams,
) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(TreasuryBondsError::InvalidAmount.into());
    }

    let treasury_bonds = &mut ctx.accounts.treasury_bonds;
    let seller_investor = &mut ctx.accounts.seller_investor;
    let buyer_investor = &mut ctx.accounts.buyer_investor;
    let unit_cost_of_treasury_bonds: u32 = treasury_bonds.unit_cost_of_treasury_bonds;
    let total_units_treasury_bonds_seller: u32 = seller_investor.total_units_treasury_bonds;
    let available_funds_seller: u32 = seller_investor.available_funds;
    let total_units_treasury_bonds_buyer: u32 = buyer_investor.total_units_treasury_bonds;
    let available_funds_buyer: u32 = buyer_investor.available_funds;
    let decimals: u8 = treasury_bonds.decimals;
    let _amount = params.amount;

    // investor's(seller) available funds should exceed zero
    if available_funds_seller == 0 {
        return Err(TreasuryBondsError::InsufficientFunds.into());
    }

    // investor's(seller) available funds should match transfer amount
    if available_funds_seller == _amount {
    } else {
        return Err(TreasuryBondsError::MismatchedAmount.into());
    }

    // Get unit_cost_of_treasury_bonds from the product of unit_cost_of_treasury_bonds and actual_amount
    let unit_cost_of_treasury_bonds = unit_cost_of_treasury_bonds
        .checked_mul(_amount)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Deduct sold unit_cost_of_treasury_bonds from seller_investor's total_units_treasury_bonds
    seller_investor.total_units_treasury_bonds = total_units_treasury_bonds_seller
        .checked_sub(unit_cost_of_treasury_bonds)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Deduct actual_amount(sold unit_cost_of_treasury_bonds) from seller_investor's available funds
    seller_investor.available_funds = available_funds_seller
        .checked_sub(_amount)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Increment buyer's total_units_treasury_bonds with new unit_treasury_bonds
    buyer_investor.total_units_treasury_bonds = total_units_treasury_bonds_buyer
        .checked_add(unit_cost_of_treasury_bonds)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Increment buyer's available_funds with new _amount
    buyer_investor.available_funds = available_funds_buyer
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

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: ctx.accounts.owner.to_account_info(),
                from: ctx.accounts.from_account.to_account_info(),
                to: ctx.accounts.to_account.to_account_info(),
            },
        ),
        _amount,
    )?;

    Ok(())
}
