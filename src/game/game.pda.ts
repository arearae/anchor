import { PublicKey } from '@solana/web3.js';
import { GAME_PROG_ID } from '../index';

export const findGameAccountPDA = async (game: PublicKey, owner: PublicKey) => {
    return PublicKey.findProgramAddress([Buffer.from('game_account'), game.toBytes(), owner.toBytes()], GAME_PROG_ID)
};

export const findGameStakeAccountPDA = async (gameAccount: PublicKey, gameTokenMint: PublicKey) => {
    return PublicKey.findProgramAddress([Buffer.from('game_stake_account'), gameAccount.toBytes(), gameTokenMint.toBytes()], GAME_PROG_ID)
};

export const findGameStakeTokenAccountPDA = async (gameAccount: PublicKey, gameTokenMint: PublicKey) => {
    return PublicKey.findProgramAddress([Buffer.from('game_stake_ata'), gameAccount.toBytes(), gameTokenMint.toBytes()], GAME_PROG_ID)
};

export const findDistrictPDA = async (order: string) => {
    return PublicKey.findProgramAddress([Buffer.from('district'), Buffer.from(order)], GAME_PROG_ID)
};

export const findHousePDA = async (order: string, districtOrder: string) => {
    return PublicKey.findProgramAddress([Buffer.from('house'), Buffer.from(districtOrder), Buffer.from(order)], GAME_PROG_ID)
};