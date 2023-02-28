#!/bin/bash

while ! nc -z localhost 9933; do
  sleep 1
done

curl -H "Content-Type: application/json" -d'{"id":1, "jsonrpc":"2.0", "method": "rpc_methods", "params":[]}' http://localhost:9933 