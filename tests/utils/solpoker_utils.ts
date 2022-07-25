import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { Solpoker } from "../../target/types/solpoker";
import {
    TOKEN_PROGRAM_ID,
    createMint,
  } from "@solana/spl-token";
import { assert } from "chai";

import * as mlog from "mocha-logger"
import _ from "lodash";


const eUnknown = {unknown:{}};
// data types
const eContext = {context:{}};
const eGame = {game:{}};
const eUser = {user:{}};
type DataType = {unknown:{}} | {context:{}} | {game:{}} | {user:{}};
const dataTypes = [eContext, eUnknown, eGame, eUser]

// game states
const eNotStartedYet = {notYetStarted: {}};
const eNoCardsShown = {notYetStarted: {}};
const eThreeCardsShown = {notYetStarted: {}};
const eFourCardsShown = {notYetStarted: {}};
const eAllCardsShown = {notYetStarted: {}};
const eGameEnded = {notYetStarted: {}};
const eCalculatingWinner = {notYetStarted: {}};
type CurrentGameState = {notYetStarted: {}} | {noCardsShown:{}} | {threeCardsShown:{}} | {fourCardsShown:{}} | {allCardsShown:{}} | {gameEnded:{}} | {calculatingWinner:{}}
const gameStates = [eNotStartedYet, eNoCardsShown, eThreeCardsShown, eFourCardsShown, eAllCardsShown, eGameEnded, eCalculatingWinner];

// suits
const eClubs = {clubs : {}}
const eDimonds = {dimonds : {}}
const eHearts = {hearts : {}}
const eSpades = {spades : {}}
const suites = [eClubs, eDimonds, eHearts, eSpades];
export type Suit = {clubs : {}} | {dimonds : {}} | {hearts : {}} | {spades : {}}

// card value


type SolPokerInstance = anchor.IdlAccounts<Solpoker>["gameContext"];
type Metadata = Omit<anchor.IdlTypes<Solpoker>["MetaData"], "dataType"> & { dataType: DataType };
type GameInstance = Omit<anchor.IdlAccounts<Solpoker>["game"], "currentState"> & {currentState: CurrentGameState};

export class SolpokerUtils {

    connection : web3.Connection;
    program : Program<Solpoker>;
    programId : web3.PublicKey;
    zeroBN = this.getBN(0);

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
        notYetStarted: {}
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
        mlog.log(metaData.dataType.toString());
        assert(_.isEqual(metaData.dataType, eContext ))

        return [gameContext, treasuryAccount]
    }

    getBN(number: number):anchor.BN {
        return new anchor.BN(number)
    }

    printGame(gameData : GameInstance)
    {
        mlog.log("\nGAME DATA\n");
        mlog.log("manager :" + gameData.manager.toString() );
        mlog.log("game_context :" + gameData.gameContext.toString() );
        mlog.log("game_oracle :" + gameData.gameOracle.toString() );
        mlog.log("base_mint :" + gameData.baseMint.toString() );
        let currentState : CurrentGameState = gameData.currentState;
        mlog.log("current_game_state : " + gameStates.find(x => _.isEqual(x, currentState)));
        mlog.log("game_number :" + gameData.gameNumber );
        mlog.log("small_blind :" + gameData.smallBlind.toString() );
        mlog.log("max_number_of_players :" + gameData.maxNumberOfPlayers );
        mlog.log("timeout_in_unix_diff :" + gameData.timeoutInUnixDiff.toString() );
        mlog.log("last_update_time :" + gameData.lastUpdateTime.toString());
        mlog.log("big_blind_user_index :" + gameData.bigBlindUserIndex.toString() );
        mlog.log("number_of_users_joined :" + gameData.numberOfUsersJoined.toString() );
        mlog.log("current_pot :" + gameData.currentPot.toString() );
        mlog.log("bid_start_index :" + gameData.bidStartIndex.toString() );
        mlog.log("total_bids_this_round :" + gameData.totalBidsThisRound.toString() );
        mlog.log("max_bid_this_round :" + gameData.maxBidThisRound.toString() );
        mlog.log("current_player :" + gameData.currentPlayer.toString() );
        mlog.log("can_update :" + gameData.canUpdate.toString() );
    }

    async createGame(
        manager: web3.Keypair,
        gameOracle : web3.Keypair,
        gameContext : web3.PublicKey,
        mint : web3.PublicKey,
        smallBlind : anchor.BN = new anchor.BN(web3.LAMPORTS_PER_SOL),
        timeout : anchor.BN = new anchor.BN(20),
    ) : Promise<web3.PublicKey> 
    {
        const [game, _gameBump] = await web3.PublicKey.findProgramAddress( [Buffer.from("solpoker_game"), manager.publicKey.toBuffer(), mint.toBuffer()], this.programId);

        let countOfGamesCurrentlyRunning = 0;
        {
            const instance : SolPokerInstance = await this.program.account.gameContext.fetch(gameContext);
            countOfGamesCurrentlyRunning = instance.countOfGamesCurrentlyRunning;
        }
        mlog.log("Creating game")
        const tx = await this.program.methods
            .initializeGame(smallBlind, timeout)
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
        this.connection.confirmTransaction(tx);
        mlog.log("Game created")
        {
            const instance : SolPokerInstance = await this.program.account.gameContext.fetch(gameContext);
            assert( instance.countOfGamesCurrentlyRunning - countOfGamesCurrentlyRunning == 1);
        }

        const gameData : GameInstance = await this.program.account.game.fetch(game);
        const metaData : Metadata = gameData.metaData;
        assert(metaData.isInitialized == true);
        assert(metaData.version == 1);
        const dataType : DataType = metaData.dataType;
        assert(_.isEqual(dataType, eGame));
        let currentState : CurrentGameState = gameData.currentState;
        assert(_.isEqual(currentState, eNotStartedYet));
        assert(gameData.bidStartIndex == 0);
        assert(gameData.bigBlindUserIndex == 0);
        assert(gameData.canUpdate == false);
        assert(gameData.currentPlayer == 0);
        assert(gameData.currentPot.eq(this.zeroBN));
        assert(gameData.gameOracle.equals(gameOracle.publicKey));
        assert(gameData.gameNumber == 0);
        assert(gameData.maxBidThisRound.eq(this.zeroBN));
        assert(gameData.maxNumberOfPlayers == 10);
        assert(gameData.numberOfUsersJoined == 0);
        assert(gameData.smallBlind.eq(smallBlind));
        assert(gameData.timeoutInUnixDiff.eq(timeout));
        assert(gameData.totalBidsThisRound.eq(this.zeroBN))
        assert(gameData.baseMint.equals(mint));

        return game;
    }
}