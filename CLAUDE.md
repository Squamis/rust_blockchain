# Rust Blockchain

## Purpose
Learning project — Thomas is building a toy blockchain to understand how Bitcoin's chain mechanics work under the hood while learning Rust. This is about understanding every step, not shipping fast.

## Teaching approach
- Walk through every concept before writing code
- Block out functions with comments before implementing
- Thomas asks questions — answer them, don't skip ahead
- Comments should explain the "why" in plain language

## Architecture

CLI tool with interactive menu loop. All functions implemented:

```
Hashing:          hash_block, hash_transaction, verify_hash, merkle_root
Blocks:           create_genesis_block, create_block, validate_block
Chain:            view_chain, validate_chain
Mining:           mine_block (proof of work), adjust_difficulty
Transactions:     read_transaction, validate_transaction, get_balance
Persistence:      save_chain, load_chain
```

Data flow: `Transactions → Merkle Root → Block Header → Hash → Chain`

## Concepts to learn (in order)

### Phase 1: Hashing & Blocks
- What a hash is and why SHA256 (deterministic, fixed-size output, avalanche effect)
- Block structure: index, timestamp, previous_hash, data, nonce, hash
- Genesis block — the hardcoded first block (no previous hash)
- Linking blocks: each block's `previous_hash` = parent's hash (the "chain")
- Why tampering with one block breaks every block after it

### Phase 2: Proof of Work (Mining)
- The mining puzzle: find a nonce such that hash starts with N zeros
- Difficulty = how many leading zeros required
- Why this is deliberately wasteful (energy → security)
- Difficulty adjustment — Bitcoin retargets every 2016 blocks to maintain ~10 min per block
- Nonce exhaustion and extraNonce

### Phase 3: Transactions
- UTXO model (same as the wallet project) — unspent transaction outputs
- Inputs reference previous outputs, outputs create new UTXOs
- Coinbase transaction — the mining reward (new coins from nothing)
- Merkle tree — hash all transactions into a single root hash for the block header
- Why merkle trees: can prove a transaction is in a block without downloading the whole block (SPV)

### Phase 4: Validation & Persistence
- Validating a single block: hash matches, previous_hash links, proof of work valid
- Validating the full chain: walk from genesis to tip, check every link
- Save/load the chain to disk (serde + YAML, same as wallet project)

## Block struct (target)
```rust
struct Block {
    index: u64,
    timestamp: u64,
    previous_hash: String,
    transactions: Vec<Transaction>,
    merkle_root: String,
    nonce: u64,
    hash: String,
}

struct Transaction {
    sender: String,
    recipient: String,
    amount: f64,
}
```

## Status (as of 2026-03-22)

All 4 phases complete. All core functions implemented and tested.

## Next session plan

**Full top-to-bottom code review + quiz on both blockchain and Rust concepts.**

### Blockchain concepts to quiz (7 topics):
1. Hashing — SHA256, avalanche effect, why it makes chains tamper-evident
2. Block structure — what each field does and why it's there
3. Genesis block — why it's special
4. Chain linking — how previous_hash creates the chain
5. Proof of work — the mining puzzle, difficulty, nonce brute-forcing
6. Merkle trees — how they compress transactions into one root, SPV proofs
7. Transactions — pending pool, COINBASE rewards, balance model, validation

### Rust concepts to quiz (15+ topics):
1. Ownership & borrowing (`String` vs `&str`, `&Block`, `.clone()`)
2. Structs & field init shorthand
3. `Vec<T>` — dynamic arrays, `.push()`, `.last()`, `.drain()`
4. `Option<T>` — None instead of null, `.unwrap()`, `if let Some`
5. `match` — pattern matching
6. Traits — `Digest`, `Serialize`, `Deserialize`
7. `#[derive(...)]` — auto-generating trait implementations
8. `loop`, `break`, `continue`
9. Tuples — returning multiple values `(u64, String)`
10. `for` loops — ranges (`0..n`), iterating over references
11. Closures (preview) — `|x| ...` syntax, `.map()`, `.collect()` (TODO: Thomas deferred learning this)
12. String methods — `.parse()`, `.trim()`, `.starts_with()`, `.repeat()`
13. `f64`, `u64`, `usize` — when to use which numeric type
14. File I/O — `fs::write`, `fs::read_to_string`
15. Serde — serialization/deserialization to YAML

### Quiz format:
- Go function by function through main.rs
- Mix conceptual ("why does X exist?") with code-reading ("what does this line do?")
- Thomas should be able to explain each function's purpose and the Rust mechanics it uses

## Dependencies
- `sha2` — SHA256 hashing
- `hex` — hex encoding hashes
- `serde` + `serde_yaml` — serialization for save/load

## Dev environment
- Rust edition 2024
- LazyVim with rust-analyzer LSP (installed 2026-03-22)
- No sudo access — all tools installed to user space
