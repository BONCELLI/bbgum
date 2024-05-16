use std::mem::size_of;

use anchor_lang::{prelude::*, system_program::System};
use spl_account_compression::{
    program::SplAccountCompression,
    state::{
        merkle_tree_get_size, ConcurrentMerkleTreeHeader, CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1,
    },
    Node, Noop,
};

use crate::{
    error::BubblegumError,
    state::{DecompressibleState, TreeConfig, TREE_AUTHORITY_SIZE},
};

pub const MAX_ACC_PROOFS_SIZE: u32 = 17;

#[derive(Accounts)]
pub struct CreateTree<'info> {
    #[account(
        init,
        seeds = [merkle_tree.key().as_ref()],
        payer = payer,
        space = TREE_AUTHORITY_SIZE,
        bump,
    )]
    pub tree_authority: Account<'info, TreeConfig>,
    #[account(zero)]
    /// CHECK: This account must be all zeros
    pub merkle_tree: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub tree_creator: Signer<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn create_tree(
    ctx: Context<CreateTree>,
    max_depth: u32,
    max_buffer_size: u32,
    public: Option<bool>,
) -> Result<()> {
    let merkle_tree = ctx.accounts.merkle_tree.to_account_info();

    check_canopy_size(&ctx, max_depth, max_buffer_size)?;

    let seed = merkle_tree.key();
    let seeds = &[seed.as_ref(), &[ctx.bumps.tree_authority]];
    let authority = &mut ctx.accounts.tree_authority;
    authority.set_inner(TreeConfig {
        tree_creator: ctx.accounts.tree_creator.key(),
        tree_delegate: ctx.accounts.tree_creator.key(),
        total_mint_capacity: 1 << max_depth,
        num_minted: 0,
        is_public: public.unwrap_or(false),
        is_decompressible: DecompressibleState::Disabled,
    });
    let authority_pda_signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.compression_program.to_account_info(),
        spl_account_compression::cpi::accounts::Initialize {
            authority: ctx.accounts.tree_authority.to_account_info(),
            merkle_tree,
            noop: ctx.accounts.log_wrapper.to_account_info(),
        },
        authority_pda_signer,
    );
    spl_account_compression::cpi::init_empty_merkle_tree(cpi_ctx, max_depth, max_buffer_size)
}

fn check_canopy_size(
    ctx: &Context<CreateTree>,
    max_depth: u32,
    max_buffer_size: u32,
) -> Result<()> {
    let merkle_tree_bytes = ctx.accounts.merkle_tree.data.borrow();

    let (header_bytes, rest) = merkle_tree_bytes.split_at(CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1);

    let mut header = ConcurrentMerkleTreeHeader::try_from_slice(header_bytes)?;
    header.initialize(
        max_depth,
        max_buffer_size,
        &ctx.accounts.tree_authority.key(),
        Clock::get()?.slot,
    );

    let merkle_tree_size = merkle_tree_get_size(&header)?;

    let (_tree_bytes, canopy_bytes) = rest.split_at(merkle_tree_size);

    let required_canopy = max_depth.saturating_sub(MAX_ACC_PROOFS_SIZE);

    let actual_canopy_size = canopy_bytes.len() / size_of::<Node>();

    require!(
        (actual_canopy_size as u32) >= required_canopy,
        BubblegumError::InvalidCanopySize
    );

    Ok(())
}
