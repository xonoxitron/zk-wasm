# ZK-WASM

## Introduction

ZK-WASM is a software project that implements Zero Knowledge Proofs (ZKP) in Rust, compiled to WebAssembly (WASM). It provides a JavaScript bridge within an HTML file to interact with the ZK-WASM functionalities. This documentation provides an overview of the project structure, its technical specifications, and usage instructions.

## Prerequisites

### Build Script and wasm-pack Installation

The build script `build.sh` simplifies the process of building the ZK-WASM project and generating the required WebAssembly artifacts. By installing `wasm-pack`, you gain access to the `wasm-pack` command, which is used to build the project with the specified target and output directory. Following the provided steps will allow you to build the project and generate the necessary artifacts successfully.

The `build.sh` script is used to build the ZK-WASM project and generate the necessary WebAssembly artifacts. The script contains the following command:

```
wasm-pack build --target web --out-dir ./bridge
```

This command uses `wasm-pack` to build the project with the target set to `web` and output the generated artifacts to the `./bridge` directory.

To use the build script, follow these steps:

1. Open a terminal or command prompt.
2. Navigate to the root directory of the ZK-WASM project.
3. Run the `build.sh` script by executing the following command:

   ```
   sh build.sh
   ```

4. The script will execute the `wasm-pack` command and generate the necessary WebAssembly artifacts in the `./bridge` directory.

### wasm-pack Installation

To use the `wasm-pack` command in the build script, you need to have `wasm-pack` installed on your system. If you haven't installed it already, you can follow these steps:

1. Open a terminal or command prompt.
2. Run the following command to install `wasm-pack` using `cargo`, the package manager for Rust:

   ```
   cargo install wasm-pack
   ```

3. Wait for the installation to complete. Once installed, you can verify the installation by running the following command:

   ```
   wasm-pack --version
   ```

   If the installation was successful, the command will display the version number of `wasm-pack`.

Note: Make sure you have Rust and Cargo installed before installing `wasm-pack`, as it relies on these tools for the compilation process.

## Dependencies (deps)

The ZK-WASM project relies on several Rust libraries/dependencies to implement its functionality. This section provides an overview of the key dependencies used in the project.

### bellman

`bellman` is a Rust library that provides a high-level API for building zero-knowledge proof systems. It includes implementations of various proof systems, such as Groth16 and Bulletproofs. In the ZK-WASM project, `bellman` is used for proof generation, parameter generation, and proof verification. It offers functionalities like creating random proofs, generating random parameters, preparing verifying keys, and verifying proofs.

### ff

`ff` is a Rust library that provides finite field arithmetic operations for various prime fields. It offers implementations of prime field elements and related operations like addition, multiplication, exponentiation, and more. In the ZK-WASM project, `ff` is used for working with prime field elements, representing scalar values, and performing arithmetic operations on them.

### pairing

`pairing` is a Rust library that implements elliptic curve pairings for various elliptic curves. It provides support for operations on elliptic curve groups, such as point addition, scalar multiplication, and pairing computations. In the ZK-WASM project, `pairing` is used for working with the BN256 elliptic curve and performing operations on its group elements.

### sapling-crypto

`sapling-crypto` is a Rust library that implements cryptographic primitives for the Sapling protocol, which is used in the Zcash cryptocurrency. It includes functionalities like babyjubjub elliptic curve operations, Pedersen hashes, and more. In the ZK-WASM project, `sapling-crypto` is used for operations related to the babyjubjub elliptic curve, fixed-base multiplication, and Pedersen hashing.

These libraries provide essential building blocks and abstractions for implementing zero-knowledge proof systems in Rust. They handle low-level cryptographic operations, field arithmetic, elliptic curve operations, and other necessary functionalities required for zero-knowledge proof generation and verification.

By utilizing these dependencies, the ZK-WASM project can leverage the existing implementations and abstractions to achieve efficient and secure zero-knowledge proofs within the Rust programming language.

Please note that the provided information is based on the available knowledge up to September 2021, and there might have been updates or new versions of these dependencies since then. It's always recommended to refer to the official documentation and project repositories for the most up-to-date information and usage details.

## Technical Specifications

### Rust Implementation

The core implementation of ZK-WASM is written in Rust, with the following dependencies:

- `bellman`: A Rust library for building zk-SNARK circuits.
- `pairing`: A Rust library for pairing-based cryptography.
- `sapling_crypto`: A Rust library for implementing the Sapling elliptic curve cryptography.
- `wasm_bindgen`: A Rust library for generating WebAssembly bindings.

The main Rust file `lib.rs` contains the implementation of the zero knowledge proof circuits and the JavaScript interop code.

### WebAssembly Compilation

ZK-WASM is compiled to WebAssembly (WASM) to enable its execution in web browsers. The compiled code can be loaded and executed on the client-side.

### JavaScript Bridge

The HTML file `zk-wasm.html` includes a JavaScript bridge that connects the ZK-WASM functionality with the user interface. The JavaScript code imports the generated JavaScript interop code and binds the functions to the window object. It provides methods for generating keys, proving knowledge, and verifying proofs.

## Usage Instructions

### Generate Keys

To generate keys, follow these steps:

1. Open the `zk-wasm.html` file in a web browser.
2. Click on the "Generate" button.
3. The generated key will be displayed in the "key" text area, along with its size in kilobytes.

### Prove Knowledge

To prove knowledge using the generated keys, follow these steps:

1. Enter the value of `x` (in hexadecimal format) in the "x" text input field.
2. Click on the "Prove" button.
3. The resulting `proof` and `h` value will be displayed in the respective text areas.

### Verify Proof

To verify a proof using the generated keys, follow these steps:

1. Enter the `h` value and `proof` in their respective text input and text areas.
2. Click on the "Verify" button.
3. The result of the verification (true or false) will be displayed.

Note: Make sure to generate the keys before attempting to prove or verify.

## Conclusion

ZK-WASM is a software project that enables zero knowledge proofs in a web browser environment. It leverages Rust for the core implementation, compiles to WebAssembly, and provides a JavaScript bridge for interaction with the user interface. The provided usage instructions guide users through the process of generating keys, proving knowledge, and verifying proofs.
