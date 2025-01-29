<div align="center">

# Passman 🔐

**Secure multi-user CLI password manager with local encrypted storage**

[![Rust](https://img.shields.io/badge/Built_with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![GitHub repo size](https://img.shields.io/github/repo-size/R1c4rdCo5t4/passman)

Passman is a command-line password manager that enables multiple users to securely store and manage encrypted credentials locally. Passwords are stored in vaults that can only be accessed with the vault's master password.

</div>

---

## ✨ Features

- ✅ Credentials stored locally in encrypted vaults
- 👥 Multi-user support with separate vaults
- 📋 Copy credentials to clipboard
- 🛠️ Password generator and strength analyzer
- 🚫 No internet connection required

## 🔑 Security

- 🔒 AES-256-GCM-SIV encrypted vault storage
- 🔐 Argon2id key derivation
- 🧹 Zeroized memory for sensitive data
- 👤 Session management with TTL expiration

## Commands

```
  help [<command>]                           > Get this message or help for specific commands
  clear                                      > Clear the screen
  exit                                       > Exit the program
  vault new <name>                           > Create a new vault
  vault open <name>                          > Enter a vault
  vault close                                > Exit the current vault
  vault list                                 > List all vaults
  vault show [<service>] [-expose]           > Show all or specific credentials in the vault
  vault add <service>                        > Add new credentials to the vault
  vault update <service> <field <value>      > Update a specific field of credentials 
  vault delete <service>                     > Delete credentials
  vault copy <service> <field>               > Copy a specific field to the clipboard
  vault destroy                              > Delete vault
  panic                                > Clears clipboard, closes vault, clears screen and exits
  analyze <password>                         > Analyze the strength of a password
  generate <length> [-symbols] [-copy] [-avoid-ambiguous]
                                             > Generate a new password
```

---

## Running

```
cargo run
```