use anchor_lang::{prelude::*, solana_program::program_option};
use anchor_spl::token;

declare_id!("2K3UxRfLyviFU3oKWbh8VjWfddyMJzKZ3GCDeifntQd1");
const AVATAR_SEED: &[u8] = b"avatar";

#[program]
pub mod avatars {
    use super::*;

    pub fn initialize_avatar_account(
        ctx: Context<CreateAvatarAccount>,
        avatar_bump: u8,
    ) -> ProgramResult {
        ctx.accounts.avatar.bump = avatar_bump;
        Ok(())
    }

    pub fn set_avatar(ctx: Context<SetAvatar>, new_avatar_metadata: Pubkey) -> ProgramResult {
        // msg!("the new key {}", new_avatar_metadata);
        // panic!();
        ctx.accounts.avatar.metadata = new_avatar_metadata;
        Ok(())
    }

    pub fn revoke_avatar(ctx: Context<RevokeAvatar>) -> ProgramResult {
        ctx.accounts.plagiarist_avatar.metadata = Pubkey::default();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(avatar_bump: u8)]
pub struct CreateAvatarAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        seeds = [AVATAR_SEED, owner.key().as_ref()],
        bump = avatar_bump,
        payer = owner.to_account_info()
    )]
    pub avatar: Account<'info, Avatar>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetAvatar<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [AVATAR_SEED, owner.key().as_ref()],
        bump = avatar.bump,
    )]
    pub avatar: Account<'info, Avatar>,
}

//['metaplex', metaplex_program_id, mint_id]
/*
make sure it's actually an nft
if someone is using an NFT that you own as their avatar, you can revoke their access
revoker needs to show
- signature
- token account
checks
- revoker owner token account, token account has > 1
- metadata for that token account is the metadata in some other person's account
*/
#[derive(Accounts)]
pub struct RevokeAvatar<'info> {
    pub revoker: Signer<'info>,
    #[account(
        mut,
        constraint = plagiarist_avatar.metadata == avatar_mint.key()
    )]
    pub plagiarist_avatar: Account<'info, Avatar>,
    #[account(
        constraint = avatar_mint.supply == 1,
        constraint = avatar_mint.mint_authority == program_option::COption::None,
    )]
    pub avatar_mint: Account<'info, token::Mint>,
    #[account(
        constraint = revoker_token_account.mint == avatar_mint.key(),
        constraint = revoker_token_account.amount >= 1,
    )]
    pub revoker_token_account: Account<'info, token::TokenAccount>,
}

#[account]
#[derive(Default)]
pub struct Avatar {
    metadata: Pubkey,
    bump: u8,
}

//

/*

avatar spec
- lets you create a pda that contains a token mint address for the nft that you want to use as an avatar
- so, it would just be an address like (“avatar”, signer,) and that address would hold a mint key, which is an nft with metadata
- so anyone can check your wallet and pull an avatar
- i could do this in like a couple hours
- funcs
    - create avatar account
    - change avatar
    - revoke access
- revoke lets you revoke avatar control for any avatar account that is using an nft that you own
    - pass in the token account, and the account that is using it, and you can reset their avatar to the default, if it matches the mint you passed in

*/
