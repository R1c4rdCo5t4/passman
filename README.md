# Passman

Simple CLI password manager build using Rust that allows multiple users to store encrypted passwords locally.

![](https://skillicons.dev/icons?i=rust)

## Commands

```
help
help <command>
exit

session new
session login
session status
session extend
session logout
session panic
session delete

credentials show [<name>] [-mask]
credentials add -name <name> -user <user> -password <password> [-generate ...]
credentials update -field <field>
credentials delete <name>
credentials copy -field <field>
credentials search <query>
credentials generate -length <length> [-symbols] [-copy] [-avoid-ambiguous] [-min-symbols <int>] [-min-digits <int>]
credentials analyze <password>

folders show [<name>]
folders add <name>
folders delete <name>
folders rename <prev-name> <new-name>


```

## How to run 

```
cargo run
```