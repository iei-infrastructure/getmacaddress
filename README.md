# getmacaddress

![build](https://github.com/organizations/iei-infrastructure/getmacaddress/actions/workflows/rust.yml/badge.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight Rust utility for retrieving the **hostname** and **MAC address** of Wi-Fi (IEEE 802.11) network interfaces on Windows.

---

## ✨ Features

- ✅ Displays hostname and physical MAC addresses of Wi-Fi adapters
- ✅ Filters out virtual interfaces
- ✅ Copies physical MAC address to clipboard
- ✅ Terminal-colored output (PowerShell, Windows Terminal)
- ✅ Single `.exe`, no installation needed

---

## 🧾 Requirements

- Windows 10 or later
- Rust toolchain (stable)
- GNU toolchain (`x86_64-pc-windows-gnu`) if building without MSVC

---

## 🚀 Getting Started

### 🔧 Clone the Repository

```bash
git clone https://github.com/organizations/iei-infrastructure/getmacaddress/getmacaddress.git
cd getmacaddress
```

## 🛠️ Build the Project
### Option 1: Build with MSVC (default)
```bash
cargo build --release
```
### Option 2: Build with GNU (no MSVC)
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The executable will be located at:
```bash
target/release/getmacaddress.exe
# or if using GNU
target/x86_64-pc-windows-gnu/release/getmacaddress.exe
```

## 🧪 Run the Program
```bash
./target/release/getmacaddress.exe
```

Example output:
```
 ___ _____ ___   _____           _
|_ _| ____|_ _| |_   _|__   ___ | |___
 | ||  _|  | |    | |/ _ \ / _ \| / __|
 | || |___ | |    | | (_) | (_) | \__ \
|___|_____|___|   |_|\___/ \___/|_|___/

getmacaddress v0.1.0
Copyright (C) 2025 PT. Indonesia Epson Industry

A lightweight utility for retrieving hostname and MAC address information.

Hostname   : PC012345

Wi-Fi Interfaces:

Description: Microsoft Wi-Fi Direct Virtual Adapter #3
MAC Address: 00-A5-54-04-05-06

Description: Microsoft Wi-Fi Direct Virtual Adapter #4
MAC Address: 02-A5-54-04-07-09

Description: Intel(R) Wi-Fi 6E AX211 160MHz
MAC Address: 00-A5-54-04-09-0B ✔ MAC address copied to clipboard. You can paste it anywhere.

Hit Enter to continue...
```

### 🏁 Creating a Release (GitHub Actions)

To create a GitHub release with an .exe binary attached:

1. Commit all your changes:
   ```bash
   git add .
   git commit -m "Prepare release v0.1.0"
   ```

2. Tag the commit:
   ```bash
   git tag v0.1.0
   ```

3. Push the tag to GitHub:
   ```bash
   git push origin v0.1.0
   ```

➡️ This triggers the GitHub Actions workflow (rust.yml) to:

- Build the .exe
- Upload it as an artifact
- Create a release with the .exe attached

## 📝 License

This project is licensed under the MIT License.

© 2025 PT. Indonesia Epson Industry