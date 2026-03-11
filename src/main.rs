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
            // Create a genesis block and mine two more blocks
            // Difficulty 4 = hash must start with "0000" (~65,000 guesses on average)
            let difficulty = 4;
            let genesis = create_genesis_block();
            let block1 = create_block(&genesis, "Alice pays Bob 10 BTC", difficulty);
            let block2 = create_block(&block1, "Bob pays Carol 5 BTC", difficulty);
            println!("\nChain of 3 blocks created with difficulty {}!", difficulty);
        }
        "2" => { todo!("Interactive mining not wired up yet") }
        "3" => add_transaction(),
        "4" => view_chain(),
        "5" => validate_chain(),
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

fn view_chain() {
    // Print all blocks in the chain with their key fields
    todo!()
}

fn validate_chain() {
    // Walk from genesis to tip, validate every block and every link
    todo!()
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
