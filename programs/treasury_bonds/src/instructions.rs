// admin instructions
pub mod init;
pub mod register_treasury_bonds;

// public instructions
pub mod buy_treasury_bonds;
pub mod create_token;
pub mod register_investor;
pub mod sell_treasury_bonds;
pub mod transfer_token;

// bring everything in scope
pub use {
    buy_treasury_bonds::*, create_token::*, init::*, register_investor::*,
    register_treasury_bonds::*, sell_treasury_bonds::*, transfer_token::*,
};
