# Rooky Chess Protocol

Rooky is a protocol for sharing chess games and annotations using the Nostr protocol. 
It allows users to publish, share, and view chess games in a decentralized and secure manner.

Rooky extends NIP-64, using PGN format for the chess data interoperability.

## Features 

### Rooky Core

The main library `rooky-core` provides the common structures and definition to create 
Nostr messages with PGN  content. It uses the `shakmaty` and `pgn-reader` crates to provide 
an easy data structure to parse, manage, and serialize chess games.

### ChessBoard JS

This include WebAssembly bindings for the ChessboardJS library, used to display  interactive 
chess boards on the application. WebAssembly allows for high-performance rendering of chess boards in the browser.


### External

This defines a set of helper functions to interact with external APIs from sites like `lichess` and `Chess.com`.
Uses browser native code to create efficient streams of games to avoid overhead on the connections.

## Technologies

`Rooky` is built on Rust and leverages the `nostro2` to create easy Nostr structures that can be used with 
both software signers and signing extensions.

The `RookyGame` structures support full `serde` serializations to ensure easy compatibility with other 
systems and libraries.

The `RookyGame` struct also uses the `shakmaty` crate to provide a rich set of chess functionalities, including 
checking the legality of the moves before applying them, and managing and interacting with the game state.

The `RookyGame` can be converted to a Nostr note for interaction with the network, or can produce the raw PGN 
if needed, eg for text downloads.

## Installation

To use Rooky in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
rooky-core = "0.1"
```

## 📜 Contribution Guide
Thank you for your interest in contributing to Rooky! We follow a **GitFlow forking model**, similar to how [Bitcoin](https://github.com/bitcoin/bitcoin/blob/master/CONTRIBUTING.md) development works. Below is the process for contributing.

### 1. Understanding the Contribution Model
This project follows a GitFlow forking model, inspired by how contributions are made to Bitcoin Core. Instead of pushing directly to the main repository, contributors fork the repository and submit changes via pull requests (PRs). Maintainers review, test, and merge contributions accordingly.

### 2. Prerequistes
Ensure you have the following tools installed:
- [Git](https://git-scm.com/) ⚙️
- [Rust & Cargo](https://www.rust-lang.org/tools/install) 🦀

### 3. Setting up your Fork
1. **Fork the Repository: Navigate to the main repository and click the Fork button.**
2. **Clone your Fork**
   ```sh
   git clone https://github.com/your-username/repository-name.git
   cd repository-name
   ```
3. **Add the Upstream Remote:**
   ```sh
   git remote add upstream https://github.com/satsangatech/rooky-chess-protocol
   ```
4. **Fetch upstream changes**
   ```sh
   git fetch upstream
   ```

### 4. Creating a new feature branch
1. **Ensure you are on the latest develop branch:**
   ```sh
   git checkout develop
   git pull upstream develop
   ```
2. **Create a new branch for your feature or fix:**
   ```sh
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes and commit them:**
   ```sh
   git add .
   git commit -m "Describe your change concisely"
   ```
4. **Push your changes to your fork**
   ```sh
   git push origin feature/your-feature-name
   ```
   
### 5. Submitting a Pull Request
1. Navigate to your fork on GitHub.
2. Click on the New Pull Request button.
3. Select develop as the base branch and your feature/your-feature-name branch as the compare branch.
4. Provide a clear title and description for your pull request.
5. Click Create Pull Request.

### 6. Review Process
- Code Review: Maintainers and contributors will review your PR, suggesting necessary changes.
- Rebasing: If changes occur in develop before your PR is merged, rebase your branch:
  ```sh
  git fetch upstream
  git rebase upstream/develop
  git push --force
  ```
- Final Merging: Once approved, a maintainer will merge your PR.

### 7. Keeping your fork updated
To keep your local fork in sync with the upstream repository:
```sh
git checkout develop
git pull upstream develop
```

### 8. Contribution Guidelines
- Follow coding standards and best practices.
- Provide clear commit messages.
- Write tests if applicable.
- Be patient and responsive during the review process.
---

## 🔗 Useful Links

- 📜 Official Rust Docs: [Rust Documentation](https://doc.rust-lang.org/)
- 🔑 Nostr Protocol: [Nostr](https://nostr.com/)
- 🔑 Nostr-PGN: [NIP-64](https://github.com/nostr-protocol/nips/blob/master/64.md)
- 🔑 Chessboard JS: [Docs](https://chessboardjs.com/index.html)
- 💰 Bitcoin Contributions: [Bitcoin Contributing](https://github.com/bitcoin/bitcoin/blob/master/CONTRIBUTING.md)

