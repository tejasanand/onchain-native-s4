use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::instruction::{deposit_funds, withdraw_funds};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum AccountInstruction {
    Deposit(u64),
    Withdraw(),
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = AccountInstruction::try_from_slice(input)?;
    match instruction {
        AccountInstruction::Deposit(amount) => deposit_funds(program_id, accounts, amount),
        AccountInstruction::Withdraw() => withdraw_funds(program_id, accounts),
    }
}