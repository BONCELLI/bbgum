/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Option,
  OptionOrNullable,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  bool,
  mapSerializer,
  option,
  struct,
  u32,
  u8,
} from '@metaplex-foundation/umi/serializers';
import { findTreeConfigPda } from '../accounts';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  expectPublicKey,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type PrepareTreeInstructionAccounts = {
  treeConfig?: PublicKey | Pda;
  merkleTree: PublicKey | Pda;
  payer?: Signer;
  treeCreator?: PublicKey | Pda;
  logWrapper?: PublicKey | Pda;
  compressionProgram?: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
};

// Data.
export type PrepareTreeInstructionData = {
  discriminator: Array<number>;
  maxDepth: number;
  maxBufferSize: number;
  public: Option<boolean>;
};

export type PrepareTreeInstructionDataArgs = {
  maxDepth: number;
  maxBufferSize: number;
  public: OptionOrNullable<boolean>;
};

export function getPrepareTreeInstructionDataSerializer(): Serializer<
  PrepareTreeInstructionDataArgs,
  PrepareTreeInstructionData
> {
  return mapSerializer<
    PrepareTreeInstructionDataArgs,
    any,
    PrepareTreeInstructionData
  >(
    struct<PrepareTreeInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['maxDepth', u32()],
        ['maxBufferSize', u32()],
        ['public', option(bool())],
      ],
      { description: 'PrepareTreeInstructionData' }
    ),
    (value) => ({ ...value, discriminator: [41, 56, 189, 77, 58, 12, 142, 71] })
  ) as Serializer<PrepareTreeInstructionDataArgs, PrepareTreeInstructionData>;
}

// Args.
export type PrepareTreeInstructionArgs = PrepareTreeInstructionDataArgs;

// Instruction.
export function prepareTree(
  context: Pick<Context, 'eddsa' | 'identity' | 'payer' | 'programs'>,
  input: PrepareTreeInstructionAccounts & PrepareTreeInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplBubblegum',
    'BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY'
  );

  // Accounts.
  const resolvedAccounts: ResolvedAccountsWithIndices = {
    treeConfig: { index: 0, isWritable: true, value: input.treeConfig ?? null },
    merkleTree: { index: 1, isWritable: true, value: input.merkleTree ?? null },
    payer: { index: 2, isWritable: true, value: input.payer ?? null },
    treeCreator: {
      index: 3,
      isWritable: false,
      value: input.treeCreator ?? null,
    },
    logWrapper: {
      index: 4,
      isWritable: false,
      value: input.logWrapper ?? null,
    },
    compressionProgram: {
      index: 5,
      isWritable: false,
      value: input.compressionProgram ?? null,
    },
    systemProgram: {
      index: 6,
      isWritable: false,
      value: input.systemProgram ?? null,
    },
  };

  // Arguments.
  const resolvedArgs: PrepareTreeInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.treeConfig.value) {
    resolvedAccounts.treeConfig.value = findTreeConfigPda(context, {
      merkleTree: expectPublicKey(resolvedAccounts.merkleTree.value),
    });
  }
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.treeCreator.value) {
    resolvedAccounts.treeCreator.value = context.identity.publicKey;
  }
  if (!resolvedAccounts.logWrapper.value) {
    resolvedAccounts.logWrapper.value = context.programs.getPublicKey(
      'splNoop',
      'noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV'
    );
    resolvedAccounts.logWrapper.isWritable = false;
  }
  if (!resolvedAccounts.compressionProgram.value) {
    resolvedAccounts.compressionProgram.value = context.programs.getPublicKey(
      'splAccountCompression',
      'cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK'
    );
    resolvedAccounts.compressionProgram.isWritable = false;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getPrepareTreeInstructionDataSerializer().serialize(
    resolvedArgs as PrepareTreeInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
