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
import { encodeUUID } from "./utils/encode";

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
  const ownerId = encodeUUID("a1d12868-688d-40fb-85a0-72b21fd377e2");
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
  const userId = encodeUUID("bcfe3881-f13a-4af0-ba83-611046788ff6");
  const [userPDA] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user"),
      anchor.utils.bytes.utf8.encode(userId.toString()),
    ],
    program.programId
  );

  const managerKey = getManagerKeypair();

  const teamId = 2;

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

  const invoiceId = 7;
  const [invoicePDA] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("team_invoice"),
      anchor.utils.bytes.utf8.encode(teamId.toString()),
      anchor.utils.bytes.utf8.encode(invoiceId.toString()),
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
        userId,
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
        userId,
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
        teamId: new anchor.BN(teamId),
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
        teamId: new anchor.BN(teamId),
        userId,
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
      .claim({ userId, claimId: new anchor.BN(2) })
      .signers([userKeypair])
      .accountsPartial({
        signer: userKeypair.publicKey,
        user: userPDA,
        usdcMint,
        recipientUsdcAccount: userUsdcAccount,
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
        userId,
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
        userId,
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
        userId,
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

  it("creates team invoice", async () => {
    await program.methods
      .createTeamInvoice({
        teamId: new anchor.BN(teamId),
        invoiceId: new anchor.BN(invoiceId),
        requestedAmount: new anchor.BN(1 * Math.pow(10, 6)),
      })
      .signers([ownerKeypair])
      .accountsPartial({
        signer: ownerKeypair.publicKey,
        team: teamPDA,
        invoice: invoicePDA,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const invoice = await program.account.teamInvoice.fetch(invoicePDA);
    expect(
      invoice.requestedAmount.div(new anchor.BN(10 ** 6)).toNumber()
    ).to.equal(1);
  });

  it("pays invoice", async () => {
    await program.methods
      .payTeamInvoice({
        teamId: new anchor.BN(teamId),
        invoiceId: new anchor.BN(invoiceId),
      })
      .signers([userKeypair])
      .accountsPartial({
        signer: userKeypair.publicKey,
        team: teamPDA,
        invoice: invoicePDA,
        usdcMint,
        usdcPayerAccount: userUsdcAccount,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const invoice = await program.account.teamInvoice.fetch(invoicePDA);
    expect(invoice.isPaid).to.equal(true);
    const team = await program.account.team.fetch(teamPDA);
    expect(
      Number(BigInt(team.balance.div(new anchor.BN(10 ** 6)).toNumber()))
    ).to.greaterThan(0);
  });
});
