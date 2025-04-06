# tiny-kv

Redis clone with a gRPC interface. Reflection is supported.

Inspired by [tinykv](https://github.com/talent-plan/tinykv/blob/course/doc/project1-StandaloneKV.md)

## Setup

- Install [`protoc`](https://protobuf.dev/installation/)

## Build

```cmd
cargo build
```

## Features

- Reflection support
  - allows the server to shared the `grpc` contract with clients.
