#!/usr/bin/env bash

curl --request POST \
  'http://127.0.0.1:3000/app/data-test/endpoint/data/v1/action/updateMany' \
  --header 'apiKey: TEST' \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data-raw '{
      "dataSource": "grafbase",
      "database": "test",
      "collection": "test",
      "filter": { "foo": { "$eq": "lol" } },
      "update": { "$set": { "foo": "rofl" } }
  }' |jq
