# 🦑 ink!lings

![alt text](<inklings.png>)

> Interactive exercises for learning [ink!](https://use.ink/) - the Rust-based eDSL for writing smart contracts on Polkadot SDK chains.

ink!lings provides a hands-on, interactive way to learn ink! smart contract development through a series of progressively challenging exercises. Similar to [Rustlings](https://github.com/rust-lang/rustlings) for Rust and [Starklings](https://github.com/shramee/starklings-cairo1) for Cairo, ink!lings guides you through the fundamentals and advanced concepts of ink! development.

**ink!** is a Rust-based embedded domain-specific language (eDSL) for writing smart contracts for blockchains built with the Polkadot SDK (formerly Substrate).

## ink! v6 and PolkaVM

**ink! v6 is a major evolution** that compiles to **PolkaVM** (based on RISC-V) instead of WebAssembly:

- **PolkaVM Target**: RISC-V based virtual machine optimized for blockchain
- **Better Performance**: More efficient execution and lower gas costs
- **Enhanced Security**: Improved sandboxing and deterministic execution
- **Polkadot Ecosystem**: Powers smart contracts on Polkadot parachains

## Features

### 1. Comprehensive Learning Path

- **Progressive Difficulty**: Exercises organized from beginner to advanced
- **Interactive Feedback**: Instant verification and helpful error messages
- **Real Examples**: Learn by fixing real ink! contracts
- **Best Practices**: Exercises teach ink! idioms and patterns

### 2. Powerful CLI Tool

- **Auto-Verification**: Compile and test your solutions automatically
- **Watch Mode**: Auto-verify on file save for rapid iteration
- **Hints System**: Get contextual help when stuck
- **Progress Tracking**: Track completed exercises and attempts

### 3. Progress Tracking

- **Completion Stats**: See your overall progress and category breakdowns
- **Attempt Tracking**: Identify challenging exercises
- **Resume Capability**: Pick up where you left off
- **Visual Progress Bar**: Motivating visual feedback

## Installation

### Prerequisites

- **Rust**: 1.75 or later
- **Operating System**: macOS, Linux, or Windows (WSL recommended)

### Step 1: Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Step 2: Clone ink!lings

```bash
git clone https://github.com/Lynette7/ink-lings.git
cd ink-lings
```

### Step 3: Run Setup

```bash
# Set up the correct Rust toolchain
rustup default stable
rustup component add rust-src

# Build the ink!lings CLI
cargo build --release

# Optionally, install globally
cargo install --path .
```

### Step 4: Verify Installation

```bash
# Run this from the inklings directory
cargo run -- list

# Or if you installed globally
inklings list
```

You should see a list of available exercises!

## Quick Start

### Your First Exercise

```bash
# List all exercises
cargo run -- list

# Start with the first exercise
cd exercises/01_intro/intro1

# Read the instructions in src/lib.rs
cat src/lib.rs

# Fix the TODOs, then verify your solution
cargo run -- verify 01_intro/intro1

# Get a hint if you're stuck
cargo run -- hint 01_intro/intro1
```

### Watch Mode (Recommended)

```bash
# Auto-verify whenever you save the file
cargo run -- watch 01_intro/intro1
```

Now edit `exercises/01_intro/intro1/src/lib.rs` in your favorite editor. Every time you save, ink!lings will automatically verify your solution!

## Usage

### CLI Commands

```bash
# List all exercises with completion status
inklings list

# Verify a specific exercise
inklings verify 01_intro/intro1

# Watch mode - auto-verify on save
inklings watch 01_intro/intro1

# Get a hint for an exercise
inklings hint 01_intro/intro1

# View your progress statistics
inklings progress

# Reset all progress
inklings reset
```

## Exercise Structure

Inklings exercises are organized into progressive categories:

### 01_intro - Introduction to ink! contracts

Learn the basics of ink! contract structure, storage, and messages.

- **intro1**: Your first ink! contract - Understanding attributes and structure
- **intro2**: Adding storage - Working with contract state

### 02_basics - Basic Contract Features

Master fundamental ink! concepts and patterns.

- **basic1**: Working with numbers - Arithmetic and error handling
- **basic2**: Using Mappings - Key-value storage in contracts

### 03_storage - Advanced Storage

Learn efficient storage patterns for complex data structures.

- **storage1**: Lazy storage - Optimizing gas with Lazy`<T>`
- **storage2**: Storage vectors - Working with collections

### 04_events - Events and Logging

Understand how to emit and index events for off-chain observers.

- **events1**: Emitting events - Basic event emission
- **events2**: Event topics - Indexing and filtering events

### Exercise Format

Each exercise follows this structure:

```bash
exercises/01_intro/intro1/
├── Cargo.toml          # Project configuration
└── .gitignore
└── lib.rs          # Contract code with TODOs and instructions
```

The `lib.rs` file contains:

- Detailed instructions
- Learning objectives
- TODOs to complete

## Progress Tracking

ink!lings automatically tracks your progress and saves it to `~/.inklings_progress.json`.

### View Your Progress

```bash
inklings progress
```

Output example:

```bash
📊 Your Inklings Progress

Overall: 5/10 exercises completed (50%)

[█████████████████████████░░░░░░░░░░░░░░░] 50%

By Category:
  intro: 2/2 (100%)
  basics: 2/2 (100%)
  storage: 1/2 (50%)
  events: 0/2 (0%)
  advanced: 0/2 (0%)

Most Challenging:
  Using Mappings - 5 attempt(s)
  Lazy storage - 3 attempt(s)

Last Worked On:
  03_storage/storage1 - Lazy storage

Continue with:
  inklings verify 03_storage/storage1
```

### Progress Features

- **Completed Exercises**: Marked with checkmarks in `inklings list`
- **Attempt Tracking**: See how many tries each exercise took
- **Difficulty Insights**: Identify which concepts need more practice
- **Resume Support**: Continue where you left off
- **Visual Progress**: Progress bars and percentages

### Get Help

If you're stuck:

1. **Check the hints**: `inklings hint <exercise>`
2. **Read the exercise comments**: They contain detailed guidance
3. **Open an issue**: [GitHub Issues](https://github.com/Lynette7/ink-lings/issues)

### Building Contracts

```bash
# Build for PolkaVM (RISC-V)
cargo contract build --release

# The output will be in target/ink/
ls target/ink/*.contract
```

The `.contract` file contains:

- Contract bytecode (PolkaVM/RISC-V)
- Metadata (ABI, storage layout)
- Source code hash

## Contributing

Contributions are welcome! Here's how you can help:

### Adding New Exercises

1. Create the exercise structure:

   ```bash
   mkdir -p exercises/XX_category/exerciseN/src
   ```

2. Add a `Cargo.toml`:

   ```toml
   [package]
   name = "exerciseN"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   ink = { version = "6.0.0-beta", default-features = false }

   [lib]
   path = "src/lib.rs"

   [features]
   default = ["std"]
   std = ["ink/std"]
   ```

3. Write the exercise in `lib.rs` with:
   - Clear instructions
   - TODOs for students to complete
   - Unit tests
   - Helpful comments

4. Add metadata to `info/exercises.toml`:

   ```toml
   [[exercises]]
   id = "XX_category/exerciseN"
   name = "Exercise Name"
   path = "exercises/XX_category/exerciseN"
   mode = "test"
   hint = """
   Your detailed hint here...
   """
   ```

5. Add solution to `solutions/XX_category/exerciseN/`

6. Test your exercise:

   ```bash
   cargo run -- verify XX_category/exerciseN
   ```

### Improving Documentation

- Fix typos or unclear instructions
- Add more detailed explanations
- Improve code comments
- Translate to other languages

### Reporting Bugs

Open an issue with:

- What you expected
- What actually happened
- Steps to reproduce
- Your environment (`rustc --version`, `cargo-contract --version`)

### Code Style

- Follow Rust conventions
- Use `cargo fmt` for formatting
- Run `cargo clippy` for lints
- Add tests for new features

## Resources

### Official Documentation

- [ink! Documentation](https://use.ink/) - Official ink! docs
- [ink! Examples](https://github.com/use-ink/ink-examples) - Example contracts
- [cargo-contract](https://github.com/use-ink/cargo-contract) - Build tool

### Polkadot Ecosystem

- [Polkadot Wiki](https://wiki.polkadot.network/) - Learn about Polkadot
- [Polkadot Docs](https://docs.polkadot.io/) - Polkadot documentation
- [Polkadot Stack Exchange](https://substrate.stackexchange.com/) - Q&A forum

### Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust basics
- [Rustlings](https://github.com/rust-lang/rustlings) - Rust exercises

### Community

- [Polkadot Forum](https://forum.polkadot.network/) - Community discussions

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [Rustlings](https://github.com/rust-lang/rustlings)
- Inspired by [Starklings](https://github.com/shramee/starklings-cairo1)
- Built for [ink!](https://github.com/paritytech/ink)

### Happy Learning! 🦑
