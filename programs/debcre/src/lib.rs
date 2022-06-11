use anchor_lang::prelude::*;

declare_id!("BKa2CykVL5EcJ2E7b6u1Q4jfzo6YpuFiypXdDmGs1chN");

#[program]
pub mod debcre {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
