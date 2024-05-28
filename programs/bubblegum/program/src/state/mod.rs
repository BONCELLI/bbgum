pub mod leaf_schema;
pub mod metaplex_adapter;
pub mod metaplex_anchor;

use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use leaf_schema::LeafSchema;

pub const TREE_AUTHORITY_SIZE: usize = 8 + 32 + 32 + 8 + 8 + 1 + 1 + 6; // 6 bytes padding
pub const VOUCHER_SIZE: usize = 8 + 1 + 32 + 32 + 32 + 8 + 32 + 32 + 4 + 32;
pub const VOUCHER_PREFIX: &str = "voucher";
pub const ASSET_PREFIX: &str = "asset";
pub const COLLECTION_CPI_PREFIX: &str = "collection_cpi";

pub const MAX_ACC_PROOFS_SIZE: u32 = 17;

// TODO: set real keys before mainnet deploy
pub const REALM: Pubkey = solana_program::pubkey!("EzsKaQq68FLZwRaiUx7t17LWVVzsE8wRkhBghFrZGGwG");
pub const REALM_GOVERNING_MINT: Pubkey =
    solana_program::pubkey!("Dqa4iCUDXvSh5FwhopFJM76xdxQb5vSw39LvggbUWH9o");

// TODO: change to real one
pub const MINIMUM_STAKE: u64 = 100000000;

#[account]
#[derive(Copy, Debug, PartialEq, Eq)]
pub struct TreeConfig {
    pub tree_creator: Pubkey,
    pub tree_delegate: Pubkey,
    pub total_mint_capacity: u64,
    pub num_minted: u64,
    pub is_public: bool,
    pub is_decompressible: DecompressibleState,
}

impl TreeConfig {
    pub fn increment_mint_count(&mut self) {
        self.num_minted = self.num_minted.saturating_add(1);
    }

    pub fn contains_mint_capacity(&self, requested_capacity: u64) -> bool {
        let remaining_mints = self.total_mint_capacity.saturating_sub(self.num_minted);
        requested_capacity <= remaining_mints
    }
}

#[account]
#[derive(Debug, Eq, PartialEq)]
pub struct Voucher {
    pub leaf_schema: LeafSchema,
    pub index: u32,
    pub merkle_tree: Pubkey,
}

impl Voucher {
    pub fn new(leaf_schema: LeafSchema, index: u32, merkle_tree: Pubkey) -> Self {
        Self {
            leaf_schema,
            index,
            merkle_tree,
        }
    }

    fn pda_for_prefix(&self, prefix: &str) -> Pubkey {
        Pubkey::find_program_address(
            &[
                prefix.as_ref(),
                self.merkle_tree.as_ref(),
                self.leaf_schema.nonce().to_le_bytes().as_ref(),
            ],
            &crate::id(),
        )
        .0
    }

    pub fn pda(&self) -> Pubkey {
        self.pda_for_prefix(VOUCHER_PREFIX)
    }

    pub fn decompress_mint_pda(&self) -> Pubkey {
        self.pda_for_prefix(ASSET_PREFIX)
    }
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
#[repr(u8)]
pub enum BubblegumEventType {
    /// Marker for 0 data.
    Uninitialized,
    /// Leaf schema event.
    LeafSchemaEvent,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum DecompressibleState {
    Enabled = 0,
    Disabled = 1,
}
