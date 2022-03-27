import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Connection, Signer} from '@solana/web3.js'
import { ExchangeBooth } from "../target/types/exchange_booth";
import { createMint } from '@solana/spl-token';
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";

describe("exchange-booth", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.ExchangeBooth as Program<ExchangeBooth>;
  const connection = program.provider.connection;
  const wallet = program.provider.wallet;
  // According to one of the anchor devs on discord, there's no clean way to
  // create mints using just a Wallet object. You have to either abuse the
  // type system like the line below, or create a custom transaction object
  // which the Wallet can sign.
  const walletKeypair = (wallet as NodeWallet).payer;

  const mint0 = await createDefaultMint(connection, walletKeypair);
  const mint1 = await createDefaultMint(connection, walletKeypair);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});

// Create a mint where the fee-payer, mint authority, and freeze authority are all the same
async function createDefaultMint(connection: Connection, authority: Signer) {
  console.log("Creating mint...");

  const mint = await createMint(
    connection,
    authority,
    authority.publicKey,
    authority.publicKey,
    9 // location of the decimal point
  );

  console.log("Mint %s created successfully!", mint.toBase58());

  return mint
}