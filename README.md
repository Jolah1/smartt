# 🧠 Smartt Editor

> A lightweight terminal-based text editor built in Rust using Crossterm.

---

## 📌 Overview

**Smartt Editor** is a minimal, fast, and extensible text editor that runs directly in your terminal.

It is built from scratch in Rust with a focus on:
- ⚡ Performance
- 🧩 Simplicity
- 🛠️ Low-level terminal control
- 🧠 Learning systems programming concepts

This project is inspired by building editors like `kilo`, but implemented using modern Rust tooling.

---

## ✨ Features

- 📄 Open and render files
- ⌨️ Real-time keyboard input handling
- 🧭 Cursor navigation (arrow keys, Home, End, Page Up/Down)
- 🖥️ Full terminal rendering with buffering
- 🧹 Clean terminal state management (raw mode handling)
- 🎯 Minimal and fast

---

## 🎥 Demo Behavior

When you run the editor:

- Displays a welcome screen:
Jolah-Smartt Editor --- Version 0.1.0

- `~` fills empty lines like classic editors
- Opens a file if passed as an argument
- Supports navigation using arrow keys

Exit with:


Ctrl + Q


---

## 🏗️ Architecture

The editor is structured into modular components:

### 🧾 `EditorRows`
- Handles file reading
- Stores and manages text buffer

### 🎯 `CursorController`
- Manages cursor position
- Handles movement logic and bounds

### 🖥️ `EditorContents`
- Output buffer for efficient rendering
- Implements `Write` trait for batching terminal updates

### 📺 `Output`
- Responsible for:
  - Drawing rows
  - Refreshing screen
  - Cursor rendering

### ⌨️ `Reader`
- Handles keyboard input using event polling

### 🧠 `Editor`
- Core orchestrator
- Runs event loop
- Processes keypresses

---

## ⚡ Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/Jolah1/smartt.git
cd smartt
2. Build the project
cargo build
3. Run the editor
cargo run
4. Open a file
cargo run <filename>
```

Example:

cargo run notes.txt
🎮 Controls
Key	Action
↑ ↓ ← →	Move cursor
Home	Move to start of line
End	Move to end of line
Page Up / Down	Scroll
Ctrl + Q	Quit editor
🧠 How It Works
The terminal is switched to raw mode
Input is captured without line buffering
Screen is:
Cleared
Redrawn on every frame
Output is buffered before flushing (efficient rendering)
Cursor is repositioned after each frame
🔐 Terminal Safety

The editor ensures proper cleanup using a Drop implementation:

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

This guarantees your terminal won’t break if the program exits unexpectedly.

🚧 Roadmap
 Text editing (insert/delete)
 File saving
 Syntax highlighting
 Status bar
 Search functionality
 Line numbers
 Undo/Redo
🤝 Contributing

Contributions are welcome!

Fork the repo
Create a feature branch
Make your changes
Submit a PR 🚀

Rust Developer
Bitcoin & Lightning Builder
Open Source Contributor
📚 Inspiration
The original kilo text editor
Low-level terminal programming concepts
Rust systems programming ecosystem
⭐ Support

If you find this project useful:

⭐ Star the repo
🍴 Fork it
🧠 Learn from it

📜 License

MIT License
