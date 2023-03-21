# rust-simplified-blockchain

  Simple example for building a blockchain in Rust

  https://user-images.githubusercontent.com/77025415/226765487-3b2cba10-2b44-4745-9667-c0c2fe0953c4.mov

  Developed by: <a href="https://www.github.com/gabrielAlonsoCabral">@GabrielAlonsoCabral</a>  
 <br/>


USAGE

```bash
RUST_LOG=info cargo run
```

This starts the client locally. The blockchain is not persisted anywhere.

You can start it in multiple terminals to get multiple connected peer-to-peer clients.

In each client, you can enter the following commands:

* `ls p` - list peers
* `ls c` - print local chain
* `create b $data` - `$data` is just a string here - this creates (mines) a new block with the data entry `$data` and broadcasts it
* `EXAMPLE $data: rust_blockchain > Peer Id: 12D3KooWSXGZJJEnh3tndGEVm6ACQ5pdaPKL34ktmCsUqkqSVTWX` 

Once a block is created by a node, it's broadcasted and the blockchain in all other nodes is updated (if it's a valid block).

On startup, a node asks another node on the network for their blockchain and, if it's valid and longer than the current local blockchain, it updates it's own chain to the longest one it receives.


This is a VERY overly simplified, offline-running, highly inefficient and insecure blockchain implementation.
DO NOT USE THIS FOR PRODUCTION CASES!
