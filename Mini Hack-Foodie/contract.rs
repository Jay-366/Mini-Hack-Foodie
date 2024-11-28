use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Get the accounts needed
    let account = next_account_info(accounts_iter)?;
    if !account.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Parse the instruction data
    match instruction_data[0] {
        0 => {
            // Mint tokens
            msg!("Minting tokens");
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            let mut balance = account.try_borrow_mut_data()?;
            let current_balance = u64::from_le_bytes(balance[..8].try_into().unwrap());
            let new_balance = current_balance + amount;
            balance[..8].copy_from_slice(&new_balance.to_le_bytes());
        }
        1 => {
            // Transfer tokens
            msg!("Transferring tokens");
            let recipient_account = next_account_info(accounts_iter)?;
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());

            // Deduct from sender
            let mut sender_balance = account.try_borrow_mut_data()?;
            let sender_current_balance = u64::from_le_bytes(sender_balance[..8].try_into().unwrap());
            if sender_current_balance < amount {
                msg!("Insufficient funds");
                return Err(ProgramError::InsufficientFunds);
            }
            let sender_new_balance = sender_current_balance - amount;
            sender_balance[..8].copy_from_slice(&sender_new_balance.to_le_bytes());

            // Add to recipient
            let mut recipient_balance = recipient_account.try_borrow_mut_data()?;
            let recipient_current_balance =
                u64::from_le_bytes(recipient_balance[..8].try_into().unwrap());
            let recipient_new_balance = recipient_current_balance + amount;
            recipient_balance[..8].copy_from_slice(&recipient_new_balance.to_le_bytes());
        }
        _ => {
            msg!("Invalid instruction");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}
