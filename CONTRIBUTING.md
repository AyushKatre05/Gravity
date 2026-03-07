# 🤝 Contributing to Gravity

First off, thank you for considering contributing to Gravity! It is people like you that make Gravity such a powerful tool.

## 🎯 Our Main Goal: Multi-Language Support

Currently, Gravity is a specialized tool for **Rust** codebases. Our primary objective is to expand this capability to include other languages like **Python, JavaScript, TypeScript, Go, and Java**.

### How Language Analysis Works in Gravity

To add support for a new language, you'll primarily interact with two components in the `backend`:

1.  **[parser.rs](file:///c:/Users/ayush/OneDrive/Desktop/project/Gravity/backend/src/parser.rs)**: This is where the directory walking and file parsing happens. 
    *   Currently, it filters for `.rs` files.
    *   It uses Regex for basic extraction, but we are moving towards full [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) integration for more robust parsing.
    *   **Contribution Opportunity**: Implement a language-specific parsing logic that identifies functions, imports, and class/struct definitions for your language.

2.  **[complexity.rs](file:///c:/Users/ayush/OneDrive/Desktop/project/Gravity/backend/src/complexity.rs)**: This module calculates the complexity "score" for functions.
    *   **Contribution Opportunity**: Define what "complexity" means for your language. Does it follow cyclomatic complexity rules? How should different language features (like decorators or decorators) affect the score?

3.  **[db.rs](file:///c:/Users/ayush/OneDrive/Desktop/project/Gravity/backend/src/db.rs) & Migrations**:
    *   If your language required new metadata (e.g., Python `decorators`), you might need to extend the database schema in `backend/migrations/`.

## 🛠️ Development Workflow

1.  **Fork the repo** and create your branch from `main`.
2.  **Run locally** using Docker Compose to ensure the environment works.
3.  **Write tests** for your new parser logic.
4.  **Submit a PR**: Detailed descriptions of your changes help us review faster!

## 📜 Code of Conduct

Please be respectful and helpful. We aim to build a welcoming community around code intelligence.

## ❓ Questions?

Feel free to open an issue or reach out via our discussions page if you need help understanding the architecture or where to start!
