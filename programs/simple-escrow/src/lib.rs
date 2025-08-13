use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("6GPwutRaJS5UgUfu5tQn5uYkrWoxoGPJoL8gdMsGiZsY");

#[program]
pub mod simple_escrow {
    use super::*;

    pub fn initialize_escrow(ctx: Context, amount: u64, recipient: Pubkey) -> Result<()> {
        Ok(())
    }
    pub fn withdraw(ctx: Context) -> Result<()> {
        Ok(())
    }
    pub fn cancel_escrow(ctx: Context) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {}

#[derive(Accounts)]
pub struct Withdraw<'info> {}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {}

#[account]
pub struct Escrow {
    pub depositer: Pubkey,
    pub receipent: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub created_at: i64,
    pub is_completed: bool,
    pub bump: u8,
}

impl Escrow {
    const LEN: usize = 8 + 32 + 32 + 32 + 8 + 8 + 1 + 1;
}

#[error_code]
pub enum EscrowError {
    #[msg("Invalid amount: must be greater than 0")]
    InvalidAmount,
    #[msg("Insufficient funds in token account")]
    InsufficientFunds,
    #[msg("Escrow already completed")]
    AlreadyCompleted,
    #[msg("Unauthorized: wrong signer")]
    Unauthorized,
}
