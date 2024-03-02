# solcounter
The Solana Counter Program is a smart contract designed to manage a simple counter on the Solana blockchain. It allows users to increment, decrement, reset, and update the counter value. The program is written in Rust and utilizes Solana's programming framework.

The core functionality of the program includes:

1.Defining a counter account struct to hold the counter value. 

2.Implementing instructions for incrementing, decrementing, resetting, and updating the counter.

3.Handling transactions by decoding instructions, accessing accounts involved, and executing corresponding actions.

4.Providing tests to ensure the correctness of the program's functionality.


The project consists of two main files:

1.lib.rs: Contains the implementation of the counter program, including the entry point function for processing instructions, defining the counter account struct, and handling various instructions.
2.test.rs: Includes unit tests for verifying the functionality of the counter program. It initializes test data, executes instructions, and asserts the expected counter values after each operation.
The project utilizes serialization and deserialization traits from the Borsh crate for data management and follows Solana's conventions for handling program entry points, account information, and program results
