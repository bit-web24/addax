# addax

## Introduction

The Addax Blockchain is a decentralized digital currency network designed for exchanging DAX coins. Each DAX coin represents 1 rupee and is represented as a whole number. The blockchain ensures secure and transparent transactions between participants without relying on a central authority.

## Core Components

### Block

A block is a fundamental component of the blockchain. It consists of the following elements:

    Index: A unique identifier for each block in the chain.
    Timestamp: The time at which the block was created.
    Transactions: A list of transactions contained within the block.
    Previous Hash: The hash of the previous block in the chain.
    Hash: The hash value representing the current block's data.

### Transaction

A transaction represents the transfer of DAX coins between participants. It includes the following details:

    Sender: The sender's address or identifier.
    Receiver: The recipient's address or identifier.
    Amount: The number of DAX coins being transferred (represented as a whole number).
    Transaction ID: A unique identifier for the transaction.
    Timestamp: The time at which the transaction was created.
    Additional Data: Any additional information associated with the transaction.

### Blockchain

The blockchain is the ledger that maintains a sequential chain of blocks. It facilitates transaction validation and ensures the integrity of the network. Key features of the blockchain include:

    Block Validation: Each block is validated to ensure its integrity by verifying the previous block's hash.
    Consensus: Miners reach consensus through a consensus algorithm to agree on the order of blocks and maintain a consistent blockchain.
    Transaction Verification: Transactions are validated to ensure they comply with predefined rules and contain valid data.
    Network Communication: Participants communicate with each other to propagate blocks, transactions, and reach consensus.
    Security: The blockchain employs cryptographic techniques, such as hashing and digital signatures, to secure transactions and prevent tampering.

## Implementation Details

The DAX Coin Blockchain is implemented in Rust. It leverages sha2, hex, serde, tokio for cryptographic operations and network communication.

### Key implementation details:

    Block Structure: Each block consists of an index, timestamp, transactions, previous hash, and hash.
    Transaction Structure: Transactions contain sender, receiver, amount, transaction ID, timestamp, and additional data.
    Validation: Blocks and transactions are validated to ensure integrity and compliance with predefined rules.
    Consensus Algorithm: [Specify the consensus algorithm used, such as Proof of Work (PoW) or Proof of Stake (PoS)].
    Network Communication: Participants communicate through [specify network protocol or mechanism].
    Wallets: Participants hold wallets to store their DAX coins and perform transactions.
    Transaction Verification: Transactions are verified to ensure sufficient funds, valid sender and receiver, and accurate amounts.
