use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct BondIssuer {
    #[max_len(30)]
    pub issuer: String, // issues the bond for purchase eg Republic of Kenya
}
