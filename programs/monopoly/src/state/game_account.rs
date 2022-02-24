use anchor_lang::prelude::*;

#[account]
pub struct GameAccount {
    /// each vault is registered with a single bank, used for indexing
    pub game: Pubkey,

    pub owner: Pubkey,

    pub total_stake: u64,

    pub total_money: u64,

    pub authority: Pubkey,
}