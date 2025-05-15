use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer}};
use crate::state::{Config, Pool};

pub fn launch(
    ctx: Context<Launch>,
    name: String,
    ticker: String,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let pool = &mut ctx.accounts.pool;
    let payer = &ctx.accounts.payer;
    let token_program = &ctx.accounts.token_program;

    let token_vault = &ctx.accounts.token_vault;
    let asset_vault = &ctx.accounts.asset_vault;

    let payer_asset_ata = &ctx.accounts.payer_asset_ata;
    let ldx_fee_recipient_token_ata = &ctx.accounts.ldx_fee_recipient_token_ata;
    let token_mint = &ctx.accounts.token_mint;
    let asset_mint = &ctx.accounts.asset_mint;

    transfer(
        CpiContext::new(token_program.to_account_info(), Transfer {
            from: payer_asset_ata.to_account_info(),
            to: ldx_fee_recipient_token_ata.to_account_info(),
            authority: payer.to_account_info(),
        }),
        config.launch_fee
    )?;

    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: token_mint.to_account_info(),
                to: token_vault.to_account_info(),
                authority: pool.to_account_info(),
            },
            &[&[
                b"pool",
                token_mint.key().as_ref(),
                asset_mint.key().as_ref(),
                &[ctx.bumps.pool]
            ]]
        ),
        config.initial_supply
    )?;

    pool.name = name.as_bytes().to_vec();
    pool.symbol = ticker.as_bytes().to_vec();
    pool.creator = payer.key();
    pool.token_vault = token_vault.key();
    pool.asset_vault = asset_vault.key();
    pool.reserve_token = config.initial_supply;
    pool.reserve_asset = config.initial_liquidity;

    Ok(())
}

#[derive(Accounts)]
pub struct Launch<'info> {

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = payer,
        space = Pool::LEN,
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
        associated_token::mint = asset_mint,
        associated_token::authority = payer
    )]
    pub payer_asset_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = asset_mint,
        associated_token::authority = config.ldx_fee_recipient
    )]
    pub ldx_fee_recipient_token_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
