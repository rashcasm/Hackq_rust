use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
     account_info::{next_account_info, AccountInfo},
     entrypoint,
     entrypoint::ProgramResult,
     msg,
     program_error::ProgramError,
     pubkey::Pubkey,
};

/// Define the structure of the data account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
     pub count: u32,
}

//Define program entry point function
entrypoint!(process_instruction);

pub fn process_instruction(
		 //Program ID, that is, program address
     program_id: &Pubkey,
		 //The set of accounts involved in this command
     accounts: &[AccountInfo],
		 // command data
     _instruction_data: &[u8],
) -> ProgramResult {
     msg!("Hello World Rust program entrypoint");

     //Account iterator
     let accounts_iter = &mut accounts.iter();

     // Get the caller account
     let account = next_account_info(accounts_iter)?;

     //Verify caller identity
     if account.owner != program_id {
         msg!("Counter account does not have the correct program id");
         return Err(ProgramError::IncorrectProgramId);
     }

     // Read and write new values
     let mut counter = CounterAccount::try_from_slice(&account.data.borrow())?;
     counter.count += 1;
     counter.serialize(&mut *account.data.borrow_mut())?;

     Ok(())
}
