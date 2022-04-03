use anchor_lang::prelude::*;

declare_id!("FfrJfB9bXdvvqWTeELj47xoEjjRUJyEMdxy9UnJBG6GZ");

#[program]
pub mod solblogdapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
