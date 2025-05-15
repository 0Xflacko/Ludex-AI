use anchor_lang::prelude::*;
use crate::state::Config;

pub fn initialize(
    ctx: Context<Initialize>,
    admin: Pubkey,
    ldx_token: Pubkey,
    ldx_fee_recipient: Pubkey,
    initial_supply: u64,
    initial_liquidity: u64,
    launch_fee: u64,
    grad_threshold: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = admin;
    config.ldx_token = ldx_token;
    config.ldx_fee_recipient = ldx_fee_recipient;
    config.initial_supply = initial_supply;
    config.initial_liquidity = initial_liquidity;
    config.launch_fee = launch_fee;
    config.grad_threshold = grad_threshold;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = Config::LEN,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
