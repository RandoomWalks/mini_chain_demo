use rand::{Error, Rng};
use std::boxed::Box;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::iter;
use std::path::Iter;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

#[derive(Debug, Clone)]
struct Block {
    hash: String,
}

impl Block {
    pub fn verify_proof_of_work(&self, difficulty_lvl: usize) -> bool {
        let s1: &str = "0";
        let s2: String = s1.repeat(difficulty_lvl);
        let is_verified = self.hash.starts_with(&s2);
        is_verified
    }
}

pub fn simulate_mine() -> Block {
    // Implement a simple mining simulation that tries to find a valid hash by brute force.
    // sha1 output
    let mut rng = rand::thread_rng();
    let mut v1: Vec<u8> = (0..20).map(|_| rng.gen()).collect();

    let mut b1 = Block {
        hash: String::from_utf8(v1).unwrap_or_default(),
    };

    while !b1.verify_proof_of_work(1) {
        v1 = (0..20).map(|_| rng.gen()).collect();

        b1 = Block {
            hash: String::from_utf8(v1).unwrap_or_default(),
        };
    }
    b1
}

pub fn simulate_mine2() -> Block {
    // TODO : Complete following simulate_mine_work()
    // Implement a simple mining simulation that tries to find a valid hash by brute force.
    // sha1 output
    let mut rng = rand::thread_rng();
    let mut hash: [u8; 32] = [0u8; 32]; // sha-256 output 256 bits= 32bytes
    let mut nonce = 0u64;

    loop {
        let n1 = nonce.to_le_bytes(); // Little-endian format stores the least significant byte first.
        hash[24..].copy_from_slice(&n1);
        // let f1 = rng.fill();

        // let mut arr = [0i8; 20];
        // rng.fill(&mut arr[..]);

        // Fill the rest with random bytes
        rng.fill(&mut hash[..24]);
    }
    let mut v1: Vec<u8> = (0..20).map(|_| rng.gen()).collect();

    let mut b1 = Block {
        hash: String::from_utf8(v1).unwrap_or_default(),
    };

    while !b1.verify_proof_of_work(1) {
        v1 = (0..20).map(|_| rng.gen()).collect();

        b1 = Block {
            hash: String::from_utf8(v1).unwrap_or_default(),
        };
    }
    b1
}

use std::fmt;
use tokio::sync::mpsc;

struct Transaction {
    id: u64,
    data: String,
}

async fn send_msgs() {
    
}

#[derive(Debug)]
enum CustomError {
    Utf8Error(std::string::FromUtf8Error),
    IoError(std::io::Error),
}

impl From<std::string::FromUtf8Error> for CustomError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        CustomError::Utf8Error(value)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(value: std::io::Error) -> Self {
        CustomError::IoError(value)
    }
}

fn test_tx() -> Result<Transaction, CustomError> {
    let mut rng = rand::thread_rng();

    let v1: Vec<u8> = (0..10).map(|_| rng.gen_range(0..1)).collect();
    let res = String::from_utf8(v1).map_err(CustomError::from)?;
    Ok(Transaction {
        id: rng.gen(),
        data: res,
    })
    // match String::from_utf8(v1) {
    //     Ok(res) => Ok(Transaction {
    //         id: rng.gen(),
    //         data: res,
    //     }),
    //     Err(e) => {
    //         let e1: CustomError = e.into();
    //         Err(e1)
    //     }
    // }
}

use anyhow::{Context, Result};

fn test_tx2() -> Result<Transaction> {
    let mut rng = rand::thread_rng();

    let v1: Vec<u8> = (0..10).map(|_| rng.gen_range(0..1)).collect();
    let res =
        String::from_utf8(v1.clone()).with_context(|| format!("Failed to read file: {:#?}", v1))?;

    Ok(Transaction {
        id: rng.gen(),
        data: res,
    })

    // match String::from_utf8(v1) {
    //     Ok(res) => Ok(Transaction {
    //         id: rng.gen(),
    //         data: res,
    //     }),
    //     Err(e) => {
    //         let e1: CustomError = e.into();
    //         Err(e1)
    //     }
    // }
}

pub fn simulate_mine_work(difficulty: u32) -> Block {
    // Initialize a random number generator for generating random bytes
    let mut rng = rand::thread_rng();

    // Create a 32-byte array to store the hash value (32 bytes for SHA-256)
    let mut hash = [0u8; 32]; // hash: [0, 0, 0, 0, ..., 0] (32 bytes of zeros initially)

    // Initialize nonce to 0. The nonce will be incremented to find a valid hash
    let mut nonce = 0u64; // nonce: 0

    loop {
        // Convert nonce to 8-byte array in little-endian format
        // Example: nonce = 0
        // n1 = [0, 0, 0, 0, 0, 0, 0, 0]
        let nonce_bytes = nonce.to_le_bytes(); // nonce_bytes: [0, 0, 0, 0, 0, 0, 0, 0]

        // Update the last 8 bytes of the hash with the nonce value
        // Example: After hash[24..] = nonce_bytes
        // hash: [0, 0, 0, 0, ..., 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        hash[24..].copy_from_slice(&nonce_bytes);

        // Fill the first 24 bytes of the hash array with random bytes
        // Example: Random bytes are generated, e.g.,
        // hash: [144, 25, 78, 220, 187, 34, 89, 76, ..., random bytes, nonce_bytes]
        rng.fill(&mut hash[..24]);

        // Create a Block object with the current hash value
        // Example: Block { hash: [144, 25, 78, 220, ..., random bytes, nonce_bytes] }
        let hash_str = String::from_utf8(hash.to_vec()).unwrap();

        let block = Block { hash: hash_str };

        // Check if the current hash meets the difficulty requirement
        // The method `verify_proof_of_work` likely checks if the hash has the required number of leading zeros
        // Example: If difficulty is 3, the hash needs to start with at least 3 zeros
        if block.verify_proof_of_work(difficulty as usize ) {
            // If the hash meets the difficulty, return the block
            // Example: Returning a block with a valid hash
            return block;
        }

        // Increment the nonce for the next iteration
        // Example: nonce = 0 → 1
        nonce = nonce.wrapping_add(1);
    }
}

#[derive(Debug, Clone)]
struct Chain<T> {
    leaves: VecDeque<T>,
}

impl<T: Debug> Chain<T> {
    fn it_resolves(&self) -> bool {
        self.leaves.len() == 1
    }
    fn print_chain(&self) {
        for leaf in &self.leaves {
            println!("leaf:{:#?}", leaf);
        }
    }
}

impl<T> Iterator for Chain<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.leaves.pop_front()
    }
}

// enum Arg {
//     U8(u8),

// }

#[tokio::main]
async fn main() {
    let rnd_vec = gen_test_vec(1);
    // iterate().await;
    println!("{:#?}", rnd_vec);
    let rnd_vec2 = gen_test_vec(10);
    // iterate().await;
    println!("{:#?}", rnd_vec2);

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let bool_opt: Option<bool> = Some(rng.gen_bool(0.5));

    // let s1 = Arg::Address([rng.gen(); 20]);

    let v1: Vec<u8> = (0..10).map(|_| rng.gen()).collect();
    let mut s2: Result<String, std::string::FromUtf8Error> = String::from_utf8(v1);
    let s3: String = s2.unwrap_or_default();
}

async fn iterate() {
    let shared_state: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));

    let mut handles: Vec<task::JoinHandle<u8>> = vec![];

    let mut chain: Arc<Mutex<Chain<Block>>> = Arc::new(Mutex::new(Chain {
        leaves: VecDeque::new(),
    }));

    for i in 0..5 {
        let state_cpy: Arc<Mutex<u8>> = Arc::clone(&shared_state);
        let chain_cpy: Arc<Mutex<Chain<Block>>> = Arc::clone(&chain);

        // let seed = i.parse::<String>();
        let seed: String = i.to_string();
        let new_block = Block { hash: seed };

        handles.push(tokio::spawn(async move {
            let mut val: tokio::sync::MutexGuard<u8> = state_cpy.lock().await;
            *val += 1;

            let mut chain_cpy = chain_cpy.lock().await;
            chain_cpy.leaves.push_back(new_block);

            // drop(val);
            return *val;
        }));
    }

    let mut tmp: u8 = 1;

    for h in handles {
        match h.await {
            Ok(id) => {
                println!("task joined:{}", id);
                // assert!(id==tmp, "a = {}, b = {}", id, tmp);
                tmp += 1;

                // debug!()
            }
            Err(e) => {
                println!("Error: {:#?}", e);
            }
        }
    }
    {
        let chain = chain.lock().await;
        chain.print_chain();
    }
    {
        // let chain: tokio::sync::MutexGuard<Chain<Block>> = chain.lock().await;
        // let chain2 = chain.iter().cloned();
    }

    {
        let mut chain_guard = chain.lock().await;
        while let Some(block) = chain_guard.leaves.pop_front() {
            println!("Block hash: {}", block.hash);
        }

        for i in chain_guard.leaves.iter() {
            println!("{:#?}", i);
        }

        // let mut iter = chain.iter().peekable();
        // while let Some(item) = iter.peek() {
        //     println!("Peeked item: {:?}", item);
        //     // Use `iter.next()` if you need to consume the item
        // }
    }

    // {
    //     let chain_guard = chain.lock().await;
    //     while let Some(block) = chain_guard.clone().next() {
    //         println!("Block hash: {}", block.hash);
    //     }

    //     // for i in chain_guard.iter() {
    //     //     println!("{:#?}",i );
    //     // }

    //     // let mut iter = chain.iter().peekable();
    //     // while let Some(item) = iter.peek() {
    //     //     println!("Peeked item: {:?}", item);
    //     //     // Use `iter.next()` if you need to consume the item
    //     // }

    // }
}

#[derive(Debug, Clone)]
struct ArgS {
    a1: Box<Arg>,
    a2: Box<Arg>,
}

#[derive(Debug, Clone)]
enum Arg {
    U8(u8),
    Address(String),
    Vector(Vec<Arg>),
    Struct(ArgS),
}

impl Arg {
    pub fn random<R: Rng>(rng: &mut R, max_depth: u64) -> Self {
        // TODO: implement for Arg::Struct{}
        let mut stack = vec![(0, max_depth)];
        let mut result = None;
        while let Some((depth, remaining_depth)) = stack.pop() {
            if remaining_depth == 0 || rng.gen_bool(0.7) {
                // Generate simple types
                let simple_arg = if rng.gen_bool(0.5) {
                    Arg::U8(rng.gen())
                } else {
                    Arg::Address(String::from_utf8([rng.gen(); 20].to_vec()).unwrap_or_default())
                };

                if depth == 0 {
                    return simple_arg;
                } else {
                    result = Some(simple_arg);
                }
            } else {
                // Generate complex types
                let len = rng.gen_range(1..4);
                let mut args = Vec::with_capacity(len);
                for _ in 0..len {
                    stack.push((depth + 1, remaining_depth - 1));
                }

                if depth == 0 {
                    return Arg::Vector(args);
                } else {
                    result = Some(Arg::Vector(args));
                }
            }
        }
        result.unwrap()
    }
}

fn gen_test_vec3(max_depth: usize) {
    let x = (0, 1);
    let x = (0, 1, 2);

    let mut stack = vec![(0, max_depth)];

    let mut stack = vec![(0, max_depth), (1, max_depth - 1)];
}

fn gen_test_vec2(vec_len: usize) -> Arg {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let n: f64 = rng.gen_range(0.0..1.0);

    if vec_len == 0 {
        if rng.gen_bool(0.5) {
            // address

            return Arg::U8(rng.gen());
        } else {
            // U8
            let addr = format!("{:x}", rng.gen::<u64>());

            return Arg::Address(addr);
        }
    } else {
        if rng.gen_bool(0.5) {
            let v = vec![gen_test_vec2(vec_len - 1)];
            return Arg::Vector(v);
        } else if rng.gen_bool(0.5) {
            // address

            return Arg::U8(rng.gen());
        } else {
            // U8
            let addr = format!("{:x}", rng.gen::<u64>());

            return Arg::Address(addr);
        }
    }

    // if n < 0.3 {
    //     let mut v = Vec::new();
    //     if vec_len > 0 {
    //         v.push(gen_test_vec(vec_len - 1));
    //     }
    //     // Arg::U8(rng.gen_range(0..255))
    //     return Arg::Vector(v);
    // } else if n >= 0.3 && n < 0.6 {
    //     return Arg::U8(rng.gen_range(0..255));
    // } else {
    //     let choices = ["apple", "banana", "cherry"];
    //     let random_choice = choices[rng.gen_range(0..choices.len())];
    //     return Arg::Address(random_choice.to_string());
    // };
}

fn gen_test_vec(vec_len: usize) -> Arg {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let n: f64 = rng.gen_range(0.0..1.0);

    if n < 0.3 {
        let mut v = Vec::new();
        if vec_len > 0 {
            v.push(gen_test_vec(vec_len - 1));
        }
        // Arg::U8(rng.gen_range(0..255))
        return Arg::Vector(v);
    } else if n >= 0.3 && n < 0.6 {
        return Arg::U8(rng.gen_range(0..255));
    } else {
        let choices = ["apple", "banana", "cherry"];
        let random_choice = choices[rng.gen_range(0..choices.len())];
        return Arg::Address(random_choice.to_string());
    };
}

fn gen_test() {
    enum Arg {
        U8(u8),
        Address(String),
    }
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let n: f64 = rng.gen_range(0.0..1.0);

    let a = if n > 0.5 {
        Arg::U8(rng.gen_range(0..255))
    } else {
        let choices = ["apple", "banana", "cherry"];
        let random_choice = choices[rng.gen_range(0..choices.len())];
        Arg::Address(random_choice.to_string())
    };
}

fn demo_gen_rnd() {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let n: i32 = rng.gen_range(0..10);
    let n: f64 = rng.gen_range(0.0..1.0);
    let x: f64 = rng.gen();
    let x: bool = rng.gen();
    // let x: Vec<bool> = rng.gen();
    let y = rng.gen_range(0..5);

    let choices = ["apple", "banana", "cherry"];
    let random_choice = choices[rng.gen_range(0..choices.len())];
}

fn demo_peekable_iterator() {
    let v = vec![1, 2, 3];
    let mut pk = v.iter().peekable();

    while let Some(&&item) = pk.peek() {
        println!("Peeked item: {:?}", item);
        // Use `iter.next()` if you need to consume the item
    }

    let mut v2 = v.iter().cloned();
    let mut v3 = v2.peekable();

    while let Some(item) = v3.peek() {
        println!("Peeked item: {:?}", item);
        // Use `iter.next()` if you need to consume the item
    }
}
