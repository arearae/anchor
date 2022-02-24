use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(order: String, district_order: String)]
pub struct InitHouse<'info> {
    #[account(init_if_needed,
    seeds = [
    b"house".as_ref(),
    district_order.as_bytes(),
    order.as_bytes()
    ],
    bump,
    payer = authority,
    space = 8 + std::mem::size_of::<House>())]
    pub house: Box<Account<'info, House>>,
    //
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub game: Box<Account<'info, Game>>,

    #[account(mut)]
    pub game_stake_account: Box<Account<'info, GameStakeAccount>>,
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub district: Box<Account<'info, District>>

}

pub fn handler(ctx: Context<InitHouse>, order: String, district_order: String) -> ProgramResult {
    let district = &mut ctx.accounts.district;
    let house = &mut ctx.accounts.house;
    let order_str = order.to_string();
    let order_int = order_str.parse::<u16>().unwrap();
    let district_order_str = district_order.to_string();
    let district_order_int = district_order_str.parse::<u16>().unwrap();
    house.order = order_int;
    house.district_order = district_order_int;
    house.owner = ctx.accounts.game_stake_account.key();
    house.level = 0;
    house.authority = ctx.accounts.authority.key();
    house.game = ctx.accounts.game.key();
//
    msg!("house initialized");
    Ok(())
}