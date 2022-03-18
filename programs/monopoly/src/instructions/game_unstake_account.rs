use std::str::FromStr;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

use crate::state::*;

#[derive(Accounts)]
pub struct GameUnstakeAccount<'info> {
    #[account(mut)]
    pub game: Box<Account<'info, Game>>,
    #[account(mut, has_one = owner)]
    pub game_stake_account: Box<Account<'info, GameStakeAccount>>,

    #[account(mut)]
    pub game_account: Box<Account<'info, GameAccount>>,

    #[account(init_if_needed,
    associated_token::mint = game_token_mint,
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

    pub associated_token_program: Program<'info, AssociatedToken>,
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
    let game_stake_account = &mut ctx.accounts.game_stake_account;
    let game_token_pda_ata = &ctx.accounts.game_token_pda_ata;
//    let seed = &[
//        b"metadata".as_ref(),
//        metadata_program.as_ref(),
//        gem_mint.as_ref(),
//    ];
//
//    let (metadata_addr, _bump) = Pubkey::find_program_address(seed, &metadata_program);
    let program = Pubkey::from_str("rchGw7oZWwSq41eyXG3ofhzA3X4oJCqeRv2iAwteVxS").unwrap();
    let game = &ctx.accounts.game;
    let owner = &*ctx.accounts.owner;
    let game_addr = game.key();
    let owner_addr = owner.key();
    let seeds = &[
        b"game_account".as_ref(),
        &game_addr.as_ref(),
        &owner_addr.as_ref(),
    ];
    let (_addr, bump) = Pubkey::find_program_address(seeds, &program);
    let sign_seeds = &[
        b"game_account".as_ref(),
        &game_addr.as_ref(),
        &owner_addr.as_ref(),
        &[bump],
    ];
//    let signer = &[&seeds[..]];

    token::transfer(
        ctx.accounts
            .transfer_ctx()
            .with_signer(&[sign_seeds]),
        1,
    )?;
    game_stake_account.status = 0;
    msg!("withdrawn from ${} gem box", game_token_pda_ata.key());
    Ok(())
}