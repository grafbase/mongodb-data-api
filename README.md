# MongoDB Atlas Data API

A small server serving MongoDB Atlas Data API for test usage.
Connects to MongoDB using TCP and provides an HTTP API, which
might be useful if TCP is not available.

Please do not use in production, skips everything else except
things we might need in the CI. Super unsafe, probably destroys
everything.

## Usage

```console
Usage: mongodb-data-api [OPTIONS] --mongodb-url <MONGODB_URL>

Options:
      --hostname <HOSTNAME>        [default: 127.0.0.1]
      --port <PORT>                [default: 3000]
      --mongodb-url <MONGODB_URL>  
  -h, --help                       Print help
  -V, --version                    Print version
```

To connect to a MongoDB instance, pass the URL as a parameter to
the API:

```console
> mongodb-data-api --mongodb-url "mongodb://grafbase:grafbase@localhost:27018"
```

This connects to the MongoDB defined in the provided docker-compose
file. When the service is running (the default address is http:/127.0.0.1:3000),
you can start sending requests to it.

## Docker

The repository contains an example docker-compose file for using the API together with a local
MongoDB. The image can be found from [Docker Hub](https://hub.docker.com/repository/docker/grafbase/mongodb-data-api/).

### Find One

[Official documentation](https://www.mongodb.com/docs/atlas/app-services/data-api/openapi/#operation/findOne)

```console
> curl --request POST \
  'http://127.0.0.1:3000/app/data-test/endpoint/data/v1/action/findOne' \
  --header 'apiKey: TEST' \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data-raw '{
      "dataSource": "grafbase",
      "database": "test",
      "collection": "test",
      "projection": { "_id": 1, "foo": 1 },
      "filter": { "_id": { "$oid": "64d50a0b967f134bfb3fb620" } }
  }'
```

### Find

[Official documentation](https://www.mongodb.com/docs/atlas/app-services/data-api/openapi/#operation/find)

```console
> curl --request POST \
  'http://127.0.0.1:3000/app/data-test/endpoint/data/v1/action/find' \
  --header 'apiKey: TEST' \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data-raw '{
      "dataSource": "grafbase",
      "database": "test",
      "collection": "test",
      "projection": { "_id": 1, "foo": 1 },
      "filter": { "foo": { "$eq": "lol" } }
  }'
```

### Insert One

[Official documentation](https://www.mongodb.com/docs/atlas/app-services/data-api/openapi/#operation/insertOne)

```console
> curl --request POST \
  'http://127.0.0.1:3000/app/data-test/endpoint/data/v1/action/insertOne' \
  --header 'apiKey: TEST' \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data-raw '{
      "dataSource": "grafbase",
      "database": "test",
      "collection": "test",
      "document": { "foo": "lol" }
  }'
```

### Insert Many

[Official documentation](https://www.mongodb.com/docs/atlas/app-services/data-api/openapi/#operation/insertMany)

```console
> curl --request POST \
  'http://127.0.0.1:3000/app/data-test/endpoint/data/v1/action/insertMany' \
  --header 'apiKey: TEST' \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data-raw '{
      "dataSource": "grafbase",
      "database": "test",
      "collection": "test",
      "documents": [{ "foo": "bar" }, { "foo": "lolbar" }]
  }'
```

### Update One

[Official documentation](https://www.mongodb.com/docs/atlas/app-services/data-api/openapi/#operation/updateOne)

```console
> curl --request POST \
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
  }'
```

### Update Many

[Official documentation](https://www.mongodb.com/docs/atlas/app-services/data-api/openapi/#operation/updateMany)

```console
> curl --request POST \
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
  }'
```

### Delete One

[Official documentation](https://www.mongodb.com/docs/atlas/app-services/data-api/openapi/#operation/deleteOne)

```console
> curl --request POST \
  'http://127.0.0.1:3000/app/data-test/endpoint/data/v1/action/deleteOne' \
  --header 'apiKey: TEST' \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data-raw '{
      "dataSource": "grafbase",
      "database": "test",
      "collection": "test",
      "filter": { "_id": { "$oid": "64d50a0b967f134bfb3fb620" } }
  }'
```

### Delete Many

[Official documentation](https://www.mongodb.com/docs/atlas/app-services/data-api/openapi/#operation/deleteMany)

```console
> curl --request POST \
  'http://127.0.0.1:3000/app/data-test/endpoint/data/v1/action/deleteMany' \
  --header 'apiKey: TEST' \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data-raw '{
      "dataSource": "grafbase",
      "database": "test",
      "collection": "test",
      "filter": { "foo": { "$eq": "rofl" } }
  }'
```

### Aggregate

[Official documentation](https://www.mongodb.com/docs/atlas/app-services/data-api/openapi/#operation/aggregate)

We don't really need this (yet). It's implemented in this service, but not tested.

### Drop Database

This is an unofficial command, but useful for cleaning up after a test is run. Drops the database defined in the request.

```console
> curl --request POST \
  'http://127.0.0.1:3000/app/data-test/endpoint/data/v1/action/dropDatabase' \
  --header 'apiKey: TEST' \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data-raw '{
      "dataSource": "grafbase",
      "database": "test"
  }'
```
