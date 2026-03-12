use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

// A block in the chain — starts simple, we'll add transactions later
struct Block {
    index: u64,              // position in the chain (0 = genesis)
    timestamp: u64,          // seconds since Unix epoch (Jan 1 1970)
    previous_hash: String,   // SHA256 hash of the previous block (the "link")
    data: String,            // placeholder for transactions — just a string for now
    nonce: u64,              // the number we increment during mining (proof of work)
    hash: String,            // this block's own SHA256 hash
}

fn main() {
    println!("=== Rust Blockchain ===\n");
    println!("1. Create genesis block");
    println!("2. Mine a block");
    println!("3. Add transaction");
    println!("4. View chain");
    println!("5. Validate chain");
    println!("6. Save chain");
    println!("7. Load chain");
    println!("8. Exit\n");

    print!("Choose an option: ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            // Create a chain as a Vec (dynamic array) of blocks
            let difficulty = 4;
            let mut chain: Vec<Block> = Vec::new();

            // Genesis block goes in first
            chain.push(create_genesis_block());

            // Mine two more blocks — each links to the last block in the chain
            // chain.last().unwrap() gets a reference to the most recent block
            chain.push(create_block(chain.last().unwrap(), "Alice pays Bob 10 BTC", difficulty));
            chain.push(create_block(chain.last().unwrap(), "Bob pays Carol 5 BTC", difficulty));

            println!("\nChain of {} blocks created!", chain.len());

            // Validate the whole chain
            validate_chain(&chain, difficulty);
        }
        "2" => { todo!("Interactive mining not wired up yet") }
        "3" => add_transaction(),
        "4" => view_chain(&Vec::new()),
        "5" => { todo!("Need a chain to validate") }
        "6" => save_chain(),
        "7" => load_chain(),
        "8" => println!("Goodbye!"),
        _ => println!("Invalid option: '{}'", input.trim()),
    }
}

// === Hashing ===

fn hash_block(index: u64, timestamp: u64, previous_hash: &str, data: &str, nonce: u64) -> String {
    // Concatenate all block fields into one string, then SHA256 it
    // If ANY field changes, the hash completely changes (avalanche effect)
    // This is what makes the chain tamper-evident
    let input = format!("{}{}{}{}{}", index, timestamp, previous_hash, data, nonce);

    // Create a SHA256 hasher, feed it the bytes, get the result
    let mut hasher = Sha256::new();       // create the hasher
    hasher.update(input.as_bytes());       // feed it our string as bytes
    let result = hasher.finalize();        // compute the hash (32 bytes)

    hex::encode(result)                    // convert to hex string like "a3b1c9..."
}

fn verify_hash() {
    // Recompute the hash of a block and check it matches the stored hash
    todo!()
}

fn merkle_root() {
    // Hash all transactions in pairs, then hash the pairs, repeat until one hash remains
    // This single hash summarizes ALL transactions in the block
    todo!()
}

// === Blocks ===

fn create_genesis_block() -> Block {
    // The genesis block is the first block in the chain — block 0
    // It has no predecessor, so previous_hash is just "0"
    // Bitcoin's real genesis block contains the message:
    //   "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks"

    let index = 0;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)  // seconds since Jan 1 1970
        .unwrap()
        .as_secs();
    let previous_hash = "0";
    let data = "Genesis block";
    let nonce = 0;  // no mining for genesis — it's hardcoded

    // Compute hash from the fields, THEN build the block
    let hash = hash_block(index, timestamp, previous_hash, data, nonce);

    println!("Genesis block created!");
    println!("  Index: {}", index);
    println!("  Hash: {}", hash);

    Block {
        index,
        timestamp,
        previous_hash: String::from(previous_hash),
        data: String::from(data),
        nonce,
        hash,
    }
}

fn create_block(previous_block: &Block, data: &str, difficulty: usize) -> Block {
    // Build a new block that links to the previous one
    // The "chain" is this link: our previous_hash = parent's hash

    let index = previous_block.index + 1;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let previous_hash = &previous_block.hash;

    // Mine the block — find a nonce that gives us a hash with enough leading zeros
    println!("Mining block {}...", index);
    let (nonce, hash) = mine_block(index, timestamp, previous_hash, data, difficulty);

    println!("  Previous hash: {}...", &previous_hash[..16]);
    println!("  Hash: {}", hash);

    Block {
        index,
        timestamp,
        previous_hash: previous_hash.clone(),
        data: String::from(data),
        nonce,
        hash,
    }
}

fn validate_block() {
    // Check: hash is correct, previous_hash matches parent, proof of work is valid
    todo!()
}

// === Chain ===

fn view_chain(chain: &Vec<Block>) {
    // Print all blocks in the chain
    println!("\n=== BLOCKCHAIN ({} blocks) ===\n", chain.len());
    for block in chain {
        println!("Block {}", block.index);
        println!("  Timestamp: {}", block.timestamp);
        println!("  Data: {}", block.data);
        println!("  Nonce: {}", block.nonce);
        println!("  Prev hash: {}...", &block.previous_hash[..16.min(block.previous_hash.len())]);
        println!("  Hash: {}...", &block.hash[..16]);
        println!();
    }
}

fn validate_chain(chain: &Vec<Block>, difficulty: usize) {
    // Walk every block and verify three things:
    // 1. The stored hash actually matches the block's contents
    // 2. The previous_hash matches the prior block's hash (the chain link)
    // 3. The hash satisfies the difficulty requirement (proof of work)

    let target = "0".repeat(difficulty);
    let mut valid = true;

    for i in 0..chain.len() {
        let block = &chain[i];

        // Check 1: Recompute the hash — does it match what's stored?
        // If someone changed the data, the recomputed hash won't match
        let recomputed = hash_block(
            block.index, block.timestamp,
            &block.previous_hash, &block.data, block.nonce,
        );
        if recomputed != block.hash {
            println!("INVALID: Block {} hash doesn't match!", i);
            valid = false;
        }

        // Check 2: Does previous_hash link to the actual previous block?
        // Skip block 0 (genesis has no predecessor)
        if i > 0 {
            let prev_block = &chain[i - 1];
            if block.previous_hash != prev_block.hash {
                println!("INVALID: Block {} previous_hash doesn't match Block {}!", i, i - 1);
                valid = false;
            }
        }

        // Check 3: Does the hash meet the difficulty requirement?
        // A valid mined block's hash must start with N zeros
        if i > 0 && !block.hash.starts_with(&target) {
            println!("INVALID: Block {} doesn't meet difficulty {}!", i, difficulty);
            valid = false;
        }
    }

    if valid {
        println!("\nChain is VALID — all {} blocks verified.", chain.len());
    }
}

// === Mining ===

fn mine_block(index: u64, timestamp: u64, previous_hash: &str, data: &str, difficulty: usize) -> (u64, String) {
    // Proof of work: keep trying nonces until the hash starts with 'difficulty' zeros
    // This is the "work" in proof of work — pure brute force, no shortcut
    // Returns the winning nonce and the valid hash

    // Build the target prefix — e.g., difficulty=4 means hash must start with "0000"
    let target = "0".repeat(difficulty);

    let mut nonce: u64 = 0;
    loop {
        let hash = hash_block(index, timestamp, previous_hash, data, nonce);

        // Does it start with enough zeros?
        if hash.starts_with(&target) {
            println!("  Mined! Nonce: {} (tried {} times)", nonce, nonce + 1);
            return (nonce, hash);
        }

        nonce += 1;
    }
}

fn adjust_difficulty() {
    // If blocks are being mined too fast, increase difficulty (more zeros)
    // If too slow, decrease. Bitcoin retargets every 2016 blocks.
    todo!()
}

// === Transactions ===

fn add_transaction() {
    // Create a transaction: sender, recipient, amount
    // Added to a pending pool, included in the next mined block
    todo!()
}

fn validate_transaction() {
    // Check: sender has enough balance, transaction isn't a duplicate
    todo!()
}

fn get_balance() {
    // Walk the chain, sum up all transactions for an address (UTXO model)
    todo!()
}

// === Persistence ===

fn save_chain() {
    // Serialize the entire chain to YAML and write to disk
    todo!()
}

fn load_chain() {
    // Read YAML from disk and deserialize back into chain structs
    todo!()
}
