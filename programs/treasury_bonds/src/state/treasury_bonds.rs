use crate::state::bond_issuer::BondIssuer;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TreasuryBonds {
    pub owner: Pubkey, // publickey of the treasury bonds admin
    pub issuer: BondIssuer,
    #[max_len(3)]
    pub country: String, // home country where treasury bonds is auctioned
    #[max_len(20)]
    pub issue_no: String, // issue no of bond
    pub type_of_bond: u8, // type of bond i.e Fixed coupon Treasury bonds, Infrastructure bonds
    pub tenor: u8,        // maturity period i.e between 2-30 years
    pub coupon_rate: u8,  // coupon rate (%)
    pub total_amounts_offered: u32, // total amounts offered for the given bond
    pub total_amounts_accepted: u32, // total amounts accepted from bondholders (investors)
    pub is_initialized: bool, // is treasury bonds initiated
    #[max_len(10)]
    pub investors: Vec<Pubkey>, // list of the investors
    pub unit_cost_of_treasury_bonds: u32, // unit cost of treasury bonds
    pub decimals: u8,     // decimals for the token mint
    #[max_len(20)]
    pub value_date: String, // value date of bond
    #[max_len(20)]
    pub redemption_date: String, // redemption date of bond
}
