//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct ComputeScore {
    pub config: solana_program::pubkey::Pubkey,

    pub state_account: solana_program::pubkey::Pubkey,

    pub validator_history: solana_program::pubkey::Pubkey,

    pub validator_list: solana_program::pubkey::Pubkey,

    pub cluster_history: solana_program::pubkey::Pubkey,
}

impl ComputeScore {
    pub fn instruction(
        &self,
        args: ComputeScoreInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: ComputeScoreInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.state_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.validator_history,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.validator_list,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.cluster_history,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = ComputeScoreInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::STEWARD_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ComputeScoreInstructionData {
    discriminator: [u8; 8],
}

impl ComputeScoreInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [161, 101, 4, 93, 120, 62, 41, 20],
        }
    }
}

impl Default for ComputeScoreInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputeScoreInstructionArgs {
    pub validator_list_index: u64,
}

/// Instruction builder for `ComputeScore`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` state_account
///   2. `[]` validator_history
///   3. `[]` validator_list
///   4. `[]` cluster_history
#[derive(Clone, Debug, Default)]
pub struct ComputeScoreBuilder {
    config: Option<solana_program::pubkey::Pubkey>,
    state_account: Option<solana_program::pubkey::Pubkey>,
    validator_history: Option<solana_program::pubkey::Pubkey>,
    validator_list: Option<solana_program::pubkey::Pubkey>,
    cluster_history: Option<solana_program::pubkey::Pubkey>,
    validator_list_index: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ComputeScoreBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn state_account(&mut self, state_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state_account = Some(state_account);
        self
    }
    #[inline(always)]
    pub fn validator_history(
        &mut self,
        validator_history: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.validator_history = Some(validator_history);
        self
    }
    #[inline(always)]
    pub fn validator_list(&mut self, validator_list: solana_program::pubkey::Pubkey) -> &mut Self {
        self.validator_list = Some(validator_list);
        self
    }
    #[inline(always)]
    pub fn cluster_history(
        &mut self,
        cluster_history: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.cluster_history = Some(cluster_history);
        self
    }
    #[inline(always)]
    pub fn validator_list_index(&mut self, validator_list_index: u64) -> &mut Self {
        self.validator_list_index = Some(validator_list_index);
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
        let accounts = ComputeScore {
            config: self.config.expect("config is not set"),
            state_account: self.state_account.expect("state_account is not set"),
            validator_history: self
                .validator_history
                .expect("validator_history is not set"),
            validator_list: self.validator_list.expect("validator_list is not set"),
            cluster_history: self.cluster_history.expect("cluster_history is not set"),
        };
        let args = ComputeScoreInstructionArgs {
            validator_list_index: self
                .validator_list_index
                .clone()
                .expect("validator_list_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `compute_score` CPI accounts.
pub struct ComputeScoreCpiAccounts<'a, 'b> {
    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub state_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_history: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_list: &'b solana_program::account_info::AccountInfo<'a>,

    pub cluster_history: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `compute_score` CPI instruction.
pub struct ComputeScoreCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub state_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_history: &'b solana_program::account_info::AccountInfo<'a>,

    pub validator_list: &'b solana_program::account_info::AccountInfo<'a>,

    pub cluster_history: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: ComputeScoreInstructionArgs,
}

impl<'a, 'b> ComputeScoreCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ComputeScoreCpiAccounts<'a, 'b>,
        args: ComputeScoreInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            state_account: accounts.state_account,
            validator_history: accounts.validator_history,
            validator_list: accounts.validator_list,
            cluster_history: accounts.cluster_history,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.state_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.validator_history.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.validator_list.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.cluster_history.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = ComputeScoreInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STEWARD_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.state_account.clone());
        account_infos.push(self.validator_history.clone());
        account_infos.push(self.validator_list.clone());
        account_infos.push(self.cluster_history.clone());
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

/// Instruction builder for `ComputeScore` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` state_account
///   2. `[]` validator_history
///   3. `[]` validator_list
///   4. `[]` cluster_history
#[derive(Clone, Debug)]
pub struct ComputeScoreCpiBuilder<'a, 'b> {
    instruction: Box<ComputeScoreCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ComputeScoreCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ComputeScoreCpiBuilderInstruction {
            __program: program,
            config: None,
            state_account: None,
            validator_history: None,
            validator_list: None,
            cluster_history: None,
            validator_list_index: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn state_account(
        &mut self,
        state_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.state_account = Some(state_account);
        self
    }
    #[inline(always)]
    pub fn validator_history(
        &mut self,
        validator_history: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.validator_history = Some(validator_history);
        self
    }
    #[inline(always)]
    pub fn validator_list(
        &mut self,
        validator_list: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.validator_list = Some(validator_list);
        self
    }
    #[inline(always)]
    pub fn cluster_history(
        &mut self,
        cluster_history: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.cluster_history = Some(cluster_history);
        self
    }
    #[inline(always)]
    pub fn validator_list_index(&mut self, validator_list_index: u64) -> &mut Self {
        self.instruction.validator_list_index = Some(validator_list_index);
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
        let args = ComputeScoreInstructionArgs {
            validator_list_index: self
                .instruction
                .validator_list_index
                .clone()
                .expect("validator_list_index is not set"),
        };
        let instruction = ComputeScoreCpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            state_account: self
                .instruction
                .state_account
                .expect("state_account is not set"),

            validator_history: self
                .instruction
                .validator_history
                .expect("validator_history is not set"),

            validator_list: self
                .instruction
                .validator_list
                .expect("validator_list is not set"),

            cluster_history: self
                .instruction
                .cluster_history
                .expect("cluster_history is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ComputeScoreCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    state_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    validator_history: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    validator_list: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    cluster_history: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    validator_list_index: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}