use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub ldx_token: Pubkey,
    pub ldx_fee_recipient: Pubkey,
    pub initial_supply: u64,
    pub initial_liquidity: u64,
    pub launch_fee: u64,
    pub grad_threshold: u64,
}


impl Config {
    pub const LEN: usize = 8 + // discriminator
        32 + // admin
        32 + // ldx_token
        32 + // ldx_fee_recipient
        8 + // initial_supply
        8 + // initial_liquidity
        8 + // launch_fee
        8; // grad_threshold
}

#[account]
pub struct Pool {
    pub name: Vec<u8>,
    pub symbol: Vec<u8>,
    pub creator: Pubkey,
    pub token_vault: Pubkey,
    pub asset_vault: Pubkey,
    pub reserve_token: u64,
    pub reserve_asset: u64,
}

impl Pool {
    pub const LEN: usize = 8 + // discriminator
        4 + 32 + // name (dynamic array of 32 bytes)
        4 + 32 + // symbol (dynamic array of 32 bytes)
        32 + // creator
        32 + // token_vault
        32 + // asset_vault
        8 + // reserve_token
        8; // reserve_asset
}
