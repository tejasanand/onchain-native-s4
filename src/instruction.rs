use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult,
    program::invoke, program_error::ProgramError, pubkey::Pubkey, system_instruction,
};

pub fn deposit_funds(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let target_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    invoke(
        &system_instruction::transfer(payer.key, target_account.key, amount),
        &[
            payer.clone(),
            target_account.clone(),
            system_program.clone(),
        ],
    )?;

    let mut account_data = target_account.try_borrow_mut_data()?;
    let mut total_amount = u64::from_le_bytes(account_data[..8].try_into().unwrap());
    total_amount += amount;
    account_data[..8].copy_from_slice(&total_amount.to_le_bytes());

    Ok(())
}

pub fn withdraw_funds(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_data = target_account.try_borrow_mut_data()?;
    let total_amount = u64::from_le_bytes(account_data[..8].try_into().unwrap());
    let withdrawal_amount = total_amount / 10;

    if withdrawal_amount == 0 {
        return Err(ProgramError::InsufficientFunds);
    }

    invoke(
        &system_instruction::transfer(target_account.key, recipient.key, withdrawal_amount),
        &[
            target_account.clone(),
            recipient.clone(),
            system_program.clone(),
        ],
    )?;

    let remaining_balance = total_amount - withdrawal_amount;
    // account_data[..8].copy_from_slice(&remaining_balance.to_le_bytes());

    Ok(())
}