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

    // public instructions
    pub fn register_treasury_bonds(
        ctx: Context<RegisterTreasuryBonds>,
        params: RegisterTreasuryBondsParams,
    ) -> Result<()> {
        instructions::register_treasury_bonds(ctx, &params)
    }

    pub fn register_investor(
        ctx: Context<RegisterInvestor>,
        params: RegisterInvestorParams,
    ) -> Result<()> {
        instructions::register_investor(ctx, &params)
    }
}
