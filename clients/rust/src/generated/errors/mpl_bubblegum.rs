//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum MplBubblegumError {
    /// 6000 (0x1770) - Asset Owner Does not match
    #[error("Asset Owner Does not match")]
    AssetOwnerMismatch,
    /// 6001 (0x1771) - PublicKeyMismatch
    #[error("PublicKeyMismatch")]
    PublicKeyMismatch,
    /// 6002 (0x1772) - Hashing Mismatch Within Leaf Schema
    #[error("Hashing Mismatch Within Leaf Schema")]
    HashingMismatch,
    /// 6003 (0x1773) - Unsupported Schema Version
    #[error("Unsupported Schema Version")]
    UnsupportedSchemaVersion,
    /// 6004 (0x1774) - Creator shares must sum to 100
    #[error("Creator shares must sum to 100")]
    CreatorShareTotalMustBe100,
    /// 6005 (0x1775) - No duplicate creator addresses in metadata
    #[error("No duplicate creator addresses in metadata")]
    DuplicateCreatorAddress,
    /// 6006 (0x1776) - Creator did not verify the metadata
    #[error("Creator did not verify the metadata")]
    CreatorDidNotVerify,
    /// 6007 (0x1777) - Creator not found in creator Vec
    #[error("Creator not found in creator Vec")]
    CreatorNotFound,
    /// 6008 (0x1778) - No creators in creator Vec
    #[error("No creators in creator Vec")]
    NoCreatorsPresent,
    /// 6009 (0x1779) - User-provided creator Vec must result in same user-provided creator hash
    #[error("User-provided creator Vec must result in same user-provided creator hash")]
    CreatorHashMismatch,
    /// 6010 (0x177A) - User-provided metadata must result in same user-provided data hash
    #[error("User-provided metadata must result in same user-provided data hash")]
    DataHashMismatch,
    /// 6011 (0x177B) - Creators list too long
    #[error("Creators list too long")]
    CreatorsTooLong,
    /// 6012 (0x177C) - Name in metadata is too long
    #[error("Name in metadata is too long")]
    MetadataNameTooLong,
    /// 6013 (0x177D) - Symbol in metadata is too long
    #[error("Symbol in metadata is too long")]
    MetadataSymbolTooLong,
    /// 6014 (0x177E) - Uri in metadata is too long
    #[error("Uri in metadata is too long")]
    MetadataUriTooLong,
    /// 6015 (0x177F) - Basis points in metadata cannot exceed 10000
    #[error("Basis points in metadata cannot exceed 10000")]
    MetadataBasisPointsTooHigh,
    /// 6016 (0x1780) - Tree creator or tree delegate must sign.
    #[error("Tree creator or tree delegate must sign.")]
    TreeAuthorityIncorrect,
    /// 6017 (0x1781) - Not enough unapproved mints left
    #[error("Not enough unapproved mints left")]
    InsufficientMintCapacity,
    /// 6018 (0x1782) - NumericalOverflowError
    #[error("NumericalOverflowError")]
    NumericalOverflowError,
    /// 6019 (0x1783) - Incorrect account owner
    #[error("Incorrect account owner")]
    IncorrectOwner,
    /// 6020 (0x1784) - Cannot Verify Collection in this Instruction
    #[error("Cannot Verify Collection in this Instruction")]
    CollectionCannotBeVerifiedInThisInstruction,
    /// 6021 (0x1785) - Collection Not Found on Metadata
    #[error("Collection Not Found on Metadata")]
    CollectionNotFound,
    /// 6022 (0x1786) - Collection item is already verified.
    #[error("Collection item is already verified.")]
    AlreadyVerified,
    /// 6023 (0x1787) - Collection item is already unverified.
    #[error("Collection item is already unverified.")]
    AlreadyUnverified,
    /// 6024 (0x1788) - Incorrect leaf metadata update authority.
    #[error("Incorrect leaf metadata update authority.")]
    UpdateAuthorityIncorrect,
    /// 6025 (0x1789) - This transaction must be signed by either the leaf owner or leaf delegate
    #[error("This transaction must be signed by either the leaf owner or leaf delegate")]
    LeafAuthorityMustSign,
    /// 6026 (0x178A) - Collection Not Compatable with Compression, Must be Sized
    #[error("Collection Not Compatable with Compression, Must be Sized")]
    CollectionMustBeSized,
    /// 6027 (0x178B) - Metadata mint does not match collection mint
    #[error("Metadata mint does not match collection mint")]
    MetadataMintMismatch,
    /// 6028 (0x178C) - Invalid collection authority
    #[error("Invalid collection authority")]
    InvalidCollectionAuthority,
    /// 6029 (0x178D) - Invalid delegate record pda derivation
    #[error("Invalid delegate record pda derivation")]
    InvalidDelegateRecord,
    /// 6030 (0x178E) - Edition account doesnt match collection
    #[error("Edition account doesnt match collection")]
    CollectionMasterEditionAccountInvalid,
    /// 6031 (0x178F) - Collection Must Be a Unique Master Edition v2
    #[error("Collection Must Be a Unique Master Edition v2")]
    CollectionMustBeAUniqueMasterEdition,
    /// 6032 (0x1790) - Could not convert external error to BubblegumError
    #[error("Could not convert external error to BubblegumError")]
    UnknownExternalError,
    /// 6033 (0x1791) - Decompression is disabled for this tree.
    #[error("Decompression is disabled for this tree.")]
    DecompressionDisabled,
    /// 6034 (0x1792) - Missing collection mint account
    #[error("Missing collection mint account")]
    MissingCollectionMintAccount,
    /// 6035 (0x1793) - Missing collection metadata account
    #[error("Missing collection metadata account")]
    MissingCollectionMetadataAccount,
    /// 6036 (0x1794) - Collection mismatch
    #[error("Collection mismatch")]
    CollectionMismatch,
    /// 6037 (0x1795) - Metadata not mutable
    #[error("Metadata not mutable")]
    MetadataImmutable,
    /// 6038 (0x1796) - Can only update primary sale to true
    #[error("Can only update primary sale to true")]
    PrimarySaleCanOnlyBeFlippedToTrue,
    /// 6039 (0x1797) - Creator did not unverify the metadata
    #[error("Creator did not unverify the metadata")]
    CreatorDidNotUnverify,
    /// 6040 (0x1798) - Only NonFungible standard is supported
    #[error("Only NonFungible standard is supported")]
    InvalidTokenStandard,
    /// 6041 (0x1799) - Canopy size should be set bigger for this tree
    #[error("Canopy size should be set bigger for this tree")]
    InvalidCanopySize,
    /// 6042 (0x179A) - Invalid log wrapper program
    #[error("Invalid log wrapper program")]
    InvalidLogWrapper,
    /// 6043 (0x179B) - Invalid compression program
    #[error("Invalid compression program")]
    InvalidCompressionProgram,
}

impl solana_program::program_error::PrintProgramError for MplBubblegumError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}
