# tiny-kv

Redis clone with a gRPC interface and reflection support.

Inspired by [tinykv](https://github.com/talent-plan/tinykv/blob/course/doc/project1-StandaloneKV.md)

## Setup

- Install [`protoc`](https://protobuf.dev/installation/)

  ```cmd
  sudo apt install -y protobuf-compiler
  ```

- Install build essentials

  ```cmd
  sudo apt install build-essential
  ```

## Build

```cmd
cargo build
```

## Testing

Run the service

```cmd
cargo run
```

Test the service by using [`grpcui`](https://github.com/fullstorydev/grpcui)

```cmd
grpcui -plaintext localhost:50051
```

## Features

- In-memory data storage
- Reflection support
  - allows the server to shared the `grpc` contract with clients.
