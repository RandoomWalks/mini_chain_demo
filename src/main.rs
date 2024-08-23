use hex; // Hex encoding for byte array conversion
use rand::Rng; // Random number generation utilities
use sha2::{Digest, Sha256}; // Cryptographic hashing using SHA-256
use std::cmp::Ordering; // Utility for comparison between values
use std::collections::{HashMap, HashSet}; // HashMap and HashSet for data storage
use std::sync::Arc; // Thread-safe shared reference counting
use tokio::sync::{broadcast, mpsc, Mutex}; // Asynchronous synchronization primitives
use tokio::time::{sleep, Duration}; // Time utilities for delays

// Common structures used across multiple exercises

#[derive(Clone, Debug)]
struct Block {
    hash: String, // The hash of the block, used as an identifier
    parent_hash: Option<String>, // Hash of the parent block (None for genesis)
    number: u64, // Block number in the chain (0 for genesis)
    cumulative_difficulty: u64, // Total difficulty up to this block (used for fork choice)
    gas_used: u64, // Amount of gas consumed by transactions in this block
    edit_score: u64, // Custom scoring for chain reorganization purposes
}

#[derive(Clone, Debug)]
struct Transaction {
    id: u64, // Unique transaction ID
    from: String, // Sender address
    to: String, // Recipient address
    amount: u64, // Amount to be transferred
    nonce: u64, // Transaction nonce to ensure order and prevent replay attacks
}

struct Node {
    id: u64, // Unique identifier for the node
    chain: HashMap<String, Block>, // Local blockchain (map from block hash to block data)
    head: String, // Hash of the current head (most recent block)
    peers: Vec<u64>, // List of peer node IDs connected to this node
}

// Exercise 1: Chain Resolution Check
// This function checks if there is exactly one leaf node in the blockchain
// (i.e., a block with no child), indicating a fully resolved chain.
impl Node {
    fn is_resolved(&self) -> bool {
        let mut leaves = HashSet::new(); // Set to track potential leaf nodes
        for block in self.chain.values() {
            leaves.insert(block.hash.clone()); // Start by assuming all blocks are leaves
        }
        for block in self.chain.values() {
            if let Some(parent) = &block.parent_hash {
                leaves.remove(parent); // Remove blocks that are parents (i.e., not leaves)
            }
        }
        leaves.len() == 1 // The chain is resolved if only one leaf remains
    }
}

// Exercise 2: Random Argument Generation for Ethereum Testing
// This generates random arguments to simulate test data for Ethereum smart contract calls.
#[derive(Clone, Debug)]
enum Argument {
    U8(u8), // 8-bit unsigned integer
    U256([u8; 32]), // 256-bit unsigned integer (common in Ethereum)
    Address([u8; 20]), // Ethereum address (20 bytes)
    Vector(Vec<Argument>), // Nested vector (array) of arguments
    Struct(Vec<(String, Argument)>), // Nested struct (map of named arguments)
    Signer(Box<Argument>), // Argument representing a signing key or address
}

// Recursive generation of random arguments with depth control
impl Argument {
    fn random<R: Rng>(rng: &mut R, max_depth: u64) -> Self {
        let mut stack = vec![(0, max_depth)];
        let mut result = None;

        while let Some((depth, remaining_depth)) = stack.pop() {
            if remaining_depth == 0 || rng.gen_bool(0.7) {
                // Base cases: return a simple type (U8, U256, etc.) with 70% probability
                result = Some(match rng.gen_range(0..=4) {
                    0 => Argument::U8(rng.gen()), // Generate a random 8-bit integer
                    1 => Argument::U256(rng.gen()), // Generate a random 256-bit integer
                    2 => Argument::Address(rng.gen()), // Generate a random Ethereum address
                    3 => Argument::Signer(Box::new(Argument::Address(rng.gen()))), // Generate a random signer (address)
                    _ => {
                        // Generate a random struct with a few fields
                        let mut struct_fields = Vec::new();
                        for i in 0..rng.gen_range(1..=3) {
                            struct_fields.push((
                                format!("field_{}", i),
                                Argument::random(rng, remaining_depth.saturating_sub(1)),
                            ));
                        }
                        Argument::Struct(struct_fields)
                    }
                });
            } else {
                // Recursive case: create a nested vector of arguments
                let len = rng.gen_range(1..4);
                let mut args = Vec::with_capacity(len);
                for _ in 0..len {
                    stack.push((depth + 1, remaining_depth - 1)); // Add to the stack for further nesting
                }
                result = Some(Argument::Vector(args));
            }
        }
        result.unwrap() // Return the generated argument
    }
}

// Exercise 3: Proof of Work Verification
impl Block {
    // Verify if the block hash meets the required difficulty (leading zeros)
    fn verify_proof_of_work(&self, difficulty: usize) -> bool {
        self.hash.starts_with(&"0".repeat(difficulty)) // Check if the hash has enough leading zeros
    }

    // Simulate mining by iterating over nonces until a valid hash is found
    fn mine(&mut self, difficulty: usize) {
        let mut nonce = 0u64;
        loop {
            let input = format!(
                "{}{}{}",
                self.parent_hash.as_ref().unwrap_or(&String::new()),
                self.number,
                nonce
            ); // Combine parent hash, block number, and nonce as input
            let mut hasher = Sha256::new();
            hasher.update(input); // Update the hasher with input data
            let result = hasher.finalize();
            let hash = hex::encode(result); // Convert the hash result to a hexadecimal string
            if hash.starts_with(&"0".repeat(difficulty)) {
                self.hash = hash; // If the hash meets the difficulty, accept it
                break;
            }
            nonce += 1; // Increment nonce and try again
        }
    }
}

// Exercise 4: Chain Selection Algorithm
// This implements the chain selection logic to update the chain head based on difficulty and edit score.
impl Node {
    fn add_block(&mut self, block: &Block) {
        self.chain.insert(block.hash.clone(), block.clone()); // Add the block to the local chain
        self.update_head(&block.hash); // Update the head if necessary
    }

    fn update_head(&mut self, new_block_hash: &str) {
        let new_block = &self.chain[new_block_hash];
        let current_head = &self.chain[&self.head];

        // Choose the new head based on cumulative difficulty and edit score (tie-breaker)
        if new_block.cumulative_difficulty > current_head.cumulative_difficulty
            || (new_block.cumulative_difficulty == current_head.cumulative_difficulty
                && new_block.edit_score < current_head.edit_score)
        {
            self.head = new_block_hash.to_string(); // Update the head if conditions are met
        }
    }
}

// Exercise 5: Simulating Ethereum Node Behavior
// This async method simulates block propagation with network delays and conditions.
impl Node {
    async fn propagate_block(&mut self, block: Block, network_id: u64) {
        let delay: u64 = block.gas_used / 1000; // Base delay proportional to gas used
        let mut rng = rand::thread_rng();
        let random_factor = rng.gen_range(0.8..1.2); // Random factor to simulate network conditions
        let delay = (delay as f64 * random_factor) as u64;

        sleep(Duration::from_millis(delay)).await; // Simulate network latency with a delay

        // Simulate receiving the block and processing it locally
        self.receive_block(block.clone()).await;
    }

    // Method to handle the receipt of a block
    async fn receive_block(&mut self, block: Block) {
        if !self.chain.contains_key(&block.hash) {
            // Check if the block is already in the chain
            if let Some(parent_hash) = &block.parent_hash {
                if self.chain.contains_key(parent_hash) {
                    // Only add the block if its parent is known (not orphaned)
                    self.add_block(&block);
                    println!("Node {} received new block: {}", self.id, block.hash);
                } else {
                    println!("Node {} received orphan block: {}", self.id, block.hash);
                }
            }
        }
    }
}

// Exercise 6: Implementing a Simple Verifier Trait
// This trait defines a generic verification interface for different validators.
trait Verifier {
    fn verify(&self, data: &[u8]) -> bool; // Verify input data based on specific criteria
}

struct AddressVerifier; // Verifier for Ethereum addresses (20 bytes)

impl Verifier for AddressVerifier {
    fn verify(&self, data: &[u8]) -> bool {
        data.len() == 20 // An Ethereum address must be 20 bytes long
    }
}

struct SignatureVerifier; // Verifier for ECDSA signatures (65 bytes)

impl Verifier for SignatureVerifier {
    fn verify(&self, data: &[u8]) -> bool {
        data.len() == 65 && (data[0] == 27 || data[0] == 28) // Valid signature must be 65 bytes, with a v value of 27 or 28
    }
}

struct ContractValidator {
    verifiers: Vec<Box<dyn Verifier>>, // Dynamic collection of verifiers
}

impl ContractValidator {
    fn new() -> Self {
        Self { verifiers: vec![] } // Initialize with an empty verifier list
    }

    fn add_verifier(&mut self, verifier: Box<dyn Verifier>) {
        self.verifiers.push(verifier); // Add a verifier to the list
    }

    fn validate(&self, data: &[u8]) -> bool {
        // Validate data by passing it through all verifiers
        for verifier in &self.verifiers {
            if !verifier.verify(data) {
                return false; // If any verifier fails, validation fails
            }
        }
        true // Data is valid if all verifiers pass
    }
}

// Exercise 7: Broadcasting Messages in a Network of Nodes
// This simulates broadcasting a message across a network with random delays.
#[derive(Clone, Debug)]
struct Message {
    id: u64, // Unique message ID
    content: String, // Message content
}

async fn broadcast_message(origin: u64, message: Message, network: &mut HashMap<u64, Node>) {
    let mut rng = rand::thread_rng();
    if let Some(origin_node) = network.get(&origin) {
        let peers = origin_node.peers.clone(); // Clone to avoid borrowing `network` mutably
        for neighbor in peers {
            if let Some(neighbor_node) = network.get_mut(&neighbor) {
                let delay = rng.gen_range(50..150); // Random delay to simulate network conditions
                let msg = message.clone(); // Clone the message for each peer
                tokio::spawn(async move {
                    sleep(Duration::from_millis(delay)).await;
                    println!("Node {} received: {:?}", neighbor, msg);
                });
            }
        }
    }
}

// Exercise 8: Simulating Forks and Chain Reorganizations
// This simulates adding a new block to all nodes in a network.
fn simulate_network(nodes: &mut [Node], new_block: Block) {
    for node in nodes.iter_mut() {
        node.add_block(&new_block); // Add the block to each node's local chain
    }
}

// Exercise 9: Simulating the Effect of Block Size on Forking
// This async function simulates block propagation with delays across nodes.
async fn simulate_network_with_delay(nodes: &mut [Node], new_block: Block, origin: usize) {
    nodes[origin].receive_block(new_block.clone()).await; // First, the origin node receives the block
    for (i, node) in nodes.iter_mut().enumerate() {
        if i != origin {
            // Propagate the block to other nodes
            node.receive_block(new_block.clone()).await;
        }
    }
}

// Exercise 10: Implementing Conditional Traits for Gas Calculation (EIP-1559)
// This defines different gas calculation strategies for pre- and post-London upgrades.
trait GasCalculator {
    fn calculate_gas(&self, gas_used: u64, base_fee: u64) -> u64; // Calculate the gas cost based on network state
}

struct PreLondonCalculator; // Gas calculation before the London upgrade

impl GasCalculator for PreLondonCalculator {
    fn calculate_gas(&self, gas_used: u64, _base_fee: u64) -> u64 {
        gas_used * 20 // Simplified gas price of 20 Gwei
    }
}

struct PostLondonCalculator; // Gas calculation after the London upgrade

impl GasCalculator for PostLondonCalculator {
    fn calculate_gas(&self, gas_used: u64, base_fee: u64) -> u64 {
        let priority_fee = 2; // Simplified priority fee of 2 Gwei
        gas_used * (base_fee + priority_fee) // Calculate total gas cost
    }
}

enum NetworkState {
    PreLondon, // Network state before the London upgrade
    PostLondon, // Network state after the London upgrade
}

// Get the appropriate gas calculator based on network state
impl NetworkState {
    fn get_calculator(&self) -> Box<dyn GasCalculator> {
        match self {
            NetworkState::PreLondon => Box::new(PreLondonCalculator),
            NetworkState::PostLondon => Box::new(PostLondonCalculator),
        }
    }
}

impl Node {
    fn get_chain(&self) -> Vec<String> {
        let mut chain_vec = vec![];
        let mut current = &self.head; // current head of the chain (the most recent block).

        while let Some(block) = self.chain.get(current) { 
            chain_vec.push(current.clone()); // 
            if let Some(parent) = &block.parent_hash { // 3->2->1->root
                current = parent;
            } else {
                break;
            }
        }
        chain_vec.reverse(); // root->1->2->3
        chain_vec
    }
}

// Exercise 11: Chain Edit Score and Fork Choice Algorithm
// This calculates the edit score of a block, which is used for fork choice in reorg scenarios.
fn calculate_edit_score(node: &Node, block_hash: &str) -> u64 {
    let mut score = 0;
    let mut current = block_hash;
    let main_chain = node.get_chain(); // Retrieve the main chain

    while let Some(block) = node.chain.get(current) {
        if main_chain.contains(&block.hash) {
            break; // Stop if the block is on the main chain
        }
        score += 1; // Increment the score for each block off the main chain
        if let Some(parent) = &block.parent_hash {
            current = parent;
        } else {
            break;
        }
    }
    score // Return the calculated edit score
}

// Exercise 12: Handling Multiple Consumers in a Blockchain System
// This manages a pool of pending transactions and processes them in parallel.
struct TransactionPool {
    transactions: HashMap<u64, Transaction>, // Pending transactions
    processed: HashSet<u64>, // Set of processed transactions to avoid duplicates
}

impl TransactionPool {
    fn new() -> Self {
        TransactionPool {
            transactions: HashMap::new(),
            processed: HashSet::new(),
        }
    }

    // Add a transaction to the pool if it hasn't been processed
    fn add_transaction(&mut self, tx: Transaction) -> bool {
        if !self.processed.contains(&tx.id) {
            self.transactions.insert(tx.id, tx); // Add transaction to the pool
            true
        } else {
            false // Reject duplicate transactions
        }
    }

    // Retrieve and remove the next transaction for processing
    fn get_next_transaction(&mut self) -> Option<Transaction> {
        let next_tx = self.transactions.values().next().cloned();
        if let Some(tx) = &next_tx {
            self.transactions.remove(&tx.id); // Remove the transaction from the pool
            self.processed.insert(tx.id); // Mark it as processed
        }
        next_tx
    }
}

// Worker function that processes transactions from the pool in parallel
async fn worker(id: u64, pool: Arc<Mutex<TransactionPool>>, mut rx: broadcast::Receiver<()>) {
    while rx.recv().await.is_ok() {
        let mut pool = pool.lock().await;
        if let Some(tx) = pool.get_next_transaction() {
            println!("Worker {} processing transaction {:?}", id, tx);
            sleep(Duration::from_millis(100)).await; // Simulate transaction processing delay
        }
    }
}

// Main function to demonstrate the combined functionality
#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();

    // Initialize network
    let mut network = HashMap::new();
    for i in 1..=5 {
        network.insert(
            i,
            Node {
                id: i,
                chain: HashMap::new(),
                head: "genesis".to_string(),
                peers: vec![],
            },
        );
    }

    // Connect nodes
    for i in 1..=5 {
        for j in i + 1..=5 {
            network.get_mut(&i).unwrap().peers.push(j);
            network.get_mut(&j).unwrap().peers.push(i);
        }
    }

    // Create genesis block
    let genesis = Block {
        hash: "genesis".to_string(),
        parent_hash: None,
        number: 0,
        cumulative_difficulty: 0,
        gas_used: 0,
        edit_score: 0,
    };

    for node in network.values_mut() {
        node.add_block(&genesis);
    }

    // Demonstrate block propagation and forking
    let block1 = Block {
        hash: "block1".to_string(),
        parent_hash: Some("genesis".to_string()),
        number: 1,
        cumulative_difficulty: 10,
        gas_used: 1_000_000,
        edit_score: 0,
    };

    let block2a = Block {
        hash: "block2a".to_string(),
        parent_hash: Some("block1".to_string()),
        number: 2,
        cumulative_difficulty: 20,
        gas_used: 2_000_000,
        edit_score: 0,
    };

    let block2b = Block {
        hash: "block2b".to_string(),
        parent_hash: Some("block1".to_string()),
        number: 2,
        cumulative_difficulty: 20,
        gas_used: 1_500_000,
        edit_score: 1,
    };

    let network_id = 1; // Assign a network ID

    network
        .get_mut(&1)
        .unwrap()
        .propagate_block(block1.clone(), network_id)
        .await;
    network
        .get_mut(&2)
        .unwrap()
        .propagate_block(block2a.clone(), network_id)
        .await;
    network
        .get_mut(&3)
        .unwrap()
        .propagate_block(block2b.clone(), network_id)
        .await;

    // Print network state
    for (id, node) in &network {
        println!(
            "Node {}: Head = {}, Chain length = {}",
            id,
            node.head,
            node.chain.len()
        );
    }

    // Demonstrate transaction processing
    let pool = Arc::new(Mutex::new(TransactionPool::new()));
    let (tx, _) = broadcast::channel(100);

    let mut worker_handles = vec![];

    // Spawn multiple workers for transaction processing
    for i in 1..=3 {
        let pool_clone = Arc::clone(&pool);
        let rx = tx.subscribe();
        let handle = tokio::spawn(async move {
            worker(i, pool_clone, rx).await;
        });
        worker_handles.push(handle);
    }

    // Submit transactions
    for i in 1..=10 {
        let transaction = Transaction {
            id: i,
            from: format!("0x{:040x}", rng.gen::<u128>()),
            to: format!("0x{:040x}", rng.gen::<u128>()),
            amount: rng.gen_range(1..1000),
            nonce: i,
        };
        pool.lock().await.add_transaction(transaction);
        tx.send(()).unwrap();
    }

    // Wait for transaction processing
    sleep(Duration::from_secs(2)).await;

    // Demonstrate Proof of Work
    let mut pow_block = Block {
        hash: String::new(),
        parent_hash: Some("block2a".to_string()),
        number: 3,
        cumulative_difficulty: 30,
        gas_used: 1_800_000,
        edit_score: 0,
    };
    pow_block.mine(4); // Mine with difficulty 4
    println!("Mined block with PoW: {}", pow_block.hash);
    println!("PoW valid: {}", pow_block.verify_proof_of_work(4));

    // Demonstrate gas calculation
    let gas_calculator = PostLondonCalculator;
    let gas_used = 21000; // Typical gas used for a simple transaction
    let base_fee = 100; // 100 Gwei
    let gas_cost = gas_calculator.calculate_gas(gas_used, base_fee);
    println!("Gas cost for a simple transaction: {} wei", gas_cost);

    // Demonstrate chain edit score
    let node1 = network.get_mut(&1).unwrap();
    node1.add_block(&pow_block);
    let edit_score = calculate_edit_score(node1, &pow_block.hash);
    println!("Edit score for the new block: {}", edit_score);

    // Demonstrate random argument generation
    let random_arg = Argument::random(&mut rng, 3);
    println!("Random Ethereum argument: {:?}", random_arg);

    // Demonstrate contract validation
    let mut validator = ContractValidator::new();
    validator.add_verifier(Box::new(AddressVerifier));
    validator.add_verifier(Box::new(SignatureVerifier));

    let valid_address = vec![0u8; 20];
    let valid_signature = vec![27u8; 65];
    println!("Valid address? {}", validator.validate(&valid_address));
    println!("Valid signature? {}", validator.validate(&valid_signature));

    // Demonstrate message broadcasting
    let message = Message {
        id: 1,
        content: "Hello, Ethereum network!".to_string(),
    };
    broadcast_message(1, message, &mut network).await;

    // Clean up
    drop(tx);
    for handle in worker_handles {
        handle.await.unwrap();
    }

    println!("Simulation completed successfully!");
}
