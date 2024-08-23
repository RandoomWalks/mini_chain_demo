use hex;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio::time::{sleep, Duration};

// Common structures used across multiple exercises

#[derive(Clone, Debug)]
struct Block {
    hash: String,
    parent_hash: Option<String>,
    number: u64,
    cumulative_difficulty: u64,
    gas_used: u64,
    edit_score: u64,
}

#[derive(Clone, Debug)]
struct Transaction {
    id: u64,
    from: String,
    to: String,
    amount: u64,
    nonce: u64,
}

struct Node {
    id: u64,
    chain: HashMap<String, Block>,
    head: String,
    peers: Vec<u64>,
}

// Exercise 1: Chain Resolution Check
impl Node {
    fn is_resolved(&self) -> bool {
        let mut leaves = HashSet::new();
        for block in self.chain.values() {
            leaves.insert(block.hash.clone());
        }
        for block in self.chain.values() {
            if let Some(parent) = &block.parent_hash {
                leaves.remove(parent);
            }
        }
        leaves.len() == 1
    }
}

// Exercise 2: Random Argument Generation for Ethereum Testing
#[derive(Clone, Debug)]
enum Argument {
    U8(u8),
    U256([u8; 32]),
    Address([u8; 20]),
    Vector(Vec<Argument>),
    Struct(Vec<(String, Argument)>),
    Signer(Box<Argument>),
}

impl Argument {
    fn random<R: Rng>(rng: &mut R, max_depth: u64) -> Self {
        let mut stack = vec![(0, max_depth)];
        let mut result = None;

        while let Some((depth, remaining_depth)) = stack.pop() {
            if remaining_depth == 0 || rng.gen_bool(0.7) {
                result = Some(match rng.gen_range(0..=4) {
                    0 => Argument::U8(rng.gen()),
                    1 => Argument::U256(rng.gen()),
                    2 => Argument::Address(rng.gen()),
                    3 => Argument::Signer(Box::new(Argument::Address(rng.gen()))),
                    _ => {
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
                let len = rng.gen_range(1..4);
                let mut args = Vec::with_capacity(len);
                for _ in 0..len {
                    stack.push((depth + 1, remaining_depth - 1));
                }
                result = Some(Argument::Vector(args));
            }
        }
        result.unwrap()
    }
}

// Exercise 3: Proof of Work Verification
impl Block {
    fn verify_proof_of_work(&self, difficulty: usize) -> bool {
        self.hash.starts_with(&"0".repeat(difficulty))
    }

    fn mine(&mut self, difficulty: usize) {
        let mut nonce = 0u64;
        loop {
            let input = format!(
                "{}{}{}",
                self.parent_hash.as_ref().unwrap_or(&String::new()),
                self.number,
                nonce
            );
            let mut hasher = Sha256::new();
            hasher.update(input);
            let result = hasher.finalize();
            let hash = hex::encode(result);
            if hash.starts_with(&"0".repeat(difficulty)) {
                self.hash = hash;
                break;
            }
            nonce += 1;
        }
    }
}

// Exercise 4: Chain Selection Algorithm
impl Node {
    fn add_block(&mut self, block: &Block) {
        self.chain.insert(block.hash.clone(), block.clone());
        self.update_head(&block.hash);
    }

    fn update_head(&mut self, new_block_hash: &str) {
        let new_block = &self.chain[new_block_hash];
        let current_head = &self.chain[&self.head];

        if new_block.cumulative_difficulty > current_head.cumulative_difficulty
            || (new_block.cumulative_difficulty == current_head.cumulative_difficulty
                && new_block.edit_score < current_head.edit_score)
        {
            self.head = new_block_hash.to_string();
        }
    }
}

// Exercise 5: Simulating Ethereum Node Behavior
impl Node {
    async fn propagate_block(&mut self, block: Block, network_id: u64) {
        let delay: u64 = block.gas_used / 1000; // Base delay proportional to gas used
                                                // for &peer_id in &self.peers {
                                                //     if let Some(peer) = network.get_mut(&peer_id) {
        let mut rng = rand::thread_rng();
        let random_factor = rng.gen_range(0.8..1.2); // Random factor to simulate network conditions
        let delay = (delay as f64 * random_factor) as u64;

        sleep(Duration::from_millis(delay)).await;

        // TODO - figuer out if need this
        // if rng.gen_bool(0.05) { // 5% chance of dropping the message
        //     println!("Block propagation from Node {} to Node {} failed", self.id, network_id);
        //     continue;
        // }

        self.receive_block(block.clone()).await;
        //     }
        // }
    }

    // async fn propagate_block(&self, block: Block, network: &mut HashMap<u64, Node>) {
    //     let delay: u64 = block.gas_used / 1000; // Base delay proportional to gas used
    //     for &peer_id in &self.peers {
    //         if let Some(peer) = network.get_mut(&peer_id) {
    //             let mut rng = rand::thread_rng();
    //             let random_factor = rng.gen_range(0.8..1.2); // Random factor to simulate network conditions
    //             let delay = (delay as f64 * random_factor) as u64;

    //             sleep(Duration::from_millis(delay)).await;

    //             if rng.gen_bool(0.05) { // 5% chance of dropping the message
    //                 println!("Block propagation from Node {} to Node {} failed", self.id, peer_id);
    //                 continue;
    //             }

    //             peer.receive_block(block.clone()).await;
    //         }
    //     }
    // }

    async fn receive_block(&mut self, block: Block) {
        if !self.chain.contains_key(&block.hash) {
            if let Some(parent_hash) = &block.parent_hash {
                if self.chain.contains_key(parent_hash) {
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
trait Verifier {
    fn verify(&self, data: &[u8]) -> bool;
}

struct AddressVerifier;

impl Verifier for AddressVerifier {
    fn verify(&self, data: &[u8]) -> bool {
        data.len() == 20
    }
}

struct SignatureVerifier;

impl Verifier for SignatureVerifier {
    fn verify(&self, data: &[u8]) -> bool {
        data.len() == 65 && (data[0] == 27 || data[0] == 28)
    }
}

struct ContractValidator {
    verifiers: Vec<Box<dyn Verifier>>,
}

impl ContractValidator {
    fn new() -> Self {
        Self { verifiers: vec![] }
    }

    fn add_verifier(&mut self, verifier: Box<dyn Verifier>) {
        self.verifiers.push(verifier);
    }

    fn validate(&self, data: &[u8]) -> bool {
        for verifier in &self.verifiers {
            if !verifier.verify(data) {
                return false;
            }
        }
        true
    }
}

// Exercise 7: Broadcasting Messages in a Network of Nodes
#[derive(Clone, Debug)]
struct Message {
    id: u64,
    content: String,
}

async fn broadcast_message(origin: u64, message: Message, network: &mut HashMap<u64, Node>) {
    let mut rng = rand::thread_rng();
    // if let Some(origin_node) = network.get(&origin) {
    //     for &neighbor in &origin_node.peers {

    //         if let Some(neighbor_node) = network.get_mut(&neighbor) {
    //             let delay = rng.gen_range(50..150);
    //             let msg = message.clone();
    //             tokio::spawn(async move {
    //                 sleep(Duration::from_millis(delay)).await;
    //                 println!("Node {} received: {:?}", neighbor, msg);
    //             });
    //         }
    //     }
    // }
    if let Some(origin_node) = network.get(&origin) {
        let peers = origin_node.peers.clone(); // Clone to avoid borrowing `network` mutably
        for neighbor in peers {
            if let Some(neighbor_node) = network.get_mut(&neighbor) {
                // Use `neighbor_node` here
                let delay = rng.gen_range(50..150);
                let msg = message.clone();
                tokio::spawn(async move {
                    sleep(Duration::from_millis(delay)).await;
                    println!("Node {} received: {:?}", neighbor, msg);
                });
            }
        }
    }
}

// Exercise 8: Simulating Forks and Chain Reorganizations
fn simulate_network(nodes: &mut [Node], new_block: Block) {
    for node in nodes.iter_mut() {
        node.add_block(&new_block);
    }
}

// Exercise 9: Simulating the Effect of Block Size on Forking
async fn simulate_network_with_delay(nodes: &mut [Node], new_block: Block, origin: usize) {
    nodes[origin].receive_block(new_block.clone()).await;
    for (i, node) in nodes.iter_mut().enumerate() {
        if i != origin {
            node.receive_block(new_block.clone()).await;
        }
    }
}

// Exercise 10: Implementing Conditional Traits for Gas Calculation (EIP-1559)
trait GasCalculator {
    fn calculate_gas(&self, gas_used: u64, base_fee: u64) -> u64;
}

struct PreLondonCalculator;

impl GasCalculator for PreLondonCalculator {
    fn calculate_gas(&self, gas_used: u64, _base_fee: u64) -> u64 {
        gas_used * 20 // Simplified gas price of 20 Gwei
    }
}

struct PostLondonCalculator;

impl GasCalculator for PostLondonCalculator {
    fn calculate_gas(&self, gas_used: u64, base_fee: u64) -> u64 {
        let priority_fee = 2; // Simplified priority fee of 2 Gwei
        gas_used * (base_fee + priority_fee)
    }
}

enum NetworkState {
    PreLondon,
    PostLondon,
}

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
fn calculate_edit_score(node: &Node, block_hash: &str) -> u64 {
    let mut score = 0;
    let mut current = block_hash;
    let main_chain = node.get_chain(); // TODO

    while let Some(block) = node.chain.get(current) {
        if main_chain.contains(&block.hash) {
            break;
        }
        score += 1;
        if let Some(parent) = &block.parent_hash {
            current = parent;
        } else {
            break;
        }
    }
    score
}

// Exercise 12: Handling Multiple Consumers in a Blockchain System
struct TransactionPool {
    transactions: HashMap<u64, Transaction>,
    processed: HashSet<u64>,
}

impl TransactionPool {
    fn new() -> Self {
        TransactionPool {
            transactions: HashMap::new(),
            processed: HashSet::new(),
        }
    }

    fn add_transaction(&mut self, tx: Transaction) -> bool {
        if !self.processed.contains(&tx.id) {
            self.transactions.insert(tx.id, tx);
            true
        } else {
            false
        }
    }

    fn get_next_transaction(&mut self) -> Option<Transaction> {
        let next_tx = self.transactions.values().next().cloned();
        if let Some(tx) = &next_tx {
            self.transactions.remove(&tx.id);
            self.processed.insert(tx.id);
        }
        next_tx
    }
}

async fn worker(id: u64, pool: Arc<Mutex<TransactionPool>>, mut rx: broadcast::Receiver<()>) {
    while rx.recv().await.is_ok() {
        let mut pool = pool.lock().await;
        if let Some(tx) = pool.get_next_transaction() {
            println!("Worker {} processing transaction {:?}", id, tx);
            sleep(Duration::from_millis(100)).await;
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
    // let (tx, _) = mpsc::channel(100);
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
