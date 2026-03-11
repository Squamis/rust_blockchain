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

**Next session**:
- Chain validation — verify all links and hashes are correct
- Transactions struct and Merkle root
- Persistence (save/load chain)
