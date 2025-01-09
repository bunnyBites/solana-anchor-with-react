use anchor_lang::prelude::*;

declare_id!("8iufEv7F5kQ5TW62WA27aoqAnFS3qBusRXwfuKWpNqEG");

#[program]
pub mod movie_review {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        require!(title.len() < 20, MovieReviewError::TitleTooLong);

        require!(description.len() < 50, MovieReviewError::DescriptionTooLong);

        require!(rating < 5 && rating > 1, MovieReviewError::InvalidRating);

        msg!(
            "Title: {}, Description: {}, rating: {}",
            title,
            description,
            rating
        );

        let movie_account = &mut ctx.accounts.movie_account;

        movie_account.bump = ctx.bumps.movie_account;
        movie_account.description = description;
        movie_account.title = title;
        movie_account.rating = rating;
        movie_account.reviewer = ctx.accounts.reviewer.key();

        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn update(
        ctx: Context<Update>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        require!(title.len() < 20, MovieReviewError::TitleTooLong);

        require!(description.len() < 50, MovieReviewError::DescriptionTooLong);

        require!(rating < 5 || rating > 1, MovieReviewError::InvalidRating);

        msg!(
            "Title: {}, Description: {}, rating: {}",
            title,
            description,
            rating
        );

        let movie_account = &mut ctx.accounts.movie_account;
        movie_account.description = description;
        movie_account.rating = rating;
        Ok(())
    }

    pub fn delete_movie(_ctx: Context<Delete>, title: String) -> Result<()> {
        msg!("Account Deleted!! for movie: {}", title);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Update<'info> {
    #[account(
      mut,
      seeds=[title.as_bytes(), reviewer.key().as_ref()],
      bump=movie_account.bump,
      realloc= 8 + Movie::INIT_SPACE,
      realloc::payer = reviewer,
      realloc::zero = true,
    )]
    pub movie_account: Account<'info, Movie>,

    #[account(mut)]
    pub reviewer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Delete<'info> {
    #[account(
      mut,
      close=reviewer,
      seeds=[title.as_bytes(), reviewer.key().as_ref()],
      bump=movie_account.bump
    )]
    pub movie_account: Account<'info, Movie>,

    #[account(mut)]
    pub reviewer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Initialize<'info> {
    #[account(
      init,
      seeds=[title.as_bytes(), reviewer.key().as_ref()],
      bump,
      payer = reviewer,
      space = 8 + Movie::INIT_SPACE,
    )]
    pub movie_account: Account<'info, Movie>,

    #[account(mut)]
    pub reviewer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Movie {
    #[max_len(20)]
    pub title: String,

    #[max_len(50)]
    pub description: String,

    pub rating: u8,
    pub reviewer: Pubkey,
    pub bump: u8,
}

#[error_code]
pub enum MovieReviewError {
    #[msg("Rating should be between 1 and 5")]
    InvalidRating,

    #[msg("Title length is greater than 20")]
    TitleTooLong,

    #[msg("Description length is greater than 50")]
    DescriptionTooLong,
}
