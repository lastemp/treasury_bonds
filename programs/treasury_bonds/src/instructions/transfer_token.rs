//! TransferToken instruction handler

use {
    crate::{error::TreasuryBondsError, state::treasury_bonds::TreasuryBonds},
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{
        associated_token,
        associated_token::AssociatedToken,
        token::{transfer, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(params: TransferTokenParams)]
pub struct TransferToken<'info> {
    #[account(mut,
        constraint = treasury_bonds.is_initialized @ TreasuryBondsError::AccountNotInitialized
    )]
    pub treasury_bonds: Account<'info, TreasuryBonds>,
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
pub struct TransferTokenParams {
    pub amount: u32,
}

pub fn transfer_token(ctx: Context<TransferToken>, params: &TransferTokenParams) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(TreasuryBondsError::InvalidAmount.into());
    }

    let treasury_bonds = &ctx.accounts.treasury_bonds;
    let decimals = treasury_bonds.decimals;
    let _amount = params.amount;

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
