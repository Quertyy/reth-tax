# Reth token tax

Having recently taken an interest in Rust and Reth, I decided to take advantage of its modularity to add some custom code. 
This code adds a new custom eth_tokenTax api to the rpc eth namespace.
At the moment, only token/weth pairs on UniswapV2 are supported.

### Acknowledgements
This project is based on the [Univ2-Tri-Arb repo from duoxehyon](https://github.com/duoxehyon/univ2-tri-arb) for retrieving univ2 token taxes, as well as on the examples provided in the [reth repo](https://github.com/paradigmxyz/reth).

### Build
```rust
cargo run -- node --http --http.api web3,eth,trace
```

And call the method with [cast](https://github.com/foundry-rs/foundry) or curl:

```bash
cast rpc eth_tokenTax '["0xaC1419Ee74F203C6b9DAa3635ad7169b7ebb5C1A","0x1396D6F2e9056954DFc2775204bB3e2Eb8ab8a5B","0x1151CB3d861920e07a38e03eEAd12C32178567F6","0x72e4f9F808C49A2a61dE9C5896298920Dc4EEEa9"]' | jq
```

```json
[
  {
    "Success": {
      "token": "0xac1419ee74f203c6b9daa3635ad7169b7ebb5c1a",
      "buy": "0x5",
      "sell": "0x5"
    }
  },
  {
    "Success": {
      "token": "0x1396d6f2e9056954dfc2775204bb3e2eb8ab8a5b",
      "buy": "0x1",
      "sell": "0x1"
    }
  },
  {
    "CallError": {
      "PairAddressDoesNotExist": "0x1151cb3d861920e07a38e03eead12c32178567f6"
    }
  },
  {
    "Success": {
      "token": "0x72e4f9f808c49a2a61de9c5896298920dc4eeea9",
      "buy": null,
      "sell": null
    }
  }
]
```
