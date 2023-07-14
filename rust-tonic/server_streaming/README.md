# Rust Implementation using Tonic gRPC library

To learn more about Tonic, see [tonic](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md).

You will need to install Rust. See [rustup](https://rustup.rs/).

To run the client:

```bash
$ cargo run  --release --bin client
```

To run the server:

```bash
$ cargo run  --release --bin server
```

While the speed of the client is impressive, the speed of the server is very slow compared to other languages.

Even I just send a very simple response, the speed is very slow:

```rust
let data = vec![0; bytes_to_send as usize];
```