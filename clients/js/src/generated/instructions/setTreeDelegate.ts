/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  AccountMeta,
  Context,
  PublicKey,
  Serializer,
  Signer,
  TransactionBuilder,
  mapSerializer,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import { addObjectProperty, isWritable } from '../shared';

// Accounts.
export type SetTreeDelegateInstructionAccounts = {
  treeAuthority: PublicKey;
  treeCreator: Signer;
  newTreeDelegate: PublicKey;
  merkleTree: PublicKey;
  systemProgram?: PublicKey;
};

// Data.
export type SetTreeDelegateInstructionData = { discriminator: Array<number> };

export type SetTreeDelegateInstructionDataArgs = {};

export function getSetTreeDelegateInstructionDataSerializer(
  context: Pick<Context, 'serializer'>
): Serializer<
  SetTreeDelegateInstructionDataArgs,
  SetTreeDelegateInstructionData
> {
  const s = context.serializer;
  return mapSerializer<
    SetTreeDelegateInstructionDataArgs,
    any,
    SetTreeDelegateInstructionData
  >(
    s.struct<SetTreeDelegateInstructionData>(
      [['discriminator', s.array(s.u8(), { size: 8 })]],
      { description: 'SetTreeDelegateInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [253, 118, 66, 37, 190, 49, 154, 102],
    })
  ) as Serializer<
    SetTreeDelegateInstructionDataArgs,
    SetTreeDelegateInstructionData
  >;
}

// Instruction.
export function setTreeDelegate(
  context: Pick<Context, 'serializer' | 'programs'>,
  input: SetTreeDelegateInstructionAccounts
): TransactionBuilder {
  const signers: Signer[] = [];
  const keys: AccountMeta[] = [];

  // Program ID.
  const programId = {
    ...context.programs.getPublicKey(
      'mplBubblegum',
      'BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY'
    ),
    isWritable: false,
  };

  // Resolved inputs.
  const resolvingAccounts = {};
  addObjectProperty(
    resolvingAccounts,
    'systemProgram',
    input.systemProgram ?? {
      ...context.programs.getPublicKey(
        'splSystem',
        '11111111111111111111111111111111'
      ),
      isWritable: false,
    }
  );
  const resolvedAccounts = { ...input, ...resolvingAccounts };

  // Tree Authority.
  keys.push({
    pubkey: resolvedAccounts.treeAuthority,
    isSigner: false,
    isWritable: isWritable(resolvedAccounts.treeAuthority, true),
  });

  // Tree Creator.
  signers.push(resolvedAccounts.treeCreator);
  keys.push({
    pubkey: resolvedAccounts.treeCreator.publicKey,
    isSigner: true,
    isWritable: isWritable(resolvedAccounts.treeCreator, false),
  });

  // New Tree Delegate.
  keys.push({
    pubkey: resolvedAccounts.newTreeDelegate,
    isSigner: false,
    isWritable: isWritable(resolvedAccounts.newTreeDelegate, false),
  });

  // Merkle Tree.
  keys.push({
    pubkey: resolvedAccounts.merkleTree,
    isSigner: false,
    isWritable: isWritable(resolvedAccounts.merkleTree, false),
  });

  // System Program.
  keys.push({
    pubkey: resolvedAccounts.systemProgram,
    isSigner: false,
    isWritable: isWritable(resolvedAccounts.systemProgram, false),
  });

  // Data.
  const data = getSetTreeDelegateInstructionDataSerializer(context).serialize(
    {}
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}