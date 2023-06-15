/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Account,
  Context,
  Pda,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  publicKey as toPublicKey,
} from '@metaplex-foundation/umi';
import {
  MerkleTreeAccountData,
  getMerkleTreeAccountDataSerializer,
} from '../../hooked';
import {
  CompressionAccountTypeArgs,
  ConcurrentMerkleTreeHeaderDataArgs,
  getCompressionAccountTypeSerializer,
  getConcurrentMerkleTreeHeaderDataSerializer,
} from '../types';

export type MerkleTree = Account<MerkleTreeAccountData>;

export function deserializeMerkleTree(
  context: Pick<Context, 'serializer'>,
  rawAccount: RpcAccount
): MerkleTree {
  return deserializeAccount(
    rawAccount,
    getMerkleTreeAccountDataSerializer(context)
  );
}

export async function fetchMerkleTree(
  context: Pick<Context, 'rpc' | 'serializer'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<MerkleTree> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  assertAccountExists(maybeAccount, 'MerkleTree');
  return deserializeMerkleTree(context, maybeAccount);
}

export async function safeFetchMerkleTree(
  context: Pick<Context, 'rpc' | 'serializer'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<MerkleTree | null> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  return maybeAccount.exists
    ? deserializeMerkleTree(context, maybeAccount)
    : null;
}

export async function fetchAllMerkleTree(
  context: Pick<Context, 'rpc' | 'serializer'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<MerkleTree[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, 'MerkleTree');
    return deserializeMerkleTree(context, maybeAccount);
  });
}

export async function safeFetchAllMerkleTree(
  context: Pick<Context, 'rpc' | 'serializer'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<MerkleTree[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) =>
      deserializeMerkleTree(context, maybeAccount as RpcAccount)
    );
}

export function getMerkleTreeGpaBuilder(
  context: Pick<Context, 'rpc' | 'serializer' | 'programs'>
) {
  // const s = context.serializer;
  const programId = context.programs.getPublicKey(
    'splAccountCompression',
    'cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      discriminator: CompressionAccountTypeArgs;
      treeHeader: ConcurrentMerkleTreeHeaderDataArgs;
    }>({
      discriminator: [0, getCompressionAccountTypeSerializer(context)],
      treeHeader: [1, getConcurrentMerkleTreeHeaderDataSerializer(context)],
    })
    .deserializeUsing<MerkleTree>((account) =>
      deserializeMerkleTree(context, account)
    );
}