import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { IfNeededTest } from "../target/types/if_needed_test";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

describe("if-needed-test", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.IfNeededTest as Program<IfNeededTest>;

  it("Is initialized!", async () => {
    // Add your test here.

    let itemMint = Keypair.generate();
    let claimAcctAddress = await findItemClaimAccountAddress(
      itemMint.publicKey
    );

    const first = await program.methods
      .claim()
      .accounts({
        payer: provider.wallet.publicKey,
        claimAccount: claimAcctAddress,
        itemMint: itemMint.publicKey,
        claimProgram: program.programId,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log(first);

    let claimAcct = await program.account.claimAccount.fetch(claimAcctAddress);
    console.log(claimAcct);

    const second = await program.methods
      .claim()
      .accounts({
        payer: provider.wallet.publicKey,
        claimAccount: claimAcctAddress,
        itemMint: itemMint.publicKey,
        claimProgram: program.programId,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log(second);
  });

  const findItemClaimAccountAddress = async (itemMint: PublicKey) => {
    return await PublicKey.findProgramAddress(
      [Buffer.from("claim"), itemMint.toBuffer()],
      program.programId
    ).then(([address, bump]) => {
      return address;
    });
  };
});
