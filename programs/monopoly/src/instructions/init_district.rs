use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(order: String)]
pub struct InitDistrict<'info> {
    #[account(init_if_needed,
    seeds = [
    b"district".as_ref(),
    order.as_bytes()
    ],
    bump,
    payer = authority,
    space = 8 + std::mem::size_of::<District>())]
    pub district: Box<Account<'info, District>>,
//
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub game: Box<Account<'info, Game>>,
    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<InitDistrict>, order: String) -> ProgramResult {
    let district = &mut ctx.accounts.district;
    let my_string = order.to_string();  // `parse()` works with `&str` and `String`!
    let my_int = my_string.parse::<u16>().unwrap();
//
    district.order = my_int;
    district.occ_number = 0;
    district.authority = ctx.accounts.authority.key();

    msg!("district initialized");
    Ok(())
}