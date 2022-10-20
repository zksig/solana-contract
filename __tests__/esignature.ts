import { describe, expect, it } from "@jest/globals";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { ESignature } from "../target/types/e_signature";

describe("ESignature Contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ESignature as Program<ESignature>;

  it("signs an agreement", async () => {
    const cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi";
    const [agreement] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("a"),
        anchor.utils.bytes.utf8.encode("My Agreementt"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    const [packet] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("p"),
        agreement.toBuffer(),
        anchor.utils.bytes.utf8.encode("employee"),
      ],
      program.programId
    );

    await program.methods
      .createAgreement("My Agreementt", cid, cid, 1)
      .accounts({
        agreement,
        originator: provider.wallet.publicKey,
      })
      .rpc();

    expect((await program.account.agreement.fetch(agreement)).status).toEqual({
      pending: {},
    });

    await program.methods
      .createSignaturePacket("employee", null)
      .accounts({
        agreement,
        packet,
        originator: provider.wallet.publicKey,
      })
      .rpc();

    expect(await program.account.eSignaturePacket.fetch(packet)).toEqual(
      expect.objectContaining({
        identifier: "employee",
        signer: null,
        signed: false,
      })
    );

    await program.methods
      .signSignaturePacket("employee")
      .accounts({
        agreement,
        packet,
        signer: provider.wallet.publicKey,
      })
      .rpc();

    expect(await program.account.eSignaturePacket.fetch(packet)).toEqual(
      expect.objectContaining({
        identifier: "employee",
        signer: provider.wallet.publicKey,
        signed: true,
      })
    );
  });
});
