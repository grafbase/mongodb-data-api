#!/usr/bin/env bash

curl --request POST \
  'http://127.0.0.1:3000/app/data-test/endpoint/data/v1/action/updateOne' \
  --header 'apiKey: TEST' \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data-raw '{
      "dataSource": "grafbase",
      "database": "test",
      "collection": "test",
      "filter": { "_id": { "$eq": { "$oid": "64d5dda2154ceb5e6cfdf94e" } } },
      "update": { "$set": { "foo": "musti" } }
  }' |jq
