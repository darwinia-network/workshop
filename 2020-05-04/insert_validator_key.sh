#!/usr/bin/bash

sleep 2

curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d \
'{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "babe",
    "tomorrow ritual harsh grab admit jewel slice raw subject open rather uncover",
    "0x70bf51d123581d6e51af70b342cac75ae0a0fc71d1a8d388719139af9c042b18"
  ]
}'

curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d \
'{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "gran",
    "tomorrow ritual harsh grab admit jewel slice raw subject open rather uncover",
    "0x4285ec25e8f094c108c93415be4fa712e4e1c70f0f642d51af54694c494dd060"
  ]
}'
