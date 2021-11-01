import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Avatars } from "../target/types/avatars";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import {
  getMetadataAddress,
  getAvatarMetadataKey,
} from "./helpers/metadataHelpers";
import * as BufferLayout from "buffer-layout";

describe("avatars", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Avatars as Program<Avatars>;

  let wallet: any = provider.wallet;
  let payer: Keypair = wallet.payer;
  let avatar = null;
  let avatarBump = null;

  it("config", async () => {
    let [_avatar, _avatarBump] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("avatar"), payer.publicKey.toBuffer()],
      program.programId
    );
    avatar = _avatar;
    avatarBump = _avatarBump;
  });

  it("create avatar account", async () => {
    const tx = await program.rpc.initializeAvatarAccount(avatarBump, {
      accounts: {
        owner: payer.publicKey,
        avatar: avatar,
        systemProgram: SystemProgram.programId,
      },
      signers: [payer],
    });
    console.log("init sig ", tx);
  });

  it("set a new avatar", async () => {
    let new_avatar_metadata = new PublicKey(
      "HxoWxe9e35bzAmGoq8rJnbA1NMmRNvQDb8nxHrC21KRr"
    );
    const tx = await program.rpc.setAvatar(new_avatar_metadata, {
      accounts: {
        owner: payer.publicKey,
        avatar: avatar,
      },
      signers: [payer],
    });
    console.log("set sig ", tx);
  });

  it("see the result", async () => {
    // let new_avatar = await program.account.avatar.fetch(avatar);
    // console.log(new_avatar);

    let avatarMetdataKey = await getAvatarMetadataKey(
      payer.publicKey,
      provider.connection
    );
    console.log(avatarMetdataKey.toBase58());
  });
});
