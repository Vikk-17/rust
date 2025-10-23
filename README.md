# Rusty Repo

This repo will be focused on
- [ ] Syntax and concepts
    - [x] Phase 1 till I/O project
        - [x] Basic
        - [x] Ownership
        - [x] Struct, enums
        - [x] Pattern matching
        - [x] Generics, Lifetime, Trait
        - [x] Error handling
        - [x] Modular codebase with `use` and `mod`
        - [x] Use of cargo 
        - [x] Writing automated test
        - Need to revisit once at a time or maybe during project

    - [ ] Advanced Concepts

---

## Projects
- [ ] Tui for wifi connections (Rusty Netowrks)
- [ ] TerminalRTC
- [ ] Rusty Vault
- [ ] Text analyzer and file analyzer


## Open Source Projects

- [Cloudillo](https://github.com/cloudillo/cloudillo)
- [Spacepix](https://github.com/CodeCanna/spacepix)
- [Crunch](https://github.com/liamaharon/crunch)

---

### GSOC Projects


---
## Stages

### Level 1: **Tutorial Toy Projects**
- The business logic mostly lives inside main.rs 
- The code is mostly copy-pasted or lightly modified
- No Error handling, test coverage, and project structure


### Level 2: **Structuring Real Woriking Applications** 

- Modular codebase with reusable components
- Code has basic unit or integration test
- Using 3rd party crates like `tokio`, `serde`, and `thiserror`
- Pratical implementations of a few idiomatic patterns
- Minimal architectural desgin
- Little to no consideration for performance, reliability, or maintainability
- Minimal validation and recovery logic
- Little to no good, systematic logging and oversvability
- Handling happy path logic

### Level 3: **The Production Grade System**

- A clean, composable architecture that could be extended
- Well thought out interfaces, configuration logic, and code structure
- Idiomatic Rust patterns and best practices
- Leveraging the type system to enforce constraints at compile time 
- Robust error handling and recovery logic
- Thoroguh testing
- Benchmarking and other performance metrics


### Level 4: **Domain Specific Systems**

- Example:
    - Backend / Cloud Infra: Load balancer
    - Blockchain: Custom decentralized ledger

### Level 5: **Leading Rust Adoption at Workplace**

- Deliver business critical feature
- Trust
- Mentoring
