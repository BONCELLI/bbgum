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
  Pda,
  PublicKey,
  Serializer,
  Signer,
  TransactionBuilder,
  mapSerializer,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import { addAccountMeta } from '../shared';

// Accounts.
export type VerifyLeafInstructionAccounts = {
  merkleTree: PublicKey | Pda;
};

// Data.
export type VerifyLeafInstructionData = {
  discriminator: Array<number>;
  root: Uint8Array;
  leaf: Uint8Array;
  index: number;
};

export type VerifyLeafInstructionDataArgs = {
  root: Uint8Array;
  leaf: Uint8Array;
  index: number;
};

export function getVerifyLeafInstructionDataSerializer(
  context: Pick<Context, 'serializer'>
): Serializer<VerifyLeafInstructionDataArgs, VerifyLeafInstructionData> {
  const s = context.serializer;
  return mapSerializer<
    VerifyLeafInstructionDataArgs,
    any,
    VerifyLeafInstructionData
  >(
    s.struct<VerifyLeafInstructionData>(
      [
        ['discriminator', s.array(s.u8(), { size: 8 })],
        ['root', s.bytes({ size: 32 })],
        ['leaf', s.bytes({ size: 32 })],
        ['index', s.u32()],
      ],
      { description: 'VerifyLeafInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [124, 220, 22, 223, 104, 10, 250, 224],
    })
  ) as Serializer<VerifyLeafInstructionDataArgs, VerifyLeafInstructionData>;
}

// Args.
export type VerifyLeafInstructionArgs = VerifyLeafInstructionDataArgs;

// Instruction.
export function verifyLeaf(
  context: Pick<Context, 'serializer' | 'programs'>,
  input: VerifyLeafInstructionAccounts & VerifyLeafInstructionArgs
): TransactionBuilder {
  const signers: Signer[] = [];
  const keys: AccountMeta[] = [];

  // Program ID.
  const programId = context.programs.getPublicKey(
    'splAccountCompression',
    'cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK'
  );

  // Resolved inputs.
  const resolvedAccounts = {
    merkleTree: [input.merkleTree, false] as const,
  };
  const resolvingArgs = {};
  const resolvedArgs = { ...input, ...resolvingArgs };

  addAccountMeta(keys, signers, resolvedAccounts.merkleTree, false);

  // Data.
  const data =
    getVerifyLeafInstructionDataSerializer(context).serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}