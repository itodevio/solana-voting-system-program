import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Strawsoll } from "../target/types/strawsoll";

const log = (obj: object) => console.log(JSON.stringify(obj, null, 2));

describe("strawsoll", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Strawsoll as Program<Strawsoll>;
  const keypair = anchor.web3.Keypair.generate();

  it("Init poll", async () => {
    await program.methods
      .initialize(['Solana', 'Ethereum', 'Bitcoin'])
      .accounts({
        owner: (program.provider as anchor.AnchorProvider).wallet.publicKey,
        poll: keypair.publicKey
      })
      .signers([keypair])
      .rpc();

    const state = await program.account
      .poll
      .fetch(keypair.publicKey);

    log({ state });
    expect(state.options).to.eql([
      {
        label: 'Solana',
        id: 1,
        votes: 0
      },
      {
        label: 'Ethereum',
        id: 2,
        votes: 0
      },
      {
        label: 'Bitcoin',
        id: 3,
        votes: 0
      }
    ]);
  });

  it("Vote option", async () => {
    const voterKeypair = anchor.web3.Keypair.generate();

    const transaction_signature = await program.methods
      .vote(1)
      .accounts({
        poll: keypair.publicKey,
        voter: voterKeypair.publicKey,
      })
      .signers([voterKeypair])
      .rpc();

    const state = await program.account
      .poll
      .fetch(keypair.publicKey);
    log({ state });
  });

  it("Cannot vote multiple times", async () => {
    const voterKeypair = anchor.web3.Keypair.generate();

    await program.methods
      .vote(1)
      .accounts({
        poll: keypair.publicKey,
        voter: voterKeypair.publicKey
      })
      .signers([voterKeypair])
      .rpc();

    try {
      await program.methods
        .vote(1)
        .accounts({
          poll: keypair.publicKey,
          voter: voterKeypair.publicKey,
        })
        .signers([voterKeypair])
        .rpc();
    } catch (err) {
      log(err.error.errorCode.code);
      expect(err.error.errorCode.code).to.eq('UserAlreadyVoted')
    }

    const state = await program.account
      .poll
      .fetch(keypair.publicKey);
    log({ state });
  });
});
