# kaffe
A self-hosted coffee tracking app built using Rust

# TODO
- [x] Define command line input
- [x] Write helper default function for wrong command input
- [x] Figure out help message behavior
- [x] Figure out ID system 
    - Will use SQLite database tags
- [ ] Improve error handing
    - Marked in-line
- [ ] Save temporary structs of each data type
    - [x] Equipment is in json form
    - [ ] Bag and Coffee are in csv form
        - IDEA! Give structs w/ complex fields (e.g. Vec<>) a dedicated CSV constructor. CSV constructor can be another struct with String fields, wherein the String fields are constructed into vectors during runtime.
        - Better idea: Use a `_str` variant of each `Vec<String>` field, and figure out how to use serde to deserialize from that `_str` field
- [ ] Begin wizard