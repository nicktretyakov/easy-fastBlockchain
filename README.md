# Easy Fast Blockchain

A simple and efficient blockchain implementation designed for educational and experimental purposes. This project consists of two main components:

- **gRPC Backend**: A Rust-based server that implements core blockchain functionalities using gRPC for communication.
- **Web Client**: A React+TypeScript web application powered by Vite and gRPC-Web that allows interaction with the blockchain.

---

## Table of Contents

1. [Features](#features)
2. [Architecture](#architecture)
3. [Prerequisites](#prerequisites)
4. [Getting Started](#getting-started)
   - [Clone the Repository](#clone-the-repository)
   - [Setup gRPC Backend](#setup-grpc-backend)
   - [Setup Web Client](#setup-web-client)
5. [Usage](#usage)
6. [Project Structure](#project-structure)
7. [Contributing](#contributing)
8. [License](#license)

---

## Features

- **Core Blockchain Operations**: Create and add new blocks, manage transactions, and verify chain integrity.
- **UTXO Model & Wallets**: Supports unspent transaction outputs (UTXO) and wallet generation.
- **Proof-of-Work**: Simple PoW algorithm for block validation.
- **gRPC API**: High-performance Rust backend exposing gRPC endpoints (`AddBlock`, `GetBlock`, `GetBlockchain`).
- **Web Client**: React interface using gRPC-Web to interact with the blockchain in real-time.
- **Lightweight & Educational**: Minimal setup with clear code to learn blockchain internals.

---

## Architecture

```text
┌─────────────────┐       gRPC       ┌──────────────────────┐
│   Web Client    │ <--------------> │   Rust gRPC Backend  │
│  (React + TS)   │                  │  (tonic + tokio)     │
└─────────────────┘                  └──────────────────────┘
```  

- The **backend** listens on `localhost:50051`, compiles `.proto` definitions at build time, and serves blockchain services with CORS support via `tonic-web`.
- The **client** generates TypeScript stubs from the same `.proto` file, runs on `localhost:5173` (Vite default), and communicates with the backend over gRPC-Web.

---

## Prerequisites

- **Rust & Cargo** (version ≥ 1.60)
- **Protobuf Compiler** (`protoc`)
- **Node.js & npm** (version ≥ 14)

---

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/nicktretyakov/easy-fastBlockchain.git
cd easy-fastBlockchain
```

### Setup gRPC Backend

1. Navigate into the backend folder:

   ```bash
   cd grpc-backend
   ```

2. Install Rust dependencies and build the project (compiles `.proto` via `build.rs`):

   ```bash
   cargo build --release
   ```

3. Run the gRPC server:

   ```bash
   cargo run --release
   ```

   The server will start on **`[::1]:50051`**.

### Setup Web Client

1. In a new terminal, navigate to the client folder:

   ```bash
   cd client
   ```

2. Install npm dependencies:

   ```bash
   npm install
   ```

3. Generate TypeScript bindings from the `.proto` definitions:

   ```bash
   npm run proto:generate
   ```

4. Start the development server:

   ```bash
   npm run dev
   ```

   The client will be available at **http://localhost:5173**.

---

## Usage

- **Add a Block**: Use the web interface to submit block data. The backend handles PoW and appends the block.
- **View Blockchain**: Real-time stream of existing blocks is displayed in the client.
- **Inspect a Block**: Query a block by its hash to view detailed properties (timestamp, nonce, transactions, etc.).

Feel free to explore the TypeScript or Rust service calls under `src/services` and `src/grpc-api`.

---

## Project Structure

```text
easy-fastBlockchain/
├── client/             # React + TypeScript web client
│   ├── public/         # Static assets
│   ├── src/            # React components & gRPC-Web code
│   ├── package.json
│   └── vite.config.ts
├── grpc-backend/       # Rust gRPC backend
│   ├── proto/          # Protobuf definitions
│   ├── src/            # Service implementations & core blockchain logic
│   ├── build.rs        # Compile-time proto compiler
│   └── Cargo.toml
└── README.md           # Project overview (this file)
```  

---

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch: `git checkout -b feature/my-feature`
3. Make your changes and add tests.
4. Commit and push: `git push origin feature/my-feature`
5. Open a pull request with a clear description of your changes.

Please ensure your code follows existing style and includes documentation.

---

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

