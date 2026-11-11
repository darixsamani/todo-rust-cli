# 📝 Todo CLI

A lightweight and efficient command-line todo manager built with **Rust**. Manage your daily tasks directly from your terminal with simple commands, persistent JSON storage, and colored output.

The project demonstrates practical Rust programming concepts, including structs, enums, pattern matching, error handling, file I/O, and serialization.

## ✨ Features

- **Task Management:** Add, complete, list, and delete tasks.
- **JSON Persistence:** Automatically saves tasks in `~/.todos.json`.
- **Colored Terminal Output:** Easily distinguish completed and pending tasks.
- **Command-Line Interface:** Simple and intuitive commands powered by Clap.
- **Safe Error Handling:** Zero `unwrap()` calls in the main application logic.
- **Lightweight:** Fast execution with a minimal dependency footprint.

## 🛠️ Built With

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Clap](https://img.shields.io/badge/Clap-CLI-orange?style=for-the-badge)
![Serde](https://img.shields.io/badge/Serde-Serialization-green?style=for-the-badge)
![JSON](https://img.shields.io/badge/JSON-Data%20Storage-black?style=for-the-badge)
![Cargo](https://img.shields.io/badge/Cargo-Package%20Manager-DEA584?style=for-the-badge&logo=rust&logoColor=white)

### Core Technologies

| Technology | Purpose |
|---|---|
| Rust | Main programming language |
| Clap | Command-line argument parsing |
| Serde | Data serialization and deserialization |
| Serde JSON | JSON file persistence |
| Cargo | Project and dependency management |

## 📦 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

### Clone the repository

```bash
git clone https://github.com/darixsamani/todo-rust-cli.git
cd todo-rust-cli
```

### Build the project

```bash
cargo build --release
```

### Run the application

```bash
cargo run -- add "Learn Rust"
```

You can also install the compiled binary locally:

```bash
cargo install --path .
```

After installation, use the `todo` command directly from your terminal.

## 🚀 Usage

Todo CLI provides four main commands.

### 1. Add a task

Create a new todo item:

```bash
todo add "Learn ownership"
```

Example output:

```text
✅ Added: Learn ownership (#1)
```

### 2. List all tasks

Display all your tasks:

```bash
todo list
```

Example output:

```text
  1. [ ] Learn ownership
  2. [x] Build a CLI application
  3. [ ] Practice Rust pattern matching
```

Pending tasks and completed tasks are displayed with different colors.

### 3. Complete a task

Mark a task as completed using its ID:

```bash
todo done 1
```

Example output:

```text
🎉 Completed!
```

### 4. Delete a task

Remove a task using its ID:

```bash
todo delete 1
```

Example output:

```text
🗑️ Deleted task #1
```

## 💾 Data Persistence

Your tasks are stored locally in a JSON file:

```text
~/.todos.json
```

Example data structure:

```json
[
  {
    "id": 1,
    "title": "Learn ownership",
    "completed": false
  },
  {
    "id": 2,
    "title": "Build a CLI application",
    "completed": true
  }
]
```

The application automatically loads existing tasks when executed and saves changes to the JSON file.

Your tasks remain available between application sessions.

## 🧠 Rust Concepts Demonstrated

This project provides practical experience with fundamental Rust concepts:

| Concept | Application |
|---|---|
| Structs | Representing todo tasks |
| Enums | Modeling CLI commands and application states |
| Pattern Matching | Handling different commands and task states |
| Ownership & Borrowing | Managing task data safely |
| Error Handling | Handling file and serialization errors |
| File I/O | Reading and writing JSON files |
| Serialization | Converting Rust structures into JSON |
| Collections | Managing multiple todo items |

## 📁 Project Structure

```text
todo-rust-cli/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    └── main.rs
```

*This is a suggested initial structure; it can be expanded as the project grows.*

## 🎯 Project Goals

The main goal is to apply fundamental Rust concepts by building a practical command-line application.

Specific objectives:

- Understand how to build CLI applications with Clap.
- Practice defining and using structs and enums.
- Become comfortable with Rust pattern matching.
- Learn how to read and write files.
- Understand JSON serialization and deserialization.
- Practice robust error handling without relying on `unwrap()` in the main logic.

## 🔮 Possible Future Improvements

- [ ] Add task priorities (Low, Medium, High).
- [ ] Add due dates and reminders.
- [ ] Support task editing.
- [ ] Add filtering by completion status.
- [ ] Add task search.
- [ ] Support exporting and importing tasks.
- [ ] Add unit and integration tests.
- [ ] Organize the application into multiple modules.

## 🤝 Contributing

Contributions, suggestions, and improvements are welcome!

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes.
4. Push your branch.
5. Open a Pull Request.

## 📄 License

This project is under MIT Licence