<div align="center">

# Passman 🔐

**Secure multi-user CLI password manager with local encrypted storage**

[![Rust](https://img.shields.io/badge/Built_with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![GitHub repo size](https://img.shields.io/github/repo-size/R1c4rdCo5t4/passman)

</div>

---

## ✨ Features

- ✅ Credentials stored locally in encrypted vaults
- 👥 Multi-user support with separate vaults
- 📋 Copy credentials to clipboard
- 🛠️ Password generator and strength analyzer

## 🔑 Security

- 🔒 AES-256-GCM-SIV encryption
- 🔐 Argon2id key derivation
- 🧹 Credential leak prevention with the `secrecy` and `zeroize` libraries
- 🛡️ Session and clipboard TTL

## 👨‍💻 Commands

```
  help [<command>]                           > Get this message or help for specific commands
  clear                                      > Clear the screen
  exit                                       > Exit the program
  panic                                      > Clears clipboard, closes vault, clears screen and exits
  vault new <name>                           > Create a new vault
  vault open <name>                          > Enter a vault
  vault close                                > Exit the current vault
  vault list                                 > List all vaults
  vault show [<entry>] [-expose]             > Show all or specific credentials in the vault
  vault add <entry>                          > Add new credentials to the vault
  vault update <entry> -<field> <value>      > Update a specific field of credentials 
  vault delete <entry>                       > Delete credentials
  vault copy <entry> [-<field>]              > Copy a specific field to the clipboard
  vault destroy                              > Delete vault
  analyze <password>                         > Analyze the strength of a password
  generate <length> [-symbols]               > Generate a new password
           [-copy]  [-avoid-ambiguous]                               
```

---

## 🚀 Running

### 📋 Prerequisites

- [Rust](https://www.rust-lang.org/)

### 🛠️ Setup

1. Clone the repository

```sh
git clone https://github.com/R1c4rdCo5t4/passman.git
```

2. Change to the project directory

```sh
cd passman
```

3. Build the project

```sh
cargo build
```

4. Run the project

```sh
cargo run
```

## 🤝 Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## 📄 License

See [`LICENSE`](/LICENSE) for more information.