use anchor_lang::prelude::*;

declare_id!("FvWM5Lq2fFX2o65Tpr5RyWMFbtPELYT4d3cGFimSr3CH");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;
        counter_account.count = 0;

        msg!("Successfully created Account!!");
        msg!("Current counter value: {}", counter_account.count);
        Ok(())
    }

    pub fn increement_count(ctx: Context<Update>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;

        counter_account.count += 1;

        msg!(
            "Counter increemented!! counter value :{}",
            counter_account.count
        );
        Ok(())
    }

    pub fn decreement_count(ctx: Context<Update>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;

        counter_account.count -= 1;

        msg!(
            "Counter decreeement successfully!! counter value: {}",
            counter_account.count
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = DISCRIMINATE_SPACE + Counter::INIT_SPACE)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: i32,
}

const DISCRIMINATE_SPACE: usize = 8;
