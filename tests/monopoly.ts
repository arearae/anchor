import * as anchor from '@project-serum/anchor';
import {Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram} from "@solana/web3.js";
import { BN } from '@project-serum/anchor';
import { NodeWallet } from '../src/game_common';
import {TOKEN_PROGRAM_ID} from "@solana/spl-token";
import {findGameAccountPDA, GameClient} from "../src/game";

describe('monopoly', () => {
  anchor.setProvider(anchor.Provider.env());
  const _provider = anchor.getProvider();
  console.log(_provider)

  const gb = new GameClient(
      _provider.connection,
      // @ts-ignore
      _provider.wallet as anchor.Wallet
  );
  const nw = new NodeWallet(
      _provider.connection,
      // @ts-ignore
      _provider.wallet as anchor.Wallet
  );
  const game = Keypair.generate();

  const gameManager = nw.wallet.publicKey;

  it('Is initialized!', async () => {
    const tx = await gb.initGame(game, gameManager, gameManager);
    const gameId = await gb.fetchGameAcc(game.publicKey);
    console.log(gameId);
    // const gameId = await gb.fetchGameAcc(game.publicKey);
    // console.log(gameId);
    const owner = await nw.createFundedWallet(10000 * LAMPORTS_PER_SOL);
    const owner2 = await nw.createFundedWallet(100 * LAMPORTS_PER_SOL);
    const tx1 = await gb.initGameAccount(game.publicKey, owner, gameManager);
    const tx2 = await gb.initGameAccount(game.publicKey, owner2, gameManager);
    const pdas = await gb.fetchGameAccountByGame(game.publicKey);
    const {tokenMint, tokenAcc} = await nw.createMintAndFundATA(owner.publicKey, new BN(1));
    console.log(pdas);
    const [gameAccount1, _] = await findGameAccountPDA(game.publicKey, owner.publicKey);
    const tx3 = await gb.initGameStakeAccount(gameAccount1, owner, gameManager, tokenAcc, tokenMint, game.publicKey);
    const gc = await gb.fetchGameStakeAccount(game.publicKey, owner.publicKey, tokenMint);
    console.log(7777, gc)
    const ga = await gb.fetchGameAccount(game.publicKey, owner.publicKey);
    console.log(6666, ga);
    // const district = await gb.initDistrict(game.publicKey, gameManager, '0')
    const district1 = await gb.initDistrict(game.publicKey, gameManager, "0");
    const district2 = await gb.initDistrict(game.publicKey, gameManager, "1");
    const districts = await gb.fetchGameDistrict();
    console.log(8888, districts)
    const house1 = await gb.initHouse(game.publicKey, gameManager, "0", "0", owner.publicKey, tokenMint);
    console.log(house1)
  });

});
