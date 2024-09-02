#![doc = include_str!("../README.md")]
#![allow(clippy::result_large_err)]

cfg_if::cfg_if! {
    if #[cfg(feature = "localnet")] {
        declare_id!("Bridge1p5gheXUvJ6jGWGeCsgPKgnE3YgdGKRVCMY9o");
    } else if #[cfg(feature = "mainnet")] {
        declare_id!("worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth");
    } else if #[cfg(feature = "testnet")] {
        declare_id!("3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5");
    }
}

mod constants;
pub use constants::{MAX_MESSAGE_PAYLOAD_SIZE, PROGRAM_EMITTER_SEED_PREFIX, SOLANA_CHAIN};

pub mod error;

pub mod legacy;

mod processor;
pub(crate) use processor::*;

pub mod sdk;

pub mod state;

pub mod types;

pub(crate) mod utils;

use anchor_lang::prelude::*;

#[program]
pub mod wormhole_core_bridge_solana {
    use super::*;

    /// Processor used to intialize a created account as [EncodedVaa](crate::state::EncodedVaa). An
    /// authority (the write authority) is established with this instruction.
    pub fn init_encoded_vaa(ctx: Context<InitEncodedVaa>) -> Result<()> {
        processor::init_encoded_vaa(ctx)
    }

    /// Processor used to close an [EncodedVaa](crate::state::EncodedVaa). This instruction requires
    /// an authority (the write authority) to interact witht he encoded VAA account.
    pub fn close_encoded_vaa(ctx: Context<CloseEncodedVaa>) -> Result<()> {
        processor::close_encoded_vaa(ctx)
    }

    /// Processor used to write to an [EncodedVaa](crate::state::EncodedVaa) account. This
    /// instruction requires an authority (the write authority) to interact with the encoded VAA
    /// account.
    pub fn write_encoded_vaa(
        ctx: Context<WriteEncodedVaa>,
        args: WriteEncodedVaaArgs,
    ) -> Result<()> {
        processor::write_encoded_vaa(ctx, args)
    }

    /// Processor used to verify an [EncodedVaa](crate::state::EncodedVaa) account as a version 1
    /// VAA (guardian signatures attesting to this observation). This instruction requires an
    /// authority (the write authority) to interact with the encoded VAA account.
    pub fn verify_encoded_vaa_v1(ctx: Context<VerifyEncodedVaaV1>) -> Result<()> {
        processor::verify_encoded_vaa_v1(ctx)
    }

    /// Processor used to close an [EncodedVaa](crate::state::EncodedVaa) account to create a
    /// [PostedMessageV1](crate::state::PostedMessageV1) account in its place.
    ///
    /// NOTE: Because the legacy verify signatures instruction was not required for the Posted VAA
    /// account to exist, the encoded [SignatureSet](crate::state::SignatureSet) is the default
    /// [Pubkey].
    pub fn post_vaa_v1(ctx: Context<PostVaaV1>) -> Result<()> {
        processor::post_vaa_v1(ctx)
    }

    /// Processor used to close a [SignatureSet](crate::state::SignatureSet), which was used to
    /// verify the VAA using the legacy parse and verify procedure.
    pub fn close_signature_set(ctx: Context<CloseSignatureSet>) -> Result<()> {
        processor::close_signature_set(ctx)
    }

    /// Process legacy Core Bridge instructions. See [legacy](crate::legacy) for more info.
    pub fn process_legacy_instruction(
        program_id: &Pubkey,
        account_infos: &[AccountInfo],
        ix_data: &[u8],
    ) -> Result<()> {
        legacy::process_legacy_instruction(program_id, account_infos, ix_data)
    }
}
