import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js"
import { Connection, Signer, Keypair } from '@solana/web3.js'
import { ExchangeBooth } from "../target/types/exchange_booth";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { publicKey, token } from "@project-serum/anchor/dist/cjs/utils";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";

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

  var admin: Keypair;
  var mint0: PublicKey, mint1: PublicKey;

  before(async () => {
    mint0 = await createDefaultMint(connection, walletKeypair);
    mint1 = await createDefaultMint(connection, walletKeypair);
  });

  beforeEach(async () => {
    admin = new Keypair();

    console.log("Requesting airdrop of 2 sol...");
    await connection.requestAirdrop(wallet.publicKey, 2e9);
    await connection.requestAirdrop(admin.publicKey, 2e9);
    console.log("Airdrop received!");

    // Tokens don't go to the account at our public key.
    // Instead we have to create an associated token account,
    // the address of which is computed deterministically based on the mint and public key.
    console.log("Creating admin token accounts...");
    const tokenAccount0 = await getOrCreateAssociatedTokenAccount(connection, walletKeypair, mint0, admin.publicKey);
    const tokenAccount1 = await getOrCreateAssociatedTokenAccount(connection, walletKeypair, mint1, admin.publicKey);
    console.log("Created admin token accounts!")

    // Give the admin some of each token, at the default wallet's expense
    console.log("Minting tokens to admin wallet...")
    const mintTx0 = await mintTo(connection, walletKeypair, mint0, tokenAccount0.address, walletKeypair, 100);
    const mintTx1 = await mintTo(connection, walletKeypair, mint1, tokenAccount1.address, walletKeypair, 100);
    console.log("Minted tokens! txid0: %s, txid1: %s", mintTx0, mintTx1);
  });

  it("Initializes the exchange booth successfully", async () => {
    initializeExchangeBoothHappyPath();

    // TODO: Validate that the expected addresses were written to the chain
  });

  it("Allows the admin to deposit successfully", async () => {
    initializeExchangeBoothHappyPath();
  });

  // Wraps the logic needed to initialize an exchange booth. This is not called in
  // beforeEach because we also want to test cases where this is expected to fail
  async function initializeExchangeBoothHappyPath(): Promise<ExchangeBoothInfo> {
        // Each individual exchange booth and vault is stored at a program-derived address,
        // so we have to compute these PDAs in order to initialize them
        const [exchangeBooth, _bump] = publicKey.findProgramAddressSync(
          [
            anchor.utils.bytes.utf8.encode("exchange_booth"),
            admin.publicKey.toBytes(),
            mint0.toBytes(),
            mint1.toBytes(),
          ], program.programId);
    
        const [vault0, _bump0] = findProgramAddressSync(
          [anchor.utils.bytes.utf8.encode("vault"), admin.publicKey.toBytes(), mint0.toBytes()],
          program.programId
        );
    
        const [vault1, _bump1] = findProgramAddressSync(
          [anchor.utils.bytes.utf8.encode("vault"), admin.publicKey.toBytes(), mint1.toBytes()],
          program.programId
        );
    
        // Now we have all the accounts we need and can initialize the exchange booth on-chain
        const txid = await program.rpc.initializeExchangeBooth({
          accounts: {
            exchangeBooth,
            admin: admin.publicKey,
            mint0,
            mint1,
            vault0,
            vault1,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY
          },
          signers: [admin]
        });
        console.log("Initialized exchange booth! txid: %s", txid);

        return {
          publicKey: exchangeBooth,
          vault0,
          vault1
        };
    }
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

type ExchangeBoothInfo = {
  publicKey: PublicKey,
  vault0: PublicKey,
  vault1: PublicKey
}