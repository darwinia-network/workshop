Hands On Notes for Darwinia Bridge Relayer
---
This the notes for workshop, please refer the [slides](https://slides.com/yanganto/darwinia-chain-relay-workshop/#/)

# Basic flow on local testnet
## Download the docker image
`docker pull yanganto/darwinia-workshop`

## Run the docker image and export the ports we use
`docker run -p 9933:9933 -p 9944:9944 darwinia-workshop`

### What is runing in the container
- darwinia binary on local testnet, and you already a validator
  `/darwinia --dev -leoc=trace  --base-path /tmp/darwinia-develop/alice`

## Let's know what's happend in detail
- get the container id then attach into
  - `docker container list`
  - `docker exec -it <container_id> bash`
  - The `-leoc=trace` will open the trace level logging for offchain worker
  - logs are saved in /var/log/supervisor
  - Comparing logs with [source code](https://github.com/darwinia-network/darwinia-common/blob/master/frame/bridge/eth/offchain/src/lib.rs) 

## What is the worker do?
  - Find the the best ethereum header record on chain
  - Do request to a shadow service
  - Relay and verify

## Setup a key for local shadow service
`export INFURA_KEY=https://mainnet.infura.io/v3/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`

## Insert your SR255519 keys for the offchain worker
 - Here is the action to provide the identity to the relayer
 - such that the relayer will start relay as the identity
  ```
  curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d \
  '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"author_insertKey",
    "params": [
      "rlwk",
      "tomorrow ritual harsh grab admit jewel slice raw subject open rather uncover",
      "0x70bf51d123581d6e51af70b342cac75ae0a0fc71d1a8d388719139af9c042b18"
    ]
  }' 
  ```

## Use Darwinia Public Shadow Service
 - test service connection
  ```
  curl http://107.167.191.203:4001 -H "Content-Type:application/json;charset=utf-8" -d \
  '{
    id: 1,
    jsonrpc: "2.0",
    method: "shadow_getEthHeaderWithProofByNumber",
    params: {"block_num": 9966784, "transaction": false}
  }' 
  ```

## Refernce
- [The relayer incentive model](https://github.com/darwinia-network/darwinia-common/pull/108)
- Shadow service solutions
  - [darwinia.js](https://github.com/darwinia-network/darwinia.js)
  - [darwinia.go](https://github.com/darwinia-network/darwinia.go)
