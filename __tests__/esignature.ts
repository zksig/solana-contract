import { describe, expect, it, beforeAll } from "@jest/globals";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { ESignature } from "../target/types/e_signature";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.ESignature as Program<ESignature>;

describe("ESignature Contract", () => {
  let profile: PublicKey;
  let agreement: PublicKey;

  let managerSigner: Keypair;
  let managerProgram: Program<ESignature>;
  let managerProfile: PublicKey;
  let managerConstraint: PublicKey;

  let employeeSigner: Keypair;
  let employeeProgram: Program<ESignature>;
  let employeeProfile: PublicKey;
  let employeeConstraint: PublicKey;

  beforeAll(async () => {
    [profile] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("profile"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    [agreement] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("agreement"),
        anchor.utils.bytes.utf8.encode("0"),
        profile.toBuffer(),
      ],
      program.programId
    );

    const manager = await createAccount();
    managerSigner = manager.signer;
    managerProgram = manager.signerProgram;
    [managerProfile] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("profile"),
        managerSigner.publicKey.toBuffer(),
      ],
      program.programId
    );
    [managerConstraint] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("constraint"),
        anchor.utils.bytes.utf8.encode("0"),
        agreement.toBuffer(),
      ],
      program.programId
    );

    const employee = await createAccount();
    employeeSigner = employee.signer;
    employeeProgram = employee.signerProgram;
    [employeeProfile] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("profile"),
        employeeSigner.publicKey.toBuffer(),
      ],
      program.programId
    );
    [employeeConstraint] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("constraint"),
        anchor.utils.bytes.utf8.encode("1"),
        agreement.toBuffer(),
      ],
      program.programId
    );

    await managerProgram.methods
      .createProfile()
      .accounts({
        profile: managerProfile,
        owner: managerSigner.publicKey,
      })
      .rpc();

    await employeeProgram.methods
      .createProfile()
      .accounts({
        profile: employeeProfile,
        owner: employeeSigner.publicKey,
      })
      .rpc();
  });

  it("creates a profile", async () => {
    await program.methods
      .createProfile()
      .accounts({
        profile,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    expect(await program.account.profile.fetch(profile)).toEqual(
      expect.objectContaining({
        owner: provider.wallet.publicKey,
        agreementsCount: 0,
        signaturesCount: 0,
      })
    );
  });

  it("creates an agreements", async () => {
    const cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi";

    await program.methods
      .createAgreement("My Agreementt", cid, cid, 2)
      .accounts({
        agreement,
        profile,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    expect(await program.account.agreement.fetch(agreement)).toEqual(
      expect.objectContaining({
        profile,
        identifier: "My Agreementt",
        cid,
        descriptionCid: cid,
        status: { pending: {} },
        signedPackets: 0,
        totalPackets: 2,
      })
    );

    expect(await program.account.profile.fetch(profile)).toEqual(
      expect.objectContaining({
        owner: provider.wallet.publicKey,
        agreementsCount: 1,
        signaturesCount: 0,
      })
    );
  });

  it("creates signature constraints", async () => {
    await program.methods
      .createSignatureConstraint(0, "manager", managerSigner.publicKey)
      .accounts({
        constraint: managerConstraint,
        agreement,
        profile,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    await program.methods
      .createSignatureConstraint(1, "employee", null)
      .accounts({
        constraint: employeeConstraint,
        agreement,
        profile,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    expect(
      await program.account.signatureConstraint.fetch(managerConstraint)
    ).toEqual(
      expect.objectContaining({
        agreement,
        index: 0,
        signer: managerSigner.publicKey,
        used: false,
      })
    );

    expect(
      await program.account.signatureConstraint.fetch(employeeConstraint)
    ).toEqual(
      expect.objectContaining({
        agreement,
        index: 1,
        signer: null,
        used: false,
      })
    );
  });

  it("fails to sign a signature packet when there is a bad signer", async () => {
    const [packet] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("packet"),
        anchor.utils.bytes.utf8.encode("0"),
        employeeSigner.publicKey.toBuffer(),
      ],
      program.programId
    );

    expect(
      async () =>
        await employeeProgram.methods
          .signSignaturePacket(0)
          .accounts({
            packet,
            agreement,
            constraint: managerConstraint,
            profile: employeeProfile,
            signer: employeeSigner.publicKey,
          })
          .rpc()
    ).rejects.toThrow("MismatchedSigner");
  });

  it("can sign a signature packet when there is a signer constraint", async () => {
    const [packet] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("packet"),
        anchor.utils.bytes.utf8.encode("0"),
        managerSigner.publicKey.toBuffer(),
      ],
      program.programId
    );

    await managerProgram.methods
      .signSignaturePacket(0)
      .accounts({
        packet,
        agreement,
        constraint: managerConstraint,
        profile: managerProfile,
        signer: managerSigner.publicKey,
      })
      .rpc();

    expect(await program.account.eSignaturePacket.fetch(packet)).toEqual(
      expect.objectContaining({
        agreement,
        signer: managerSigner.publicKey,
        index: 0,
        signed: true,
      })
    );

    expect(
      await program.account.signatureConstraint.fetch(managerConstraint)
    ).toEqual(
      expect.objectContaining({
        agreement,
        index: 0,
        signer: managerSigner.publicKey,
        used: true,
      })
    );

    expect(await program.account.profile.fetch(managerProfile)).toEqual(
      expect.objectContaining({
        owner: managerSigner.publicKey,
        agreementsCount: 0,
        signaturesCount: 1,
      })
    );
  });

  it("fails to sign a signature packet when its already been signed", async () => {
    const [packet] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("packet"),
        anchor.utils.bytes.utf8.encode("1"),
        managerSigner.publicKey.toBuffer(),
      ],
      program.programId
    );

    expect(
      async () =>
        await managerProgram.methods
          .signSignaturePacket(0)
          .accounts({
            packet,
            agreement,
            constraint: managerConstraint,
            profile: managerProfile,
            signer: managerSigner.publicKey,
          })
          .rpc()
    ).rejects.toThrow("UsedConstraint");
  });

  it("can sign a signature packet when there is no signer constraint", async () => {
    const [packet] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("packet"),
        anchor.utils.bytes.utf8.encode("0"),
        employeeSigner.publicKey.toBuffer(),
      ],
      program.programId
    );

    await employeeProgram.methods
      .signSignaturePacket(1)
      .accounts({
        packet,
        agreement,
        constraint: employeeConstraint,
        profile: employeeProfile,
        signer: employeeSigner.publicKey,
      })
      .rpc();

    expect(await program.account.eSignaturePacket.fetch(packet)).toEqual(
      expect.objectContaining({
        agreement,
        signer: employeeSigner.publicKey,
        index: 1,
        signed: true,
      })
    );

    expect(
      await program.account.signatureConstraint.fetch(employeeConstraint)
    ).toEqual(
      expect.objectContaining({
        agreement,
        index: 1,
        signer: employeeSigner.publicKey,
        used: true,
      })
    );

    expect(await program.account.profile.fetch(employeeProfile)).toEqual(
      expect.objectContaining({
        owner: employeeSigner.publicKey,
        agreementsCount: 0,
        signaturesCount: 1,
      })
    );
  });

  it("fails to sign a signature packet when the agreement is no longer pending", async () => {
    const [packet] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("packet"),
        anchor.utils.bytes.utf8.encode("1"),
        employeeSigner.publicKey.toBuffer(),
      ],
      program.programId
    );

    expect(
      async () =>
        await employeeProgram.methods
          .signSignaturePacket(1)
          .accounts({
            packet,
            agreement,
            constraint: employeeConstraint,
            profile: employeeProfile,
            signer: employeeSigner.publicKey,
          })
          .rpc()
    ).rejects.toThrow("NonPendingAgreement");
  });
});

async function createAccount() {
  const signer = Keypair.generate();
  const signerProgram = new Program(
    program.idl,
    program.programId,
    new anchor.AnchorProvider(
      provider.connection,
      new anchor.Wallet(signer),
      {}
    )
  );
  const tx = new anchor.web3.Transaction();
  tx.add(
    anchor.web3.SystemProgram.transfer({
      fromPubkey: provider.publicKey,
      toPubkey: signer.publicKey,
      lamports: 100000000,
    })
  );
  await provider.sendAndConfirm(tx);

  return { signer, signerProgram };
}
