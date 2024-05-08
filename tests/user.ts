import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { getProgram } from "./utils/program";
import { getOwnerKeypair, getUserKeypair } from "./utils/wallets";
import { expect } from "chai";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

describe("User instructions", () => {
  const usdcMint = new PublicKey(
    "BWikzMjEiqPASTPQ9Lz8aEtmsUnyR7bjD3WrmnZJgUYc"
  );
  // Configure the client to use the local cluster.
  const program = getProgram();

  const ownerKeypair = getOwnerKeypair();
  const ownerUsdcAccount = getAssociatedTokenAddressSync(
    usdcMint,
    ownerKeypair.publicKey
  );
  const ownerId = 0;
  const [ownerPDA] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user"),
      anchor.utils.bytes.utf8.encode(ownerId.toString()),
    ],
    program.programId
  );

  const userKeypair = getUserKeypair();
  const userUsdcAccount = getAssociatedTokenAddressSync(
    usdcMint,
    userKeypair.publicKey
  );
  const userId = 1;
  const [userPDA] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user"),
      anchor.utils.bytes.utf8.encode(userId.toString()),
    ],
    program.programId
  );

  it("init owner", async () => {
    // Add your test here.
    const tx = await program.methods
      .initUser(new anchor.BN(ownerId))
      .signers([ownerKeypair])
      .accountsPartial({
        signer: ownerKeypair.publicKey,
        user: ownerPDA,
        usdcMint,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const owner = await program.account.user.fetch(ownerPDA);

    expect(Number(BigInt(owner.balance.toNumber()))).to.equal(0);
  });

  it("init user", async () => {
    await program.methods
      .initUser(new anchor.BN(userId))
      .signers([userKeypair])
      .accountsPartial({
        signer: userKeypair.publicKey,
        user: userPDA,
        usdcMint,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const user = await program.account.user.fetch(userPDA);

    expect(Number(BigInt(user.balance.toNumber()))).to.equal(0);
  });

  it("edits user", async () => {
    await program.methods
      .editUser({
        userId: new anchor.BN(userId),
        newAuthority: userKeypair.publicKey,
      })
      .signers([ownerKeypair])
      .accountsPartial({
        signer: ownerKeypair.publicKey,
        user: userPDA,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const user = await program.account.user.fetch(userPDA);

    expect(user.authority.toBase58()).to.equal(
      userKeypair.publicKey.toBase58()
    );
  });

  it("pays user", async () => {
    await program.methods
      .payUser({
        userId: new anchor.BN(userId),
        amount: new anchor.BN(1 * Math.pow(10, 6)),
      })
      .signers([ownerKeypair])
      .accountsPartial({
        signer: ownerKeypair.publicKey,
        user: userPDA,
        usdcMint,
        usdcPayerAccount: ownerUsdcAccount,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const user = await program.account.user.fetch(userPDA);
    expect(Number(BigInt(user.balance.toNumber()))).to.greaterThan(0);
  });

  it("claims", async () => {
    await program.methods
      .claim(new anchor.BN(userId))
      .signers([userKeypair])
      .accountsPartial({
        signer: userKeypair.publicKey,
        user: userPDA,
        usdcMint,
        recipientPayerAccount: userUsdcAccount,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const user = await program.account.user.fetch(userPDA);
    expect(Number(BigInt(user.balance.toNumber()))).to.equal(0);
  });
});
