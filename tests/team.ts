import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { getProgram } from "./utils/program";
import {
  getManagerKeypair,
  getOwnerKeypair,
  getUserKeypair,
} from "./utils/wallets";
import { expect } from "chai";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

describe("Team instructions", () => {
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
  const ownerId = "0";
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
  const userId = "1";
  const [userPDA] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user"),
      anchor.utils.bytes.utf8.encode(userId.toString()),
    ],
    program.programId
  );

  const managerKey = getManagerKeypair();

  const teamId = 1;

  const [teamPDA] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("team"),
      anchor.utils.bytes.utf8.encode(teamId.toString()),
    ],
    program.programId
  );

  const [teamMemberPDA] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("team_member"),
      anchor.utils.bytes.utf8.encode(teamId.toString()),
      anchor.utils.bytes.utf8.encode(userId.toString()),
    ],
    program.programId
  );

  it("init team", async () => {
    // Add your test here.
    const tx = await program.methods
      .initTeam(new anchor.BN(teamId))
      .signers([ownerKeypair])
      .accountsPartial({
        signer: ownerKeypair.publicKey,
        team: teamPDA,
        usdcMint,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const team = await program.account.team.fetch(teamPDA);

    expect(Number(BigInt(team.balance.toNumber()))).to.equal(0);
  });

  it("add member", async () => {
    await program.methods
      .addMember({
        userId: new anchor.BN(userId),
        teamId: new anchor.BN(teamId),
        intialPay: new anchor.BN(0 * Math.pow(10, 6)),
      })
      .signers([ownerKeypair])
      .accountsPartial({
        signer: ownerKeypair.publicKey,
        team: teamPDA,
        teamMember: teamMemberPDA,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const teamMember = await program.account.teamMember.fetch(teamMemberPDA);

    expect(Number(BigInt(teamMember.pay.toNumber()))).to.equal(0);
  });

  it("edits member", async () => {
    await program.methods
      .editMember({
        userId: new anchor.BN(userId),
        teamId: new anchor.BN(teamId),
        newPay: new anchor.BN(1 * Math.pow(10, 6)),
        editId: new anchor.BN(1),
      })
      .signers([ownerKeypair])
      .accountsPartial({
        signer: ownerKeypair.publicKey,
        team: teamPDA,
        teamMember: teamMemberPDA,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const teamMember = await program.account.teamMember.fetch(teamMemberPDA);

    expect(Number(BigInt(teamMember.pay.toNumber()))).to.equal(
      1 * Math.pow(10, 6)
    );
  });

  it("pays team", async () => {
    await program.methods
      .payTeam({
        teamId: new anchor.BN(userId),
        amount: new anchor.BN(1 * Math.pow(10, 6)),
        paymentId: new anchor.BN(1),
      })
      .signers([ownerKeypair])
      .accountsPartial({
        signer: ownerKeypair.publicKey,
        team: teamPDA,
        usdcMint,
        usdcPayerAccount: ownerUsdcAccount,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const team = await program.account.team.fetch(teamPDA);
    expect(Number(BigInt(team.balance.toNumber()))).to.greaterThan(0);
  });

  it("pays member", async () => {
    await program.methods
      .payMember({
        teamId: new anchor.BN(userId),
        userId: new anchor.BN(userId),
        amount: null,
        paymentId: new anchor.BN(1),
      })
      .signers([managerKey])
      .accountsPartial({
        signer: managerKey.publicKey,
        user: userPDA,
        team: teamPDA,
        teamMember: teamMemberPDA,
        usdcMint,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const user = await program.account.user.fetch(userPDA);
    expect(Number(BigInt(user.balance.toNumber()))).to.greaterThan(
      0 * Math.pow(10, 6)
    );
  });

  it("claims", async () => {
    await program.methods
      .claim({ userId: new anchor.BN(userId), claimId: new anchor.BN(2) })
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

  it("remove member", async () => {
    await program.methods
      .removeMember({
        teamId: new anchor.BN(teamId),
        userId: new anchor.BN(userId),
        removeId: new anchor.BN(1),
      })
      .signers([ownerKeypair])
      .accountsPartial({
        signer: ownerKeypair.publicKey,
        team: teamPDA,
        teamMember: teamMemberPDA,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const teamMember = await program.account.teamMember.fetchNullable(
      teamMemberPDA
    );
    expect(teamMember).to.be.null;
  });

  it("leaves team", async () => {
    await program.methods
      .addMember({
        userId: new anchor.BN(userId),
        teamId: new anchor.BN(teamId),
        intialPay: new anchor.BN(0 * Math.pow(10, 6)),
      })
      .signers([ownerKeypair])
      .accountsPartial({
        signer: ownerKeypair.publicKey,
        team: teamPDA,
        teamMember: teamMemberPDA,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    await program.methods
      .leaveTeam({
        teamId: new anchor.BN(teamId),
        userId: new anchor.BN(userId),
        leaveId: new anchor.BN(1),
      })
      .signers([userKeypair])
      .accountsPartial({
        signer: userKeypair.publicKey,
        user: userPDA,
        teamMember: teamMemberPDA,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const teamMember = await program.account.teamMember.fetchNullable(
      teamMemberPDA
    );
    expect(teamMember).to.be.null;
  });
});
