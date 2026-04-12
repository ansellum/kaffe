# kaffe
A self-hosted coffee tracking app built using Rust

# TODO
- [ ] Write custom error type
- [ ] automatically capticalize or lowercase items for standardization
  - varietals, region, country, etc.

## Wizard

- [ ] Autocomplete every SQL-retrievable field
  - [x] Look into `complex_autocompletion.rs` example file in inquire, and documentation (obviously)
  - [ ] Pre-initialize items in item wizards s.t. autocomplete can inherit previous entries
    - country -> region -> farm -> producer
- [x] Actually test wizard input for equipment (and coffee once completed)
- [ ] Finish bag and brew wizards
- [ ] Customize styling