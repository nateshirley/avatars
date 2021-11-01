use anchor_lang::{prelude::*, solana_program::program_option};
use anchor_spl::token;
use spl_token_metadata;

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
        ctx.accounts.avatar.metadata = new_avatar_metadata;
        Ok(())
    }

    pub fn revoke_avatar(ctx: Context<RevokePoser>) -> ProgramResult {
        ctx.accounts.posing_avatar.metadata = Pubkey::default();
        Ok(())
    }
}

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

#[derive(Accounts)]
pub struct RevokePoser<'info> {
    pub revoker: Signer<'info>,
    #[account(
        mut,
        constraint = posing_avatar.metadata == metadata_address(avatar_mint.key())
    )]
    pub posing_avatar: Account<'info, Avatar>,
    #[account(
        constraint = avatar_mint.supply == 1,
        constraint = avatar_mint.mint_authority == program_option::COption::None, //make sure it's actually an NFT
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

pub fn metadata_address(mint: Pubkey) -> Pubkey {
    const METADATA_SEED: &[u8] = b"metadata";
    let id = spl_token_metadata::id();
    let (metadata, _bump) = Pubkey::find_program_address(
        &[
            METADATA_SEED,
            spl_token_metadata::id().as_ref(),
            mint.as_ref(),
        ],
        &id,
    );
    metadata
}
