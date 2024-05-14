// admin instructions
pub mod init;

// public instructions
pub mod register_investor;
pub mod register_treasury_bonds;

// bring everything in scope
pub use {init::*, register_investor::*, register_treasury_bonds::*};
