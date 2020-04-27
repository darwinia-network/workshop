# Basic flow and bug on local testnet
## Download the docker image
`docker pull yanganto/darwinia-workshop`

## Run the docker image and export the ports we use
`docker run -p 9933:9933 -p 9944:9944 yanganto/darwinia-workshop`

### What is runing in the container
- darwinia binary on local testnet
  `/darwinia --dev -leoc=trace  --base-path /tmp/darwinia-develop/alice`


## Insert your SR255519 keys for the offchain worker
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

# How to relay on the crab network
1. Start darwinia without dev lag
  `/darwinia`
1. Insert your key for babe and grandpa
```
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d \
'{
  "jsonrpc":"2.0",
  "id":1,
  "method":"babe",
  "params": [
    "rlwk",
    "tomorrow ritual harsh grab admit jewel slice raw subject open rather uncover",
    "0x70bf51d123581d6e51af70b342cac75ae0a0fc71d1a8d388719139af9c042b18"
  ]
}' 
```
```
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d \
'{
  "jsonrpc":"2.0",
  "id":1,
  "method":"babe",
  "params": [
    "rlwk",
    "tomorrow ritual harsh grab admit jewel slice raw subject open rather uncover",
    "0x70bf51d123581d6e51af70b342cac75ae0a0fc71d1a8d388719139af9c042b18"
  ]
}' 
```
