#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
mod state;
mod error;
mod utils;

use instructions::*;

declare_id!("GXz7nU9XqTPcrQY5MJ2vdKLmHfBsNw13ELioZyAeM5Rb");

#[program]
pub mod spl_launchpad {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey, ldx_token: Pubkey, ldx_fee_recipient: Pubkey, initial_supply: u64, initial_liquidity: u64, launch_fee: u64, grad_threshold: u64) -> Result<()> {
        initialize::initialize(ctx, admin, ldx_token, ldx_fee_recipient, initial_supply, initial_liquidity, launch_fee, grad_threshold)
    }

    pub fn launch(ctx: Context<Launch>, name: String, ticker: String) -> Result<()> {
        launch::launch(ctx, name, ticker)
    }

    pub fn buy(ctx: Context<Purchase>, amount_in: u64) -> Result<()> {
        purchase::buy(ctx, amount_in)
    }

    pub fn sell(ctx: Context<Purchase>, amount_out: u64) -> Result<()> {
        purchase::sell(ctx, amount_out)
    }

    pub fn proxy_initialize(ctx: Context<ProxyInitialize>, init_amount_0: u64, init_amount_1: u64, open_time: u64) -> Result<()> {
        graduate::proxy_initialize(ctx, init_amount_0, init_amount_1, open_time)
    }

    pub fn proxy_deposit(ctx: Context<ProxyDeposit>, lp_token_amount: u64, maximum_token_0_amount: u64, maximum_token_1_amount: u64) -> Result<()> {
        graduate::proxy_deposit(ctx, lp_token_amount, maximum_token_0_amount, maximum_token_1_amount)
    }

    pub fn create_token_mint(ctx: Context<CreateTokenMint>) -> Result<()> {
        token_ops::create_token_mint(ctx)
    }

    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        token_ops::create_token_account(ctx)
    }
}
