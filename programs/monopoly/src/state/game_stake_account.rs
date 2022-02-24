use anchor_lang::prelude::*;

#[account]
pub struct GameStakeAccount {
    pub game_account: Pubkey,

    pub game_token_mint: Pubkey,

    pub game_token_pda_ata: Pubkey,

    pub game_token_source: Pubkey,

    pub game: Pubkey,

    pub owner: Pubkey,

    pub money: u64,

    pub authority: Pubkey,

    pub status: u16,
}