import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Grime } from "../target/types/grime";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";

import * as bs58 from "bs58";
import { SystemProgram, Keypair, PublicKey,SYSVAR_CLOCK_PUBKEY } from "@solana/web3.js";

//to do things
const owner = Keypair.fromSecretKey(
  bs58.decode("2LU9Gir9pDVEsUWrRHLUUdPaVM642EmMGubgyZg2LNYk1uyD4LNRR5HshCENmfTUD3nPMeN7FCJKxEdu48YSEpta")
);

// const staker = Keypair.fromSecretKey(
//   bs58.decode("")
// );

describe("grime", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Grime as Program<Grime>;
  const mint = new PublicKey("GJeK3GxUy13YXEp5LprbraSmXfSzDM7uAnuGMvymvCpU");
  
  it("Is initialized!", async () => {
    // Add your test here.
    const [global, _1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("GLOBAL_STATE_SEED")
      ],
      program.programId
    );

    const [tokenVault, _2] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("TOKEN_VAULT_SEED")
      ],
      program.programId
    );

    const tx = await program.rpc.initialize(
      {
        accounts: {
          owner: owner.publicKey,
          global,
          mint,
          tokenVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID
        },
        signers: [owner]
      }
    )
    console.log("Your transaction signature", tx);
  });

  // it("set pause", async() => {
  //   const [global, _1] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("GLOBAL_STATE_SEED")
  //     ],
  //     program.programId
  //   );
  //   const tx = await program.rpc.setPause(
  //     {
  //       accounts: {
  //         owner: owner.publicKey,
  //         global
  //       },
  //       signers: [owner]
  //     }
  //   );
  //   console.log("tx->", tx);
  // });

  // it("set start", async() => {
  //   const [global, _1] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("GLOBAL_STATE_SEED")
  //     ],
  //     program.programId
  //   );
  //   const tx = await program.rpc.setStart(
  //     {
  //       accounts: {
  //         owner: owner.publicKey,
  //         global
  //       },
  //       signers: [owner]
  //     }
  //   );
  //   console.log("tx->", tx);
  // });
  
  
  // it("deposit token by owner or users", async() => {
  //   const depositAmount = "11999999000000000";

  //   const [global, _1] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("GLOBAL_STATE_SEED")
  //     ],
  //     program.programId
  //   );

  //   const [tokenVault, _2] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("TOKEN_VAULT_SEED")
  //     ],
  //     program.programId
  //   );

  //   const userTokenAccount = await getAssociatedTokenAddress(
  //     mint,
  //     owner.publicKey
  //   );

  //   try {
  //     const tx = await program.rpc.deposit(
  //       new anchor.BN(depositAmount),{
  //         accounts: {
  //           user: owner.publicKey,
  //           global,
  //           mint,
  //           tokenVault,
  //           userTokenAccount,
  //           systemProgram: SystemProgram.programId,
  //           tokenProgram: TOKEN_PROGRAM_ID
  //         },
  //         signers: [owner]
  //       }
  //     );
  //     console.log(tx);
  //   } catch (error) {
  //     console.log(error);
  //   }
  // });
  /*
  it("stake", async() => {
    const option = 1;
    const index = 1;
    const [global, _1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("GLOBAL_STATE_SEED")
      ],
      program.programId
    );

    const [tokenVault, _2] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("TOKEN_VAULT_SEED")
      ],
      program.programId
    );

    const [userData, _3] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("USER_INFI_SEED"),
        staker.publicKey.toBuffer(),
      ],
      program.programId
    );

    const [userInfo, _4] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER_INFO_SEED"),
        staker.publicKey.toBuffer(),
        new anchor.BN(option).toBuffer("le", 1),
        new anchor.BN(index).toBuffer("le", 4),

      ],
      program.programId
    );
    
    const userTokenAccount = await getAssociatedTokenAddress(
      mint,
      staker.publicKey
    );

    const stakeAmount = 1000000;
    const tx = await program.rpc.stake(
      option,
      index,
      new anchor.BN(stakeAmount), {
        accounts: {
          user: staker.publicKey,
          global,
          userData,
          userInfo,
          mint,
          tokenVault,
          userTokenAccount,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          clock: SYSVAR_CLOCK_PUBKEY,
        },
        signers: [staker]
      }
    );
    console.log("tx->", tx);
  });

  it("unstake", async() => {
    const option = 1;
    const index = 1;
    const [global, _1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("GLOBAL_STATE_SEED")
      ],
      program.programId
    );

    const [tokenVault, _2] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("TOKEN_VAULT_SEED")
      ],
      program.programId
    );

    const [userData, _3] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("USER_INFI_SEED"),
        staker.publicKey.toBuffer(),
      ],
      program.programId
    );

    const [userInfo, _4] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER_INFO_SEED"),
        staker.publicKey.toBuffer(),
        new anchor.BN(option).toBuffer("le", 1),
        new anchor.BN(index).toBuffer("le", 4),

      ],
      program.programId
    );
    
    const userTokenAccount = await getAssociatedTokenAddress(
      mint,
      staker.publicKey
    );

    const tx = await program.rpc.unstake(
      option,
      {
        accounts: {
          user: staker.publicKey,
          global,
          userData,
          userInfo,
          mint,
          tokenVault,
          userTokenAccount,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          clock: SYSVAR_CLOCK_PUBKEY,
        },
        signers: [staker]
      }
    );
    console.log("tx->", tx);
  })
  */
});
