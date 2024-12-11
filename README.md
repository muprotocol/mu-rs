# Mu Protocol

**Mu Protocol** is a decentralized cloud framework that empowers developers to build, test, and deploy applications on blockchain (Web3) using familiar Web2 technologies. Written in Rust, Mu Protocol delivers high performance and reliability for decentralized application development.


## Features

- **Rust-Powered Performance**: Built with Rust for speed, safety, and concurrency.
- **Seamless Web2 to Web3 Transition**: Use familiar tools and workflows to interact with decentralized infrastructure.
- **Decentralized Cloud Framework**: Host and manage applications on a distributed network with high availability.
- **End-to-End Development**: Simplify your Web3 app lifecycle from building to deploying.


## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (version 1.70+ recommended)
- [Cargo](https://doc.rust-lang.org/cargo/) (included with Rust)
- [Docker](https://www.docker.com/) (for local development)
- Basic knowledge of Rust or blockchain concepts (optional but helpful)

### Installation

1. Install with Cargo:
```
cargo install mu-cli
```


## CLI Overview

Mu Protocol includes a powerful CLI to manage your project. Here's a quick look at its usage:

```
Usage: mu <COMMAND>

Commands:
  init      Initialize a new project
  function  Work with Mu functions
  frontend  Work with Mu frontends
  build     Build the project
  deploy    Deploy the project
  dev       Run the project in development mode
  help      Print this message or the help of the given subcommand(s)
```


## Quick Start Guide

### Writing Mu Functions

Functions are the core building blocks in Mu Protocol. To create and deploy a simple "Hello, World" function:

1. Initialize a New Project:
```
mu init my-web3-app
cd my-web3-app
```

2. Add a new function:
```
mu function add hello-world
```

3. Open the generated file in the `functions/` directory and implement your logic in Rust.

4. Test your function locally:
```
mu dev
```

5. Deploy your function:
```
mu deploy
```

### Working with Frontends

Mu Protocol supports integrating Web2 frontends with decentralized backends. Use the `frontend` command to manage your frontend codebase.

## License

Mu Protocol is open-source software licensed under the [Apache 2.0 License](./LICENSE).

## Community & Support

- **Discord**: Join our community for help and discussions.
- **GitHub Issues**: Report bugs or request features.
- **Twitter**: Follow us for the latest updates.

Happy Building with Mu Protocol! 🚀
