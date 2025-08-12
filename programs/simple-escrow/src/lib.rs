use anchor_lang::prelude::*;

declare_id!("6GPwutRaJS5UgUfu5tQn5uYkrWoxoGPJoL8gdMsGiZsY");

#[program]
pub mod simple_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
