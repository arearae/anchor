use anchor_lang::prelude::*;

#[account]
pub struct District {
    pub game: Pubkey,

    pub order: u16,

    pub occ_number: u64,

    pub authority: Pubkey,
}