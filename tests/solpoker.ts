import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { Solpoker } from "../target/types/solpoker";
import mlog from "mocha-logger"; 
import { assert } from "chai";
import {
  Token,
  TOKEN_PROGRAM_ID,
  u64,
  createMint,
} from "@solana/spl-token";

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
    mlog.log("mint created " + mint)

    let _gameBump = 0;
    let _accBump = 0;  
    [gameContext, _gameBump] = await web3.PublicKey.findProgramAddress([Buffer.from("solpoker_instance"), adminPub.toBuffer(), mint.toBuffer()], progamId);
    [treasuryAccount, _accBump] = await web3.PublicKey.findProgramAddress([Buffer.from("solpoker_manager_treasury"), adminPub.toBuffer(), mint.toBuffer()], progamId);
    mlog.log("programming address created with programId " + progamId)

    // Add your test here.
    const tx = await program.methods
    .initializeInstance(10)
      .accounts({
      manager : adminPub,
      baseMint : mint,
      gameContext : gameContext,
      treasuryAccount : treasuryAccount,
      tokenProgram : TOKEN_PROGRAM_ID,
      systemProgram : web3.SystemProgram.programId,
      rent : web3.SYSVAR_RENT_PUBKEY,
    }).signers([adminKey]).rpc();
    console.log("Your transaction signature", tx);
  });
});
