# Enhance-the-performance-of-the-TFHE-rs-library

## Overview

This project focuses on enhancing the performance of the (TFHE-rs) library, a Rust implementation of the TFHE (Fast Fully Homomorphic Encryption) scheme. 
The enhancements target cryptographic primitives, including programmable bootstrapping and key switching, as well as integer operations such as multiplication 
and division for large integers (64 bits and above).

## Objectives

- Optimize cryptographic primitives for improved efficiency.
- Enhance integer operations using advanced algorithms.
- Maintain high security standards (at least 128 bits) and low error probabilities (≤ 2^(-40)).
- Provide clear documentation and benchmarking for performance evaluation.

## Features

- **Efficient Programmable Bootstrapping**: Improved performance through parallel processing and SIMD instructions.
- **Optimized Key Switching**: Enhanced key switching operations leveraging AVX512.
- **Fast Integer Operations**: Implementation of Karatsuba multiplication and Newton-Raphson division algorithms.
- **Comprehensive Benchmarking**: Tools to measure accuracy and performance on AWS m6i.metal instances.

## Requirements

- Rust (version 1.56 or later)
- Cargo (Rust package manager)
- AVX512 compatible hardware (for optimized performance)

## Installation

To get started with the TFHE-rs Performance Enhancement project, clone the repository and build the project:

```bash
git clone https://github.com/yourusername/tfhe-rs-performance-enhancement.git
cd tfhe-rs-performance-enhancement
cargo build --release


