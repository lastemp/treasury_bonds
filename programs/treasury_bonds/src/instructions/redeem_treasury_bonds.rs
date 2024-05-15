//! RedeemTreasuryBonds instruction handler

use {
    crate::{
        error::TreasuryBondsError,
        state::{deposit_base::DepositBase, investor::Investor, treasury_bonds::TreasuryBonds},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
    },
};

#[derive(Accounts)]
#[instruction(params: RedeemTreasuryBondsParams)]
pub struct RedeemTreasuryBonds<'info> {
    #[account(mut,
        constraint = treasury_bonds.is_initialized @ TreasuryBondsError::AccountNotInitialized,
        constraint = treasury_bonds.is_matured @ TreasuryBondsError::InvalidBondMaturityStatus
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
    #[account(mut,
        constraint = deposit_account.is_initialized @ TreasuryBondsError::AccountNotInitialized
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
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RedeemTreasuryBondsParams {
    pub amount: u32,
}

pub fn redeem_treasury_bonds(
    ctx: Context<RedeemTreasuryBonds>,
    params: &RedeemTreasuryBondsParams,
) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(TreasuryBondsError::InvalidAmount.into());
    }

    let treasury_bonds = &mut ctx.accounts.treasury_bonds;
    let investor = &mut ctx.accounts.investor;
    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let mint_token = &ctx.accounts.mint_token;
    let deposit_account = &ctx.accounts.deposit_account;
    let pda_auth = &mut ctx.accounts.pda_auth;
    let treasury_vault = &mut ctx.accounts.treasury_vault;
    let token_program = &ctx.accounts.token_program;
    let unit_cost_of_treasury_bonds: u32 = treasury_bonds.unit_cost_of_treasury_bonds;
    let total_available_funds = treasury_bonds.total_available_funds;
    let total_units_treasury_bonds: u32 = investor.total_units_treasury_bonds;
    let available_funds: u32 = investor.available_funds;
    let decimals: u8 = treasury_bonds.decimals;
    let _amount = params.amount;

    // investor's available funds should exceed zero
    if available_funds == 0 {
        return Err(TreasuryBondsError::InsufficientFunds.into());
    }

    // investor's available funds should match transfer amount
    if available_funds == _amount {
    } else {
        return Err(TreasuryBondsError::MismatchedAmount.into());
    }

    // treasury's available funds should exceed(or equal) transfer amount
    if total_available_funds >= _amount {
    } else {
        return Err(TreasuryBondsError::InsufficientFunds.into());
    }

    // Get unit_cost_of_treasury_bonds from the product of unit_cost_of_treasury_bonds and actual_amount
    let unit_cost_of_treasury_bonds = unit_cost_of_treasury_bonds
        .checked_mul(_amount)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Deduct sold unit_cost_of_treasury_bonds from investor's total_units_treasury_bonds
    investor.total_units_treasury_bonds = total_units_treasury_bonds
        .checked_sub(unit_cost_of_treasury_bonds)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Deduct actual_amount(sold unit_cost_of_treasury_bonds) from investor's available funds
    investor.available_funds = available_funds
        .checked_sub(_amount)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Deduct actual_amount(sold unit_cost_of_treasury_bonds) from total_available_funds
    treasury_bonds.total_available_funds = total_available_funds
        .checked_sub(_amount)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    let base: u32 = 10;
    let exponent = treasury_bonds.decimals as u32;
    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    let result = (base).pow(exponent);
    let _amount = (_amount as u64)
        .checked_mul(result as u64)
        .ok_or(TreasuryBondsError::InvalidArithmeticOperation)?;

    // Transfer funds from treasury vault to recipient
    let cpi_accounts = TransferChecked {
        from: sender_tokens.to_account_info(),
        mint: mint_token.to_account_info(),
        to: recipient_tokens.to_account_info(),
        authority: treasury_vault.to_account_info(),
    };

    let seeds = &[
        b"treasury-vault",
        pda_auth.to_account_info().key.as_ref(),
        &[deposit_account.admin_treasury_vault_bump.unwrap()],
    ];

    let signer = &[&seeds[..]];

    let cpi = CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, signer);

    transfer_checked(cpi, _amount, decimals)?;

    Ok(())
}
