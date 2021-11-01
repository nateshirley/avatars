import { PublicKey, Connection } from "@solana/web3.js";
import * as BufferLayout from "buffer-layout";
import * as anchor from "@project-serum/anchor";

const publicKey = (property) => {
  return BufferLayout.blob(32, property);
};
const AvatarLayout = BufferLayout.struct([
  BufferLayout.seq(BufferLayout.u8(), 8, "discriminator"),
  publicKey("metadata"),
  BufferLayout.u8("bump"),
]);

const AVATAR_PROGRAM_ID = new PublicKey(
  "2K3UxRfLyviFU3oKWbh8VjWfddyMJzKZ3GCDeifntQd1"
);
export const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

export const getMetadataAddress = async (mintKey: PublicKey) => {
  return await PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mintKey.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );
};

export const getAvatarMetadataKey = async (
  owner: PublicKey,
  connection: Connection
) => {
  let [_avatar, _avatarBump] = await PublicKey.findProgramAddress(
    [anchor.utils.bytes.utf8.encode("avatar"), owner.toBuffer()],
    AVATAR_PROGRAM_ID
  );
  let rawAvatar = await connection.getAccountInfo(_avatar);
  let avatarDecoded = AvatarLayout.decode(rawAvatar.data);
  return new PublicKey(avatarDecoded.metadata);
};

// export const getAvatar = async (owner: PublicKey) => {
//   let;
// };
