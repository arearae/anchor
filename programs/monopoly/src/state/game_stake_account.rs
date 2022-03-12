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

    pub authority_seed: Pubkey,

    pub authority_bump_seed: [u8; 1]
}

impl GameStakeAccount {
    pub fn game_seeds(&self) -> [&[u8]; 2] {
        [self.authority_seed.as_ref(), &self.authority_bump_seed]
    }
}