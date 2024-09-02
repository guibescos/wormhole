//! Legacy Core Bridge state and instruction processing.

pub use crate::ID;

pub mod accounts;

pub mod instruction;

pub mod state;

pub(crate) mod utils;

/// Collection of methods to interact with the Core Bridge program via CPI. The structs defined in
/// this module mirror the structs deriving [Accounts](anchor_lang::prelude::Accounts), where each
/// field is an [AccountInfo]. **Integrators: Please use [sdk](crate::sdk) instead of this module.**
///
/// NOTE: This is similar to how [cpi](mod@crate::cpi) is generated via Anchor's
/// [program][anchor_lang::prelude::program] macro.
#[cfg(feature = "cpi")]
pub mod cpi {
    use anchor_lang::prelude::*;
    use solana_program::program::invoke_signed;

    use super::*;

    /// Context to post a new Core Bridge message.
    #[derive(Accounts)]
    pub struct PostMessage<'info> {
        /// Core Bridge config account (mut).
        ///
        /// Seeds = \["Bridge"\], seeds::program = core_bridge_program.
        ///
        /// CHECK: This account is used to determine how many lamports to transfer for Wormhole fee.
        pub config: AccountInfo<'info>,

        /// Core Bridge Message (mut).
        ///
        /// CHECK: This message will be created if it does not exist.
        pub message: AccountInfo<'info>,

        /// Core Bridge Emitter (optional, read-only signer).
        ///
        /// CHECK: This account pubkey will be used as the emitter address. This account is required
        /// if the message account has not been prepared beforehand.
        pub emitter: Option<AccountInfo<'info>>,

        /// Core Bridge Emitter Sequence (mut).
        ///
        /// Seeds = \["Sequence", emitter.key\], seeds::program = core_bridge_program.
        ///
        /// CHECK: This account is used to determine the sequence of the Wormhole message for the
        /// provided emitter.
        pub emitter_sequence: AccountInfo<'info>,

        /// Payer (mut signer).
        ///
        /// CHECK: This account pays for new accounts created and pays for the Wormhole fee.
        pub payer: AccountInfo<'info>,

        /// Core Bridge Fee Collector (optional, read-only).
        ///
        /// Seeds = \["fee_collector"\], seeds::program = core_bridge_program.
        ///
        /// CHECK: This account is used to collect fees.
        pub fee_collector: Option<AccountInfo<'info>>,

        /// System Program.
        ///
        /// CHECK: Required to create accounts and transfer lamports to the fee collector.
        pub system_program: AccountInfo<'info>,
    }

    /// Context to post a new or reuse an existing Core Bridge message.
    #[derive(Accounts)]
    pub struct PostMessageUnreliable<'info> {
        /// Core Bridge config account (mut).
        ///
        /// seeds = \["Bridge"\], seeds::program = core_bridge_program
        ///
        /// CHECK: This account is used to determine how many lamports to transfer for Wormhole fee.
        pub config: AccountInfo<'info>,

        /// Core Bridge Message (mut).
        ///
        /// CHECK: This message will be created if it does not exist.
        pub message: AccountInfo<'info>,

        /// Core Bridge Emitter (read-only signer).
        ///
        /// CHECK: This account pubkey will be used as the emitter address.
        pub emitter: AccountInfo<'info>,

        /// Core Bridge Emitter Sequence (mut).
        ///
        /// Seeds = \["Sequence", emitter.key\], seeds::program = core_bridge_program.
        ///
        /// CHECK: This account is used to determine the sequence of the Wormhole message for the
        /// provided emitter.
        pub emitter_sequence: AccountInfo<'info>,

        /// Payer (mut signer).
        ///
        /// CHECK: This account pays for new accounts created and pays for the Wormhole fee.
        pub payer: AccountInfo<'info>,

        /// Core Bridge Fee Collector (optional, read-only).
        ///
        /// Seeds = \["fee_collector"\], seeds::program = core_bridge_program.
        ///
        /// CHECK: This account is used to collect fees.
        pub fee_collector: Option<AccountInfo<'info>>,

        /// System Program.
        ///
        /// CHECK: Required to create accounts and transfer lamports to the fee collector.
        pub system_program: AccountInfo<'info>,
    }
}
