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

    const company = anchor.web3.Keypair.generate();

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(from.publicKey, 10000000000),
      "confirmed"
    );

    let fromBalance = await provider.connection.getBalance(from.publicKey);
    let creatorBalance = await provider.connection.getBalance(to.publicKey)
    let companyBalance = await provider.connection.getBalance(company.publicKey)

    console.log(fromBalance, creatorBalance, companyBalance)

    let amountToSend = new anchor.BN(100000000);

    const tx = await program.methods.transferNativeSol(amountToSend).accounts({
      from: from.publicKey,
      to: to.publicKey,
      company: company.publicKey,
      user: from.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([from]).rpc();

    console.log(tx)

    fromBalance = await provider.connection.getBalance(from.publicKey);
    creatorBalance = await provider.connection.getBalance(to.publicKey)
    companyBalance = await provider.connection.getBalance(company.publicKey)


    console.log(fromBalance, creatorBalance, companyBalance)


    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
  });
});
