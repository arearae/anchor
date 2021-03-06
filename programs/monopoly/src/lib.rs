use anchor_lang::prelude::*;
use instructions::*;

declare_id!("monvVbcL8zas8HjPcx3FbHiAK4Nqi7VZrruM7gcyZin");

pub mod instructions;
pub mod state;

#[program]
pub mod monopoly {
    use super::*;

    pub fn init_game(ctx: Context<InitGame>) -> ProgramResult {
        instructions::init_game::handler(ctx)
    }

    pub fn init_game_account(ctx: Context<InitGameAccount>) -> ProgramResult {
        instructions::init_game_account::handler(ctx)
    }

    pub fn init_game_stake_account(ctx: Context<InitGameStakeAccount>) -> ProgramResult {
        instructions::init_game_stake_account::handler(ctx)
    }

    pub fn init_district(ctx: Context<InitDistrict>, order: String) -> ProgramResult {
        instructions::init_district::handler(ctx, order)
    }

    pub fn init_house(ctx: Context<InitHouse>, order: String, district_order: String) -> ProgramResult {
        instructions::init_house::handler(ctx, order, district_order)
    }

    pub fn game_unstake_account(ctx: Context<GameUnstakeAccount>) -> ProgramResult {
        instructions::game_unstake_account::handler(ctx)
    }
}
