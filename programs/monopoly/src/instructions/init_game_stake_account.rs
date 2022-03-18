use std::str::FromStr;
use std::io::Write;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::*;

#[derive(Accounts)]
pub struct InitGameStakeAccount<'info> {
    #[account(init_if_needed,
    seeds = [
    b"game_stake_account".as_ref(),
    game_account.key().as_ref(),
    game_token_mint.key().as_ref()
    ],
    bump,
    payer = owner,
    space = 8 + std::mem::size_of::<GameStakeAccount>())]
    pub game_stake_account: Box<Account<'info, GameStakeAccount>>,

    #[account(mut)]
    pub game_account: Box<Account<'info, GameAccount>>,

    #[account(mut)]
    pub game_token_source: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub game: Box<Account<'info, Game>>,

    pub game_token_mint: Box<Account<'info, Mint>>,
    #[account(init_if_needed,
    token::mint = game_token_mint,
    token::authority = game_account,
    seeds = [
    b"game_stake_ata".as_ref(),
    game_account.key().as_ref(),
    game_token_mint.key().as_ref()
    ],
    bump,
    payer = owner)]
    pub game_token_pda_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitGameStakeAccount<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.game_token_source.to_account_info(),
                to: self.game_token_pda_ata.to_account_info(),
                authority: self.owner.to_account_info(),
            },
        )
    }
}

pub fn handler(ctx: Context<InitGameStakeAccount>, time: String) -> ProgramResult {
    token::transfer(
        ctx.accounts
            .transfer_ctx(),
        1,
    )?;
    // record total number of vaults in bank's state
    let game_account = &mut ctx.accounts.game_account;
    let game_stake_account = &mut ctx.accounts.game_stake_account;
    let game_token_mint = &ctx.accounts.game_token_mint;

    game_account.total_stake += 1;

    game_stake_account.game = ctx.accounts.game.key();
    game_stake_account.game_token_mint = ctx.accounts.game_token_mint.key();
    game_stake_account.owner = ctx.accounts.owner.key();
    game_stake_account.game_token_source = ctx.accounts.game_token_source.key();
    game_stake_account.game_token_pda_ata = ctx.accounts.game_token_pda_ata.key();
    game_stake_account.money = 0;
    game_stake_account.status = 0;
    game_stake_account.game_account = game_account.key();
    game_stake_account.authority = ctx.accounts.authority.key();
    game_stake_account.status = 1;
//    (&mut game_stake_account.time[..]).write_all(time.as_bytes())?;
    msg!("new game account founded by {}", &ctx.accounts.authority.key());
    Ok(())
}
