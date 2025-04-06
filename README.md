# tiny-kv

Redis clone with a gRPC interface and reflection support.

Inspired by [tinykv](https://github.com/talent-plan/tinykv/blob/course/doc/project1-StandaloneKV.md)

## Setup

- Install [`protoc`](https://protobuf.dev/installation/)

## Build

```cmd
cargo build
```

## Features

- In-memory data storage
- Reflection support
  - allows the server to shared the `grpc` contract with clients.
