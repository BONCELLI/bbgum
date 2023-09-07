/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { PublicKey } from '@metaplex-foundation/umi';
import {
  GetDataEnumKind,
  GetDataEnumKindContent,
  Serializer,
  array,
  dataEnum,
  publicKey as publicKeySerializer,
  struct,
  u32,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';

export type ConcurrentMerkleTreeHeaderData = {
  __kind: 'V1';
  /**
   * Buffer of changelogs stored on-chain.
   * Must be a power of 2; see above table for valid combinations.
   */
  maxBufferSize: number;
  /**
   * Depth of the SPL ConcurrentMerkleTree to store.
   * Tree capacity can be calculated as power(2, max_depth).
   * See above table for valid options.
   */
  maxDepth: number;
  /**
   * Authority that validates the content of the trees.
   * Typically a program, e.g., the Bubblegum contract validates that leaves are valid NFTs.
   */
  authority: PublicKey;
  /**
   * Slot corresponding to when the Merkle tree was created.
   * Provides a lower-bound on what slot to start (re-)building a tree from.
   */
  creationSlot: bigint;
  /**
   * Needs padding for the account to be 8-byte aligned
   * 8-byte alignment is necessary to zero-copy the SPL ConcurrentMerkleTree
   */
  padding: Array<number>;
};

export type ConcurrentMerkleTreeHeaderDataArgs = {
  __kind: 'V1';
  /**
   * Buffer of changelogs stored on-chain.
   * Must be a power of 2; see above table for valid combinations.
   */
  maxBufferSize: number;
  /**
   * Depth of the SPL ConcurrentMerkleTree to store.
   * Tree capacity can be calculated as power(2, max_depth).
   * See above table for valid options.
   */
  maxDepth: number;
  /**
   * Authority that validates the content of the trees.
   * Typically a program, e.g., the Bubblegum contract validates that leaves are valid NFTs.
   */
  authority: PublicKey;
  /**
   * Slot corresponding to when the Merkle tree was created.
   * Provides a lower-bound on what slot to start (re-)building a tree from.
   */
  creationSlot: number | bigint;
  /**
   * Needs padding for the account to be 8-byte aligned
   * 8-byte alignment is necessary to zero-copy the SPL ConcurrentMerkleTree
   */
  padding: Array<number>;
};

export function getConcurrentMerkleTreeHeaderDataSerializer(): Serializer<
  ConcurrentMerkleTreeHeaderDataArgs,
  ConcurrentMerkleTreeHeaderData
> {
  return dataEnum<ConcurrentMerkleTreeHeaderData>(
    [
      [
        'V1',
        struct<GetDataEnumKindContent<ConcurrentMerkleTreeHeaderData, 'V1'>>([
          ['maxBufferSize', u32()],
          ['maxDepth', u32()],
          ['authority', publicKeySerializer()],
          ['creationSlot', u64()],
          ['padding', array(u8(), { size: 6 })],
        ]),
      ],
    ],
    { description: 'ConcurrentMerkleTreeHeaderData' }
  ) as Serializer<
    ConcurrentMerkleTreeHeaderDataArgs,
    ConcurrentMerkleTreeHeaderData
  >;
}

// Data Enum Helpers.
export function concurrentMerkleTreeHeaderData(
  kind: 'V1',
  data: GetDataEnumKindContent<ConcurrentMerkleTreeHeaderDataArgs, 'V1'>
): GetDataEnumKind<ConcurrentMerkleTreeHeaderDataArgs, 'V1'>;
export function concurrentMerkleTreeHeaderData<
  K extends ConcurrentMerkleTreeHeaderDataArgs['__kind']
>(
  kind: K,
  data?: any
): Extract<ConcurrentMerkleTreeHeaderDataArgs, { __kind: K }> {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}
export function isConcurrentMerkleTreeHeaderData<
  K extends ConcurrentMerkleTreeHeaderData['__kind']
>(
  kind: K,
  value: ConcurrentMerkleTreeHeaderData
): value is ConcurrentMerkleTreeHeaderData & { __kind: K } {
  return value.__kind === kind;
}
