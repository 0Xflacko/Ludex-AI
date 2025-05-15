use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer, Mint, Token, TokenAccount, Transfer}};

use crate::state::Pool;
use crate::utils::amm::get_amounts_out;
use crate::error::ArithmeticError;
pub fn buy(ctx: Context<Purchase>, amount_in: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let payer = &ctx.accounts.payer;
    let token_program = &ctx.accounts.token_program;

    let token_vault = &ctx.accounts.token_vault;
    let asset_vault = &ctx.accounts.asset_vault;

    let user_token_ata = &ctx.accounts.user_token_ata;
    let user_asset_ata = &ctx.accounts.user_asset_ata;

    let token_mint = &ctx.accounts.token_mint;
    let asset_mint = &ctx.accounts.asset_mint;
    let amount_out = get_amounts_out(amount_in, pool.reserve_asset, pool.reserve_token)?;

    pool.reserve_asset = pool.reserve_asset.checked_add(amount_in).ok_or(ArithmeticError::Overflow)?;
    pool.reserve_token = pool.reserve_token.checked_sub(amount_out).ok_or(ArithmeticError::Overflow)?;

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: user_asset_ata.to_account_info(),
                to: asset_vault.to_account_info(),
                authority: payer.to_account_info(),
            }
        ),
        amount_in
    )?;

    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: token_vault.to_account_info(),
                to: user_token_ata.to_account_info(),
                authority: pool.to_account_info(),
            },
            &[&[
                b"pool", 
                token_mint.key().as_ref(), 
                asset_mint.key().as_ref(),
                &[ctx.bumps.pool]
            ]]
        ),
        amount_out
    )?;

    Ok(())
}

pub fn sell(ctx: Context<Purchase>, amount_in: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let payer = &ctx.accounts.payer;
    let token_program = &ctx.accounts.token_program;

    let token_vault = &ctx.accounts.token_vault;
    let asset_vault = &ctx.accounts.asset_vault;

    let user_token_ata = &ctx.accounts.user_token_ata;
    let user_asset_ata = &ctx.accounts.user_asset_ata;

    let token_mint = &ctx.accounts.token_mint;
    let asset_mint = &ctx.accounts.asset_mint;

    let amount_out = get_amounts_out(amount_in, pool.reserve_token, pool.reserve_asset)?;

    pool.reserve_token = pool.reserve_token.checked_add(amount_in).ok_or(ArithmeticError::Overflow)?;
    pool.reserve_asset = pool.reserve_asset.checked_sub(amount_out).ok_or(ArithmeticError::Overflow)?;

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: user_token_ata.to_account_info(),
                to: token_vault.to_account_info(),
                authority: payer.to_account_info(),
            }
        ),
        amount_in
    )?;

    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: asset_vault.to_account_info(),
                to: user_asset_ata.to_account_info(),
                authority: pool.to_account_info(),
            },
            &[&[
                b"pool", 
                token_mint.key().as_ref(), 
                asset_mint.key().as_ref(),
                &[ctx.bumps.pool]
            ]]
        ),
        amount_out
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(
        mut,
        seeds = [b"pool", token_mint.key().as_ref(), asset_mint.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub asset_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = pool
    )]
    pub token_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = asset_mint,
        associated_token::authority = pool
    )]
    pub asset_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = payer
    )]
    pub user_token_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = asset_mint,
        associated_token::authority = payer
    )]
    pub user_asset_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}