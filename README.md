# Passman

Simple CLI password manager build using Rust that allows multiple users to store encrypted passwords locally.

![](https://skillicons.dev/icons?i=rust)

## Commands

```
  help [<command>]                           > Get detailed help for a specific command
  clear                                      > Clear the screen
  exit                                       > Exit the program
  vault new <name>                           > Create a new vault
  vault open <name>                          > Enter a vault
  vault close                                > Exit the current vault
  vault list                                 > List all vaults
  vault show [<service>] [-mask]             > Show all or specific credentials in the vault (mask hides the password)
  vault add <service> <username> <password>  > Add new credentials to the vault
  vault update <service> <field <value>      > Update a specific field of credentials 
  vault delete <service>                     > Delete credentials
  vault copy <field>                         > Copy a specific field to the clipboard
  vault search <query>                       > Search in the vault by query
  vault destroy                              > Delete vault
  analyze <password>                         > Analyze the strength of a password
  generate <length> [-symbols] [-copy] [-avoid-ambiguous]
                                             > Generate a new password
```

## How to run 

```
cargo run
```