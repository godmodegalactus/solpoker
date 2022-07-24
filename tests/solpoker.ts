import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { Solpoker } from "../target/types/solpoker";
import mlog from "mocha-logger"; 
import { assert } from "chai";
import {
  TOKEN_PROGRAM_ID,
  createMint,
} from "@solana/spl-token";

import { SolpokerUtils } from "./utils/solpoker_utils";

describe("solpoker", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const conn = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.Solpoker as Program<Solpoker>;
  const progamId = program.programId;

  const adminKey = web3.Keypair.generate();
  const mintAuthotity = web3.Keypair.generate();
  const adminPub = adminKey.publicKey;
  const utils = new SolpokerUtils(conn, program);

  let mint : web3.PublicKey = null;
  let gameContext : web3.PublicKey = null;
  let treasuryAccount : web3.PublicKey = null;

  // solana logger
  // let logsCallback = (logs: anchor.web3.Logs, context: anchor.web3.Context) => {
  //   mlog.log( logs.logs.join("\n") )
  // };
  // const listner = conn.onLogs(progamId, logsCallback)

  it("Context Initialized", async () => {
    const signature = await conn.requestAirdrop(adminPub, 10 * web3.LAMPORTS_PER_SOL);
    await conn.confirmTransaction(signature);
    
    mint = await createMint(
      conn,
      adminKey,
      mintAuthotity.publicKey,
      null,
      6
    );
    mlog.log("mint created " + mint);
    [gameContext, treasuryAccount] = await utils.createInstance(adminKey, mint);
    mlog.log("game context created " + gameContext)
  });

  const gameOracle = web3.Keypair.generate();
  let game : web3.PublicKey = null;
  it("Game Created", async () => {
    game = await utils.createGame( adminKey, gameOracle, gameContext, mint, 0);
    mlog.log("game initialized", game);
  })

});
