//! treasury_bonds program entrypoint

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("7EzeMYFy3nrsfLFvBBTZ3NCndKuSkmYHW4Fd2nGDB9uX");

#[program]
pub mod treasury_bonds {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }

    pub fn register_treasury_bonds(
        ctx: Context<RegisterTreasuryBonds>,
        params: RegisterTreasuryBondsParams,
    ) -> Result<()> {
        instructions::register_treasury_bonds(ctx, &params)
    }

    // public instructions
    pub fn register_investor(
        ctx: Context<RegisterInvestor>,
        params: RegisterInvestorParams,
    ) -> Result<()> {
        instructions::register_investor(ctx, &params)
    }

    pub fn buy_treasury_bonds(
        ctx: Context<BuyTreasuryBonds>,
        params: BuyTreasuryBondsParams,
    ) -> Result<()> {
        instructions::buy_treasury_bonds(ctx, &params)
    }

    pub fn sell_treasury_bonds(
        ctx: Context<SellTreasuryBonds>,
        params: SellTreasuryBondsParams,
    ) -> Result<()> {
        instructions::sell_treasury_bonds(ctx, &params)
    }

    pub fn create_token(ctx: Context<CreateToken>, params: CreateTokenParams) -> Result<()> {
        instructions::create_token(ctx, &params)
    }

    pub fn transfer_token(ctx: Context<TransferToken>, params: TransferTokenParams) -> Result<()> {
        instructions::transfer_token(ctx, &params)
    }
}
