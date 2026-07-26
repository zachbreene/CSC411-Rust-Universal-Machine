<h1 align=center> RUM: Rust Universal Machine </h1>
<h2 align=center> A CSC411: Computer Organization Project by Zach Breene and C. Wyatt Polasek </h2>
<h4 align=center> Created at the University of Rhode Island, December 2023 </h4>

## Introduction
The primary objective of this project was to implement a Universal Machine emulator built entirely in Rust. The emulator replicates the specific architecture of a Universal Machine, managing a segmented memory structure, executing a distinct instruction set architecture, and handling CPU register manipulations to process and run binary programs.

---

## Implementation + Functions
### rum/src/

This directory houses the core modular components of the Universal Machine, all configured via standard `Cargo.toml` management, tied together by `lib.rs`, and initialized through `main.rs`. <br>

&emsp; ***Core Modules Method***

* The `execution.rs` module houses the main execution loop and handles the processing and transitioning of machine states.
* The `instructions.rs` module defines the machine's instruction set architecture and contains the parsing logic to decode incoming 32-bit binary commands.
* The `memory.rs` module manages the machine's segmented memory structure, providing secure allocation, read, and write operations.
* The `registers.rs` module implements the CPU registers necessary for storing and manipulating data efficiently during the execution cycle.
* The `loading.rs` module contains the specific logic required to parse and load external binary `.um` and `.umz` programs into the machine's memory segments.
* `bitpack`: Handles the mathematical extraction and packing of bit fields from the 32-bit instruction words to properly decode operation codes and register identifiers.
* `Cargo.toml`: The package manifest containing the project's metadata and dependencies.

---

## Test Binaries
### rum-binaries-main/

This directory contains a collection of pre-compiled test packages to validate the functionality of the execution, memory, and instruction modules. <br>

&emsp; ***Available Test Programs***

* `advent.umz`: A compressed Universal Machine executable used for complex execution and state testing.
* `cat.um`: A basic Universal Machine program used to verify standard input and output functionality.
* `codex.umz`: An additional compressed Universal Machine executable to rigorously test instruction parsing and memory bounds.
* `midmark.um`: This is a smaller dataset used for initial profiling and quick performance checks. It allows developers to measure baseline execution times and identify bottlenecks
* `sandmark.umz1`: This is a comprehensive stress test that exercises more complex VM operations, including large array manipulations and aliasing checks. It is used to evaluate the final performance of the Universal Machine implementation after code tuning and optimization stages

---

## How To Run
**IMPORTANT: Ensure you have a working Rust environment.**

* Navigate to the root directory containing the `Cargo.toml` configuration file.
* The program accepts command-line arguments to load and execute a provided Universal Machine binary from the test folder.
* To run an executable, compile and execute the project using Cargo: `cargo run [filename.um]` or `cargo run [filename.umz]`.

---

## Contribution
* **Authors:** Zach Breene and C. Wyatt Polasek
* **Design Methodology:** This project utilized a modular architecture approach in Rust, implementing and validating each component (memory, registers, instructions) individually before combining them into the main execution loop. Testing emphasized ensuring the segmented memory and registers flawlessly handled the unpacking and execution of Universal Machine instructions.
