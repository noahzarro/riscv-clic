# `riscv`

> Low level access to RISC-V processors

This repository was forked from [this repository][repo].
This fork adds support for the CLIC peripheral and the CLIC CSRs.

## Features

This repository provides:
- Access to CRSs like `mstatus` or `mcause`
- Access to core peripherals like the CLIC
- Wrappers to assembly instructions like `WFI`

## Usage

Most functions can be accessed directly, however for a few peripheral functions that change internal state, a peripheral singleton has to be acquired first.
This can be done by calling `Peripherals::take();`.

[repo]: [https://github.com/rust-embedded/riscv]
