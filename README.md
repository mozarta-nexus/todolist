Todo List Manager

A modern command-line task management tool built with Rust, featuring a beautiful interface and powerful functionality.



✨ Features

• 🎨 Beautiful Interface - Colorful output, emojis, and table displays

• 📊 Smart Filtering - Filter tasks by status, priority, category, and date

• 🏷️ Tag System - Add custom tags to tasks

• 📅 Flexible Dates - Support for absolute dates (YYYY-MM-DD) and relative dates (+ndays)

• 📁 Data Persistence - Automatic saving to local files

• 🔄 Category Management - Task categorization and category operations

• ⚡ High Performance - Built with Rust for fast startup and execution

📦 Installation

Prerequisites

• Rust 1.70+ and Cargo

• Terminal with color support

Build from Source

```shell
# Clone the repository
git clone https://github.com/your-username/todolist.git
cd todolist

# Build the project
cargo build --release

# Install to system path (optional)
cargo install --path .
```

🚀 Quick Start

Add a Task
```shell
# Simple task
todo add "Buy groceries"

# Task with priority and category
todo add "Complete project report" -p high -c work

# Set due date and tags
todo add "Prepare for meeting" -d +2days -t "work,urgent" -p critical

```

List Tasks

```shell
# List all tasks
todo list

# List only pending tasks
todo list --status pending

# List high priority tasks
todo list --priority high

# List tasks due today
todo list --today

# List overdue tasks
todo list --overdue
```

Manage Tasks
```shell

# Mark task as complete
todo complete 1

# Edit a task
todo edit 1 --name "Updated task name" --priority medium

# Remove a task
todo remove 1
```

📋 Complete Usage Guide

Add Task (add)

```bash
todo add <NAME> [OPTIONS]
```
Options:

• -p, --priority <PRIORITY> - Priority level (lowmedium high
critical)

• -c, --category <CATEGORY> - Category name

• -d, --due <DUE> - Due date (YYYY-MM-DD or +ndays)

• -t, --tags <TAGS> - Tags (comma-separated)

Examples:
```bash
todo add "Read book" -p low -c personal -d +7days -t "learning,books"
```

List Tasks (list)

```bash
todo list [OPTIONS]
```
Options:

• -s, --status <STATUS> - Filter by status (pendingcompleted
all)

• -p, --priority <PRIORITY> - Filter by priority

• -c, --category <CATEGORY> - Filter by category

• --overdue - Show only overdue tasks

• --today - Show only tasks due today

Edit Task (edit)
```bash
todo edit <ID> [OPTIONS]
```
Options:

• -n, --name <NAME> - New task name

• -p, --priority <PRIORITY> - New priority level

• -c, --category <CATEGORY> - New category

• -d, --due <DUE> - New due date

Other Commands

• complete <ID> - Mark task as completed

• remove <ID> - Remove a task

• category list - List all categories

🎨 Interface Showcase

Task List Example
```bash
✨ Todo List Manager
========================================

ID  Status  Priority  Task          Category  Due Date  Tags
1   PENDING HIGH      Buy groceries personal in 2 days food,shopping
2   DONE    MEDIUM    Read book     personal -          learning
3   PENDING CRITICAL  Fix bug       work     OVERDUE    urgent

```

Color Coding

• 🔴 Red - Overdue tasks, critical priority

• 🟡 Yellow - High priority, due today

• 🟢 Green - Completed tasks

• 🔵 Blue - Low priority

• ⚪ Gray - Text for completed tasks

🔧 Configuration

Application data is automatically saved to system data directories:
• Linux: ~/.local/share/todolist/tasks.json

• macOS: ~/Library/Application Support/todolist/tasks.json

• Windows: %APPDATA%\todolist\tasks.json

🏗️ Project Structure
```shell
todolist/
├── src/
│   ├── main.rs          # Main program entry
│   ├── core/
│   │   ├── arg.rs       # CLI argument definitions
│   │   ├── task.rs      # Task data structures
│   │   └── store.rs     # Data storage management
│   ├── ui.rs            # User interface enhancements
│   └── error.rs         # Error handling
├── Cargo.toml           # Project configuration and dependencies
└── README.md           # Project documentation
```

🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (git checkout -b feature/AmazingFeature)
3. Commit your changes (git commit -m 'Add some AmazingFeature')
4. Push to the branch (git push origin feature/AmazingFeature)
5. Open a Pull Request

📄 License

This project is licensed under the Apache 2.0 License - see the LICENSE file for details.

🙏 Acknowledgments

• https://github.com/clap-rs/clap - Powerful command-line argument parsing

• https://github.com/chronotope/chrono - Date and time handling

• https://github.com/mackwic/colored - Terminal color output

• https://github.com/phsym/prettytable-rs - Beautiful table displays

📞 Support

If you encounter issues or have suggestions:

1. Check https://github.com/mozarta-nexus/todolist/issues for existing discussions
2. Create a new issue describing the problem
3. Or contact via email: 1447153224@qq.com

Enjoy productive task management! ✨