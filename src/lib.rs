// Import necessary traits and modules
use borsh::{BorshDeserialize, BorshSerialize}; // Import traits for serialization and deserialization
use borsh_derive::{BorshDeserialize, BorshSerialize}; // Import macros for deriving serialization and deserialization
use solana_program::{ // Import modules from the Solana program library
    account_info::{next_account_info, AccountInfo}, // Import account info module for managing accounts
    entrypoint, // Import module for defining entry points to the program
    entrypoint::ProgramResult, // Import module for program result handling
    msg, // Import module for logging messages
    pubkey::Pubkey, // Import module for managing public keys
};

use crate::instructions::CounterInstructions; // Import custom instructions module

pub mod instructions; // Declare a submodule named "instructions"

// Define a struct representing a counter account
#[derive(Debug, BorshDeserialize, BorshSerialize)] // Derive traits for serialization and deserialization
pub struct CounterAccount {
    pub counter: u32, // Define a public field named "counter" of type u32
}

// Declare an entry point function for the program, which will handle instruction processing
entrypoint!(process_instruction);

// Define the process_instruction function, which executes instructions received by the program
pub fn process_instruction(
    _program_id: &Pubkey,                     // The program's public key (unused)
    accounts: &[AccountInfo],                 // Array of accounts involved in the transaction
    instructions_data: &[u8],                 // Binary data containing instructions for the program
) -> ProgramResult {                         // Return type indicating success or failure of the program

    // Log a message indicating the entry point of the counter program
    msg!("Counter program entry point");

    // Decode the instruction data into a CounterInstructions enum variant
    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;

    // Create an iterator over the accounts array to access each account
    let accounts_iter = &mut accounts.iter();
    
    // Retrieve the next account from the iterator
    let account = next_account_info(accounts_iter)?;

    // Deserialize the account data into a CounterAccount struct
    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    // Match the decoded instruction to perform corresponding actions
    match instruction {
        // If the instruction is Increment, increase the counter by 1
        CounterInstructions::Increment(args) => {
            counter_account.counter += args.value;
        }
        // If the instruction is Decrement, decrease the counter by 1
        CounterInstructions::Decrement(args) => {
            counter_account.counter -= args.value;
        }
        // If the instruction is Reset, set the counter to 0
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        // If the instruction is Update, set the counter to the provided value
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    // Serialize the updated counter_account back into the account data
    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    
    // Return Ok(()) to indicate successful execution of the instruction
    Ok(())
}


// This module contains tests for the counter program.
#[cfg(test)]
mod test {
    // Import all items from the parent module into the current scope.
    use super::*;
    // Import specific items from the solana_program crate.
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    // Import the mem module from the standard library for memory manipulation.
    use std::mem;

    // This function is a test function for the counter program.
    #[test]
    fn test_counter() {
        // Initialize the program ID with the default value.
        let program_id = Pubkey::default();
        // Initialize the key with the default value.
        let key = Pubkey::default();
        // Initialize the amount of lamports to 0.
        let mut lamports = 0;
        // Initialize the data vector with 0s, its length is the size of u32.
        let mut data = vec![0; mem::size_of::<u32>()];
        // Initialize the owner with the default value.
        let owner = Pubkey::default();

        // Create an AccountInfo object with the initialized values.
        let account = AccountInfo::new(
            &key, // Account public key
            false, // Is signer
            true, // Is writable
            &mut lamports, // Reference to lamports
            &mut data, // Reference to data
            &owner, // Owner public key
            false, // Is executable
            Epoch::default(), // Current epoch
        );

        // Create a vector containing the account created above.
        let accounts = vec![account];

        // Initialize instruction data vectors for different operations.
        let increment_instruction_data: Vec<u8> = vec![0];
        let decrement_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data: Vec<u8> = vec![2];
        let reset_instruction_data: Vec<u8> = vec![3];

        // Execute the process_instruction function with the increment instruction data.
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();
        // Assert that the counter value in the account data has been incremented to 1.
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );

        // Execute the process_instruction function with the decrement instruction data.
        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();
        // Assert that the counter value in the account data has been decremented to 0.
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        // Define a value to update the counter with.
        let update_value = 33u32;
        // Extend the update instruction data with the binary representation of the update value.
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());

        // Execute the process_instruction function with the update instruction data.
        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();
        // Assert that the counter value in the account data has been updated to 33.
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            33
        );

        // Execute the process_instruction function with the reset instruction data.
        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();
        // Assert that the counter value in the account data has been reset to 0.
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
