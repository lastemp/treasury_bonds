import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TreasuryBonds } from "../target/types/treasury_bonds";
import {
  Account,
  createAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("treasury_bonds", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.TreasuryBonds as Program<TreasuryBonds>;
  const adminOwner = anchor.web3.Keypair.generate();
  const treasuryBondsOwner = anchor.web3.Keypair.generate();
  const depositAccount = anchor.web3.Keypair.generate();
  /* const usdcMint = new anchor.web3.PublicKey(
    "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
  ); // USDC devnet */

  const payer = wallet.payer;
  const associateTokenProgram = new anchor.web3.PublicKey(
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
  );
  const mintToken = anchor.web3.Keypair.generate(); // dummy usdc token created for test purposes
  const tokenAccount = anchor.utils.token.associatedAddress({
    mint: mintToken.publicKey,
    owner: payer.publicKey,
  });

  let firstInvestorOwner = anchor.web3.Keypair.generate();
  let firstInvestorOwnerATA = anchor.web3.Keypair.generate();

  let secondInvestorOwner = anchor.web3.Keypair.generate();
  let secondInvestorOwnerATA = anchor.web3.Keypair.generate();

  let treasuryVaultATA: Account;

  // pdaAuth
  let [pdaAuth, adminPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("auth"),
      depositAccount.publicKey.toBuffer(),
    ],
    program.programId
  );
  let [treasuryVault, adminTreasuryBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("treasury-vault"), pdaAuth.toBuffer()],
      program.programId
    );

  let [treasuryBondsConfigs] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("treasury-bonds-configs")],
    program.programId
  );

  let [treasuryBonds] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("treasury-bonds"),
      treasuryBondsOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [firstInvestor] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("investor"),
      firstInvestorOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [secondInvestor] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("investor"),
      secondInvestorOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  // admin owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      adminOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // first investor owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      firstInvestorOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // second investor owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      secondInvestorOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // treasury bonds owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      treasuryBondsOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  it("Is initialized!", async () => {
    try {
      const tx = await program.methods
        .init()
        .accounts({
          owner: adminOwner.publicKey,
          treasuryBondsConfigs: treasuryBondsConfigs,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([adminOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.treasuryBondsConfigs.fetch(
        treasuryBondsConfigs
      );
      console.log("treasuryBondsConfigs: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register treasury bonds!", async () => {
    // typeOfBond
    // 1 - Fixed coupon Treasury bonds
    // 2 - Infrastructure bonds

    try {
      let bondIssuer = {
        issuer: "Republic of Kenya",
      };

      let initParams = {
        issuer: bondIssuer,
        country: "KE",
        issueNo: "FXD1/2024/05",
        typeOfBond: 1, // 1 - Fixed coupon Treasury bonds, 2 - Infrastructure bonds
        tenor: 5, // years
        couponRate: 12, // %
        totalAmountsOffered: 100, // USD
        minimumBidAmount: 1, // USD
        unitCostOfTreasuryBonds: 1, // unit cost of treasury bonds
        decimals: 9, // token mint in smallest unit i.e 9 decimals
        valueDate: "15-05-2024",
        redemptionDate: "15-05-2029",
      };

      const tx = await program.methods
        .registerTreasuryBonds(initParams)
        .accounts({
          owner: treasuryBondsOwner.publicKey,
          treasuryBondsConfigs: treasuryBondsConfigs,
          treasuryBonds: treasuryBonds,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([treasuryBondsOwner, depositAccount])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.treasuryBonds.fetch(treasuryBonds);
      let result2 = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      let result3 = await program.account.treasuryBondsConfigs.fetch(
        treasuryBondsConfigs
      );
      console.log("treasury bonds: ", result);
      console.log("deposit account: ", result2);
      console.log("treasury bonds configs: ", result3);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is create token!", async () => {
    console.log("mint token: ", mintToken.publicKey.toBase58());
    console.log("token account: ", tokenAccount.toBase58());

    try {
      let initParams = {
        amount: new anchor.BN(200),
      };

      const tx = await program.methods
        .createToken(initParams)
        .accounts({
          owner: payer.publicKey,
          treasuryBonds: treasuryBonds,
          mintToken: mintToken.publicKey,
          tokenAccount: tokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is token transfer - first investor", async () => {
    console.log(
      "investor owner token account: ",
      firstInvestorOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        firstInvestorOwner,
        mintToken.publicKey,
        firstInvestorOwner.publicKey,
        firstInvestorOwnerATA
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let initParams = {
        amount: new anchor.BN(70),
      };
      const tx = await program.methods
        .transferToken(initParams)
        .accounts({
          owner: payer.publicKey,
          treasuryBonds: treasuryBonds,
          mintToken: mintToken.publicKey,
          fromAccount: tokenAccount,
          toAccount: firstInvestorOwnerATA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is token transfer - second investor", async () => {
    console.log(
      "investor owner token account: ",
      secondInvestorOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        secondInvestorOwner,
        mintToken.publicKey,
        secondInvestorOwner.publicKey,
        secondInvestorOwnerATA
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let initParams = {
        amount: new anchor.BN(100),
      };
      const tx = await program.methods
        .transferToken(initParams)
        .accounts({
          owner: payer.publicKey,
          treasuryBonds: treasuryBonds,
          mintToken: mintToken.publicKey,
          fromAccount: tokenAccount,
          toAccount: secondInvestorOwnerATA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register first investor!", async () => {
    try {
      let initParams = {
        fullNames: "paul john",
        country: "KE",
      };

      const tx = await program.methods
        .registerInvestor(initParams)
        .accounts({
          owner: firstInvestorOwner.publicKey,
          investor: firstInvestor,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstInvestorOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.investor.fetch(firstInvestor);
      console.log("investor: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register second investor!", async () => {
    try {
      let initParams = {
        fullNames: "philip samuel",
        country: "KE",
      };

      const tx = await program.methods
        .registerInvestor(initParams)
        .accounts({
          owner: secondInvestorOwner.publicKey,
          investor: secondInvestor,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondInvestorOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.investor.fetch(secondInvestor);
      console.log("investor: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is buy treasury bonds!", async () => {
    try {
      treasuryVaultATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mintToken.publicKey,
        treasuryVault,
        true
      );
      console.log(
        "treasuryVaultATA address: " + treasuryVaultATA.address.toBase58()
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let initParams = {
        // 10 amount of token to transfer (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(10),
      };

      const tx = await program.methods
        .buyTreasuryBonds(initParams)
        .accounts({
          owner: firstInvestorOwner.publicKey,
          treasuryBonds: treasuryBonds,
          investor: firstInvestor,
          senderTokens: firstInvestorOwnerATA.publicKey,
          recipientTokens: treasuryVaultATA.address,
          mintToken: mintToken.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstInvestorOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.investor.fetch(firstInvestor);
      console.log("investor: ", result);

      let result2 = await program.account.treasuryBonds.fetch(treasuryBonds);
      console.log("treasury bonds: ", result2);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is sell treasury bonds!", async () => {
    try {
      let initParams = {
        // 10 amount of token to transfer (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(10),
      };
      const tx = await program.methods
        .sellTreasuryBonds(initParams)
        .accounts({
          owner: secondInvestorOwner.publicKey,
          treasuryBonds: treasuryBonds,
          sellerInvestor: firstInvestor,
          buyerInvestor: secondInvestor,
          mintToken: mintToken.publicKey,
          fromAccount: secondInvestorOwnerATA.publicKey, // buyer
          toAccount: firstInvestorOwnerATA.publicKey, // seller
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken, secondInvestorOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      console.log("deposit account: ", result);

      let result2 = await program.account.investor.fetch(firstInvestor);
      console.log("first investor: ", result2);

      let result3 = await program.account.investor.fetch(secondInvestor);
      console.log("second investor: ", result3);

      let result4 = await program.account.treasuryBonds.fetch(treasuryBonds);
      console.log("treasury bonds: ", result4);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is redeem treasury bonds!", async () => {
    try {
      let initParams = {
        // 10 amount of token to transfer (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(10),
      };
      const tx = await program.methods
        .redeemTreasuryBonds(initParams)
        .accounts({
          owner: secondInvestorOwner.publicKey,
          treasuryBonds: treasuryBonds,
          investor: secondInvestor,
          senderTokens: treasuryVaultATA.address,
          recipientTokens: secondInvestorOwnerATA.publicKey,
          mintToken: mintToken.publicKey,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondInvestorOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      console.log("deposit account: ", result);

      let result2 = await program.account.investor.fetch(secondInvestor);
      console.log("investor: ", result2);

      let result3 = await program.account.treasuryBonds.fetch(treasuryBonds);
      console.log("treasury bonds: ", result3);
    } catch (error) {
      console.log(error);
    }
  });
});
