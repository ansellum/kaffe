# kaffe
A self-hosted coffee tracking app built using Rust

# TODO
- [x] Define command line input
- [x] Write helper default function for wrong command input
- [x] Figure out help message behavior
- [x] Figure out ID system 
    - Will use SQLite database tags
- [x] Save temporary structs of each data type
    - [x] Equipment is in json form
    - [x] Bag and Coffee are in csv form
        - [x] IDEA! Give structs w/ complex fields (e.g. Vec<>) a dedicated CSV constructor. CSV constructor can be another struct with String fields, wherein the String fields are constructed into vectors during runtime.
        - [x] Better idea: Use a `_str` variant of each `Vec<String>` field, and figure out how to use serde to deserialize from that `_str` field
- [ ] Fix schema s.t. only unique coffees by (roaster, name) are allowed

- [ ] Begin wizard
- [ ] Improve error handing
    - Marked in-line