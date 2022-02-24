use anchor_lang::prelude::*;

pub const LATEST_GAME_VERSION: u16 = 0;

//#[proc_macros::assert_size(120)]
#[repr(C)]
#[account]
pub struct Game {
    pub version: u16,

    pub game_manager: Pubkey,

    pub user_number: u64,
}