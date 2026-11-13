<div align="center">

# 📝 todo-rust-cli

**A fast, minimal todo manager for your terminal, written in Rust.**

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Cargo](https://img.shields.io/badge/Cargo-CE412B?style=for-the-badge&logo=rust&logoColor=white)
![Clap](https://img.shields.io/badge/clap-CLI-blue?style=for-the-badge)
![Serde](https://img.shields.io/badge/serde__json-JSON-orange?style=for-the-badge)
![License: MIT](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

</div>

---

## ✨ Features

- ➕ Add, ✅ complete, 📋 list and 🗑️ delete tasks from the command line
- 💾 Persistent storage as JSON in `~/.todos.json`
- 🎨 Colored output: pending and completed tasks are visually distinct
- 🛡️ Robust error handling: **zero `unwrap()`** in the main logic
- 🦀 Idiomatic Rust: structs, enums, pattern matching, `Result`-based flow

## 🚀 Installation

```bash
git clone https://github.com/darixsamani/todo-rust-cli.git
cd todo-rust-cli
cargo install --path .
```

Or run it without installing:

```bash
cargo run -- <command>
```

> [!NOTE]
> Requires Rust 1.70+ ([install via rustup](https://rustup.rs)).

## 📖 Usage

```console
$ todo add "Learn ownership"
✅ Added: Learn ownership (#1)

$ todo list
  1. [ ] Learn ownership

$ todo done 1
🎉 Completed!
```

### Commands

| Command | Description | Example |
|---------|-------------|---------|
| `add <title>` | Add a new task | `todo add "Learn ownership"` |
| `list` | Show all tasks | `todo list` |
| `done <id>` | Mark a task as completed | `todo done 1` |
| `delete <id>` | Remove a task | `todo delete 1` |

Run `todo --help` for the full CLI reference.

## 💾 Data Storage

Tasks are stored as JSON in `~/.todos.json`. The file is created automatically on first use.

```json
[
  { "id": 1, "title": "Learn ownership", "done": true },
  { "id": 2, "title": "Read the Rust Book", "done": false }
]
```

> [!TIP]
> The file is plain JSON, so you can back it up, sync it, or edit it by hand.

## 🗂️ Project Structure

```text
todo-rust-cli/
├── Cargo.toml
└── src/
    └── main.rs      # CLI parsing, commands, persistence
```

## 🧠 Rust Concepts Demonstrated

| Concept | Where it is used |
|---------|------------------|
| **Structs** | `Todo` model (id, title, done) |
| **Enums** | `Commands` subcommands parsed by clap |
| **Pattern matching** | Dispatching commands, handling `Result` / `Option` |
| **File I/O** | Reading and writing `~/.todos.json` |
| **serde_json** | (De)serializing tasks |
| **Error handling** | `Result` and `?` instead of `unwrap()` |

## 🛠️ Built With

| Crate | Purpose |
|-------|---------|
| [clap](https://crates.io/crates/clap) | Command-line argument parsing |
| [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) | JSON serialization |
| [colored](https://crates.io/crates/colored) | Terminal colors |

## 🧪 Development

```bash
cargo build           # compile
cargo test            # run tests
cargo clippy          # lint
cargo fmt             # format
```

## 🗺️ Roadmap

- [x] `edit` command to rename a task
- [ ] Due dates and priorities
- [x] `list --pending` / `--done` filters
- [ ] Custom storage path via env variable

## 🤝 Contributing

Issues and pull requests are welcome. Please run `cargo fmt` and `cargo clippy` before submitting.

## 📄 License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for details.