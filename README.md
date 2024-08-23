# Blockchain Simulation with Rust and Tokio

This project is a simulation of blockchain concepts and Ethereum-like node behaviors using Rust and Tokio. It consists of multiple exercises demonstrating various aspects of blockchain, such as chain resolution, proof-of-work mining, transaction processing, and network behavior.

## Project Overview

The project is split into a series of exercises. Each exercise focuses on a specific concept or feature that is relevant to blockchain development. The simulation demonstrates how nodes propagate blocks, handle forks, and verify proof-of-work.

### How to Run

1. Ensure you have Rust installed. If not, you can install it [here](https://rustup.rs/).
2. Clone this repository.
3. Run the project using the following command:

   ```bash
   cargo run
   ```

### Dependencies

- **Tokio:** For asynchronous execution, including handling time delays and network behavior.
- **Rand:** For generating random numbers to simulate unpredictable network conditions and random data generation.
- **SHA2:** For implementing SHA-256 hashing in proof-of-work mining.
- **Hex:** For encoding hashed values as hexadecimal strings.

## Exercises

### Exercise 1: Chain Resolution Check

**Purpose:** Determine if the blockchain is fully resolved. A resolved chain should have exactly one leaf block (a block with no children). This exercise shows how to identify whether a chain is fragmented or fully resolved.

**Key Concepts:** Chain resolution, leaf nodes, HashSet operations.

### Exercise 2: Random Argument Generation for Ethereum Testing

**Purpose:** Simulate the generation of random arguments that could be passed to smart contracts in Ethereum. The generated arguments include primitive types (like integers) as well as nested structures (like structs and vectors). This is useful for stress-testing smart contracts.

**Key Concepts:** Random data generation, nested structures, depth-limited recursion.

### Exercise 3: Proof of Work Verification

**Purpose:** Simulate the mining of blocks using proof-of-work. The exercise implements a basic mining loop that finds a hash with a required number of leading zeros (difficulty). It also verifies whether a given block’s hash satisfies the proof-of-work conditions.

**Key Concepts:** Proof-of-work, mining loop, SHA-256 hashing.

### Exercise 4: Chain Selection Algorithm

**Purpose:** Implement a chain selection algorithm where the chain’s head (most recent block) is updated based on cumulative difficulty and a custom edit score. The selection criteria determine which fork the node follows.

**Key Concepts:** Chain head selection, cumulative difficulty, fork choice.

### Exercise 5: Simulating Ethereum Node Behavior

**Purpose:** Simulate block propagation in a network of nodes. Each node propagates blocks to its peers with a simulated delay based on network conditions and the gas used by the block.

**Key Concepts:** Asynchronous block propagation, network delays, peer-to-peer communication.

### Exercise 6: Implementing a Simple Verifier Trait

**Purpose:** Create a generic verification interface for different validators, such as verifying Ethereum addresses and signatures. This is useful in blockchain systems for validating input data.

**Key Concepts:** Traits, dynamic dispatch, verification logic.

### Exercise 7: Broadcasting Messages in a Network of Nodes

**Purpose:** Simulate broadcasting a message across a network of nodes, with each node receiving the message after a random delay. This models message propagation in a decentralized network.

**Key Concepts:** Message broadcasting, asynchronous message handling, peer networks.

### Exercise 8: Simulating Forks and Chain Reorganizations

**Purpose:** Simulate a scenario where multiple nodes receive and add a new block to their chains. This exercise models chain reorganizations when a new fork is introduced.

**Key Concepts:** Forks, chain reorganization, block propagation.

### Exercise 9: Simulating the Effect of Block Size on Forking

**Purpose:** Introduce network delays when propagating blocks, showing how block size and delay impact the likelihood of forks in the network.

**Key Concepts:** Network latency, block size, fork probability.

### Exercise 10: Implementing Conditional Traits for Gas Calculation (EIP-1559)

**Purpose:** Implement different gas calculation strategies for pre- and post-London Ethereum upgrades. This demonstrates how traits can be used to handle protocol changes over time.

**Key Concepts:** Gas calculation, trait-based conditional logic, EIP-1559.

### Exercise 11: Chain Edit Score and Fork Choice Algorithm

**Purpose:** Implement an edit score calculation to determine the fork choice in reorganization scenarios. The edit score tracks how far a block deviates from the main chain, influencing which fork is preferred.

**Key Concepts:** Fork choice, chain scoring, edit score calculation.

### Exercise 12: Handling Multiple Consumers in a Blockchain System

**Purpose:** Implement a transaction pool and simulate multiple worker nodes processing transactions in parallel. This models how transactions are handled in a decentralized system with multiple consumers.

**Key Concepts:** Transaction pools, asynchronous task scheduling, broadcast channels.

## Project Structure

- **Main Simulation:** The `main` function demonstrates how all the components work together in a simulated environment.
- **Node Behavior:** Each node maintains its own blockchain, propagates blocks to peers, and processes transactions.
- **Blockchain Data Structures:** The `Block` and `Transaction` structs represent the core data in the system.
- **Network Simulation:** Nodes communicate with each other asynchronously, simulating real-world network conditions.

## Conclusion

This project showcases how fundamental blockchain concepts can be implemented and simulated in Rust. Each exercise builds on the previous one, introducing more complex concepts and behaviors typically found in decentralized systems like Ethereum. The exercises provide a hands-on way to learn about blockchain architecture, consensus algorithms, and network behaviors.

For any contributions, suggestions, or improvements, feel free to submit a pull request or open an issue. Happy coding!

