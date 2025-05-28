# 🧱 Toy Blockchain in Rust

A simple educational blockchain implementation in Rust — built from scratch with minimal dependencies. This project demonstrates key blockchain concepts like hashing, block structure, and proof-of-work consensus, all in ~100 lines of code.

## 🚀 Features

- **Block and Blockchain structs** - Core data structures for blockchain implementation
- **SHA-256 hashing** - Cryptographic hashing of block data using the `sha2` crate
- **Proof-of-Work mining** - Simple difficulty-based mining algorithm
- **Blockchain validity checking** - Verification of chain integrity
- **Genesis block creation** - Automatic creation of the first block
- **Simple CLI output** - Clear console output showing mining progress

## 📦 Project Structure

```
toy_blockchain/
├── src/
│   └── main.rs      # Rust code implementing the blockchain
├── Cargo.toml       # Rust dependencies (sha2 crate)
└── README.md        # You're here!
```

## 🧠 Concepts Covered

This project demonstrates several important blockchain and Rust concepts:

- **Structs and ownership in Rust** - Building data structures with proper memory management
- **SHA-256 hashing** - Using the `sha2` crate for cryptographic functions
- **Timestamps and nonce calculation** - Understanding block timing and mining mechanics
- **Mining loop (Proof of Work)** - Implementing the computational puzzle that secures the blockchain
- **Blockchain immutability and validation** - Ensuring data integrity through cryptographic linking

## 🛠 How to Run

### 1. Prerequisites

First, install Rust if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Clone and Run

```bash
git clone https://github.com/yourusername/toy_blockchain.git
cd toy_blockchain
cargo run
```

You'll see mined blocks and hash outputs in the console as the blockchain grows!

## 📋 Example Output

```yaml
⛏️ Mining block 0...
⛏️ Mined block 0: 00d9b4c8a1f2e3d5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9
⛏️ Mining block 1...
⛏️ Mined block 1: 007f2a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7
⛏️ Mining block 2...
⛏️ Mined block 2: 00b5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5

✅ Blockchain is valid!

Block 0:
  Previous Hash: 0000000000000000000000000000000000000000000000000000000000000000
  Data: Genesis Block
  Nonce: 12847
  Hash: 00d9b4c8a1f2e3d5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9

Block 1:
  Previous Hash: 00d9b4c8a1f2e3d5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9
  Data: Transaction: Alice -> Bob 10 coins
  Nonce: 45923
  Hash: 007f2a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7
```

## 🔧 Configuration

You can modify the mining difficulty by changing the `DIFFICULTY` constant in `main.rs`. Higher values make mining slower but more secure:

```rust
const DIFFICULTY: usize = 2; // Number of leading zeros required
```

## 🏗️ Code Architecture

The blockchain consists of three main components:

### Block Structure
```rust
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    nonce: u64,
    hash: String,
}
```

### Blockchain Structure
```rust
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}
```

### Key Methods
- `new()` - Creates a new blockchain with genesis block
- `mine_block()` - Performs proof-of-work mining
- `add_block()` - Adds a new block to the chain
- `is_valid()` - Validates the entire blockchain

## 🧪 Testing

Run the built-in tests to verify blockchain functionality:

```bash
cargo test
```

## 🚧 Limitations & Future Enhancements

This is an educational implementation with several simplifications:

### Current Limitations
- Single-threaded mining
- No network/peer-to-peer functionality
- No transaction validation
- Fixed mining difficulty
- No persistent storage

### Potential Enhancements
- [ ] Add transaction pool and UTXO model
- [ ] Implement adjustable difficulty algorithm
- [ ] Add network layer for distributed nodes
- [ ] Include digital signatures for transactions
- [ ] Add persistence with database storage
- [ ] Implement Merkle trees for efficient verification
- [ ] Add wallet functionality

## 💡 Educational Value

This project is perfect for:

- **Rust beginners** wanting to build something practical
- **Blockchain enthusiasts** learning core concepts
- **Computer science students** studying distributed systems
- **Developers** exploring cryptocurrency fundamentals

The code is intentionally simple and well-commented to aid learning.

## 🔗 Related Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust programming
- [Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf) - Original blockchain concept
- [Blockchain Demo](https://andersbrownworth.com/blockchain/) - Interactive blockchain visualization
- [SHA-2 Specification](https://tools.ietf.org/html/rfc6234) - Understanding the hash function

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure your code is well-tested and follows Rust conventions.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

```
MIT License

Copyright (c) 2025 [Your Name]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 🙏 Acknowledgments

- Inspired by Satoshi Nakamoto's Bitcoin implementation
- Built with the amazing Rust programming language
- Uses the `sha2` crate for cryptographic hashing

## 📞 Contact

Have questions or suggestions? Feel free to:

- Open an issue on GitHub
- Email: [your.email@example.com]
- Twitter: [@yourusername]

---

**Made with ❤️ and Rust** 

*Happy learning and happy coding!* 🦀
