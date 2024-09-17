/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  findMasterEditionPda,
  findMetadataPda,
} from '@metaplex-foundation/mpl-token-metadata';
import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  bytes,
  mapSerializer,
  string,
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
export type FinalizeTreeWithRootAndCollectionInstructionAccounts = {
  treeConfig?: PublicKey | Pda;
  merkleTree: PublicKey | Pda;
  payer?: Signer;
  treeCreatorOrDelegate?: Signer;
  staker: Signer;
  collectionAuthority?: Signer;
  registrar: PublicKey | Pda;
  voter: PublicKey | Pda;
  mining: PublicKey | Pda;
  feeReceiver: PublicKey | Pda;
  /**
   * If there is no collecton authority record PDA then
   * this must be the Bubblegum program address.
   */

  collectionAuthorityRecordPda?: PublicKey | Pda;
  collectionMint: PublicKey | Pda;
  collectionMetadata?: PublicKey | Pda;
  collectionEdition?: PublicKey | Pda;
  logWrapper?: PublicKey | Pda;
  compressionProgram?: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
};

// Data.
export type FinalizeTreeWithRootAndCollectionInstructionData = {
  discriminator: Array<number>;
  root: Uint8Array;
  rightmostLeaf: Uint8Array;
  rightmostIndex: number;
  metadataUrl: string;
  metadataHash: string;
};

export type FinalizeTreeWithRootAndCollectionInstructionDataArgs = {
  root: Uint8Array;
  rightmostLeaf: Uint8Array;
  rightmostIndex: number;
  metadataUrl: string;
  metadataHash: string;
};

export function getFinalizeTreeWithRootAndCollectionInstructionDataSerializer(): Serializer<
  FinalizeTreeWithRootAndCollectionInstructionDataArgs,
  FinalizeTreeWithRootAndCollectionInstructionData
> {
  return mapSerializer<
    FinalizeTreeWithRootAndCollectionInstructionDataArgs,
    any,
    FinalizeTreeWithRootAndCollectionInstructionData
  >(
    struct<FinalizeTreeWithRootAndCollectionInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['root', bytes({ size: 32 })],
        ['rightmostLeaf', bytes({ size: 32 })],
        ['rightmostIndex', u32()],
        ['metadataUrl', string()],
        ['metadataHash', string()],
      ],
      { description: 'FinalizeTreeWithRootAndCollectionInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [194, 98, 45, 168, 183, 72, 67, 155],
    })
  ) as Serializer<
    FinalizeTreeWithRootAndCollectionInstructionDataArgs,
    FinalizeTreeWithRootAndCollectionInstructionData
  >;
}

// Args.
export type FinalizeTreeWithRootAndCollectionInstructionArgs =
  FinalizeTreeWithRootAndCollectionInstructionDataArgs;

// Instruction.
export function finalizeTreeWithRootAndCollection(
  context: Pick<Context, 'eddsa' | 'identity' | 'payer' | 'programs'>,
  input: FinalizeTreeWithRootAndCollectionInstructionAccounts &
    FinalizeTreeWithRootAndCollectionInstructionArgs
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
    treeCreatorOrDelegate: {
      index: 3,
      isWritable: false,
      value: input.treeCreatorOrDelegate ?? null,
    },
    staker: { index: 4, isWritable: false, value: input.staker ?? null },
    collectionAuthority: {
      index: 5,
      isWritable: false,
      value: input.collectionAuthority ?? null,
    },
    registrar: { index: 6, isWritable: false, value: input.registrar ?? null },
    voter: { index: 7, isWritable: false, value: input.voter ?? null },
    mining: { index: 8, isWritable: false, value: input.mining ?? null },
    feeReceiver: {
      index: 9,
      isWritable: true,
      value: input.feeReceiver ?? null,
    },
    collectionAuthorityRecordPda: {
      index: 10,
      isWritable: false,
      value: input.collectionAuthorityRecordPda ?? null,
    },
    collectionMint: {
      index: 11,
      isWritable: false,
      value: input.collectionMint ?? null,
    },
    collectionMetadata: {
      index: 12,
      isWritable: true,
      value: input.collectionMetadata ?? null,
    },
    collectionEdition: {
      index: 13,
      isWritable: false,
      value: input.collectionEdition ?? null,
    },
    logWrapper: {
      index: 14,
      isWritable: false,
      value: input.logWrapper ?? null,
    },
    compressionProgram: {
      index: 15,
      isWritable: false,
      value: input.compressionProgram ?? null,
    },
    systemProgram: {
      index: 16,
      isWritable: false,
      value: input.systemProgram ?? null,
    },
  };

  // Arguments.
  const resolvedArgs: FinalizeTreeWithRootAndCollectionInstructionArgs = {
    ...input,
  };

  // Default values.
  if (!resolvedAccounts.treeConfig.value) {
    resolvedAccounts.treeConfig.value = findTreeConfigPda(context, {
      merkleTree: expectPublicKey(resolvedAccounts.merkleTree.value),
    });
  }
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.treeCreatorOrDelegate.value) {
    resolvedAccounts.treeCreatorOrDelegate.value = context.identity;
  }
  if (!resolvedAccounts.collectionAuthority.value) {
    resolvedAccounts.collectionAuthority.value = context.identity;
  }
  if (!resolvedAccounts.collectionMetadata.value) {
    resolvedAccounts.collectionMetadata.value = findMetadataPda(context, {
      mint: expectPublicKey(resolvedAccounts.collectionMint.value),
    });
  }
  if (!resolvedAccounts.collectionEdition.value) {
    resolvedAccounts.collectionEdition.value = findMasterEditionPda(context, {
      mint: expectPublicKey(resolvedAccounts.collectionMint.value),
    });
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
  const data =
    getFinalizeTreeWithRootAndCollectionInstructionDataSerializer().serialize(
      resolvedArgs as FinalizeTreeWithRootAndCollectionInstructionDataArgs
    );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}