# Local MongoDB Atlas Data API

A small server serving MongoDB Atlas Data API for test usage.
Please do not use in production, skips everything else except
things we might need in the CI. Super unsafe, probably destroys
everything.

It's not even done yet.

## Usage

```console
Usage: local-atlas-data-api [OPTIONS] --mongodb-url <MONGODB_URL>

Options:
      --hostname <HOSTNAME>        [default: 127.0.0.1]
      --port <PORT>                [default: 3000]
      --mongodb-url <MONGODB_URL>  
  -h, --help                       Print help
  -V, --version                    Print version
```
