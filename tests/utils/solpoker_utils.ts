import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { Solpoker } from "../../target/types/solpoker";
import {
    TOKEN_PROGRAM_ID,
    createMint,
  } from "@solana/spl-token";
import { assert } from "chai";

import * as mlog from "mocha-logger"

// const DataType = {
//     Unknown: {},
//     Context: {},
//     Game: {},
//     User: {},
//   };

type DataType = | {unknown:{}} | {context:{}} | {game:{}} | {user:{}};
type SolPokerInstance = anchor.IdlAccounts<Solpoker>["gameContext"];
type Metadata = anchor.IdlTypes<Solpoker>["MetaData"];
type GameInstance = anchor.IdlAccounts<Solpoker>["game"];

export class SolpokerUtils {

    connection : web3.Connection;
    program : Program<Solpoker>;
    programId : web3.PublicKey;

    constructor(connection : web3.Connection, program : Program<Solpoker>){
        this.connection = connection;
        this.program = program;
        this.programId = program.programId;
    }
    
    async createInstance(manager : web3.Keypair, mint : web3.PublicKey) : Promise<[web3.PublicKey, web3.PublicKey]>
    {  
        const adminPub = manager.publicKey;
        const [gameContext, _gameBump] = await web3.PublicKey.findProgramAddress([Buffer.from("solpoker_instance"), adminPub.toBuffer(), mint.toBuffer()], this.programId);
        const [treasuryAccount, _accBump] = await web3.PublicKey.findProgramAddress([Buffer.from("solpoker_manager_treasury"), adminPub.toBuffer(), mint.toBuffer()], this.programId);
       
        // Add your test here.
        const tx = await this.program.methods
            .initializeInstance(10)
            .accounts({
                manager : manager.publicKey,
                baseMint : mint,
                gameContext : gameContext,
                treasuryAccount : treasuryAccount,
                tokenProgram : TOKEN_PROGRAM_ID,
                systemProgram : web3.SystemProgram.programId,
                rent : web3.SYSVAR_RENT_PUBKEY,
            }).signers([manager]).rpc();
        console.log("Your transaction signature", tx);
        this.connection.confirmTransaction(tx);

        type DT = anchor.IdlTypes<Solpoker>["DataType"] & { ty : DataType };
        
        const instance : SolPokerInstance = await this.program.account.gameContext.fetch(gameContext);

        assert(instance.manager.equals(manager.publicKey));
        assert(instance.baseMint.equals(mint));
        assert(instance.countOfGamesCurrentlyRunning == 0);
        assert(instance.managerFeesInBps == 10);
        assert(instance.treasuryAccount.equals(treasuryAccount));
        assert(instance.treasuryCollected.eq(new anchor.BN(0)));
        
        const metaData : Metadata = instance.metaData;
        assert(metaData.isInitialized == true);
        assert(metaData.version == 1);
        const dataType : DT = metaData.dataType;
        //mlog.log(dataType.ty == DataType.)

        return [gameContext, treasuryAccount]
    }

    async createGame(
        manager: web3.Keypair,
        gameOracle : web3.Keypair,
        gameContext : web3.PublicKey,
        mint : web3.PublicKey,
        gameNumber : number,
    ) : Promise<web3.PublicKey> 
    {
        const [game, _gameBump] = await web3.PublicKey.findProgramAddress( [Buffer.from("solpoker_game"), manager.publicKey.toBuffer(), mint.toBuffer(), new anchor.BN(gameNumber).toBuffer("le")], this.programId);

        let countOfGamesCurrentlyRunning = 0;
        {
            const instance : SolPokerInstance = await this.program.account.gameContext.fetch(gameContext);
            countOfGamesCurrentlyRunning = instance.countOfGamesCurrentlyRunning;
        }
        
        await this.program.methods
            .initializeGame(gameNumber, new anchor.BN(web3.LAMPORTS_PER_SOL),new anchor.BN(20))
            .accounts({
                manager : manager.publicKey,
                gameOracle: gameOracle.publicKey,
                gameContext,
                baseMint : mint,
                game,
                systemProgram : web3.SystemProgram.programId,
            })
            .signers([manager, gameOracle])
            .rpc();

        type DataType = anchor.IdlTypes<Solpoker>["DataType"];
        const gameData : GameInstance = await this.program.account.game.fetch(game);

        {
            const instance : SolPokerInstance = await this.program.account.gameContext.fetch(gameContext);
            assert( instance.countOfGamesCurrentlyRunning - countOfGamesCurrentlyRunning == 1);
        }
        
        const metaData : Metadata = gameData.metaData;
        assert(metaData.isInitialized == true);
        assert(metaData.version == 1);
        const dataType : DataType = metaData.dataType;
        //mlog.log(dataType == {name : "Game"});

        return game;
    }
}