# StakeNet API

## Overview
This API can be useful for any website that wants to show historical information on validator performance!

## Getting started

### Build for release

```bash
cargo b --release --bin jito-stakenet-api
```

### Check available options

```bash
./target/release/jito-stakenet-api --help

# Usage: jito-stakenet-api [OPTIONS]
# 
# Options:
#       --bind-addr <BIND_ADDR>
#           Bind address for the server [env: BIND_ADDR=] [default: 0.0.0.0:7001]
#       --json-rpc-url <JSON_RPC_URL>
#           RPC url [env: JSON_RPC_URL=] [default: https://api.mainnet-beta.solana.com]
#       --validator-history-program-id <VALIDATOR_HISTORY_PROGRAM_ID>
#           Validator history program ID (Pubkey as base58 string) [env: VALIDATOR_HISTORY_PROGRAM_ID=] [default: HistoryJTGbKQD2mRgLZ3XhqHnN811Qpez8X9kCcGHoa]
#   -h, --help
#           Print help
#   -V, --version
#           Print version
```

### Run API

```bash
./target/release/jito-stakenet-api
```

## Examples

```bash
cargo r -p jito-stakenet-api --example cache
```
