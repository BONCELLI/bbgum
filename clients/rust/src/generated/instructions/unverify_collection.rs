//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::MetadataArgs;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct UnverifyCollection {
    pub tree_config: solana_program::pubkey::Pubkey,

    pub leaf_owner: solana_program::pubkey::Pubkey,

    pub leaf_delegate: solana_program::pubkey::Pubkey,

    pub merkle_tree: solana_program::pubkey::Pubkey,

    pub payer: solana_program::pubkey::Pubkey,
    /// This account is checked to be a signer in
    /// the case of `set_and_verify_collection` where
    /// we are actually changing the NFT metadata.
    pub tree_creator_or_delegate: solana_program::pubkey::Pubkey,

    pub collection_authority: solana_program::pubkey::Pubkey,
    /// If there is no collecton authority record PDA then
    /// this must be the Bubblegum program address.
    pub collection_authority_record_pda: Option<solana_program::pubkey::Pubkey>,

    pub collection_mint: solana_program::pubkey::Pubkey,

    pub collection_metadata: solana_program::pubkey::Pubkey,

    pub collection_edition: solana_program::pubkey::Pubkey,

    pub bubblegum_signer: solana_program::pubkey::Pubkey,

    pub log_wrapper: solana_program::pubkey::Pubkey,

    pub compression_program: solana_program::pubkey::Pubkey,

    pub token_metadata_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl UnverifyCollection {
    pub fn instruction(
        &self,
        args: UnverifyCollectionInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UnverifyCollectionInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(16 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.tree_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.leaf_owner,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.leaf_delegate,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.merkle_tree,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.tree_creator_or_delegate,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collection_authority,
            true,
        ));
        if let Some(collection_authority_record_pda) = self.collection_authority_record_pda {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                collection_authority_record_pda,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_BUBBLEGUM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collection_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.collection_metadata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collection_edition,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.bubblegum_signer,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.log_wrapper,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.compression_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_metadata_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = UnverifyCollectionInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MPL_BUBBLEGUM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct UnverifyCollectionInstructionData {
    discriminator: [u8; 8],
}

impl UnverifyCollectionInstructionData {
    fn new() -> Self {
        Self {
            discriminator: [250, 251, 42, 106, 41, 137, 186, 168],
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnverifyCollectionInstructionArgs {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub metadata: MetadataArgs,
}

/// Instruction builder.
#[derive(Default)]
pub struct UnverifyCollectionBuilder {
    tree_config: Option<solana_program::pubkey::Pubkey>,
    leaf_owner: Option<solana_program::pubkey::Pubkey>,
    leaf_delegate: Option<solana_program::pubkey::Pubkey>,
    merkle_tree: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    tree_creator_or_delegate: Option<solana_program::pubkey::Pubkey>,
    collection_authority: Option<solana_program::pubkey::Pubkey>,
    collection_authority_record_pda: Option<solana_program::pubkey::Pubkey>,
    collection_mint: Option<solana_program::pubkey::Pubkey>,
    collection_metadata: Option<solana_program::pubkey::Pubkey>,
    collection_edition: Option<solana_program::pubkey::Pubkey>,
    bubblegum_signer: Option<solana_program::pubkey::Pubkey>,
    log_wrapper: Option<solana_program::pubkey::Pubkey>,
    compression_program: Option<solana_program::pubkey::Pubkey>,
    token_metadata_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    root: Option<[u8; 32]>,
    data_hash: Option<[u8; 32]>,
    creator_hash: Option<[u8; 32]>,
    nonce: Option<u64>,
    index: Option<u32>,
    metadata: Option<MetadataArgs>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UnverifyCollectionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn tree_config(&mut self, tree_config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.tree_config = Some(tree_config);
        self
    }
    #[inline(always)]
    pub fn leaf_owner(&mut self, leaf_owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.leaf_owner = Some(leaf_owner);
        self
    }
    #[inline(always)]
    pub fn leaf_delegate(&mut self, leaf_delegate: solana_program::pubkey::Pubkey) -> &mut Self {
        self.leaf_delegate = Some(leaf_delegate);
        self
    }
    #[inline(always)]
    pub fn merkle_tree(&mut self, merkle_tree: solana_program::pubkey::Pubkey) -> &mut Self {
        self.merkle_tree = Some(merkle_tree);
        self
    }
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// This account is checked to be a signer in
    /// the case of `set_and_verify_collection` where
    /// we are actually changing the NFT metadata.
    #[inline(always)]
    pub fn tree_creator_or_delegate(
        &mut self,
        tree_creator_or_delegate: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.tree_creator_or_delegate = Some(tree_creator_or_delegate);
        self
    }
    #[inline(always)]
    pub fn collection_authority(
        &mut self,
        collection_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_authority = Some(collection_authority);
        self
    }
    /// `[optional account]`
    /// If there is no collecton authority record PDA then
    /// this must be the Bubblegum program address.
    #[inline(always)]
    pub fn collection_authority_record_pda(
        &mut self,
        collection_authority_record_pda: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.collection_authority_record_pda = collection_authority_record_pda;
        self
    }
    #[inline(always)]
    pub fn collection_mint(
        &mut self,
        collection_mint: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_mint = Some(collection_mint);
        self
    }
    #[inline(always)]
    pub fn collection_metadata(
        &mut self,
        collection_metadata: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_metadata = Some(collection_metadata);
        self
    }
    #[inline(always)]
    pub fn collection_edition(
        &mut self,
        collection_edition: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_edition = Some(collection_edition);
        self
    }
    /// `[optional account, default to '4ewWZC5gT6TGpm5LZNDs9wVonfUT2q5PP5sc9kVbwMAK']`
    #[inline(always)]
    pub fn bubblegum_signer(
        &mut self,
        bubblegum_signer: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.bubblegum_signer = Some(bubblegum_signer);
        self
    }
    /// `[optional account, default to 'noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV']`
    #[inline(always)]
    pub fn log_wrapper(&mut self, log_wrapper: solana_program::pubkey::Pubkey) -> &mut Self {
        self.log_wrapper = Some(log_wrapper);
        self
    }
    /// `[optional account, default to 'cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK']`
    #[inline(always)]
    pub fn compression_program(
        &mut self,
        compression_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.compression_program = Some(compression_program);
        self
    }
    /// `[optional account, default to 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s']`
    #[inline(always)]
    pub fn token_metadata_program(
        &mut self,
        token_metadata_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_metadata_program = Some(token_metadata_program);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn root(&mut self, root: [u8; 32]) -> &mut Self {
        self.root = Some(root);
        self
    }
    #[inline(always)]
    pub fn data_hash(&mut self, data_hash: [u8; 32]) -> &mut Self {
        self.data_hash = Some(data_hash);
        self
    }
    #[inline(always)]
    pub fn creator_hash(&mut self, creator_hash: [u8; 32]) -> &mut Self {
        self.creator_hash = Some(creator_hash);
        self
    }
    #[inline(always)]
    pub fn nonce(&mut self, nonce: u64) -> &mut Self {
        self.nonce = Some(nonce);
        self
    }
    #[inline(always)]
    pub fn index(&mut self, index: u32) -> &mut Self {
        self.index = Some(index);
        self
    }
    #[inline(always)]
    pub fn metadata(&mut self, metadata: MetadataArgs) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts =
            UnverifyCollection {
                tree_config: self.tree_config.expect("tree_config is not set"),
                leaf_owner: self.leaf_owner.expect("leaf_owner is not set"),
                leaf_delegate: self.leaf_delegate.expect("leaf_delegate is not set"),
                merkle_tree: self.merkle_tree.expect("merkle_tree is not set"),
                payer: self.payer.expect("payer is not set"),
                tree_creator_or_delegate: self
                    .tree_creator_or_delegate
                    .expect("tree_creator_or_delegate is not set"),
                collection_authority: self
                    .collection_authority
                    .expect("collection_authority is not set"),
                collection_authority_record_pda: self.collection_authority_record_pda,
                collection_mint: self.collection_mint.expect("collection_mint is not set"),
                collection_metadata: self
                    .collection_metadata
                    .expect("collection_metadata is not set"),
                collection_edition: self
                    .collection_edition
                    .expect("collection_edition is not set"),
                bubblegum_signer: self.bubblegum_signer.unwrap_or(solana_program::pubkey!(
                    "4ewWZC5gT6TGpm5LZNDs9wVonfUT2q5PP5sc9kVbwMAK"
                )),
                log_wrapper: self.log_wrapper.unwrap_or(solana_program::pubkey!(
                    "noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV"
                )),
                compression_program: self.compression_program.unwrap_or(solana_program::pubkey!(
                    "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK"
                )),
                token_metadata_program: self.token_metadata_program.unwrap_or(
                    solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                ),
                system_program: self
                    .system_program
                    .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            };
        let args = UnverifyCollectionInstructionArgs {
            root: self.root.clone().expect("root is not set"),
            data_hash: self.data_hash.clone().expect("data_hash is not set"),
            creator_hash: self.creator_hash.clone().expect("creator_hash is not set"),
            nonce: self.nonce.clone().expect("nonce is not set"),
            index: self.index.clone().expect("index is not set"),
            metadata: self.metadata.clone().expect("metadata is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `unverify_collection` CPI accounts.
pub struct UnverifyCollectionCpiAccounts<'a, 'b> {
    pub tree_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub leaf_owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub leaf_delegate: &'b solana_program::account_info::AccountInfo<'a>,

    pub merkle_tree: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// This account is checked to be a signer in
    /// the case of `set_and_verify_collection` where
    /// we are actually changing the NFT metadata.
    pub tree_creator_or_delegate: &'b solana_program::account_info::AccountInfo<'a>,

    pub collection_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// If there is no collecton authority record PDA then
    /// this must be the Bubblegum program address.
    pub collection_authority_record_pda: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub collection_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub collection_metadata: &'b solana_program::account_info::AccountInfo<'a>,

    pub collection_edition: &'b solana_program::account_info::AccountInfo<'a>,

    pub bubblegum_signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub log_wrapper: &'b solana_program::account_info::AccountInfo<'a>,

    pub compression_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `unverify_collection` CPI instruction.
pub struct UnverifyCollectionCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub tree_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub leaf_owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub leaf_delegate: &'b solana_program::account_info::AccountInfo<'a>,

    pub merkle_tree: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// This account is checked to be a signer in
    /// the case of `set_and_verify_collection` where
    /// we are actually changing the NFT metadata.
    pub tree_creator_or_delegate: &'b solana_program::account_info::AccountInfo<'a>,

    pub collection_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// If there is no collecton authority record PDA then
    /// this must be the Bubblegum program address.
    pub collection_authority_record_pda: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub collection_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub collection_metadata: &'b solana_program::account_info::AccountInfo<'a>,

    pub collection_edition: &'b solana_program::account_info::AccountInfo<'a>,

    pub bubblegum_signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub log_wrapper: &'b solana_program::account_info::AccountInfo<'a>,

    pub compression_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: UnverifyCollectionInstructionArgs,
}

impl<'a, 'b> UnverifyCollectionCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UnverifyCollectionCpiAccounts<'a, 'b>,
        args: UnverifyCollectionInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            tree_config: accounts.tree_config,
            leaf_owner: accounts.leaf_owner,
            leaf_delegate: accounts.leaf_delegate,
            merkle_tree: accounts.merkle_tree,
            payer: accounts.payer,
            tree_creator_or_delegate: accounts.tree_creator_or_delegate,
            collection_authority: accounts.collection_authority,
            collection_authority_record_pda: accounts.collection_authority_record_pda,
            collection_mint: accounts.collection_mint,
            collection_metadata: accounts.collection_metadata,
            collection_edition: accounts.collection_edition,
            bubblegum_signer: accounts.bubblegum_signer,
            log_wrapper: accounts.log_wrapper,
            compression_program: accounts.compression_program,
            token_metadata_program: accounts.token_metadata_program,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(16 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.tree_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.leaf_owner.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.leaf_delegate.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.merkle_tree.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.tree_creator_or_delegate.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collection_authority.key,
            true,
        ));
        if let Some(collection_authority_record_pda) = self.collection_authority_record_pda {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *collection_authority_record_pda.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_BUBBLEGUM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collection_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.collection_metadata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collection_edition.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.bubblegum_signer.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.log_wrapper.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.compression_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_metadata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = UnverifyCollectionInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_BUBBLEGUM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(16 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.tree_config.clone());
        account_infos.push(self.leaf_owner.clone());
        account_infos.push(self.leaf_delegate.clone());
        account_infos.push(self.merkle_tree.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.tree_creator_or_delegate.clone());
        account_infos.push(self.collection_authority.clone());
        if let Some(collection_authority_record_pda) = self.collection_authority_record_pda {
            account_infos.push(collection_authority_record_pda.clone());
        }
        account_infos.push(self.collection_mint.clone());
        account_infos.push(self.collection_metadata.clone());
        account_infos.push(self.collection_edition.clone());
        account_infos.push(self.bubblegum_signer.clone());
        account_infos.push(self.log_wrapper.clone());
        account_infos.push(self.compression_program.clone());
        account_infos.push(self.token_metadata_program.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// `unverify_collection` CPI instruction builder.
pub struct UnverifyCollectionCpiBuilder<'a, 'b> {
    instruction: Box<UnverifyCollectionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UnverifyCollectionCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UnverifyCollectionCpiBuilderInstruction {
            __program: program,
            tree_config: None,
            leaf_owner: None,
            leaf_delegate: None,
            merkle_tree: None,
            payer: None,
            tree_creator_or_delegate: None,
            collection_authority: None,
            collection_authority_record_pda: None,
            collection_mint: None,
            collection_metadata: None,
            collection_edition: None,
            bubblegum_signer: None,
            log_wrapper: None,
            compression_program: None,
            token_metadata_program: None,
            system_program: None,
            root: None,
            data_hash: None,
            creator_hash: None,
            nonce: None,
            index: None,
            metadata: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn tree_config(
        &mut self,
        tree_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tree_config = Some(tree_config);
        self
    }
    #[inline(always)]
    pub fn leaf_owner(
        &mut self,
        leaf_owner: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.leaf_owner = Some(leaf_owner);
        self
    }
    #[inline(always)]
    pub fn leaf_delegate(
        &mut self,
        leaf_delegate: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.leaf_delegate = Some(leaf_delegate);
        self
    }
    #[inline(always)]
    pub fn merkle_tree(
        &mut self,
        merkle_tree: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.merkle_tree = Some(merkle_tree);
        self
    }
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// This account is checked to be a signer in
    /// the case of `set_and_verify_collection` where
    /// we are actually changing the NFT metadata.
    #[inline(always)]
    pub fn tree_creator_or_delegate(
        &mut self,
        tree_creator_or_delegate: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tree_creator_or_delegate = Some(tree_creator_or_delegate);
        self
    }
    #[inline(always)]
    pub fn collection_authority(
        &mut self,
        collection_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_authority = Some(collection_authority);
        self
    }
    /// `[optional account]`
    /// If there is no collecton authority record PDA then
    /// this must be the Bubblegum program address.
    #[inline(always)]
    pub fn collection_authority_record_pda(
        &mut self,
        collection_authority_record_pda: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.collection_authority_record_pda = collection_authority_record_pda;
        self
    }
    #[inline(always)]
    pub fn collection_mint(
        &mut self,
        collection_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_mint = Some(collection_mint);
        self
    }
    #[inline(always)]
    pub fn collection_metadata(
        &mut self,
        collection_metadata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_metadata = Some(collection_metadata);
        self
    }
    #[inline(always)]
    pub fn collection_edition(
        &mut self,
        collection_edition: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_edition = Some(collection_edition);
        self
    }
    #[inline(always)]
    pub fn bubblegum_signer(
        &mut self,
        bubblegum_signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bubblegum_signer = Some(bubblegum_signer);
        self
    }
    #[inline(always)]
    pub fn log_wrapper(
        &mut self,
        log_wrapper: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.log_wrapper = Some(log_wrapper);
        self
    }
    #[inline(always)]
    pub fn compression_program(
        &mut self,
        compression_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.compression_program = Some(compression_program);
        self
    }
    #[inline(always)]
    pub fn token_metadata_program(
        &mut self,
        token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_metadata_program = Some(token_metadata_program);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn root(&mut self, root: [u8; 32]) -> &mut Self {
        self.instruction.root = Some(root);
        self
    }
    #[inline(always)]
    pub fn data_hash(&mut self, data_hash: [u8; 32]) -> &mut Self {
        self.instruction.data_hash = Some(data_hash);
        self
    }
    #[inline(always)]
    pub fn creator_hash(&mut self, creator_hash: [u8; 32]) -> &mut Self {
        self.instruction.creator_hash = Some(creator_hash);
        self
    }
    #[inline(always)]
    pub fn nonce(&mut self, nonce: u64) -> &mut Self {
        self.instruction.nonce = Some(nonce);
        self
    }
    #[inline(always)]
    pub fn index(&mut self, index: u32) -> &mut Self {
        self.instruction.index = Some(index);
        self
    }
    #[inline(always)]
    pub fn metadata(&mut self, metadata: MetadataArgs) -> &mut Self {
        self.instruction.metadata = Some(metadata);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = UnverifyCollectionInstructionArgs {
            root: self.instruction.root.clone().expect("root is not set"),
            data_hash: self
                .instruction
                .data_hash
                .clone()
                .expect("data_hash is not set"),
            creator_hash: self
                .instruction
                .creator_hash
                .clone()
                .expect("creator_hash is not set"),
            nonce: self.instruction.nonce.clone().expect("nonce is not set"),
            index: self.instruction.index.clone().expect("index is not set"),
            metadata: self
                .instruction
                .metadata
                .clone()
                .expect("metadata is not set"),
        };
        let instruction = UnverifyCollectionCpi {
            __program: self.instruction.__program,

            tree_config: self
                .instruction
                .tree_config
                .expect("tree_config is not set"),

            leaf_owner: self.instruction.leaf_owner.expect("leaf_owner is not set"),

            leaf_delegate: self
                .instruction
                .leaf_delegate
                .expect("leaf_delegate is not set"),

            merkle_tree: self
                .instruction
                .merkle_tree
                .expect("merkle_tree is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            tree_creator_or_delegate: self
                .instruction
                .tree_creator_or_delegate
                .expect("tree_creator_or_delegate is not set"),

            collection_authority: self
                .instruction
                .collection_authority
                .expect("collection_authority is not set"),

            collection_authority_record_pda: self.instruction.collection_authority_record_pda,

            collection_mint: self
                .instruction
                .collection_mint
                .expect("collection_mint is not set"),

            collection_metadata: self
                .instruction
                .collection_metadata
                .expect("collection_metadata is not set"),

            collection_edition: self
                .instruction
                .collection_edition
                .expect("collection_edition is not set"),

            bubblegum_signer: self
                .instruction
                .bubblegum_signer
                .expect("bubblegum_signer is not set"),

            log_wrapper: self
                .instruction
                .log_wrapper
                .expect("log_wrapper is not set"),

            compression_program: self
                .instruction
                .compression_program
                .expect("compression_program is not set"),

            token_metadata_program: self
                .instruction
                .token_metadata_program
                .expect("token_metadata_program is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct UnverifyCollectionCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    tree_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    leaf_owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    leaf_delegate: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    merkle_tree: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    tree_creator_or_delegate: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection_authority_record_pda: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection_metadata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection_edition: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bubblegum_signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    log_wrapper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    compression_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_metadata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    root: Option<[u8; 32]>,
    data_hash: Option<[u8; 32]>,
    creator_hash: Option<[u8; 32]>,
    nonce: Option<u64>,
    index: Option<u32>,
    metadata: Option<MetadataArgs>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
