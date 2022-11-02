import fs from "fs/promises";
import { describe, expect, it, beforeAll } from "@jest/globals";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import * as ed from "@noble/ed25519";
import { ESignature } from "../target/types/e_signature";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.ESignature as Program<ESignature>;
const cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi";

describe("ESignature Contract", () => {
  let key: Keypair;
  let profile: PublicKey;
  let agreement: PublicKey;

  let managerSigner: Keypair;
  let managerProgram: Program<ESignature>;
  let managerProfile: PublicKey;
  let managerPacket: PublicKey;

  let employeeSigner: Keypair;
  let employeeProgram: Program<ESignature>;
  let employeeProfile: PublicKey;
  let employeePacket: PublicKey;

  beforeAll(async () => {
    key = await Keypair.fromSecretKey(
      Buffer.from(
        JSON.parse(
          await fs.readFile(process.env.ANCHOR_WALLET, { encoding: "utf-8" })
        )
      )
    );
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
    [managerPacket] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("packet"),
        anchor.utils.bytes.utf8.encode("manager"),
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
    [employeePacket] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("packet"),
        anchor.utils.bytes.utf8.encode("employee"),
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
    await program.methods
      .initializeAgreement("My Agreementt", cid, cid, cid, 2)
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

  it("creates signature packet", async () => {
    await program.methods
      .initializeSignaturePacket("manager", managerSigner.publicKey)
      .accounts({
        packet: managerPacket,
        agreement,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    await program.methods
      .initializeSignaturePacket("employee", null)
      .accounts({
        packet: employeePacket,
        agreement,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    expect(await program.account.eSignaturePacket.fetch(managerPacket)).toEqual(
      expect.objectContaining({
        agreement,
        signer: managerSigner.publicKey,
        identifier: "manager",
        encryptedCid: null,
        signed: false,
      })
    );

    expect(
      await program.account.eSignaturePacket.fetch(employeePacket)
    ).toEqual(
      expect.objectContaining({
        agreement,
        signer: null,
        identifier: "employee",
        encryptedCid: null,
        signed: false,
      })
    );
  });

  it("fails to sign a signature packet when there is a bad signer", async () => {
    const signature = await ed.sign(
      Buffer.from(`manager ${agreement.toString()}`),
      key.secretKey.slice(0, 32)
    );

    return expect(
      async () =>
        await employeeProgram.methods
          .signSignaturePacket(
            // @ts-ignore
            "manager",
            signature,
            cid,
            provider.publicKey
          )
          .accounts({
            packet: managerPacket,
            agreement,
            profile: employeeProfile,
            ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
            signer: employeeSigner.publicKey,
          })
          .preInstructions([
            anchor.web3.Ed25519Program.createInstructionWithPublicKey({
              signature,
              publicKey: key.publicKey.toBuffer(),
              message: Buffer.from(`manager ${agreement.toString()}`),
            }),
          ])
          .rpc()
    ).rejects.toThrow("MismatchedSigner");
  });

  it("fails to sign a signature packet when there is a bad signature", async () => {
    const signature = await ed.sign(
      Buffer.from(`manager ${agreement.toString()}`),
      managerSigner.secretKey.slice(0, 32)
    );

    return expect(
      async () =>
        await employeeProgram.methods
          .signSignaturePacket(
            // @ts-ignore
            "manager",
            signature,
            cid,
            provider.publicKey
          )
          .accounts({
            packet: managerPacket,
            agreement,
            profile: employeeProfile,
            ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
            signer: employeeSigner.publicKey,
          })
          .preInstructions([
            anchor.web3.Ed25519Program.createInstructionWithPublicKey({
              signature,
              publicKey: managerSigner.publicKey.toBuffer(),
              message: Buffer.from(`manager ${agreement.toString()}`),
            }),
          ])
          .rpc()
    ).rejects.toThrow("SignatureVerificationError");
  });

  it("can sign a signature packet", async () => {
    const signature = await ed.sign(
      Buffer.from(`manager ${agreement.toString()}`),
      key.secretKey.slice(0, 32)
    );

    await managerProgram.methods
      .signSignaturePacket(
        // @ts-ignore
        "manager",
        signature,
        cid,
        provider.publicKey
      )
      .accounts({
        packet: managerPacket,
        agreement,
        profile: managerProfile,
        ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
        signer: managerSigner.publicKey,
      })
      .preInstructions([
        anchor.web3.Ed25519Program.createInstructionWithPublicKey({
          signature,
          publicKey: key.publicKey.toBuffer(),
          message: Buffer.from(`manager ${agreement.toString()}`),
        }),
      ])
      .rpc();

    expect(await program.account.eSignaturePacket.fetch(managerPacket)).toEqual(
      expect.objectContaining({
        agreement,
        signer: managerSigner.publicKey,
        identifier: "manager",
        encryptedCid: cid,
        signed: true,
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

  it("can sign a signature packet", async () => {
    const signature = await ed.sign(
      Buffer.from(`employee ${agreement.toString()}`),
      key.secretKey.slice(0, 32)
    );

    await employeeProgram.methods
      .signSignaturePacket(
        // @ts-ignore
        "employee",
        signature,
        cid,
        provider.publicKey
      )
      .accounts({
        packet: employeePacket,
        agreement,
        profile: employeeProfile,
        ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
        signer: employeeSigner.publicKey,
      })
      .preInstructions([
        anchor.web3.Ed25519Program.createInstructionWithPublicKey({
          signature,
          publicKey: key.publicKey.toBuffer(),
          message: Buffer.from(`employee ${agreement.toString()}`),
        }),
      ])
      .rpc();

    expect(
      await program.account.eSignaturePacket.fetch(employeePacket)
    ).toEqual(
      expect.objectContaining({
        agreement,
        signer: employeeSigner.publicKey,
        identifier: "employee",
        encryptedCid: cid,
        signed: true,
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

  // it("fails to sign a signature packet when its already been signed", async () => {
  //   const [packet] = await PublicKey.findProgramAddress(
  //     [
  //       anchor.utils.bytes.utf8.encode("packet"),
  //       anchor.utils.bytes.utf8.encode("1"),
  //       managerSigner.publicKey.toBuffer(),
  //     ],
  //     program.programId
  //   );

  //   expect(
  //     async () =>
  //       await managerProgram.methods
  //         .signSignaturePacket(0, cid)
  //         .accounts({
  //           packet,
  //           agreement,
  //           constraint: managerConstraint,
  //           profile: managerProfile,
  //           signer: managerSigner.publicKey,
  //         })
  //         .rpc()
  //   ).rejects.toThrow("UsedConstraint");
  // });

  // it("can sign a signature packet when there is no signer constraint", async () => {
  //   const [packet] = await PublicKey.findProgramAddress(
  //     [
  //       anchor.utils.bytes.utf8.encode("packet"),
  //       anchor.utils.bytes.utf8.encode("0"),
  //       employeeSigner.publicKey.toBuffer(),
  //     ],
  //     program.programId
  //   );

  //   await employeeProgram.methods
  //     .signSignaturePacket(1, cid)
  //     .accounts({
  //       packet,
  //       agreement,
  //       constraint: employeeConstraint,
  //       profile: employeeProfile,
  //       signer: employeeSigner.publicKey,
  //     })
  //     .rpc();

  //   expect(await program.account.eSignaturePacket.fetch(packet)).toEqual(
  //     expect.objectContaining({
  //       agreement,
  //       signer: employeeSigner.publicKey,
  //       index: 1,
  //       signed: true,
  //     })
  //   );

  //   expect(
  //     await program.account.signatureConstraint.fetch(employeeConstraint)
  //   ).toEqual(
  //     expect.objectContaining({
  //       agreement,
  //       index: 1,
  //       signer: employeeSigner.publicKey,
  //       used: true,
  //     })
  //   );

  //   expect(await program.account.profile.fetch(employeeProfile)).toEqual(
  //     expect.objectContaining({
  //       owner: employeeSigner.publicKey,
  //       agreementsCount: 0,
  //       signaturesCount: 1,
  //     })
  //   );
  // });

  // it("fails to sign a signature packet when the agreement is no longer pending", async () => {
  //   const [packet] = await PublicKey.findProgramAddress(
  //     [
  //       anchor.utils.bytes.utf8.encode("packet"),
  //       anchor.utils.bytes.utf8.encode("1"),
  //       employeeSigner.publicKey.toBuffer(),
  //     ],
  //     program.programId
  //   );

  //   expect(
  //     async () =>
  //       await employeeProgram.methods
  //         .signSignaturePacket(1, cid)
  //         .accounts({
  //           packet,
  //           agreement,
  //           constraint: employeeConstraint,
  //           profile: employeeProfile,
  //           signer: employeeSigner.publicKey,
  //         })
  //         .rpc()
  //   ).rejects.toThrow("NonPendingAgreement");
  // });
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
