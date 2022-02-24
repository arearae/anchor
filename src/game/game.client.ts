import * as anchor from '@project-serum/anchor';
import { BN, Idl, Program, Provider } from '@project-serum/anchor';
import { Connection, Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import {TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { AccountUtils, isKp } from '../game_common';
import { Monopoly } from '../../target/types/monopoly';
import {
    findDistrictPDA,
    findGameAccountPDA,
    findGameStakeAccountPDA,
    findGameStakeTokenAccountPDA,
    findHousePDA
} from "./game.pda";

export class GameClient extends AccountUtils {
    wallet: anchor.Wallet;
    provider!: anchor.Provider;
    gameProgram!: anchor.Program<Monopoly>;

    constructor(
        conn: Connection,
        // @ts-ignore
        wallet: anchor.Wallet,
        idl?: Idl,
        programId?: PublicKey
    ) {
        super(conn);
        this.wallet = wallet;
        this.setProvider();
        this.setGameProgram(idl, programId);
    }

    setProvider() {
        this.provider = new Provider(
            this.conn,
            this.wallet,
            Provider.defaultOptions()
        );
        anchor.setProvider(this.provider);
    }

    setGameProgram(idl?: Idl, programId?: PublicKey) {
        //instantiating program depends on the environment
        if (idl && programId) {
            //means running in prod
            this.gameProgram = new anchor.Program<Monopoly>(
                idl as any,
                programId,
                this.provider
            );
        } else {
            //means running inside test suite
            // @ts-ignore
            this.gameProgram = anchor.workspace.Monopoly as Program<Monopoly>;
        }
    }

    async initGame(
        game: Keypair,
        gameManager: PublicKey | Keypair,
        payer: PublicKey | Keypair
    ) {
        const signers = [game];
        if (isKp(gameManager)) signers.push(<Keypair>gameManager);

        console.log('starting game at', game.publicKey.toBase58());
        console.log(game.publicKey)
        console.log(payer)
        console.log(gameManager)
        const txSig = await this.gameProgram.rpc.initGame({
            accounts: {
                game: game.publicKey,
                gameManager: isKp(gameManager)
                    ? (<Keypair>gameManager).publicKey
                    : gameManager,
                payer: isKp(payer) ? (<Keypair>payer).publicKey : payer,
                systemProgram: SystemProgram.programId,
            },
            signers,
        });
        return { txSig };
    }

    async initGameAccount(
        game: PublicKey,
        owner: Keypair,
        authority: PublicKey | Keypair,

    ) {
        const [gameAccount, bump] = await findGameAccountPDA(game, owner.publicKey);

        const signers = [];
        signers.push(owner);
        const tx2 = await this.gameProgram.rpc.initGameAccount({
            accounts: {
                gameAccount,
                game,
                owner: owner.publicKey,
                authority,
                systemProgram: SystemProgram.programId,
            },
            signers,
        })

        return { tx2 };
    }

    async initGameStakeAccount(
        gameAccount: PublicKey,
        owner: Keypair,
        authority: PublicKey,
        gameTokenSource: PublicKey,
        gameTokenMint: PublicKey
    ){
        const [gameStakeAccount, bump] = await findGameStakeAccountPDA(gameAccount, gameTokenMint);
        const [gameTokenPdaAta, bump1] = await findGameStakeTokenAccountPDA(gameAccount, gameTokenMint);
        const signers = [];
        signers.push(owner);
        const tx3 = await this.gameProgram.rpc.initGameStakeAccount({
            accounts: {
                gameStakeAccount,
                gameAccount,
                gameTokenSource,
                gameTokenMint,
                gameTokenPdaAta,
                owner: owner.publicKey,
                authority,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            },
            signers,
        })
        return { tx3 };
    }

    async initDistrict(
        game: PublicKey,
        authority: PublicKey | Keypair,
        order: string
    ) {
        const [district, bump] = await findDistrictPDA(order);
        const signers = [];
        const tx3 = await this.gameProgram.rpc.initDistrict(order, {
            accounts: {
                district,
                authority,
                game,
                systemProgram: SystemProgram.programId,
            },
            signers,
        })

        return { tx3 };
    }

    async initHouse(
        game: PublicKey,
        authority: PublicKey | Keypair,
        order: string,
        districtOrder: string,
        owner: PublicKey,
        tokenMint: PublicKey
    ){
        const [district, bump] = await findDistrictPDA(districtOrder);
        const [house, houseBump] = await findHousePDA(order, districtOrder);
        const [gameAccount, gameAccountBump] = await findGameAccountPDA(game, owner);
        const [gameStakeAccount, bump1] = await findGameStakeAccountPDA(gameAccount, tokenMint);
        const signers = [];
        const tx = await this.gameProgram.rpc.initHouse(order, districtOrder, {
            accounts: {
                house,
                district,
                authority,
                game,
                gameStakeAccount,
                systemProgram: SystemProgram.programId,
            },
            signers,
        })
        return {tx}

    }

    async fetchGameAcc(game: PublicKey) {
        return this.gameProgram.account.game.fetch(game);
    }

    async fetchGameAccount(game: PublicKey, owner: PublicKey) {
        const [gameAcc, _ ] = await findGameAccountPDA(game, owner);
        return this.gameProgram.account.gameAccount.fetch(gameAcc);
    }

    async fetchGameStakeAccount(game: PublicKey, owner: PublicKey, mintToken: PublicKey) {
        const [gameAcc, bumpGame ] = await findGameAccountPDA(game, owner);
        const [gameStakeAcc, _] = await findGameStakeAccountPDA(gameAcc, mintToken);
        return this.gameProgram.account.gameStakeAccount.fetch(gameStakeAcc);
    }

    async fetchGameDistrict() {
        return this.gameProgram.account.district.all();
    }

    async fetchGameAccountByGame(game: PublicKey) {
        const filter = game
            ? [
              {
                memcmp: {
                  offset: 8, //need to prepend 8 bytes for anchor's disc
                  bytes: game.toBase58(),
                },
              },
            ]
            : [];
        return this.gameProgram.account.gameAccount.all(filter);
    }
}