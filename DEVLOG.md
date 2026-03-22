# Dev Log

Learning project — building a toy blockchain from scratch to understand how Bitcoin's chain mechanics work under the hood, while learning Rust. AI-assisted with Claude as a tutor, not a ghostwriter.

## 2026-03-11

**Session goal**: Set up the project, implement block struct, hashing, block creation, chaining, and proof of work mining.

**What I learned**:
- A blockchain is a linked list where each link is a SHA256 hash of the previous block
- Tampering with one block changes its hash, which breaks every block after it (chain is tamper-evident)
- Proof of work makes tampering expensive — must find a nonce that gives a hash with N leading zeros
- Difficulty 4 (~65,000 guesses average), each additional zero = ~16x harder (hex character = 16 values)
- Bitcoin adjusts difficulty every 2016 blocks to maintain ~10 min per block
- Distribution (layer 4) = thousands of nodes hold copies, follow "longest chain" rule
- Merkle trees let you verify a single transaction is in a block without downloading all transactions
- `Option<T>` replaces null in Rust — forces you to handle the None case, no null pointer crashes
- Ownership: every value has exactly one owner, when owner goes out of scope value is freed
- `String` = owned heap string (like C++ std::string), `&str` = borrowed reference (like C++ const char*)
- `String::from("text")` creates an owned String, `.clone()` copies an owned value
- `&Block` = immutable borrow (read-only reference), doesn't transfer ownership
- `loop` = infinite loop (like while(true) in C++), runs until return/break
- `"0".repeat(n)` = create a string of n repeated characters
- `.starts_with()` = check string prefix
- `usize` = platform-sized unsigned int, used for indexing and lengths
- Field init shorthand: `Block { index, timestamp, ... }` when variable name matches field name
- Traits = like C++ pure virtual classes / interfaces — define methods a type must implement
- `Digest` trait gives SHA256 its `.update()` and `.finalize()` methods

**What I built**:
- `Block` struct: index, timestamp, previous_hash, data, nonce, hash
- `hash_block` — SHA256 all block fields, returns hex string
- `create_genesis_block` — hardcoded block 0 with no predecessor
- `create_block` — chains to previous block via previous_hash, includes mining
- `mine_block` — proof of work: brute force nonces until hash starts with N zeros
- Tested: 3-block chain with difficulty 4, blocks properly linked, mining takes ~60K-160K guesses

**Decisions made**:
- Start with `data: String` instead of transactions — keep it simple while learning hashing/chaining
- Using `sha2` and `hex` crates (same as wallet project)
- No GitHub remote yet

**What I built (continued)**:
- `Vec<Block>` chain storage — blocks stored in a dynamic array instead of loose variables
- `view_chain` — prints all blocks with their fields
- `validate_chain` — walks the chain checking 3 things per block:
  1. Recompute hash from fields — does it match the stored hash?
  2. Does previous_hash match the actual previous block's hash?
  3. Does the hash satisfy the difficulty requirement (leading zeros)?
- `chain.last().unwrap()` to reference the most recent block when appending
- `.min()` to safely slice short strings (genesis block's previous_hash is just "0")

**Next session**:
- ~~REVIEW: Walk through validate_chain code in detail (haven't reviewed it yet)~~ DONE
- ~~Transactions struct and Merkle root~~ DONE
- ~~Persistence (save/load chain)~~ DONE

## 2026-03-22

**Session goal**: Review validate_chain, implement transactions + Merkle root, refactor to interactive menu loop, add difficulty adjustment and persistence.

**What I learned (blockchain)**:
- Reviewed validate_chain's 3 checks: hash integrity, chain linking, proof of work — and why all three together matter (each alone is bypassable, combined they force re-mining)
- Merkle trees: hash pairs of transactions recursively until one root hash remains
- SPV (Simplified Payment Verification): lightweight wallets use Merkle proofs to verify a transaction is in a block with just log2(N) hashes instead of downloading all transactions
- COINBASE transaction: special tx with no sender, creates new coins as mining reward — this is how new Bitcoin enters circulation
- Pending transaction pool: transactions wait in a pool until a miner includes them in a block — you can't spend unconfirmed coins
- Difficulty adjustment: if blocks mine too fast, increase difficulty; too slow, decrease. Our toy chain checks every block, Bitcoin checks every 2016 blocks

**What I learned (Rust)**:
- `f64` — 64-bit floating point (real Bitcoin uses integer satoshis to avoid rounding)
- `vec![]` macro — creates a Vec with initial values, shorthand for Vec::new() + push()
- Closures: `|tx| hash_transaction(tx)` — anonymous functions. Chose to use a for loop instead for now (TODO: come back and learn .iter().map().collect() pattern)
- `.drain(..)` — removes all elements from a Vec and returns them (ownership transfer)
- `.parse()` — converts a string to another type, returns Result
- `.unwrap_or(default)` — unwrap a Result, use default if it fails
- `.to_string()` — converts &str to owned String
- `continue` — skip rest of loop body, jump to top
- `break` — exit the loop
- `if let Some(prev) = previous_block` — pattern matching on Option, safer than .unwrap()
- `Option<&Block>` — a reference that might not exist (Some or None)
- `#[derive(Serialize, Deserialize)]` — auto-generate trait implementations for converting structs to/from YAML
- `fs::write` / `fs::read_to_string` — file I/O
- `serde_yaml::to_string` / `serde_yaml::from_str` — serialize/deserialize structs

**What I built**:
- `Transaction` struct: sender, recipient, amount
- `hash_transaction` — SHA256 a single transaction for Merkle tree leaves
- `merkle_root` — recursively hash pairs of transaction hashes into a single root
- Updated `Block` struct: replaced `data: String` with `transactions: Vec<Transaction>` and `merkle_root: String`
- Updated `hash_block` to use merkle_root instead of data
- `read_transaction` — interactive transaction input from stdin
- `validate_transaction` — checks sender has enough balance (with COINBASE exemption)
- `get_balance` — walks chain summing received/sent amounts per address
- `verify_hash` — extracted from validate_chain: recompute and compare a block's hash
- `validate_block` — extracted from validate_chain: checks integrity, linking, and proof of work for a single block
- Refactored `validate_chain` to use the extracted functions
- `adjust_difficulty` — increases/decreases difficulty based on mining time vs 5s target
- `save_chain` — serialize chain to YAML with serde
- `load_chain` — deserialize chain from YAML, auto-validates on load
- Refactored `main` into a persistent menu loop with shared chain, difficulty, and pending transaction pool

**Decisions made**:
- Simple balance model (sum sent/received) instead of full UTXO — good enough for learning
- Difficulty adjustment every block (not every 2016) for faster feedback
- 5 second target block time for the toy chain
- Added serde + serde_yaml dependencies for persistence

**Next session**:
- Full top-to-bottom code review and quiz on blockchain + Rust concepts
- Consider: push to GitHub
