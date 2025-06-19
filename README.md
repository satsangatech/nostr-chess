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
