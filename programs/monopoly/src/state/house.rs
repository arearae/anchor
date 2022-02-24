use anchor_lang::prelude::*;

#[account]
pub struct House {
    pub order: u16,

    pub district_order: u16,

    pub game: Pubkey,

    pub owner: Pubkey,

    pub level: u16,

    pub authority: Pubkey,
}