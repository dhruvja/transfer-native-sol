import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TransferSol } from "../target/types/transfer_sol";

describe("transfer_sol", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider)

  const program = anchor.workspace.TransferSol as Program<TransferSol>;

  it("Is initialized!", async () => {
    // Add your test here.

    const from = anchor.web3.Keypair.generate();

    const to = anchor.web3.Keypair.generate();

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(from.publicKey, 10000000000),
      "confirmed"
    );

    let fromBalance = await provider.connection.getBalance(from.publicKey);
    let toBalance = await provider.connection.getBalance(to.publicKey)

    console.log(fromBalance, toBalance)

    const tx = await program.methods.transferNativeSol().accounts({
      from: from.publicKey,
      to: to.publicKey,
      user: from.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([from]).rpc();

    console.log(tx)

    fromBalance = await provider.connection.getBalance(from.publicKey);
    toBalance = await provider.connection.getBalance(to.publicKey)

    console.log(fromBalance, toBalance)


    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
  });
});
