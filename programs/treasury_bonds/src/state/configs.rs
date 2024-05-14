use crate::state::bond_issuer::BondIssuer;
use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct TreasuryBondsConfigs {
    #[max_len(5)]
    pub issuers: Vec<BondIssuer>,
    pub is_initialized: bool,
}
