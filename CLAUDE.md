# Rust Blockchain

## Purpose
Learning project — Thomas is building a toy blockchain to understand how Bitcoin's chain mechanics work under the hood while learning Rust. This is about understanding every step, not shipping fast.

## Teaching approach
- Walk through every concept before writing code
- Block out functions with comments before implementing
- Thomas asks questions — answer them, don't skip ahead
- Comments should explain the "why" in plain language

## Architecture

CLI tool with 5 function groups, ~14 core functions:

```
Hashing:          hash_block, verify_hash, merkle_root
Blocks:           create_genesis_block, create_block, validate_block
Chain:            add_block, validate_chain, get_chain_length
Mining:           mine_block (proof of work), adjust_difficulty
Transactions:     create_transaction, validate_transaction, get_balance
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

## Next steps (in order)
1. Implement hashing — SHA256 a block, understand the avalanche effect
2. Build Block struct and create_genesis_block
3. Implement create_block and chain linking (previous_hash)
4. Add proof of work mining (find nonce that gives N leading zeros)
5. Add transactions and merkle root
6. Chain validation
7. Persistence (save/load)

## Dependencies (planned)
- `sha2` — SHA256 hashing (already used in wallet project)
- `serde` + `serde_yaml` — serialization (already used in wallet project)
- `hex` — hex encoding hashes (already used in wallet project)

## Dev environment
- Rust edition 2024
- No sudo access — all tools installed to user space
