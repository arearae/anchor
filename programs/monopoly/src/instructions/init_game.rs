use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct InitGame<'info> {
    // game
    #[account(init, payer = payer, space = 8 + std::mem::size_of::<Game>())]
    pub game: Box<Account<'info, Game>>,
    pub game_manager: Signer<'info>,

    // misc
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitGame>) -> ProgramResult {
    let game = &mut ctx.accounts.game;

    game.version = LATEST_GAME_VERSION;
    game.game_manager = ctx.accounts.game_manager.key();

    msg!("game initialized, version {}", game.version);
    Ok(())
}