# Evaporate 

<p align="center">
  <img src="./app-logo.png" alt="Evaporate App Logo" width="150" />
</p>

Evaporate is a lightweight, cross-platform desktop application designed to securely remove your digital footprint. Built using a robust **Rust backend** (powered by Tauri v2) and a blazing-fast **vanilla JavaScript, HTML, and CSS frontend**, Evaporate handles critical file-sanitization routines across Windows, macOS, and Linux.

---

## The Motivation: Why I Built This

Have you ever traveled without your laptop, logged into a public computer workstation to handle important work, and felt the frustration of manually logging out of dozens of web pages and scouring the downloads folder to wipe your files? 

I built Evaporate to eliminate that exact anxiety. Instead of manually digging through settings menus on a public machine, you can simply carry a flash drive containing the compiled Evaporate executable. Plug it in, run it, and instantly "evaporate" your active sessions, browser caches, and download history in a single click before you walk away.

---

##  Features

* **Portable & Executable:** Runs straight from a USB flash drive with zero installation required on the host machine.
* **Secure File Wipe:** Overwrites targeted files with zeros to prevent forensic data recovery.
* **Browser Data Sanitization:** Automatically detects and wipes cookie caches and session databases for popular browsers:
  * Google Chrome
  * Microsoft Edge
  * Mozilla Firefox
* **Cross-Platform Delivery:** Natively compiled and optimized via CI/CD for:
  *  Windows (`.exe`)
  *  macOS (`.dmg`)
  *  Linux (`.deb`)
* **Privacy Focused:** Operates entirely locally on your system environment with no telemetry or remote analytics tracked.

---

##  Tech Stack

* **Frontend:** Vanilla HTML5, CSS3, JavaScript (ES6+)
* **Backend Runtime:** Tauri v2 & Rust
* **Automation:** GitHub Actions for multi-platform build and deployment pipeline

---

##  Getting Started

### Prerequisites
Ensure you have the following installed on your machine:
* **Node.js** (LTS variant recommended)
* **Rust & Cargo** (via [rustup](https://rustup.rs/))
* *Linux only:* Core compilation environments (`pkg-config`, `libdbus-1-dev`)

### Installation & Development Run

1. Clone the repository down locally:
   ```bash
   git clone https://github.com/Euuiiii/Evaporate.git
   cd Evaporate

    Install the necessary frontend dependencies:
    npm install

    Launch the application environment in live development mode:
    npm run tauri dev



# OS Security & Installation Warnings

Because these binary executables are community-driven, open-source builds that are not signed with commercial corporate code signing certificates, your operating system security filters will flag the app on initial startup. Follow these simple steps to clear the bypass:
## Windows SmartScreen Bypass

    Click More info on the blue warning popup dialogue.

    Click the Run anyway button that appears at the bottom right corner.

## macOS Gatekeeper Bypass

    Open your Mac's System Settings and navigate over to Privacy & Security.

    Scroll down to the Security section header.

    Locate the note stating "Evaporate was blocked from use because it is not from an identified developer" and click Open Anyway.

    Confirm with your system password and launch.

## Linux Execution

Linux distributions do not enforce corporate developer signing restrictions. Simply install the .deb package setup file or launch the executable binary package directly within your terminal window shell.

# Contributing

We welcome community optimization suggestions! If you want to dive into the codebase:

    Review our core repository blueprint layout in CONTRIBUTING.md.

    Ensure your Rust adjustments cleanly pass target execution routines across Windows, macOS, and Linux runners before submitting a Pull Request.

# License

This project is licensed under the MIT License.