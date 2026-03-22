use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};  // traits for converting structs to/from YAML
use std::io;
use std::fs;                           // file system operations (read/write files)
use std::time::{SystemTime, UNIX_EPOCH};

// A transaction — someone sends coins to someone else
// In real Bitcoin this would use the UTXO model with inputs/outputs,
// but we're starting simple: sender, recipient, amount
// #[derive(...)] tells the compiler to auto-generate trait implementations
// Serialize/Deserialize = can convert to/from YAML for saving to disk
#[derive(Serialize, Deserialize)]
struct Transaction {
    sender: String,    // who's paying (address or name for now)
    recipient: String, // who's receiving
    amount: f64,       // how much (f64 is fine for learning, real Bitcoin uses integer satoshis)
}

// A block in the chain
#[derive(Serialize, Deserialize)]
struct Block {
    index: u64,                     // position in the chain (0 = genesis)
    timestamp: u64,                 // seconds since Unix epoch (Jan 1 1970)
    previous_hash: String,          // SHA256 hash of the previous block (the "link")
    transactions: Vec<Transaction>, // the transactions included in this block
    merkle_root: String,            // single hash summarizing all transactions (Merkle tree root)
    nonce: u64,                     // the number we increment during mining (proof of work)
    hash: String,                   // this block's own SHA256 hash
}

fn main() {
    println!("=== Rust Blockchain ===\n");

    // These live for the whole session — shared across all menu actions
    let mut difficulty = 4;
    let mut chain: Vec<Block> = Vec::new();
    let mut pending_txs: Vec<Transaction> = Vec::new(); // transactions waiting to be mined

    // Main loop — keeps showing the menu until the user exits
    loop {
        println!("\n--- Menu ---");
        println!("1. Initialize chain (genesis block)");
        println!("2. Add transaction");
        println!("3. Mine pending transactions into a block");
        println!("4. Check balance");
        println!("5. View chain");
        println!("6. Validate chain");
        println!("7. Save chain");
        println!("8. Load chain");
        println!("9. Exit\n");

        print!("Choose an option: ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                // Initialize the chain with a genesis block
                if !chain.is_empty() {
                    println!("Chain already initialized ({} blocks).", chain.len());
                    continue;  // jump back to top of loop
                }
                chain.push(create_genesis_block());
                println!("Chain initialized with genesis block.");
            }
            "2" => {
                // Add a transaction to the pending pool (not in a block yet)
                if chain.is_empty() {
                    println!("Initialize the chain first (option 1).");
                    continue;
                }

                let tx = read_transaction();
                if validate_transaction(&chain, &tx) {
                    println!(
                        "Transaction added: {} -> {} : {} coins",
                        tx.sender, tx.recipient, tx.amount
                    );
                    pending_txs.push(tx);
                    println!("Pending transactions: {}", pending_txs.len());
                }
            }
            "3" => {
                // Mine all pending transactions into a new block
                if chain.is_empty() {
                    println!("Initialize the chain first (option 1).");
                    continue;
                }
                if pending_txs.is_empty() {
                    println!("No pending transactions to mine.");
                    continue;
                }

                // Drain pending_txs — moves all transactions out, leaving it empty
                // This is an ownership transfer: pending_txs gives up its contents to the block
                let txs: Vec<Transaction> = pending_txs.drain(..).collect();
                let tx_count = txs.len();
                chain.push(create_block(chain.last().unwrap(), txs, difficulty));
                println!("Mined block {} with {} transactions.", chain.len() - 1, tx_count);

                // Adjust difficulty for next block based on how long this one took
                difficulty = adjust_difficulty(&chain, difficulty);
            }
            "4" => {
                // Check balance for an address
                if chain.is_empty() {
                    println!("Initialize the chain first (option 1).");
                    continue;
                }

                print!("Address: ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut addr = String::new();
                io::stdin().read_line(&mut addr).unwrap();
                let addr = addr.trim();

                let balance = get_balance(&chain, addr);
                println!("{} has {} coins", addr, balance);
            }
            "5" => view_chain(&chain),
            "6" => validate_chain(&chain, difficulty),
            "7" => save_chain(&chain),
            "8" => {
                chain = load_chain();
                // Validate the loaded chain to make sure the file wasn't tampered with
                validate_chain(&chain, difficulty);
            }
            "9" => {
                println!("Goodbye!");
                break;  // exit the loop
            }
            _ => println!("Invalid option: '{}'", input.trim()),
        }
    }
}

// === Hashing ===

fn hash_transaction(tx: &Transaction) -> String {
    // Hash a single transaction — same pattern as hash_block
    // Concatenate all fields into one string, then SHA256 it
    // This gives each transaction a unique fingerprint for the Merkle tree
    let input = format!("{}{}{}", tx.sender, tx.recipient, tx.amount);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

fn hash_block(
    index: u64,
    timestamp: u64,
    previous_hash: &str,
    merkle_root: &str,
    nonce: u64,
) -> String {
    // Concatenate all block fields into one string, then SHA256 it
    // If ANY field changes, the hash completely changes (avalanche effect)
    // This is what makes the chain tamper-evident
    // Note: we hash the merkle_root (not raw transactions) — the root already covers all txs
    let input = format!(
        "{}{}{}{}{}",
        index, timestamp, previous_hash, merkle_root, nonce
    );

    // Create a SHA256 hasher, feed it the bytes, get the result
    let mut hasher = Sha256::new(); // create the hasher
    hasher.update(input.as_bytes()); // feed it our string as bytes
    let result = hasher.finalize(); // compute the hash (32 bytes)

    hex::encode(result) // convert to hex string like "a3b1c9..."
}

fn verify_hash(block: &Block) -> bool {
    // Recompute the hash from the block's fields and check it matches the stored hash
    // If someone tampered with any field, the recomputed hash won't match
    let recomputed = hash_block(
        block.index, block.timestamp,
        &block.previous_hash, &block.merkle_root, block.nonce,
    );
    recomputed == block.hash
}

fn merkle_root(transactions: &Vec<Transaction>) -> String {
    // Hash all transactions, then pair up and hash pairs, repeat until one hash remains
    // This single hash summarizes ALL transactions in the block
    //
    // Example with 4 transactions:
    //       Root
    //      /    \
    //    H12    H34
    //    / \    / \
    //   T1 T2 T3 T4

    // If no transactions, return a hash of empty string
    if transactions.is_empty() {
        return String::from("0");
    }

    // Step 1: Hash each transaction to get the leaf nodes
    // TODO: come back and learn .iter().map().collect() — a more concise way to do this
    let mut hashes: Vec<String> = Vec::new();
    for tx in transactions {
        hashes.push(hash_transaction(tx));
    }

    // Step 2: Keep pairing and hashing until one hash remains
    while hashes.len() > 1 {
        let mut next_level: Vec<String> = Vec::new();

        // Process pairs — step by 2
        let mut i = 0;
        while i < hashes.len() {
            // If odd number of hashes, duplicate the last one
            // (Bitcoin does this too — odd leaves get paired with themselves)
            let left = &hashes[i];
            let right = if i + 1 < hashes.len() {
                &hashes[i + 1]
            } else {
                &hashes[i] // duplicate last hash
            };

            // Hash the pair together
            let combined = format!("{}{}", left, right);
            let mut hasher = Sha256::new();
            hasher.update(combined.as_bytes());
            next_level.push(hex::encode(hasher.finalize()));

            i += 2;
        }

        hashes = next_level; // move up one level in the tree
    }

    // Only one hash left — that's the root
    hashes[0].clone()
}

// === Blocks ===

fn create_genesis_block() -> Block {
    // The genesis block is the first block in the chain — block 0
    // It has no predecessor, so previous_hash is just "0"
    // Bitcoin's real genesis block contains the message:
    //   "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks"

    let index = 0;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH) // seconds since Jan 1 1970
        .unwrap()
        .as_secs();
    let previous_hash = "0";
    let transactions: Vec<Transaction> = Vec::new(); // genesis has no transactions
    let mr = merkle_root(&transactions);
    let nonce = 0; // no mining for genesis — it's hardcoded

    // Compute hash from the fields, THEN build the block
    let hash = hash_block(index, timestamp, previous_hash, &mr, nonce);

    println!("Genesis block created!");
    println!("  Index: {}", index);
    println!("  Hash: {}", hash);

    Block {
        index,
        timestamp,
        previous_hash: String::from(previous_hash),
        transactions,
        merkle_root: mr,
        nonce,
        hash,
    }
}

fn create_block(
    previous_block: &Block,
    transactions: Vec<Transaction>,
    difficulty: usize,
) -> Block {
    // Build a new block that links to the previous one
    // The "chain" is this link: our previous_hash = parent's hash

    let index = previous_block.index + 1;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let previous_hash = &previous_block.hash;

    // Compute the Merkle root from the transactions
    let mr = merkle_root(&transactions);

    // Mine the block — find a nonce that gives us a hash with enough leading zeros
    println!("Mining block {}...", index);
    let (nonce, hash) = mine_block(index, timestamp, previous_hash, &mr, difficulty);

    println!("  Previous hash: {}...", &previous_hash[..16]);
    println!("  Hash: {}", hash);

    Block {
        index,
        timestamp,
        previous_hash: previous_hash.clone(),
        transactions,
        merkle_root: mr,
        nonce,
        hash,
    }
}

fn validate_block(block: &Block, previous_block: Option<&Block>, difficulty: usize) -> bool {
    // Check three things about a single block:
    // 1. Hash integrity — recomputed hash matches stored hash
    // 2. Chain link — previous_hash matches the actual previous block (if there is one)
    // 3. Proof of work — hash meets difficulty requirement (skip for genesis)
    //
    // Option<&Block> means previous_block can be Some(&block) or None (for genesis)

    // Check 1: Hash integrity
    if !verify_hash(block) {
        println!("INVALID: Block {} hash doesn't match!", block.index);
        return false;
    }

    // Check 2: Chain link (skip genesis — it has no predecessor)
    if let Some(prev) = previous_block {
        if block.previous_hash != prev.hash {
            println!(
                "INVALID: Block {} previous_hash doesn't match Block {}!",
                block.index, prev.index
            );
            return false;
        }
    }

    // Check 3: Proof of work (skip genesis — it wasn't mined)
    if block.index > 0 {
        let target = "0".repeat(difficulty);
        if !block.hash.starts_with(&target) {
            println!("INVALID: Block {} doesn't meet difficulty {}!", block.index, difficulty);
            return false;
        }
    }

    true
}

// === Chain ===

fn view_chain(chain: &Vec<Block>) {
    // Print all blocks in the chain
    println!("\n=== BLOCKCHAIN ({} blocks) ===\n", chain.len());
    for block in chain {
        println!("Block {}", block.index);
        println!("  Timestamp: {}", block.timestamp);
        println!("  Transactions: {}", block.transactions.len());
        for tx in &block.transactions {
            println!(
                "    {} -> {} : {} coins",
                tx.sender, tx.recipient, tx.amount
            );
        }
        println!(
            "  Merkle root: {}...",
            &block.merkle_root[..16.min(block.merkle_root.len())]
        );
        println!("  Nonce: {}", block.nonce);
        println!(
            "  Prev hash: {}...",
            &block.previous_hash[..16.min(block.previous_hash.len())]
        );
        println!("  Hash: {}...", &block.hash[..16]);
        println!();
    }
}

fn validate_chain(chain: &Vec<Block>, difficulty: usize) {
    // Walk every block and validate it using validate_block
    // Now that the logic is extracted, this function is just the loop + summary

    let mut valid = true;

    for i in 0..chain.len() {
        let block = &chain[i];
        // For genesis (i=0), there's no previous block → pass None
        // For all others, pass Some(&previous_block)
        let previous = if i > 0 { Some(&chain[i - 1]) } else { None };

        if !validate_block(block, previous, difficulty) {
            valid = false;
        }
    }

    if valid {
        println!("\nChain is VALID — all {} blocks verified.", chain.len());
    }
}

// === Mining ===

fn mine_block(
    index: u64,
    timestamp: u64,
    previous_hash: &str,
    data: &str,
    difficulty: usize,
) -> (u64, String) {
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

fn adjust_difficulty(chain: &Vec<Block>, current_difficulty: usize) -> usize {
    // Adjust difficulty based on how long the last block took to mine
    // Target: 5 seconds per block (short for a toy chain — Bitcoin targets 600 seconds)
    // If the last block was faster than target → increase difficulty (harder)
    // If slower → decrease difficulty (easier)
    // Minimum difficulty is 1 (at least one leading zero)
    //
    // Bitcoin checks every 2016 blocks; we check every block for faster feedback

    let target_seconds: u64 = 5;

    // Need at least 2 blocks to compare timestamps
    if chain.len() < 2 {
        return current_difficulty;
    }

    let last = &chain[chain.len() - 1];
    let previous = &chain[chain.len() - 2];
    let elapsed = last.timestamp - previous.timestamp;

    if elapsed < target_seconds && current_difficulty < 6 {
        // Too fast — make it harder (cap at 6 to avoid very long mining times)
        println!("  Difficulty UP: {} -> {} (block mined in {}s, target {}s)",
            current_difficulty, current_difficulty + 1, elapsed, target_seconds);
        current_difficulty + 1
    } else if elapsed > target_seconds * 2 && current_difficulty > 1 {
        // Too slow (more than 2x target) — make it easier
        println!("  Difficulty DOWN: {} -> {} (block mined in {}s, target {}s)",
            current_difficulty, current_difficulty - 1, elapsed, target_seconds);
        current_difficulty - 1
    } else {
        // Within acceptable range — no change
        current_difficulty
    }
}

// === Transactions ===

fn read_transaction() -> Transaction {
    // Read a transaction from the user via stdin
    // Returns a Transaction struct with the entered values

    print!("Sender: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut sender = String::new();
    io::stdin().read_line(&mut sender).unwrap();

    print!("Recipient: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut recipient = String::new();
    io::stdin().read_line(&mut recipient).unwrap();

    print!("Amount: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str).unwrap();
    // .trim() removes the newline, .parse() converts string to f64
    let amount: f64 = amount_str.trim().parse().unwrap_or(0.0);

    Transaction {
        sender: sender.trim().to_string(),     // .trim() removes newline from stdin
        recipient: recipient.trim().to_string(),
        amount,
    }
}

fn validate_transaction(chain: &Vec<Block>, tx: &Transaction) -> bool {
    // Check: does the sender have enough coins to cover this transaction?
    // "COINBASE" is a special sender — it means new coins created by mining (no balance needed)
    if tx.sender == "COINBASE" {
        return true;
    }

    let sender_balance = get_balance(chain, &tx.sender);
    if sender_balance < tx.amount {
        println!(
            "INVALID TX: {} has {} coins but tried to send {}",
            tx.sender, sender_balance, tx.amount
        );
        return false;
    }

    true
}

fn get_balance(chain: &Vec<Block>, address: &str) -> f64 {
    // Walk every transaction in every block
    // If you're the recipient, add the amount (you received coins)
    // If you're the sender, subtract the amount (you spent coins)
    // Note: real Bitcoin uses UTXOs, not running balances — this is the simple version
    let mut balance = 0.0;

    for block in chain {
        for tx in &block.transactions {
            if tx.recipient == address {
                balance += tx.amount;
            }
            if tx.sender == address {
                balance -= tx.amount;
            }
        }
    }

    balance
}

// === Persistence ===

fn save_chain(chain: &Vec<Block>) {
    // Serialize the entire chain to YAML and write to disk
    // serde_yaml converts our Vec<Block> into a YAML string
    // fs::write writes that string to a file
    let yaml = serde_yaml::to_string(chain).unwrap();
    fs::write("blockchain.yaml", &yaml).unwrap();
    println!("Chain saved to blockchain.yaml ({} blocks, {} bytes)", chain.len(), yaml.len());
}

fn load_chain() -> Vec<Block> {
    // Read YAML from disk and deserialize back into Vec<Block>
    // fs::read_to_string reads the entire file into a String
    // serde_yaml::from_str converts the YAML string back into our structs
    let yaml = fs::read_to_string("blockchain.yaml").unwrap();
    let chain: Vec<Block> = serde_yaml::from_str(&yaml).unwrap();
    println!("Chain loaded from blockchain.yaml ({} blocks)", chain.len());
    chain
}
