# Contributing to Evaporate

Thank you for your interest in improving Evaporate! To help you get started with the codebase, here is a breakdown of our cross-platform architecture.

##  Project Architecture & File Structure

The project splits concerns cleanly between a vanilla web frontend and a secure Rust system backend powered by Tauri v2.

```text
evaporate-app/
├── .github/workflows/   # CI/CD release automation configurations
├── .vscode/             # Editor runtime environment preferences
├── src/                 # Frontend application core
│   ├── assets/          # Static icons, image structures, and graphical themes
│   ├── index.html       # Primary application layout
│   ├── style.css        # Clean user interface styling sheet
│   └── ui.js            # Core user interactions and Tauri event invocations
└── src-tauri/           # Native system backend engine
    ├── capabilities/    # Tauri security plugin permission scopes
    ├── src/
    │   ├── lib.rs       # Secure wipe commands and cross-platform OS logic
    │   └── main.rs      # Desktop application lifecycle boot entry point
    ├── Cargo.toml       # Rust dependency manifest
    └── tauri.conf.json  # Core Tauri system capability permissions configuration
```

# Local Development Setup

    Prerequisites: Ensure you have Rust, Node.js, and your platform's system dependencies installed (pkg-config and libdbus-1-dev for Linux environments).

    Installation: Clone the repository and run npm install.

    Execution: Launch the application environment in development mode using:
    Bash

    npm run tauri dev

# Pull Request Guidelines

    Ensure your Rust code changes cleanly compile across Windows, macOS, and Linux runners.

    Keep platform-specific path overrides cleanly encapsulated within isolated conditional compilation attributes (#[cfg(target_os = "...")]).