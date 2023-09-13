use anchor_lang::prelude::*;

declare_id!("DmWrigyYrGHWu8NsCadhJVXY6BxaJsaDHDD7E3ywgL9W");

#[program]
pub mod token_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
