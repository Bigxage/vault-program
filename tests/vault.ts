import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { assert } from "chai";

describe("vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Vault as Program<Vault>;
  const maker = provider.wallet;

  const [statePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), maker.publicKey.toBuffer()],
    program.programId
  );

  const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), statePda.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          maker: maker.publicKey,
          state: statePda,
          vault: vaultPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      console.log("Init signature", tx);
    } catch (e) {
      console.error(e);
    }
  });

  it("Deposits 1 SOL", async () => {
    const amount = new anchor.BN(1000000000); 
    const tx = await program.methods
      .deposit(amount)
      .accounts({
        maker: maker.publicKey,
        state: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Deposit signature", tx);
  });

  it("Withdraws 0.5 SOL", async () => {
    const amount = new anchor.BN(500000000); 
    const tx = await program.methods
      .withdraw(amount)
      .accounts({
        maker: maker.publicKey,
        state: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Withdraw signature", tx);
  });

  it("Closes the Vault", async () => {
    const tx = await program.methods
      .close()
      .accounts({
        maker: maker.publicKey,
        state: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Close signature", tx);
  });
});