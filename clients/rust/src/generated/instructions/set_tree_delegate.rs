//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct SetTreeDelegate {
    pub tree_config: solana_program::pubkey::Pubkey,

    pub tree_creator: solana_program::pubkey::Pubkey,

    pub new_tree_delegate: solana_program::pubkey::Pubkey,

    pub merkle_tree: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl SetTreeDelegate {
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5);
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.tree_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.tree_creator,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.new_tree_delegate,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.merkle_tree,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        let data = SetTreeDelegateInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::MPL_BUBBLEGUM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct SetTreeDelegateInstructionData {
    discriminator: [u8; 8],
}

impl SetTreeDelegateInstructionData {
    fn new() -> Self {
        Self {
            discriminator: [253, 118, 66, 37, 190, 49, 154, 102],
        }
    }
}

/// Instruction builder.
#[derive(Default)]
pub struct SetTreeDelegateBuilder {
    tree_config: Option<solana_program::pubkey::Pubkey>,
    tree_creator: Option<solana_program::pubkey::Pubkey>,
    new_tree_delegate: Option<solana_program::pubkey::Pubkey>,
    merkle_tree: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
}

impl SetTreeDelegateBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn tree_config(&mut self, tree_config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.tree_config = Some(tree_config);
        self
    }
    #[inline(always)]
    pub fn tree_creator(&mut self, tree_creator: solana_program::pubkey::Pubkey) -> &mut Self {
        self.tree_creator = Some(tree_creator);
        self
    }
    #[inline(always)]
    pub fn new_tree_delegate(
        &mut self,
        new_tree_delegate: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.new_tree_delegate = Some(new_tree_delegate);
        self
    }
    #[inline(always)]
    pub fn merkle_tree(&mut self, merkle_tree: solana_program::pubkey::Pubkey) -> &mut Self {
        self.merkle_tree = Some(merkle_tree);
        self
    }
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn build(&self) -> solana_program::instruction::Instruction {
        let accounts = SetTreeDelegate {
            tree_config: self.tree_config.expect("tree_config is not set"),
            tree_creator: self.tree_creator.expect("tree_creator is not set"),
            new_tree_delegate: self
                .new_tree_delegate
                .expect("new_tree_delegate is not set"),
            merkle_tree: self.merkle_tree.expect("merkle_tree is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };

        accounts.instruction()
    }
}

/// `set_tree_delegate` CPI instruction.
pub struct SetTreeDelegateCpi<'a> {
    /// The program to invoke.
    pub __program: &'a solana_program::account_info::AccountInfo<'a>,

    pub tree_config: &'a solana_program::account_info::AccountInfo<'a>,

    pub tree_creator: &'a solana_program::account_info::AccountInfo<'a>,

    pub new_tree_delegate: &'a solana_program::account_info::AccountInfo<'a>,

    pub merkle_tree: &'a solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'a solana_program::account_info::AccountInfo<'a>,
}

impl<'a> SetTreeDelegateCpi<'a> {
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(5);
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.tree_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.tree_creator.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.new_tree_delegate.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.merkle_tree.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        let data = SetTreeDelegateInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_BUBBLEGUM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + 1);
        account_infos.push(self.__program.clone());
        account_infos.push(self.tree_config.clone());
        account_infos.push(self.tree_creator.clone());
        account_infos.push(self.new_tree_delegate.clone());
        account_infos.push(self.merkle_tree.clone());
        account_infos.push(self.system_program.clone());

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// `set_tree_delegate` CPI instruction builder.
pub struct SetTreeDelegateCpiBuilder<'a> {
    instruction: Box<SetTreeDelegateCpiBuilderInstruction<'a>>,
}

impl<'a> SetTreeDelegateCpiBuilder<'a> {
    pub fn new(program: &'a solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SetTreeDelegateCpiBuilderInstruction {
            __program: program,
            tree_config: None,
            tree_creator: None,
            new_tree_delegate: None,
            merkle_tree: None,
            system_program: None,
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn tree_config(
        &mut self,
        tree_config: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tree_config = Some(tree_config);
        self
    }
    #[inline(always)]
    pub fn tree_creator(
        &mut self,
        tree_creator: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tree_creator = Some(tree_creator);
        self
    }
    #[inline(always)]
    pub fn new_tree_delegate(
        &mut self,
        new_tree_delegate: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.new_tree_delegate = Some(new_tree_delegate);
        self
    }
    #[inline(always)]
    pub fn merkle_tree(
        &mut self,
        merkle_tree: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.merkle_tree = Some(merkle_tree);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn build(&self) -> SetTreeDelegateCpi<'a> {
        SetTreeDelegateCpi {
            __program: self.instruction.__program,

            tree_config: self
                .instruction
                .tree_config
                .expect("tree_config is not set"),

            tree_creator: self
                .instruction
                .tree_creator
                .expect("tree_creator is not set"),

            new_tree_delegate: self
                .instruction
                .new_tree_delegate
                .expect("new_tree_delegate is not set"),

            merkle_tree: self
                .instruction
                .merkle_tree
                .expect("merkle_tree is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
        }
    }
}

struct SetTreeDelegateCpiBuilderInstruction<'a> {
    __program: &'a solana_program::account_info::AccountInfo<'a>,
    tree_config: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    tree_creator: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    new_tree_delegate: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    merkle_tree: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'a solana_program::account_info::AccountInfo<'a>>,
}
