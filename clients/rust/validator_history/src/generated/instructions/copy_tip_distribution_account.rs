//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CopyTipDistributionAccount {
    pub validator_history_account: solana_program::pubkey::Pubkey,
    /// Used to read validator commission.
    pub vote_account: solana_program::pubkey::Pubkey,

    pub config: solana_program::pubkey::Pubkey,
    /// `owner = config.tip_distribution_program.key()` here is sufficient.
    pub tip_distribution_account: solana_program::pubkey::Pubkey,

    pub signer: solana_program::pubkey::Pubkey,
}

impl CopyTipDistributionAccount {
    pub fn instruction(
        &self,
        args: CopyTipDistributionAccountInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CopyTipDistributionAccountInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.validator_history_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vote_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.tip_distribution_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.signer,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CopyTipDistributionAccountInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::VALIDATOR_HISTORY_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CopyTipDistributionAccountInstructionData {
    discriminator: [u8; 8],
}

impl CopyTipDistributionAccountInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [208, 213, 185, 210, 103, 124, 128, 173],
        }
    }
}

impl Default for CopyTipDistributionAccountInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CopyTipDistributionAccountInstructionArgs {
    pub epoch: u64,
}

/// Instruction builder for `CopyTipDistributionAccount`.
///
/// ### Accounts:
///
///   0. `[writable]` validator_history_account
///   1. `[]` vote_account
///   2. `[]` config
///   3. `[]` tip_distribution_account
///   4. `[writable, signer]` signer
#[derive(Clone, Debug, Default)]
pub struct CopyTipDistributionAccountBuilder {
    validator_history_account: Option<solana_program::pubkey::Pubkey>,
    vote_account: Option<solana_program::pubkey::Pubkey>,
    config: Option<solana_program::pubkey::Pubkey>,
    tip_distribution_account: Option<solana_program::pubkey::Pubkey>,
    signer: Option<solana_program::pubkey::Pubkey>,
    epoch: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CopyTipDistributionAccountBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn validator_history_account(
        &mut self,
        validator_history_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.validator_history_account = Some(validator_history_account);
        self
    }
    /// Used to read validator commission.
    #[inline(always)]
    pub fn vote_account(&mut self, vote_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vote_account = Some(vote_account);
        self
    }
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    /// `owner = config.tip_distribution_program.key()` here is sufficient.
    #[inline(always)]
    pub fn tip_distribution_account(
        &mut self,
        tip_distribution_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.tip_distribution_account = Some(tip_distribution_account);
        self
    }
    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }
    #[inline(always)]
    pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.epoch = Some(epoch);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = CopyTipDistributionAccount {
            validator_history_account: self
                .validator_history_account
                .expect("validator_history_account is not set"),
            vote_account: self.vote_account.expect("vote_account is not set"),
            config: self.config.expect("config is not set"),
            tip_distribution_account: self
                .tip_distribution_account
                .expect("tip_distribution_account is not set"),
            signer: self.signer.expect("signer is not set"),
        };
        let args = CopyTipDistributionAccountInstructionArgs {
            epoch: self.epoch.clone().expect("epoch is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `copy_tip_distribution_account` CPI accounts.
pub struct CopyTipDistributionAccountCpiAccounts<'a, 'b> {
    pub validator_history_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Used to read validator commission.
    pub vote_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// `owner = config.tip_distribution_program.key()` here is sufficient.
    pub tip_distribution_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `copy_tip_distribution_account` CPI instruction.
pub struct CopyTipDistributionAccountCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_history_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Used to read validator commission.
    pub vote_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// `owner = config.tip_distribution_program.key()` here is sufficient.
    pub tip_distribution_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CopyTipDistributionAccountInstructionArgs,
}

impl<'a, 'b> CopyTipDistributionAccountCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CopyTipDistributionAccountCpiAccounts<'a, 'b>,
        args: CopyTipDistributionAccountInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            validator_history_account: accounts.validator_history_account,
            vote_account: accounts.vote_account,
            config: accounts.config,
            tip_distribution_account: accounts.tip_distribution_account,
            signer: accounts.signer,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.validator_history_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vote_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.tip_distribution_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.signer.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CopyTipDistributionAccountInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::VALIDATOR_HISTORY_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.validator_history_account.clone());
        account_infos.push(self.vote_account.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.tip_distribution_account.clone());
        account_infos.push(self.signer.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `CopyTipDistributionAccount` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` validator_history_account
///   1. `[]` vote_account
///   2. `[]` config
///   3. `[]` tip_distribution_account
///   4. `[writable, signer]` signer
#[derive(Clone, Debug)]
pub struct CopyTipDistributionAccountCpiBuilder<'a, 'b> {
    instruction: Box<CopyTipDistributionAccountCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CopyTipDistributionAccountCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CopyTipDistributionAccountCpiBuilderInstruction {
            __program: program,
            validator_history_account: None,
            vote_account: None,
            config: None,
            tip_distribution_account: None,
            signer: None,
            epoch: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn validator_history_account(
        &mut self,
        validator_history_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.validator_history_account = Some(validator_history_account);
        self
    }
    /// Used to read validator commission.
    #[inline(always)]
    pub fn vote_account(
        &mut self,
        vote_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vote_account = Some(vote_account);
        self
    }
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }
    /// `owner = config.tip_distribution_program.key()` here is sufficient.
    #[inline(always)]
    pub fn tip_distribution_account(
        &mut self,
        tip_distribution_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tip_distribution_account = Some(tip_distribution_account);
        self
    }
    #[inline(always)]
    pub fn signer(
        &mut self,
        signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.signer = Some(signer);
        self
    }
    #[inline(always)]
    pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.instruction.epoch = Some(epoch);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = CopyTipDistributionAccountInstructionArgs {
            epoch: self.instruction.epoch.clone().expect("epoch is not set"),
        };
        let instruction = CopyTipDistributionAccountCpi {
            __program: self.instruction.__program,

            validator_history_account: self
                .instruction
                .validator_history_account
                .expect("validator_history_account is not set"),

            vote_account: self
                .instruction
                .vote_account
                .expect("vote_account is not set"),

            config: self.instruction.config.expect("config is not set"),

            tip_distribution_account: self
                .instruction
                .tip_distribution_account
                .expect("tip_distribution_account is not set"),

            signer: self.instruction.signer.expect("signer is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CopyTipDistributionAccountCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    validator_history_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vote_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    tip_distribution_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    epoch: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}