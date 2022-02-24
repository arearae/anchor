use anchor_lang::prelude::*;
use gem_common::*;

use crate::state::*;

#[derive(Accounts)]
pub struct InitGameAccount<'info> {
    // vault
    #[account(init_if_needed,
    seeds = [
    b"game_account".as_ref(),
    game.key().as_ref(),
    owner.key().as_ref(),
    ],
    bump,
    payer = authority,
    space = 8 + std::mem::size_of::<GameAccount>())]
    pub game_account: Box<Account<'info, GameAccount>>,
    // bank
    #[account(mut)]
    pub game: Box<Account<'info, Game>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitGameAccount>) -> ProgramResult {
    // record total number of vaults in bank's state
    let game = &mut ctx.accounts.game;
    let game_account = &mut ctx.accounts.game_account;
    game.user_number.try_add_assign(1)?;
    game_account.owner = ctx.accounts.owner.key();
    game_account.game = game.key();
    game_account.total_stake = 0;
    game_account.total_money = 0;
    game_account.authority = ctx.accounts.authority.key();
    msg!("new game account founded by {}", &ctx.accounts.authority.key());
    Ok(())
}
