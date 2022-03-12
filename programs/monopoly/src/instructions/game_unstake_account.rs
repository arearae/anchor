use anchor_lang::prelude::*;
use gem_common::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::*;

#[derive(Accounts)]
pub struct GameUnstakeAccount<'info> {
    #[account(mut) has_one = owner]
    pub game_stake_account: Box<Account<'info, GameStakeAccount>>,

    #[account(mut)]
    pub game_account: Box<Account<'info, GameAccount>>,

    #[account(init_if_needed,
    associated_token::mint = gem_mint,
    associated_token::authority = receiver,
    payer = owner)]
    pub game_token_source: Box<Account<'info, TokenAccount>>,

    pub game_token_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub game_token_pda_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

impl<'info> GameUnstakeAccount<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.game_token_pda_ata.to_account_info(),
                to: self.game_token_source.to_account_info(),
                authority: self.game_account.to_account_info(),
            },
        )
    }
}

pub fn handler(ctx: Context<GameUnstakeAccount>) -> ProgramResult {
    let game_take_account = &ctx.accounts.game_stake_account;

    token::transfer(
        ctx.accounts
            .transfer_ctx()
            .with_signer(&[&game_take_account.game_seeds()]),
        1,
    )?;
    msg!("withdrawn from ${} gem box", game_token_pda_ata.key());
    Ok(())
}