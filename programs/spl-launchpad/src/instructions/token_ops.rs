use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;

pub fn create_token_mint(_: Context<CreateTokenMint>) -> Result<()> {
    Ok(())
}

pub fn create_token_account(_: Context<CreateTokenAccount>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTokenMint<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = authority,
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// The authority that will have permission to mint tokens
    /// This is a PDA that will be derived in a later instruction
    /// CHECK: This account is not being read or written in this instruction
    pub authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(
        init,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    /// The authority that will have permission to mint tokens
    /// This is a PDA that will be derived in a later instruction
    /// CHECK: This account is not being read or written in this instruction
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}