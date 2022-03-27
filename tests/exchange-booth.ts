import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js"
import { Connection, Signer, Keypair } from '@solana/web3.js'
import { ExchangeBooth } from "../target/types/exchange_booth";
import { Account, createMint, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from '@solana/spl-token';
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

  var admin: Keypair, user: Keypair;
  const NUMBER_OF_MINTS = 3;
  const mints = new Array<PublicKey>(NUMBER_OF_MINTS);
  const adminTokenAccounts = new Array<Account>(NUMBER_OF_MINTS);
  const userTokenAccounts = new Array<Account>(NUMBER_OF_MINTS);

  beforeEach(async () => {
    admin = new Keypair();
    user = new Keypair();

    // Top up all parties with SOL
    await connection.requestAirdrop(wallet.publicKey, 2e9);
    await connection.requestAirdrop(admin.publicKey, 2e9);
    await connection.requestAirdrop(user.publicKey, 2e9);

    for(let i = 0; i < NUMBER_OF_MINTS; i++) {
      mints[i] = await createDefaultMint(connection, walletKeypair);

      // Tokens don't go to the account at our public key.
      // Instead we have to create an associated token account,
      // the address of which is computed deterministically based on the mint and public key.
      adminTokenAccounts[i] = await getOrCreateAssociatedTokenAccount(connection, walletKeypair, mints[i], admin.publicKey);
      userTokenAccounts[i] = await getOrCreateAssociatedTokenAccount(connection, walletKeypair, mints[i], user.publicKey);

      // Give the admin and user some of each token, at the default wallet's expense
      mintTo(connection, walletKeypair, mints[i], adminTokenAccounts[i].address, walletKeypair, 100);
      mintTo(connection, walletKeypair, mints[i], userTokenAccounts[i].address, walletKeypair, 100);
    }
  });

  it("Initializes the exchange booth successfully", async () => {
    // When the admin attempts to initialize an exchange booth...
    initializeExchangeBoothHappyPath();

    // Then the exchange booth and vaults exist on-chain
    // TODO: Validate that the expected addresses were written to the chain
  });

  it("Allows the admin to deposit to vault 0 successfully", async () => {
    // Given an existing, empty exchange booth
    const exchangeBoothInfo = await initializeExchangeBoothHappyPath();

    // When the admin attempts to deposit tokens to vault0...
    const txid = await program.rpc.deposit(
      new anchor.BN(10),
      {
        accounts: {
          exchangeBooth: exchangeBoothInfo.publicKey,
          admin: admin.publicKey,
          mint: mints[0],
          from: adminTokenAccounts[0].address,
          vault: exchangeBoothInfo.vault0,
          tokenProgram: TOKEN_PROGRAM_ID
        },
        signers: [admin]
      }
    );

    // ...Then the vault contains the expected number of tokens
    // TODO: validate that the vault now contains the expected amount of tokens
  });

  it("Allows the admin to deposit to vault 1 successfully", async () => {
    // Given an existing, empty exchange booth
    const exchangeBoothInfo = await initializeExchangeBoothHappyPath();

    // When the admin attempts to deposit tokens to vault0...
    const txid = await program.rpc.deposit(
      new anchor.BN(10),
      {
        accounts: {
          exchangeBooth: exchangeBoothInfo.publicKey,
          admin: admin.publicKey,
          mint: mints[1],
          from: adminTokenAccounts[1].address,
          vault: exchangeBoothInfo.vault1,
          tokenProgram: TOKEN_PROGRAM_ID
        },
        signers: [admin]
      }
    );

    // ...Then the vault contains the expected number of tokens
    // TODO: validate that the vault now contains the expected amount of tokens
  });

  it("Allows the admin to withdraw successfully", async () => {
    // Given an existing exchange booth with tokens in vault0
    const exchangeBoothInfo = await initializeExchangeBoothHappyPath();

    await program.rpc.deposit(
      new anchor.BN(10),
      {
        accounts: {
          exchangeBooth: exchangeBoothInfo.publicKey,
          admin: admin.publicKey,
          mint: mints[0],
          from: adminTokenAccounts[0].address,
          vault: exchangeBoothInfo.vault0,
          tokenProgram: TOKEN_PROGRAM_ID
        },
        signers: [admin]
      }
    );

    // When the admin attempts to withdraw tokens from vault0...
    await program.rpc.withdraw(
      new anchor.BN(10),
      {
        accounts: {
          exchangeBooth: exchangeBoothInfo.publicKey,
          admin: admin.publicKey,
          mint: mints[0],
          to: adminTokenAccounts[0].address,
          vault: exchangeBoothInfo.vault0,
          tokenProgram: TOKEN_PROGRAM_ID
        },
        signers: [admin]
      }
    );

    // ...Then the admin has the same number of tokens as they started with
  });

  it ("Allows a third party to exchange tokens", async () => {
    // Given an existing exchange booth with tokens in vault0
    const exchangeBoothInfo = await initializeExchangeBoothHappyPath();

    await program.rpc.deposit(
      new anchor.BN(10),
      {
        accounts: {
          exchangeBooth: exchangeBoothInfo.publicKey,
          admin: admin.publicKey,
          mint: mints[1],
          from: adminTokenAccounts[1].address,
          vault: exchangeBoothInfo.vault1,
          tokenProgram: TOKEN_PROGRAM_ID
        },
        signers: [admin]
      }
    );

    // When a user attempts to exchange token 1 for token 0...
    console.log(await program.rpc.exchange(
      new anchor.BN(10),
      {
        accounts: {
          exchangeBooth: exchangeBoothInfo.publicKey,
          user: user.publicKey,
          admin: admin.publicKey,
          mint0: mints[0],
          mint1: mints[1],
          vault0: exchangeBoothInfo.vault0,
          vault1: exchangeBoothInfo.vault1,
          from: userTokenAccounts[0].address,
          to: userTokenAccounts[1].address,
          tokenProgram: TOKEN_PROGRAM_ID
        },
        signers: [user]
      }
    ));

    // ...Then both the user accounts and vault accounts have the expected balances
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
            mints[0].toBytes(),
            mints[1].toBytes(),
          ], program.programId);
    
        const [vault0, _bump0] = findProgramAddressSync(
          [anchor.utils.bytes.utf8.encode("vault"), admin.publicKey.toBytes(), mints[0].toBytes()],
          program.programId
        );
    
        const [vault1, _bump1] = findProgramAddressSync(
          [anchor.utils.bytes.utf8.encode("vault"), admin.publicKey.toBytes(), mints[1].toBytes()],
          program.programId
        );
    
        // Now we have all the accounts we need and can initialize the exchange booth on-chain
        const txid = await program.rpc.initializeExchangeBooth({
          accounts: {
            exchangeBooth,
            admin: admin.publicKey,
            mint0: mints[0],
            mint1: mints[1],
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
  return await createMint(
    connection,
    authority,
    authority.publicKey,
    authority.publicKey,
    9 // location of the decimal point
  );
}

type ExchangeBoothInfo = {
  publicKey: PublicKey,
  vault0: PublicKey,
  vault1: PublicKey
}