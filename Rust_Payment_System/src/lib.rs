use solana_sdk::*; 

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};

// struct Payload<'a>{
//     pub payer: &'a mut PubkeyData,
//     pub to: &'a mut PubkeyData,
//     pub payload_body: &'a mut [u8],

// }

// declare and export the program's entrypoint
entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {

    msg!("Solana payload program");

    // Create an iterator to safely reference accounts in the slice
    let account_info_iter = &mut accounts.iter();

    // As part of the program specification the first account is the source
    // account and the second is the destination account
    let sender = next_account_info(account_info_iter)?;
    let reciever = next_account_info(account_info_iter)?;

    if !sender.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    let message = String::from_utf8(instruction_data.to_vec())..map_err(|_| ProgramError::InvalidInstructionData)?;

    msg!("Message: {:?}", message);

    reciever.data.borrow_mut().copy_from_slice(&instruction_data);
    Ok(())
}